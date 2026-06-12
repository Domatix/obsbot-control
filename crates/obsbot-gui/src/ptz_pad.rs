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
//! button grid (writing `pan_absolute` / `tilt_absolute` deltas), a
//! zoom slider (`zoom_absolute`), and a focus row pairing
//! `focus_automatic_continuous` with `focus_absolute`. Camera-class
//! PTZ-related controls are filtered out of the generic
//! `controls_view::render_controls` path; the IDs we consume are in
//! [`PTZ_PAD_IDS`].
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

use std::path::{Path, PathBuf};
use std::rc::Rc;

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
    default: i64,
    is_active: bool,
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
    {
        let owned_path = owned_path.clone();
        let owned_serial = owned_serial.clone();
        zoom_adj.connect_value_changed(move |adj| {
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

    // Focus row(s) — append below the pad if the camera advertises focus.
    if let Some(focus_abs) = find_int(controls, CID_FOCUS_ABSOLUTE) {
        let focus_auto = find_bool(controls, CID_FOCUS_AUTOMATIC_CONTINUOUS);
        let focus_row = build_focus_row(focus_abs, focus_auto.as_ref(), &owned_path, &owned_serial);
        group.add(&focus_row);
    }

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
        default,
    } = ctrl.kind
    {
        Some(IntRange {
            current,
            min,
            max,
            step,
            default,
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

/// Build a single `AdwExpanderRow`-style pair: an `AdwSwitchRow` for
/// `focus_automatic_continuous` plus an `AdwActionRow` with a slider
/// for `focus_absolute`. The slider greys out while auto is on.
fn build_focus_row(
    focus_abs: IntRange,
    focus_auto: Option<&BoolValue>,
    path: &Rc<PathBuf>,
    serial: &Rc<Option<String>>,
) -> adw::ExpanderRow {
    let expander = adw::ExpanderRow::builder()
        .title(gettext("Focus"))
        .subtitle(gettext("Auto-focus + manual focus distance (0–100)"))
        .build();

    let auto_row = adw::SwitchRow::builder()
        .title(gettext("Auto-focus"))
        .active(focus_auto.is_some_and(|b| b.current))
        .build();
    auto_row.set_sensitive(focus_auto.is_some_and(|b| b.is_active));
    if focus_auto.is_some() {
        settings::register_row(CID_FOCUS_AUTOMATIC_CONTINUOUS, &auto_row);
    }
    expander.add_row(&auto_row);

    let abs_row = build_focus_abs_row(focus_abs, path, serial);
    // Grey out the manual slider while auto is on (matches what the
    // kernel would mark INACTIVE; the explicit listener below stays
    // in place because the T-111 generic refresh acts on every
    // gate-write but the toggle here is fast-path local UX).
    abs_row.set_sensitive(focus_abs.is_active && !focus_auto.is_some_and(|b| b.current));
    settings::register_row(CID_FOCUS_ABSOLUTE, &abs_row);
    expander.add_row(&abs_row);

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

    expander
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
        .subtitle(format!(
            "range {}..={} default {}",
            focus_abs.min, focus_abs.max, focus_abs.default
        ))
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
    use super::next_position;

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
}
