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
use crate::i18n::gettext;
use crate::settings;

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
    let toast_overlay: adw::ToastOverlay = builder
        .object("toast_overlay")
        .expect("window.ui missing object 'toast_overlay'");
    let nav_view: adw::NavigationView = builder
        .object("nav_view")
        .expect("window.ui missing object 'nav_view'");
    let body_slot: adw::Bin = builder
        .object("body_slot")
        .expect("window.ui missing object 'body_slot'");

    window.set_application(Some(app));

    // T-207 / T-208: on window close, stop the preview stream and power
    // the camera down before quitting. The OBSBOT firmware ignores
    // Sleep for ~3 s after streaming, so we hide the window for an
    // instant-feeling close, keep the app alive briefly while the
    // camera powers down, then quit. Only wired with the `live-preview`
    // feature; without it the default close (destroy + quit) applies.
    #[cfg(feature = "live-preview")]
    {
        let app_for_close = app.clone();
        window.connect_close_request(move |win| {
            crate::preview::stop_active();
            crate::preview::cancel_deferred_sleep();
            if let Some(path) = crate::preview::active_camera_path() {
                // Hiding (not destroying) the window keeps the app alive
                // without an explicit hold, so the deferred sleep below
                // can run before we quit.
                win.set_visible(false);
                let app = app_for_close.clone();
                glib::timeout_add_seconds_local_once(4, move || {
                    crate::preview::send_sleep(&path);
                    app.quit();
                });
                return glib::Propagation::Stop;
            }
            glib::Propagation::Proceed
        });
    }

    // Window-level toast surface (T-108 / T-110): bound once and
    // reused for V4L2 write failures *and* hot-plug REMOVE notices.
    // Lives as long as the window so toasts dispatched right around
    // a page navigation never end up orphaned.
    settings::bind_toast_overlay(&toast_overlay);

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
                    // T-110: detect REMOVE events for the camera the
                    // user is currently looking at, pop the controls
                    // page, and surface a toast. Has to happen before
                    // the body re-mount so the visible_page_tag()
                    // lookup still sees the controls page.
                    handle_remove_events(&prev, &latest, &nav_view);
                    body_slot.set_child(Some(&build_body(&latest, &nav_view)));
                    *prev = latest;
                }
                glib::ControlFlow::Continue
            }
        ),
    );
}

/// Stable identity for hot-plug REMOVE matching (T-110). Pair the
/// USB `(vid, pid)` with the camera serial when available — the serial
/// distinguishes two same-model cameras, the vid/pid is the fallback
/// when the kernel hasn't surfaced a serial for the device yet.
fn camera_key(cam: &CameraInfo) -> (u16, u16, Option<String>) {
    (cam.vid, cam.pid, cam.serial.clone())
}

/// Surface a "Camera disconnected" toast and pop the
/// `Adw.NavigationView` back to the cameras list whenever a camera
/// that was previously in the enumeration is no longer present AND
/// the currently-visible page corresponds to that camera.
///
/// The page tag is `controls-{vid:04x}-{pid:04x}` (set by
/// `controls_view::build_controls_page`); two identical-model cameras
/// would push to the same tag, so the per-page disambiguation is
/// approximate. For v0.2 the assumption holds: the OBSBOT Tiny 2
/// family ships with one camera at a time on a given USB port.
fn handle_remove_events(
    prev: &[CameraInfo],
    latest: &[CameraInfo],
    nav_view: &adw::NavigationView,
) {
    let latest_keys: Vec<_> = latest.iter().map(camera_key).collect();
    let removed: Vec<&CameraInfo> = prev
        .iter()
        .filter(|cam| !latest_keys.contains(&camera_key(cam)))
        .collect();

    if removed.is_empty() {
        return;
    }

    // Pop the controls page if it belongs to a removed camera. The
    // `Adw.NavigationView` API exposes the visible page; we check
    // its tag against each removed camera's `controls-` tag.
    let visible_tag: Option<String> = nav_view
        .visible_page()
        .and_then(|page| page.tag().map(|s| s.to_string()));

    if let Some(tag) = visible_tag.as_deref() {
        for cam in &removed {
            let cam_tag = format!("controls-{:04x}-{:04x}", cam.vid, cam.pid);
            if tag == cam_tag {
                nav_view.pop_to_tag("cameras");
                break;
            }
        }
    }

    // Always surface a toast regardless of which page is visible —
    // even if the user is on the camera list, they want to know the
    // camera just vanished. The window-level overlay survives
    // navigation (T-108 / T-110 wiring in `window::build`).
    let products: Vec<String> = removed.iter().map(|c| c.product.clone()).collect();
    let msg = if products.len() == 1 {
        gettext("Camera disconnected: {product}").replace("{product}", &products[0])
    } else {
        gettext("Cameras disconnected: {products}").replace("{products}", &products.join(", "))
    };
    settings::surface_error(&msg);
}

/// Decide which body widget to mount based on the current enumeration.
fn build_body(cameras: &[CameraInfo], nav_view: &adw::NavigationView) -> gtk::Widget {
    if cameras.is_empty() {
        return adw::StatusPage::builder()
            .icon_name("camera-web-symbolic")
            .title(gettext("No OBSBOT cameras detected"))
            .description(gettext("Connect an OBSBOT Tiny 2 family camera via USB."))
            .build()
            .upcast();
    }

    let page = adw::PreferencesPage::new();
    let group = adw::PreferencesGroup::builder()
        .title(gettext("Connected cameras"))
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
    let video = cam
        .video_path
        .as_ref()
        .map_or_else(|| gettext("(no video node)"), |p| p.display().to_string());
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
