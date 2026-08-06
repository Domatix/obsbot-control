// SPDX-License-Identifier: GPL-3.0-or-later
//
// Copyright (C) 2026 Domatix and contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! PTZ pad widget (T-101; simplified to pure single-step by T-101d).
//!
//! A dedicated [`adw::PreferencesGroup`] hosting a 3×3 directional
//! button grid (writing `pan_absolute` / `tilt_absolute` deltas) and a
//! zoom slider (`zoom_absolute`). Focus (`focus_automatic_continuous` +
//! `focus_absolute`) was lifted into its own [`build_focus_group`] on
//! the Main tab in T-220, but its IDs stay in [`PTZ_PAD_IDS`] so they
//! remain filtered out of the generic `controls_view::render_controls`
//! path along with the pan/tilt/zoom IDs.
//!
//! ## Input model — one action, one move
//!
//! Each directional button is a plain `gtk::Button`: **one click =
//! exactly one [`PAN_TILT_STEP`] step** (5° per `PROTOCOL §2.2`'s
//! 3600-units-per-degree). The handler reads the kernel-current
//! `pan_absolute` / `tilt_absolute`, adds one signed step, clamps to
//! the advertised range, and writes the absolute target.
//!
//! Keyboard arrows mirror this: Left/Right pan, Up/Down tilt (Up =
//! camera looks up), `Home` recenters to 0/0. **One key-press event =
//! one step.** No press-and-hold, no continuous-motion timers, no
//! per-axis accumulators, no auto-repeat suppression — the earlier
//! (v0.3.x, T-101a/b/c) hold machinery was removed in T-101d because
//! it bugged out (sticky timers, drift). If the OS auto-repeats a held
//! key it simply issues more discrete single steps; nothing keeps
//! running on its own, so nothing can stick.
//!
//! The V4L2 `pan_speed` / `tilt_speed` continuous-motion controls are
//! not used — Tiny 2 Lite firmware 5.10 accepts them but does not act
//! on them (PROTOCOL §2.3 Q9). `zoom_continuous` is also not surfaced
//! (Q2 — driver reports out-of-range values).

use std::cell::{Cell, RefCell};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::Duration;

use gtk4 as gtk;
use libadwaita as adw;

use adw::prelude::*;
use obsbot_core::{read_control, ControlDescriptor, ControlKind, ControlValue};

use crate::i18n::gettext;
use crate::settings;

/// V4L2 Camera-class control IDs consumed by the PTZ pad.
const CID_PAN_ABSOLUTE: u32 = 0x009a_0908;
const CID_TILT_ABSOLUTE: u32 = 0x009a_0909;
const CID_FOCUS_ABSOLUTE: u32 = 0x009a_090a;
const CID_FOCUS_AUTOMATIC_CONTINUOUS: u32 = 0x009a_090c;
const CID_ZOOM_ABSOLUTE: u32 = 0x009a_090d;
const CID_ZOOM_CONTINUOUS: u32 = 0x009a_090f;
const CID_PAN_SPEED: u32 = 0x009a_0920;
const CID_TILT_SPEED: u32 = 0x009a_0921;

/// Control IDs the PTZ pad consumes (so [`controls_view::render_controls`]
/// can filter them out of the generic per-class render).
pub const PTZ_PAD_IDS: &[u32] = &[
    CID_PAN_ABSOLUTE,
    CID_TILT_ABSOLUTE,
    CID_FOCUS_ABSOLUTE,
    CID_FOCUS_AUTOMATIC_CONTINUOUS,
    CID_ZOOM_ABSOLUTE,
    CID_ZOOM_CONTINUOUS,
    CID_PAN_SPEED,
    CID_TILT_SPEED,
];

const PTZ_PAD_UI: &str = "/io/github/domatix/ObsbotCamControl/ptz-pad.ui";

/// How often the T-223 watchdog re-reads `zoom_absolute` while the lock
/// is engaged. Matches `window.rs`'s hot-plug cadence so the added
/// syscall load stays in the same order of magnitude as what the app
/// already does at idle.
const ZOOM_LOCK_POLL: Duration = Duration::from_secs(2);

/// Ticks between two liveness lines in the watchdog log (T-228).
/// 30 × 2 s = one line a minute while the lock is engaged.
///
/// This is not debugging for its own sake. Field testing showed the lock
/// does not stop the zoom the camera's own L-gesture triggers, and there
/// are two very different explanations: either the firmware moves
/// `zoom_absolute` and this watchdog is failing to correct it, or the
/// gesture zooms inside the ISP without ever touching the UVC control,
/// in which case watching that control cannot work no matter what.
/// Telling them apart needs a reading taken while the gesture happens.
/// Rather than ask someone to babysit `v4l2-ctl`, the watchdog records
/// what it sees: engage the lock, do the gesture, read the log. Zero
/// drifts next to a zoom that visibly happened is the answer.
const ZOOM_LOCK_LOG_EVERY: u32 = 30;

/// V4L2 pan/tilt units: 3600 units per degree per `PROTOCOL §2.2`.
const UNITS_PER_DEGREE: i64 = 3600;
/// Step size per directional click/keypress, in degrees.
const PAN_TILT_STEP_DEGREES: i64 = 5;
/// Step size per directional click/keypress, in raw V4L2 units.
const PAN_TILT_STEP: i64 = PAN_TILT_STEP_DEGREES * UNITS_PER_DEGREE;

/// Read-once snapshot of an integer control's range + current value.
#[derive(Debug, Clone, Copy)]
struct IntRange {
    current: i64,
    min: i64,
    max: i64,
    step: u64,
    is_active: bool,
}

/// State of the T-223 zoom lock for one mounted camera.
///
/// The Tiny 2 firmware moves `zoom_absolute` on its own while its
/// on-device auto-framing is running, which shows up mid-call as an
/// abrupt jump between a wide shot and a close-up (see issue #1). The
/// lock pins the control: the slider stops writing, and a watchdog puts
/// the pinned value back whenever the device has drifted away from it.
#[derive(Default)]
struct ZoomLock {
    /// `Some(value)` while engaged, `None` while released. Read by the
    /// adjustment handler on every slider move and by the watchdog on
    /// every tick, so it doubles as the watchdog's stop condition.
    pinned: Cell<Option<i64>>,
    /// Re-entrancy guard. Snapping the slider handle back to the pinned
    /// value re-enters `connect_value_changed`; without this the second
    /// entry would snap again and recurse.
    applying: Cell<bool>,
    /// `GLib` source id of the running watchdog. Held so re-engaging the
    /// lock replaces the timer instead of stacking a second one on top:
    /// a released watchdog only notices on its next tick, so a fast
    /// off-then-on would otherwise leave two polling the device.
    watchdog: RefCell<Option<glib::SourceId>>,
    /// Watchdog ticks since the lock was engaged, and drifts corrected
    /// in that time (T-228). Reported to stderr; see
    /// [`ZOOM_LOCK_LOG_EVERY`] for what question they answer.
    polls: Cell<u32>,
    drifts: Cell<u32>,
}

/// Decide what the watchdog must write, if anything.
///
/// Pure so the decision can be unit-tested without a device.
///
/// - Released lock (`pinned` is `None`) writes nothing.
/// - A failed read (`current` is `None`) writes nothing either. Same
///   reasoning as T-216: without a fresh reading of where the control
///   actually is, an absolute write is a guess, and a wrong guess on
///   this hardware is a visible jump.
/// - Otherwise the pinned value is written back only when the device
///   has actually drifted.
fn restore_target(current: Option<i64>, pinned: Option<i64>) -> Option<i64> {
    let pinned = pinned?;
    let current = current?;
    (current != pinned).then_some(pinned)
}

/// Build the PTZ pad widget for the given camera. Returns `None` when
/// the camera does not advertise the minimum trio (pan/tilt/zoom) —
/// non-PTZ cameras get no pad.
pub fn build_ptz_pad(
    controls: &[ControlDescriptor],
    path: &Path,
    serial: Option<&str>,
) -> Option<adw::PreferencesGroup> {
    let pan = find_int(controls, CID_PAN_ABSOLUTE)?;
    let tilt = find_int(controls, CID_TILT_ABSOLUTE)?;
    let zoom = find_int(controls, CID_ZOOM_ABSOLUTE)?;

    let builder = gtk::Builder::from_resource(PTZ_PAD_UI);
    let group: adw::PreferencesGroup = builder
        .object("ptz_group")
        .expect("ptz-pad.ui missing object 'ptz_group'");

    let owned_path: Rc<PathBuf> = Rc::new(path.to_path_buf());
    let owned_serial: Rc<Option<String>> = Rc::new(serial.map(str::to_owned));

    let ctx = DirectionCtx {
        pan,
        tilt,
        path: owned_path.clone(),
        serial: owned_serial.clone(),
    };
    // Eight directional buttons, each a plain one-click-one-step.
    for (button_id, dx, dy) in [
        ("btn_n", 0, 1),
        ("btn_s", 0, -1),
        ("btn_e", 1, 0),
        ("btn_w", -1, 0),
        ("btn_ne", 1, 1),
        ("btn_nw", -1, 1),
        ("btn_se", 1, -1),
        ("btn_sw", -1, -1),
    ] {
        wire_direction(&builder, button_id, dx, dy, &ctx);
    }

    let btn_reset: gtk::Button = builder
        .object("btn_reset")
        .expect("ptz-pad.ui missing object 'btn_reset'");
    {
        let owned_path = owned_path.clone();
        let owned_serial = owned_serial.clone();
        btn_reset.connect_clicked(move |_| {
            write(
                &owned_path,
                CID_PAN_ABSOLUTE,
                "pan_absolute",
                0,
                &owned_serial,
            );
            write(
                &owned_path,
                CID_TILT_ABSOLUTE,
                "tilt_absolute",
                0,
                &owned_serial,
            );
        });
    }

    // Zoom slider — bind its adjustment to write zoom_absolute.
    let zoom_scale: gtk::Scale = builder
        .object("zoom_scale")
        .expect("ptz-pad.ui missing object 'zoom_scale'");
    let zoom_adj = gtk::Adjustment::new(
        f64::from(clamp_i32(zoom.current)),
        f64::from(clamp_i32(zoom.min)),
        f64::from(clamp_i32(zoom.max)),
        f64::from(u32::try_from(zoom.step.max(1)).unwrap_or(u32::MAX)),
        f64::from(u32::try_from(zoom.step.max(1)).unwrap_or(u32::MAX)),
        0.0,
    );
    zoom_scale.set_adjustment(&zoom_adj);
    zoom_scale.set_sensitive(zoom.is_active);
    settings::register_row(CID_ZOOM_ABSOLUTE, &zoom_scale);

    // T-223: shared between the adjustment handler below (which stops
    // writing while engaged) and the watchdog wired in `wire_zoom_lock`.
    let zoom_lock = Rc::new(ZoomLock::default());
    {
        let owned_path = owned_path.clone();
        let owned_serial = owned_serial.clone();
        let zoom_lock = zoom_lock.clone();
        zoom_adj.connect_value_changed(move |adj| {
            // While the lock is engaged the slider is inert: snap the
            // handle back to the pinned value instead of writing. The
            // guard absorbs the re-entry that `set_value` causes.
            if let Some(pinned) = zoom_lock.pinned.get() {
                if !zoom_lock.applying.replace(true) {
                    adj.set_value(f64::from(clamp_i32(pinned)));
                    zoom_lock.applying.set(false);
                }
                return;
            }
            let value = i64::from(f64_to_i32_saturating(adj.value().round()));
            write(
                &owned_path,
                CID_ZOOM_ABSOLUTE,
                "zoom_absolute",
                value,
                &owned_serial,
            );
        });
    }

    wire_zoom_lock(
        &builder,
        &zoom_scale,
        &zoom_adj,
        &zoom_lock,
        &owned_path,
        &owned_serial,
    );

    // Focus moved to its own group on the Main tab (T-220); see
    // `build_focus_group`. The pad here is pan/tilt/zoom only.

    Some(group)
}

/// Wire the T-223 "Lock zoom" switch: pin state, slider sensitivity,
/// per-camera persistence, and the watchdog that undoes the camera's own
/// zoom changes.
///
/// Engaging pins the value the *device* reports, not the one the slider
/// shows. The two drift apart precisely when the firmware has been
/// moving the zoom on its own, which is the case this exists for.
///
/// Releasing leaves the zoom wherever it currently is and issues no
/// write: the user asked to stop holding it, not to move it.
fn wire_zoom_lock(
    builder: &gtk::Builder,
    zoom_scale: &gtk::Scale,
    zoom_adj: &gtk::Adjustment,
    lock: &Rc<ZoomLock>,
    path: &Rc<PathBuf>,
    serial: &Rc<Option<String>>,
) {
    let row: adw::SwitchRow = builder
        .object("lock_zoom_row")
        .expect("ptz-pad.ui missing object 'lock_zoom_row'");

    // T-228: read from the application-wide key, not from the per-camera
    // map. That map is keyed by USB serial and the Tiny 2 Lite reports
    // none (PROTOCOL.md §5), so the lock never survived a restart on the
    // hardware it was written for.
    let saved_on = settings::zoom_lock();

    {
        let lock = lock.clone();
        let zoom_scale = zoom_scale.clone();
        let zoom_adj = zoom_adj.clone();
        let path = path.clone();
        let serial = serial.clone();
        row.connect_active_notify(move |row| {
            let on = row.is_active();
            // Drop any previous watchdog before touching `pinned`, so a
            // released timer can never outlive its lock.
            if let Some(id) = lock.watchdog.borrow_mut().take() {
                id.remove();
            }
            if on {
                let pinned = current_axis(&path, CID_ZOOM_ABSOLUTE)
                    .unwrap_or_else(|| i64::from(f64_to_i32_saturating(zoom_adj.value().round())));
                lock.pinned.set(Some(pinned));
                lock.polls.set(0);
                lock.drifts.set(0);
                apply_pinned_to_slider(&zoom_adj, &lock, pinned);
                zoom_scale.set_sensitive(false);
                eprintln!("zoom lock: engaged, pinned at {pinned}");
                start_zoom_watchdog(&zoom_scale, &zoom_adj, &lock, &path, &serial);
            } else {
                lock.pinned.set(None);
                zoom_scale.set_sensitive(true);
                eprintln!(
                    "zoom lock: released after {} polls, {} drift(s) corrected",
                    lock.polls.get(),
                    lock.drifts.get(),
                );
            }
            settings::set_zoom_lock(on);
        });
    }

    // Replaying the saved state goes through the same closure as a user
    // click, so there is one engage path and not two.
    if saved_on {
        row.set_active(true);
    }
}

/// Move the slider handle to `pinned` without the adjustment handler
/// treating it as a user edit. No-op if a snap-back is already running.
fn apply_pinned_to_slider(adj: &gtk::Adjustment, lock: &Rc<ZoomLock>, pinned: i64) {
    if lock.applying.replace(true) {
        return;
    }
    adj.set_value(f64::from(clamp_i32(pinned)));
    lock.applying.set(false);
}

/// Start the watchdog that re-reads `zoom_absolute` while the lock is
/// engaged and writes the pinned value back when the device has drifted.
///
/// The widgets are captured weakly so the timer removes itself once the
/// controls page is replaced (camera switch, hot-plug, window close).
fn start_zoom_watchdog(
    zoom_scale: &gtk::Scale,
    zoom_adj: &gtk::Adjustment,
    lock: &Rc<ZoomLock>,
    path: &Rc<PathBuf>,
    serial: &Rc<Option<String>>,
) {
    let id = glib::timeout_add_local(
        ZOOM_LOCK_POLL,
        glib::clone!(
            #[weak]
            zoom_scale,
            #[weak]
            zoom_adj,
            #[strong]
            lock,
            #[strong]
            path,
            #[strong]
            serial,
            #[upgrade_or]
            glib::ControlFlow::Break,
            move || {
                let Some(pinned) = lock.pinned.get() else {
                    return glib::ControlFlow::Break;
                };
                // T-111's post-write refresh re-applies the kernel's
                // INACTIVE flag to every registered row, which would
                // hand the slider back to the user mid-lock. Re-assert.
                zoom_scale.set_sensitive(false);
                let current = current_axis(&path, CID_ZOOM_ABSOLUTE);
                lock.polls.set(lock.polls.get() + 1);
                if let Some(target) = restore_target(current, Some(pinned)) {
                    lock.drifts.set(lock.drifts.get() + 1);
                    eprintln!(
                        "zoom lock: drift, device at {} but pinned at {target}; writing it back",
                        current.map_or_else(|| "?".to_string(), |v| v.to_string()),
                    );
                    write(&path, CID_ZOOM_ABSOLUTE, "zoom_absolute", target, &serial);
                    apply_pinned_to_slider(&zoom_adj, &lock, target);
                }
                // T-228: a periodic line so "the watchdog never ran" and
                // "the watchdog ran and saw nothing move" stop looking
                // alike in a log.
                if lock.polls.get() % ZOOM_LOCK_LOG_EVERY == 0 {
                    eprintln!(
                        "zoom lock: alive, {} polls, {} drift(s), device reads {}",
                        lock.polls.get(),
                        lock.drifts.get(),
                        current.map_or_else(|| "unreadable".to_string(), |v| v.to_string()),
                    );
                }
                glib::ControlFlow::Continue
            }
        ),
    );
    *lock.watchdog.borrow_mut() = Some(id);
}

/// Build the standalone "Focus" group (T-220): an `AdwSwitchRow` for
/// `focus_automatic_continuous` (Auto-focus) plus an `AdwActionRow` with
/// a slider for `focus_absolute` (Manual focus). Returns `None` when the
/// camera does not advertise `focus_absolute`. Lifted out of the PTZ pad
/// so autofocus sits on the Main tab next to the AI controls instead of
/// under Move; the focus V4L2 IDs stay in [`PTZ_PAD_IDS`] so they remain
/// filtered out of the generic per-class render.
pub fn build_focus_group(
    controls: &[ControlDescriptor],
    path: &Path,
    serial: Option<&str>,
) -> Option<adw::PreferencesGroup> {
    let focus_abs = find_int(controls, CID_FOCUS_ABSOLUTE)?;
    let focus_auto = find_bool(controls, CID_FOCUS_AUTOMATIC_CONTINUOUS);

    let owned_path: Rc<PathBuf> = Rc::new(path.to_path_buf());
    let owned_serial: Rc<Option<String>> = Rc::new(serial.map(str::to_owned));

    let group = adw::PreferencesGroup::builder()
        .title(gettext("Focus"))
        .build();
    add_focus_rows(
        &group,
        focus_abs,
        focus_auto.as_ref(),
        &owned_path,
        &owned_serial,
    );
    Some(group)
}

/// Look up an Integer control by V4L2 ID and snapshot its range.
fn find_int(controls: &[ControlDescriptor], id: u32) -> Option<IntRange> {
    let ctrl = controls.iter().find(|c| c.id == id)?;
    if let ControlKind::Integer {
        current,
        min,
        max,
        step,
        ..
    } = ctrl.kind
    {
        Some(IntRange {
            current,
            min,
            max,
            step,
            is_active: ctrl.is_active,
        })
    } else {
        None
    }
}

#[derive(Debug, Clone, Copy)]
struct BoolValue {
    current: bool,
    is_active: bool,
}

fn find_bool(controls: &[ControlDescriptor], id: u32) -> Option<BoolValue> {
    let ctrl = controls.iter().find(|c| c.id == id)?;
    if let ControlKind::Boolean { current, .. } = ctrl.kind {
        Some(BoolValue {
            current,
            is_active: ctrl.is_active,
        })
    } else {
        None
    }
}

/// Shared state every directional button needs to mutate.
struct DirectionCtx {
    pan: IntRange,
    tilt: IntRange,
    path: Rc<PathBuf>,
    serial: Rc<Option<String>>,
}

/// Wire one directional button: a single click writes exactly one
/// [`PAN_TILT_STEP`] step on each non-zero axis. Keyboard activation
/// (Space / Enter on a focused button) takes the same path via the
/// standard "clicked" signal. No press-and-hold, no timers.
fn wire_direction(builder: &gtk::Builder, button_id: &str, dx: i64, dy: i64, ctx: &DirectionCtx) {
    let button: gtk::Button = builder
        .object(button_id)
        .unwrap_or_else(|| panic!("ptz-pad.ui missing object '{button_id}'"));
    button.set_sensitive(ctx.pan.is_active && ctx.tilt.is_active);

    let pan = ctx.pan;
    let tilt = ctx.tilt;
    let path = ctx.path.clone();
    let serial = ctx.serial.clone();

    button.connect_clicked(move |_| {
        step_axes(dx, dy, pan, tilt, &path, &serial);
    });
}

/// Apply one discrete step on each requested axis: read the kernel-
/// current position, add `sign * PAN_TILT_STEP`, clamp to range, write.
/// Shared by the on-screen buttons and the keyboard handler.
///
/// T-216 robustness: the step is computed **relative to a freshly-read
/// current position**. If that read fails we *skip the move* entirely
/// rather than fall back to the page-open snapshot. The old fallback
/// could slam the gimbal toward an extreme: the snapshot was taken at
/// page-build time (camera possibly asleep, gimbal parked at the
/// bottom), so writing `snapshot ± step` as an absolute target made the
/// camera lurch to the parked position — a large fast move that hangs
/// the firmware on the Tiny 2 (see PROGRESS T-216). A skipped move is
/// always safe; the next click retries the read.
fn step_axes(
    dx: i64,
    dy: i64,
    pan: IntRange,
    tilt: IntRange,
    path: &Rc<PathBuf>,
    serial: &Rc<Option<String>>,
) {
    if dx != 0 {
        if let Some(current) = current_axis(path, CID_PAN_ABSOLUTE) {
            let new_pan = next_position(current, dx, PAN_TILT_STEP, pan.min, pan.max);
            write(path, CID_PAN_ABSOLUTE, "pan_absolute", new_pan, serial);
        }
    }
    if dy != 0 {
        if let Some(current) = current_axis(path, CID_TILT_ABSOLUTE) {
            let new_tilt = next_position(current, dy, PAN_TILT_STEP, tilt.min, tilt.max);
            write(path, CID_TILT_ABSOLUTE, "tilt_absolute", new_tilt, serial);
        }
    }
}

/// Pure step arithmetic: `current + sign * step`, clamped to
/// `[min, max]`. Extracted so it can be unit-tested without a device.
fn next_position(current: i64, sign: i64, step: i64, min: i64, max: i64) -> i64 {
    (current + sign * step).clamp(min, max)
}

/// Just-in-time read of an integer axis from the kernel. Returns `None`
/// on any read failure or unexpected value type (T-216) — callers then
/// **skip** the move rather than write a stale absolute. A warning names
/// the failure so an unexpected skip is still traceable in logs.
fn current_axis(path: &Path, id: u32) -> Option<i64> {
    match read_control(path, id) {
        Ok(ControlValue::Integer(v)) => Some(v),
        Ok(other) => {
            eprintln!(
                "warning: ptz: read {id:#010x} returned non-integer {other:?}; skipping move"
            );
            None
        }
        Err(err) => {
            eprintln!("warning: ptz: read {id:#010x} failed: {err}; skipping move");
            None
        }
    }
}

/// Attach an `EventControllerKey` to `target` so the arrow keys + Home
/// drive the PTZ from the keyboard (T-101b, simplified by T-101d).
///
/// Mapping mirrors the on-screen pad: Left / Right pan, Up / Down tilt
/// (Up = camera looks up, matching `btn_n`), `Home` recenters to
/// pan = tilt = 0. **One key-press = one [`PAN_TILT_STEP`] step** — no
/// timers, no hold. Modifiers (Ctrl / Alt / Super) abort propagation so
/// app-level shortcuts (e.g. Ctrl+Q) keep working.
///
/// Propagation phase is **Bubble** so a focused `gtk::Scale` /
/// `gtk::SpinButton` / `adw::ComboRow` consumes the arrow first; the
/// controller only sees keys that reached the ancestry top without
/// being handled. Returns early (no controller) when the camera does
/// not advertise pan + tilt.
pub fn wire_keyboard_arrows<W>(
    target: &W,
    controls: &[ControlDescriptor],
    path: &Path,
    serial: Option<&str>,
) where
    W: IsA<gtk::Widget>,
{
    let Some(pan) = find_int(controls, CID_PAN_ABSOLUTE) else {
        return;
    };
    let Some(tilt) = find_int(controls, CID_TILT_ABSOLUTE) else {
        return;
    };

    let owned_path: Rc<PathBuf> = Rc::new(path.to_path_buf());
    let owned_serial: Rc<Option<String>> = Rc::new(serial.map(str::to_owned));

    let controller = gtk::EventControllerKey::new();
    controller.set_propagation_phase(gtk::PropagationPhase::Bubble);
    controller.connect_key_pressed(move |_, keyval, _keycode, state| {
        // Ctrl / Alt / Super → let app-level shortcuts (Ctrl+Q, etc.)
        // reach their handlers.
        if state.intersects(
            gtk::gdk::ModifierType::CONTROL_MASK
                | gtk::gdk::ModifierType::ALT_MASK
                | gtk::gdk::ModifierType::SUPER_MASK,
        ) {
            return glib::Propagation::Proceed;
        }

        let (dx, dy) = match keyval {
            gtk::gdk::Key::Left | gtk::gdk::Key::KP_Left => (-1, 0),
            gtk::gdk::Key::Right | gtk::gdk::Key::KP_Right => (1, 0),
            gtk::gdk::Key::Up | gtk::gdk::Key::KP_Up => (0, 1),
            gtk::gdk::Key::Down | gtk::gdk::Key::KP_Down => (0, -1),
            gtk::gdk::Key::Home | gtk::gdk::Key::KP_Home => {
                write(
                    &owned_path,
                    CID_PAN_ABSOLUTE,
                    "pan_absolute",
                    0,
                    &owned_serial,
                );
                write(
                    &owned_path,
                    CID_TILT_ABSOLUTE,
                    "tilt_absolute",
                    0,
                    &owned_serial,
                );
                return glib::Propagation::Stop;
            }
            _ => return glib::Propagation::Proceed,
        };

        step_axes(dx, dy, pan, tilt, &owned_path, &owned_serial);
        glib::Propagation::Stop
    });

    target.add_controller(controller);
}

/// Add the two focus rows directly to `group`: an `AdwSwitchRow` for
/// `focus_automatic_continuous` plus an `AdwActionRow` with a slider
/// for `focus_absolute`. The slider greys out while auto is on. Shown
/// flat (no `AdwExpanderRow`) so both rows are visible without a tap.
fn add_focus_rows(
    group: &adw::PreferencesGroup,
    focus_abs: IntRange,
    focus_auto: Option<&BoolValue>,
    path: &Rc<PathBuf>,
    serial: &Rc<Option<String>>,
) {
    let auto_row = adw::SwitchRow::builder()
        .title(gettext("Auto-focus"))
        .active(focus_auto.is_some_and(|b| b.current))
        .build();
    auto_row.set_sensitive(focus_auto.is_some_and(|b| b.is_active));
    if focus_auto.is_some() {
        settings::register_row(CID_FOCUS_AUTOMATIC_CONTINUOUS, &auto_row);
    }
    group.add(&auto_row);

    let abs_row = build_focus_abs_row(focus_abs, path, serial);
    // Grey out the manual slider while auto is on (matches what the
    // kernel would mark INACTIVE; the explicit listener below stays
    // in place because the T-111 generic refresh acts on every
    // gate-write but the toggle here is fast-path local UX).
    abs_row.set_sensitive(focus_abs.is_active && !focus_auto.is_some_and(|b| b.current));
    settings::register_row(CID_FOCUS_ABSOLUTE, &abs_row);
    group.add(&abs_row);

    {
        let abs_row = abs_row.clone();
        let path = path.clone();
        let serial = serial.clone();
        auto_row.connect_active_notify(move |row| {
            let value = row.is_active();
            write(
                &path,
                CID_FOCUS_AUTOMATIC_CONTINUOUS,
                "focus_automatic_continuous",
                i64::from(value),
                &serial,
            );
            abs_row.set_sensitive(!value);
        });
    }
}

fn build_focus_abs_row(
    focus_abs: IntRange,
    path: &Rc<PathBuf>,
    serial: &Rc<Option<String>>,
) -> adw::ActionRow {
    let current_i32 = clamp_i32(focus_abs.current);
    let min_i32 = clamp_i32(focus_abs.min);
    let max_i32 = clamp_i32(focus_abs.max);
    let step_u32 = u32::try_from(focus_abs.step.max(1)).unwrap_or(u32::MAX);

    let adjustment = gtk::Adjustment::new(
        f64::from(current_i32),
        f64::from(min_i32),
        f64::from(max_i32),
        f64::from(step_u32),
        f64::from(step_u32),
        0.0,
    );

    let scale = gtk::Scale::builder()
        .orientation(gtk::Orientation::Horizontal)
        .adjustment(&adjustment)
        .draw_value(false)
        .hexpand(true)
        .width_request(200)
        .valign(gtk::Align::Center)
        .build();
    scale.set_round_digits(0);

    let row = adw::ActionRow::builder()
        .title(gettext("Manual focus"))
        .activatable(false)
        .build();
    row.add_suffix(&scale);

    {
        let path = path.clone();
        let serial = serial.clone();
        adjustment.connect_value_changed(move |adj| {
            let value = i64::from(f64_to_i32_saturating(adj.value().round()));
            write(&path, CID_FOCUS_ABSOLUTE, "focus_absolute", value, &serial);
        });
    }

    row
}

/// Write a single integer control via [`settings::write_and_save`].
/// Persists the value if a serial is available; the V4L2 write is
/// authoritative either way.
fn write(path: &Rc<PathBuf>, id: u32, name: &str, value: i64, serial: &Rc<Option<String>>) {
    settings::write_and_save(
        path.as_path(),
        id,
        ControlValue::Integer(value),
        serial.as_deref(),
        name,
    );
}

/// Saturating-clamp an `i64` to `i32`. See `controls_view::clamp_i64_to_i32`
/// for the rationale — V4L2 standard control values are `__s32`.
fn clamp_i32(v: i64) -> i32 {
    if v > i64::from(i32::MAX) {
        i32::MAX
    } else if v < i64::from(i32::MIN) {
        i32::MIN
    } else {
        i32::try_from(v).unwrap_or(0)
    }
}

/// Saturating `f64 → i32` for slider read-back. Rust's `as i32` saturates
/// since 1.45, but clippy flags it; this wrapper documents intent.
#[allow(
    clippy::cast_possible_truncation,
    reason = "saturation is intentional: GtkAdjustment already clamps to [min, max]"
)]
fn f64_to_i32_saturating(v: f64) -> i32 {
    v as i32
}

#[cfg(test)]
mod tests {
    use super::{next_position, restore_target};

    // A Tiny 2 Lite advertises pan/tilt roughly ±130° = ±468000 units;
    // the exact bounds do not matter for the arithmetic, only that
    // `next_position` steps and clamps correctly.
    const MIN: i64 = -468_000;
    const MAX: i64 = 468_000;
    const STEP: i64 = 18_000; // 5° × 3600

    #[test]
    fn steps_positive_and_negative() {
        assert_eq!(next_position(0, 1, STEP, MIN, MAX), STEP);
        assert_eq!(next_position(0, -1, STEP, MIN, MAX), -STEP);
        assert_eq!(next_position(STEP, 1, STEP, MIN, MAX), 2 * STEP);
    }

    #[test]
    fn clamps_to_max() {
        assert_eq!(next_position(MAX - 1, 1, STEP, MIN, MAX), MAX);
        assert_eq!(next_position(MAX, 1, STEP, MIN, MAX), MAX);
    }

    #[test]
    fn clamps_to_min() {
        assert_eq!(next_position(MIN + 1, -1, STEP, MIN, MAX), MIN);
        assert_eq!(next_position(MIN, -1, STEP, MIN, MAX), MIN);
    }

    #[test]
    fn zero_sign_is_a_noop() {
        assert_eq!(next_position(1234, 0, STEP, MIN, MAX), 1234);
    }

    // ---- T-223 zoom lock ----

    #[test]
    fn released_lock_never_writes() {
        assert_eq!(restore_target(Some(40), None), None);
        // Even when the device has moved, a released lock stays quiet.
        assert_eq!(restore_target(Some(100), None), None);
    }

    #[test]
    fn drifted_zoom_is_pulled_back_to_the_pinned_value() {
        assert_eq!(restore_target(Some(90), Some(40)), Some(40));
        assert_eq!(restore_target(Some(0), Some(40)), Some(40));
    }

    #[test]
    fn zoom_already_at_the_pinned_value_writes_nothing() {
        assert_eq!(restore_target(Some(40), Some(40)), None);
        // Boundaries of the advertised 0..100 range (PROTOCOL §2.2).
        assert_eq!(restore_target(Some(0), Some(0)), None);
        assert_eq!(restore_target(Some(100), Some(100)), None);
    }

    #[test]
    fn failed_read_writes_nothing() {
        // Same rule as T-216: no fresh reading, no absolute write. A
        // guess here is a visible jump on this hardware.
        assert_eq!(restore_target(None, Some(40)), None);
        assert_eq!(restore_target(None, None), None);
    }
}
