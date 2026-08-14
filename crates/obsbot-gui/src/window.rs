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

//! Main `AdwApplicationWindow`.
//!
//! T-220 restructured the window from an `AdwNavigationView` drill-down
//! (a camera *list* root → a pushed per-camera controls page) into a
//! single page that lands directly on the connected camera's config
//! panel. The window owns one `AdwHeaderBar` (`header_bar`) and one body
//! slot (`body_slot`); `controls_view::build_controls_body` builds the
//! per-camera controls widget and installs the tab `AdwViewSwitcher` into
//! that header. When more than one camera is present a `Gtk.DropDown`
//! packed at the header start switches between them (hidden with ≤1). A
//! polling hot-plug listener keeps the dropdown and the mounted body in
//! sync with the live enumeration and surfaces a disconnect toast.

// gtk-rs idiom: alias the canonical crate names to their conventional
// short forms at the module level.
use gtk4 as gtk;
use libadwaita as adw;

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;

use adw::prelude::*;
use obsbot_core::{enumerate_cameras, CameraInfo};

use crate::controls_view::build_controls_body;
use crate::i18n::gettext;
use crate::settings;

/// Hot-plug poll interval. Two seconds matches GNOME Settings' rough
/// device-panel latency while keeping the sysfs syscall load trivial.
const POLL_INTERVAL: Duration = Duration::from_secs(2);

/// Path to the window's Blueprint-compiled UI inside the embedded
/// `GResource` (see `build.rs` + `resources/window.blp` + the prefix
/// in `resources/obsbot.gresource.xml`).
const WINDOW_UI: &str = "/io/github/domatix/obsbot-control/window.ui";

/// Shared state driving the single-page view: the live camera
/// enumeration, the index of the camera currently mounted, and a guard
/// that suppresses the dropdown's `selected_notify` while we update its
/// model/selection programmatically (so a hot-plug refresh does not
/// re-trigger a body re-mount).
struct ViewState {
    cameras: RefCell<Vec<CameraInfo>>,
    selected: Cell<usize>,
    suppress_dropdown: Cell<bool>,
}

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
    let header_bar: adw::HeaderBar = builder
        .object("header_bar")
        .expect("window.ui missing object 'header_bar'");
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
    // Lives as long as the window so toasts dispatched right around a
    // body re-mount (e.g. a camera switch) are never orphaned.
    settings::bind_toast_overlay(&toast_overlay);

    let state = Rc::new(ViewState {
        cameras: RefCell::new(enumerate_cameras()),
        selected: Cell::new(0),
        suppress_dropdown: Cell::new(false),
    });

    // Camera selector (T-220): packed at the header start, visible only
    // when more than one camera is connected. Selecting an entry mounts
    // that camera's config panel.
    let dropdown = gtk::DropDown::builder()
        .tooltip_text(gettext("Select camera"))
        .build();
    header_bar.pack_start(&dropdown);

    {
        let cams = state.cameras.borrow();
        refresh_dropdown(&dropdown, &cams, 0, &state.suppress_dropdown);
        mount_current(&header_bar, &body_slot, &cams, 0);
    }

    dropdown.connect_selected_notify(glib::clone!(
        #[weak]
        header_bar,
        #[weak]
        body_slot,
        #[strong]
        state,
        move |dd| {
            if state.suppress_dropdown.get() {
                return;
            }
            let Ok(idx) = usize::try_from(dd.selected()) else {
                return;
            };
            let cams = state.cameras.borrow();
            if idx >= cams.len() {
                return;
            }
            state.selected.set(idx);
            mount_current(&header_bar, &body_slot, &cams, idx);
        }
    ));

    start_hotplug_poll(&header_bar, &body_slot, &dropdown, &state);

    window
}

/// Install the polling source. Widgets are captured weakly so the timer
/// auto-removes itself (and drops the shared `state`) when the window
/// dies.
fn start_hotplug_poll(
    header_bar: &adw::HeaderBar,
    body_slot: &adw::Bin,
    dropdown: &gtk::DropDown,
    state: &Rc<ViewState>,
) {
    glib::timeout_add_local(
        POLL_INTERVAL,
        glib::clone!(
            #[weak]
            header_bar,
            #[weak]
            body_slot,
            #[weak]
            dropdown,
            #[strong]
            state,
            #[upgrade_or]
            glib::ControlFlow::Break,
            move || {
                let latest = enumerate_cameras();
                let mut cams = state.cameras.borrow_mut();
                if *cams != latest {
                    // T-110: surface a toast for any camera that left the
                    // enumeration since the previous tick.
                    notify_disconnects(&cams, &latest);

                    // Keep the mounted camera stable across the change:
                    // re-find the previously-selected camera by identity;
                    // fall back to the first if it is gone (or the list
                    // is now empty, in which case mount_current shows the
                    // "no cameras" StatusPage).
                    let prev_key = cams.get(state.selected.get()).map(camera_key);
                    *cams = latest;
                    let new_selected = prev_key
                        .and_then(|k| cams.iter().position(|c| camera_key(c) == k))
                        .unwrap_or(0);
                    state.selected.set(new_selected);

                    refresh_dropdown(&dropdown, &cams, new_selected, &state.suppress_dropdown);
                    mount_current(&header_bar, &body_slot, &cams, new_selected);
                }
                glib::ControlFlow::Continue
            }
        ),
    );
}

/// Repopulate the camera `Gtk.DropDown` from the current enumeration and
/// select `selected`, with the `suppress` guard held so the resulting
/// `selected_notify` does not re-mount the body. The dropdown is hidden
/// unless there is more than one camera to choose between (T-220).
fn refresh_dropdown(
    dropdown: &gtk::DropDown,
    cameras: &[CameraInfo],
    selected: usize,
    suppress: &Cell<bool>,
) {
    suppress.set(true);
    let labels: Vec<String> = cameras.iter().map(camera_label).collect();
    let label_refs: Vec<&str> = labels.iter().map(String::as_str).collect();
    let model = gtk::StringList::new(&label_refs);
    dropdown.set_model(Some(&model));
    if let Ok(sel) = u32::try_from(selected) {
        dropdown.set_selected(sel);
    }
    suppress.set(false);

    dropdown.set_visible(cameras.len() > 1);
}

/// Display label for a camera in the selector — its product name. Two
/// identical-model units would share a label; acceptable for now (the
/// supported hardware is a single Tiny 2 family unit, and selection is
/// by list position, not label).
fn camera_label(cam: &CameraInfo) -> String {
    cam.product.clone()
}

/// Mount the config panel for `cameras[selected]` (or the "no cameras"
/// `StatusPage` when the index is out of range, i.e. an empty
/// enumeration). Stops any active preview from the previously-mounted
/// camera first so its V4L2 capture node is released deterministically
/// before its body widget is dropped (T-207 / T-220).
fn mount_current(
    header_bar: &adw::HeaderBar,
    body_slot: &adw::Bin,
    cameras: &[CameraInfo],
    selected: usize,
) {
    #[cfg(feature = "live-preview")]
    crate::preview::stop_active();

    if let Some(cam) = cameras.get(selected) {
        let body = build_controls_body(cam, header_bar);
        body_slot.set_child(Some(&body));
    } else {
        header_bar.set_title_widget(None::<&gtk::Widget>);
        body_slot.set_child(Some(&empty_status()));
    }
}

/// The "no OBSBOT cameras detected" placeholder shown when the
/// enumeration is empty.
fn empty_status() -> gtk::Widget {
    adw::StatusPage::builder()
        .icon_name("camera-web-symbolic")
        .title(gettext("No OBSBOT cameras detected"))
        .description(gettext("Connect an OBSBOT Tiny 2 family camera via USB."))
        .build()
        .upcast()
}

/// Stable identity for hot-plug REMOVE matching (T-110). Pair the
/// USB `(vid, pid)` with the camera serial when available — the serial
/// distinguishes two same-model cameras, the vid/pid is the fallback
/// when the kernel hasn't surfaced a serial for the device yet.
fn camera_key(cam: &CameraInfo) -> (u16, u16, Option<String>) {
    (cam.vid, cam.pid, cam.serial.clone())
}

/// Surface a "Camera disconnected" toast for every camera that was in
/// the previous enumeration but is no longer present. The window-level
/// overlay (bound in [`build`]) keeps the toast visible across the body
/// re-mount that follows.
fn notify_disconnects(prev: &[CameraInfo], latest: &[CameraInfo]) {
    let latest_keys: Vec<_> = latest.iter().map(camera_key).collect();
    let removed: Vec<&CameraInfo> = prev
        .iter()
        .filter(|cam| !latest_keys.contains(&camera_key(cam)))
        .collect();

    if removed.is_empty() {
        return;
    }

    let products: Vec<String> = removed.iter().map(|c| c.product.clone()).collect();
    let msg = if products.len() == 1 {
        gettext("Camera disconnected: {product}").replace("{product}", &products[0])
    } else {
        gettext("Cameras disconnected: {products}").replace("{products}", &products.join(", "))
    };
    settings::surface_error(&msg);
}
