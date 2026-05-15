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

//! PTZ pad widget (T-101).
//!
//! A dedicated [`adw::PreferencesGroup`] that hosts a 3×3 directional
//! button grid (writing `pan_absolute` / `tilt_absolute` deltas), a
//! vertical zoom slider (`zoom_absolute`), and a focus row pairing
//! `focus_automatic_continuous` with `focus_absolute`. Camera-class
//! PTZ-related controls are filtered out of the generic
//! `controls_view::render_controls` path; the IDs we consume are
//! listed in [`PTZ_PAD_IDS`].
//!
//! Per `PROTOCOL §2.2`, the V4L2 step for pan/tilt is `3600` units per
//! degree; we move 5° per click (= `18_000` units) to keep the pad
//! responsive without being twitchy. `zoom_continuous` is intentionally
//! not surfaced (PROTOCOL §2.3 quirk Q2 — driver reports values
//! exceeding the advertised range).
//!
//! The pan/tilt button handlers **re-read the kernel-current position
//! before every step** rather than tracking a local cache. The
//! cache-based approach used in the initial v0.2 implementation
//! drifted whenever the camera moved itself between clicks (AI
//! tracking landing on a face, preset recall, the on-device gesture),
//! producing the symptom the user observed in T-303 validation:
//! pressing "up" four times would sometimes move down, then snap to
//! the top once the stale cache finally caught up with reality.
//! Each click now issues `VIDIOC_G_EXT_CTRLS` for the controlling
//! axis, computes `clamp(current + step, min, max)`, and writes
//! that absolute target. Two ioctls per click is acceptable for
//! discrete PTZ — the smooth/continuous joystick mode is a v0.3.1
//! follow-up (T-101a) that will drive `pan_speed` / `tilt_speed`
//! while a button is held.

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
/// Step size per directional click, in degrees.
const PAN_TILT_STEP_DEGREES: i64 = 5;
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
    // Eight directional buttons.
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

/// Wire one directional button to a (`pan_delta`, `tilt_delta`) write.
/// `dx` / `dy` are the per-click step direction in units of
/// [`PAN_TILT_STEP_DEGREES`]; they multiply into V4L2 raw units before
/// writing.
///
/// Reads the current `pan_absolute` / `tilt_absolute` from the kernel
/// on every click (see the module-level doc-block for the rationale)
/// and falls back to the descriptor's snapshot value only if the read
/// itself errors — that way a transient read failure does not freeze
/// the pad.
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
        if dx != 0 {
            let current = current_axis(&path, CID_PAN_ABSOLUTE, pan.current);
            let new_pan = (current + dx * PAN_TILT_STEP).clamp(pan.min, pan.max);
            write(&path, CID_PAN_ABSOLUTE, "pan_absolute", new_pan, &serial);
        }
        if dy != 0 {
            let current = current_axis(&path, CID_TILT_ABSOLUTE, tilt.current);
            let new_tilt = (current + dy * PAN_TILT_STEP).clamp(tilt.min, tilt.max);
            write(&path, CID_TILT_ABSOLUTE, "tilt_absolute", new_tilt, &serial);
        }
    });
}

/// Just-in-time read of an integer axis from the kernel, falling back
/// to `snapshot` (the value read at page-open time) on any error. The
/// fallback keeps the pad usable on a transient read failure; the
/// next click will retry the kernel read.
fn current_axis(path: &Path, id: u32, snapshot: i64) -> i64 {
    match read_control(path, id) {
        Ok(ControlValue::Integer(v)) => v,
        _ => snapshot,
    }
}

/// Build a single `AdwExpanderRow`-style pair: an `AdwSwitchRow` for
/// `focus_automatic_continuous` plus an `AdwActionRow` with a slider
/// for `focus_absolute`. The slider greys out while auto is on
/// (preview of the T-102 generic INACTIVE handler).
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
