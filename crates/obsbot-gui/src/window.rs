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

//! Main `AdwApplicationWindow`: an `AdwNavigationView` rooting on the
//! camera list. T-013a builds the list once at startup; T-013b adds a
//! polling hot-plug listener that re-mounts the list body when the
//! enumeration changes; T-013c makes each row activatable and pushes
//! the V4L2 control sub-page built by `controls_view::build_controls_
//! page` on activation.

// gtk-rs idiom: alias the canonical crate names to their conventional
// short forms at the module level.
use gtk4 as gtk;
use libadwaita as adw;

use std::cell::RefCell;
use std::time::Duration;

use adw::prelude::*;
use obsbot_core::{enumerate_cameras, CameraInfo};

use crate::controls_view::build_controls_page;

/// Hot-plug poll interval. Two seconds matches GNOME Settings' rough
/// device-panel latency while keeping the sysfs syscall load trivial.
const POLL_INTERVAL: Duration = Duration::from_secs(2);

/// Path to the window's Blueprint-compiled UI inside the embedded
/// `GResource` (see `build.rs` + `resources/window.blp` + the prefix
/// in `resources/obsbot.gresource.xml`).
const WINDOW_UI: &str = "/io/github/domatix/ObsbotCamControl/window.ui";

/// Build the top-level window. Mounts the initial body and starts the
/// hot-plug polling source bound to the window's lifetime.
pub fn build(app: &adw::Application) -> adw::ApplicationWindow {
    let builder = gtk::Builder::from_resource(WINDOW_UI);
    let window: adw::ApplicationWindow = builder
        .object("window")
        .expect("window.ui missing object 'window'");
    let nav_view: adw::NavigationView = builder
        .object("nav_view")
        .expect("window.ui missing object 'nav_view'");
    let body_slot: adw::Bin = builder
        .object("body_slot")
        .expect("window.ui missing object 'body_slot'");

    window.set_application(Some(app));

    let initial = enumerate_cameras();
    body_slot.set_child(Some(&build_body(&initial, &nav_view)));

    start_hotplug_poll(&body_slot, &nav_view, initial);

    window
}

/// Install the polling source. The slot is captured weakly so the timer
/// auto-removes itself when the window (and therefore the slot) dies.
fn start_hotplug_poll(
    body_slot: &adw::Bin,
    nav_view: &adw::NavigationView,
    initial: Vec<CameraInfo>,
) {
    let snapshot = RefCell::new(initial);
    glib::timeout_add_local(
        POLL_INTERVAL,
        glib::clone!(
            #[weak]
            body_slot,
            #[weak]
            nav_view,
            #[upgrade_or]
            glib::ControlFlow::Break,
            move || {
                let latest = enumerate_cameras();
                let mut prev = snapshot.borrow_mut();
                if *prev != latest {
                    body_slot.set_child(Some(&build_body(&latest, &nav_view)));
                    *prev = latest;
                }
                glib::ControlFlow::Continue
            }
        ),
    );
}

/// Decide which body widget to mount based on the current enumeration.
fn build_body(cameras: &[CameraInfo], nav_view: &adw::NavigationView) -> gtk::Widget {
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
        group.add(&camera_row(cam, nav_view));
    }
    page.add(&group);
    page.upcast()
}

/// Render a single camera entry as an activatable `AdwActionRow` that
/// pushes the V4L2 detail page when tapped.
fn camera_row(cam: &CameraInfo, nav_view: &adw::NavigationView) -> adw::ActionRow {
    let video = cam.video_path.as_ref().map_or_else(
        || String::from("(no video node)"),
        |p| p.display().to_string(),
    );
    let subtitle = format!("{:04x}:{:04x} · {video}", cam.vid, cam.pid);

    let row = adw::ActionRow::builder()
        .title(&cam.product)
        .subtitle(&subtitle)
        .activatable(true)
        .build();
    row.add_prefix(&gtk::Image::from_icon_name("camera-web-symbolic"));
    row.add_suffix(&gtk::Image::from_icon_name("go-next-symbolic"));

    let cam_owned = cam.clone();
    row.connect_activated(glib::clone!(
        #[weak]
        nav_view,
        move |_| {
            nav_view.push(&build_controls_page(&cam_owned));
        }
    ));

    row
}
