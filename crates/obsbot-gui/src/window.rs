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

//! Main `AdwApplicationWindow`: header bar plus an `adw::Bin` slot that
//! holds the camera-list body. T-013a builds the list once at startup;
//! T-013b adds a polling hot-plug listener that re-enumerates every
//! `POLL_INTERVAL_SECS` and re-mounts the body when the snapshot changes.
//! V4L2 control drill-down is T-013c.

// gtk-rs idiom: alias the canonical crate names to their conventional
// short forms at the module level.
use gtk4 as gtk;
use libadwaita as adw;

use std::cell::RefCell;
use std::time::Duration;

use adw::prelude::*;
use obsbot_core::{enumerate_cameras, CameraInfo};

/// Hot-plug poll interval. Two seconds matches GNOME Settings' rough
/// device-panel latency while keeping the sysfs syscall load trivial.
const POLL_INTERVAL: Duration = Duration::from_secs(2);

/// Build the top-level window. Mounts the initial body and starts the
/// hot-plug polling source bound to the window's lifetime.
pub fn build(app: &adw::Application) -> adw::ApplicationWindow {
    let header = adw::HeaderBar::new();
    let initial = enumerate_cameras();
    let body_slot = adw::Bin::new();
    body_slot.set_child(Some(&build_body(&initial)));
    body_slot.set_vexpand(true);

    let layout = gtk::Box::new(gtk::Orientation::Vertical, 0);
    layout.append(&header);
    layout.append(&body_slot);

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("Obsbot Cam Control")
        .default_width(720)
        .default_height(540)
        .content(&layout)
        .build();

    start_hotplug_poll(&body_slot, initial);

    window
}

/// Install the polling source. The slot is captured weakly so the timer
/// auto-removes itself when the window (and therefore the slot) dies.
fn start_hotplug_poll(body_slot: &adw::Bin, initial: Vec<CameraInfo>) {
    let snapshot = RefCell::new(initial);
    glib::timeout_add_local(
        POLL_INTERVAL,
        glib::clone!(
            #[weak]
            body_slot,
            #[upgrade_or]
            glib::ControlFlow::Break,
            move || {
                let latest = enumerate_cameras();
                let mut prev = snapshot.borrow_mut();
                if *prev != latest {
                    body_slot.set_child(Some(&build_body(&latest)));
                    *prev = latest;
                }
                glib::ControlFlow::Continue
            }
        ),
    );
}

/// Decide which body widget to mount based on the current enumeration.
fn build_body(cameras: &[CameraInfo]) -> gtk::Widget {
    if cameras.is_empty() {
        return adw::StatusPage::builder()
            .icon_name("camera-web-symbolic")
            .title("No OBSBOT cameras detected")
            .description("Connect an OBSBOT Tiny 2 family camera via USB.")
            .build()
            .upcast();
    }

    let page = adw::PreferencesPage::new();
    let group = adw::PreferencesGroup::builder()
        .title("Connected cameras")
        .build();
    for cam in cameras {
        group.add(&camera_row(cam));
    }
    page.add(&group);
    page.upcast()
}

/// Render a single camera entry as an `AdwActionRow`.
fn camera_row(cam: &CameraInfo) -> adw::ActionRow {
    let video = cam.video_path.as_ref().map_or_else(
        || String::from("(no video node)"),
        |p| p.display().to_string(),
    );
    let subtitle = format!("{:04x}:{:04x} · {video}", cam.vid, cam.pid);

    let row = adw::ActionRow::builder()
        .title(&cam.product)
        .subtitle(&subtitle)
        .build();
    row.add_prefix(&gtk::Image::from_icon_name("camera-web-symbolic"));
    row
}
