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

//! Read-only V4L2 control detail page (T-013c).
//!
//! Tapping an `AdwActionRow` in the camera list (see `window.rs`)
//! pushes the page built here onto the parent `AdwNavigationView`. The
//! page calls [`obsbot_core::read_controls`] synchronously on the main
//! thread (24-ish ioctls take ~100 ms on the user's hardware) and
//! renders the results as one `AdwPreferencesGroup` per V4L2 class.

use gtk4 as gtk;
use libadwaita as adw;

use adw::prelude::*;
use obsbot_core::{read_controls, CameraInfo, ControlClass, ControlDescriptor, ControlKind};

/// Build the detail `AdwNavigationPage` for one camera.
pub fn build_controls_page(cam: &CameraInfo) -> adw::NavigationPage {
    let toolbar = adw::ToolbarView::new();
    toolbar.add_top_bar(&adw::HeaderBar::new());
    toolbar.set_content(Some(&build_body(cam)));

    adw::NavigationPage::builder()
        .title(&cam.product)
        .child(&toolbar)
        .tag(format!("controls-{:04x}-{:04x}", cam.vid, cam.pid))
        .build()
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
        Ok(controls) => render_controls(&controls).upcast(),
        Err(err) => {
            error_status("Could not read V4L2 controls", &format!("{path:?}: {err}")).upcast()
        }
    }
}

fn render_controls(controls: &[ControlDescriptor]) -> adw::PreferencesPage {
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
        group.add(&control_row(ctrl));
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

fn control_row(ctrl: &ControlDescriptor) -> adw::ActionRow {
    let subtitle = match &ctrl.kind {
        ControlKind::Integer {
            current,
            min,
            max,
            step,
        } => format!("{current} · range {min}..={max} step {step}"),
        ControlKind::Boolean { current } => {
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
