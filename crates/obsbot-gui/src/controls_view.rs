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

//! V4L2 control detail page.
//!
//! Tapping an `AdwActionRow` in the camera list (see `window.rs`) pushes
//! the page built here onto the parent `AdwNavigationView`. The page
//! calls [`obsbot_core::read_controls`] synchronously on the main thread
//! (~100 ms on the user's hardware) and renders the results as one
//! `AdwPreferencesGroup` per V4L2 class. T-100 makes User-class
//! Integer / Boolean controls writable: integers use an `AdwActionRow`
//! with a `gtk::Scale` (drag-bar) suffix plus a live value label,
//! booleans use `AdwSwitchRow`. Camera-class and menu controls stay
//! read-only until their dedicated write paths (T-101 PTZ pad,
//! T-103 white balance, T-104 exposure, T-300+ vendor XU) land.

use std::path::Path;

use gtk4 as gtk;
use libadwaita as adw;

use adw::prelude::*;
use obsbot_core::{
    read_controls, write_control, CameraInfo, ControlClass, ControlDescriptor, ControlKind,
    ControlValue,
};

/// Path to the controls-view shell inside the embedded `GResource`
/// (see `build.rs` + `resources/controls-view.blp` +
/// `resources/obsbot.gresource.xml`'s prefix).
const CONTROLS_UI: &str = "/io/github/domatix/ObsbotCamControl/controls-view.ui";

/// Build the detail `AdwNavigationPage` for one camera.
pub fn build_controls_page(cam: &CameraInfo) -> adw::NavigationPage {
    let builder = gtk::Builder::from_resource(CONTROLS_UI);
    let page: adw::NavigationPage = builder
        .object("page")
        .expect("controls-view.ui missing object 'page'");
    let body_slot: adw::Bin = builder
        .object("body_slot")
        .expect("controls-view.ui missing object 'body_slot'");

    page.set_title(&cam.product);
    page.set_tag(Some(&format!("controls-{:04x}-{:04x}", cam.vid, cam.pid)));
    body_slot.set_child(Some(&build_body(cam)));

    page
}

fn build_body(cam: &CameraInfo) -> gtk::Widget {
    let Some(path) = cam.video_path.as_deref() else {
        return error_status("No video node", "This camera has no /dev/videoN path.").upcast();
    };

    match read_controls(path) {
        Ok(controls) if controls.is_empty() => error_status(
            "No controls exposed",
            "The driver returned an empty control list.",
        )
        .upcast(),
        Ok(controls) => render_controls(&controls, path).upcast(),
        Err(err) => {
            error_status("Could not read V4L2 controls", &format!("{path:?}: {err}")).upcast()
        }
    }
}

fn render_controls(controls: &[ControlDescriptor], path: &Path) -> adw::PreferencesPage {
    let page = adw::PreferencesPage::new();

    let mut user_group: Option<adw::PreferencesGroup> = None;
    let mut camera_group: Option<adw::PreferencesGroup> = None;
    let mut other_group: Option<adw::PreferencesGroup> = None;

    for ctrl in controls {
        let group = match ctrl.class {
            ControlClass::User => user_group.get_or_insert_with(|| make_group("User Controls")),
            ControlClass::Camera => {
                camera_group.get_or_insert_with(|| make_group("Camera Controls"))
            }
            _ => other_group.get_or_insert_with(|| make_group("Other")),
        };
        group.add(&control_row(ctrl, path));
    }

    for group in [&user_group, &camera_group, &other_group]
        .into_iter()
        .flatten()
    {
        page.add(group);
    }
    page
}

fn make_group(title: &str) -> adw::PreferencesGroup {
    adw::PreferencesGroup::builder().title(title).build()
}

/// Build a row for one control. User-class Integer controls get an
/// `AdwActionRow` with a [`gtk::Scale`] (drag-bar), a [`gtk::SpinButton`]
/// (precise manual entry), and a reset-to-default button as suffixes;
/// User-class Boolean controls get an [`AdwSwitchRow`]. Every other
/// shape stays a read-only [`AdwActionRow`] until its dedicated write
/// path lands.
fn control_row(ctrl: &ControlDescriptor, path: &Path) -> gtk::Widget {
    if ctrl.class == ControlClass::User {
        match &ctrl.kind {
            ControlKind::Integer {
                current,
                min,
                max,
                step,
                default,
            } => {
                return integer_scale_row(ctrl, *current, *min, *max, *step, *default, path)
                    .upcast()
            }
            ControlKind::Boolean { current, default } => {
                return boolean_switch_row(ctrl, *current, *default, path).upcast();
            }
            _ => {}
        }
    }
    readonly_action_row(ctrl).upcast()
}

fn integer_scale_row(
    ctrl: &ControlDescriptor,
    current: i64,
    min: i64,
    max: i64,
    step: u64,
    default: i64,
    path: &Path,
) -> adw::ActionRow {
    // V4L2 standard User-class Integer controls store values as
    // `__s32` (see `struct v4l2_control` in `linux/videodev2.h`).
    // `ControlKind::Integer` widens to i64 to also cover the rarer
    // `V4L2_CTRL_TYPE_INTEGER64`, but the User-class branch we are in
    // here only ever sees s32-shaped values, so the conversions below
    // are lossless in practice. Clamp via `clamp_i64_to_i32` so an
    // out-of-spec driver can not panic the UI thread.
    let current_i32 = clamp_i64_to_i32(current);
    let min_i32 = clamp_i64_to_i32(min);
    let max_i32 = clamp_i64_to_i32(max);
    let default_i32 = clamp_i64_to_i32(default);
    // V4L2 step is positive by construction; clamp to ≥1 so the
    // Adjustment never receives a zero step_increment.
    let step_u32 = u32::try_from(step.max(1)).unwrap_or(u32::MAX);

    let adjustment = gtk::Adjustment::new(
        f64::from(current_i32),
        f64::from(min_i32),
        f64::from(max_i32),
        f64::from(step_u32),
        f64::from(step_u32), // page increment matches step — no PageUp/Down jumps yet
        0.0, // page_size = 0 keeps `value <= upper` (not `value <= upper - page_size`)
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
    // Mark default position so users can see where the "reset" sits.
    scale.add_mark(f64::from(default_i32), gtk::PositionType::Bottom, None);

    let spin_button = gtk::SpinButton::builder()
        .adjustment(&adjustment)
        .climb_rate(f64::from(step_u32))
        .digits(0)
        .numeric(true)
        .width_chars(5)
        .valign(gtk::Align::Center)
        .build();

    let reset_button = gtk::Button::builder()
        .icon_name("edit-undo-symbolic")
        .tooltip_text(format!("Reset to default ({default_i32})"))
        .valign(gtk::Align::Center)
        .css_classes(vec!["flat"])
        .build();
    {
        let adj = adjustment.clone();
        let reset_to = f64::from(default_i32);
        reset_button.connect_clicked(move |_| {
            adj.set_value(reset_to);
        });
    }

    let row = adw::ActionRow::builder()
        .title(&ctrl.name)
        .subtitle(format!(
            "range {min}..={max} step {step} · default {default_i32}"
        ))
        .activatable(false)
        .build();
    row.add_suffix(&scale);
    row.add_suffix(&spin_button);
    row.add_suffix(&reset_button);

    let id = ctrl.id;
    let name = ctrl.name.clone();
    let owned_path = path.to_path_buf();
    adjustment.connect_value_changed(move |adj| {
        let value = f64_to_i32_saturating(adj.value().round());
        let value_i64 = i64::from(value);
        if let Err(err) = write_control(&owned_path, id, ControlValue::Integer(value_i64)) {
            eprintln!(
                "warning: failed to write {name} ({id:#010x}) = {value_i64} on {}: {err}",
                owned_path.display(),
            );
        }
    });

    row
}

/// Saturating-clamp an `i64` to `i32`. Used to project V4L2 control
/// values into the `gtk::Adjustment` (f64) domain without precision
/// loss: standard V4L2 control values are `__s32` so this is lossless
/// for every well-behaved driver, and saturates for the pathological
/// case rather than panicking.
fn clamp_i64_to_i32(v: i64) -> i32 {
    if v > i64::from(i32::MAX) {
        i32::MAX
    } else if v < i64::from(i32::MIN) {
        i32::MIN
    } else {
        // try_from is infallible here (range checked just above), but
        // we keep the conversion explicit to stay clippy-clean.
        i32::try_from(v).unwrap_or(0)
    }
}

/// Saturating `f64 → i32` for slider read-back. Rust's `as i32` from
/// float saturates by spec (since 1.45), but clippy flags the cast as
/// a possible truncation; this wrapper documents intent and keeps the
/// callsite annotation-free.
#[allow(
    clippy::cast_possible_truncation,
    reason = "saturation is intentional: GtkAdjustment already clamps to [min, max]"
)]
fn f64_to_i32_saturating(v: f64) -> i32 {
    v as i32
}

fn boolean_switch_row(
    ctrl: &ControlDescriptor,
    current: bool,
    default: bool,
    path: &Path,
) -> adw::SwitchRow {
    let row = adw::SwitchRow::builder()
        .title(&ctrl.name)
        .subtitle(if default { "default On" } else { "default Off" })
        .active(current)
        .build();

    let id = ctrl.id;
    let name = ctrl.name.clone();
    let owned_path = path.to_path_buf();
    row.connect_active_notify(move |row| {
        let value = row.is_active();
        if let Err(err) = write_control(&owned_path, id, ControlValue::Boolean(value)) {
            eprintln!(
                "warning: failed to write {name} ({id:#010x}) = {value} on {}: {err}",
                owned_path.display(),
            );
        }
    });

    row
}

fn readonly_action_row(ctrl: &ControlDescriptor) -> adw::ActionRow {
    let subtitle = match &ctrl.kind {
        ControlKind::Integer {
            current,
            min,
            max,
            step,
            ..
        } => format!("{current} · range {min}..={max} step {step}"),
        ControlKind::Boolean { current, .. } => {
            if *current {
                "Yes".to_owned()
            } else {
                "No".to_owned()
            }
        }
        ControlKind::Menu {
            current_label,
            options,
        } => format!("{current_label} · {} options", options.len()),
        ControlKind::Other(type_name) => format!("({type_name})"),
        _ => String::from("(unsupported)"),
    };

    adw::ActionRow::builder()
        .title(&ctrl.name)
        .subtitle(&subtitle)
        .build()
}

fn error_status(title: &str, description: &str) -> adw::StatusPage {
    adw::StatusPage::builder()
        .icon_name("dialog-error-symbolic")
        .title(title)
        .description(description)
        .build()
}
