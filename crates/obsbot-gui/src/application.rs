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

//! `adw::Application` bootstrap: wire `activate`, register actions, set
//! accelerators.

// gtk-rs idiom: alias the canonical crate names to their conventional
// short forms at the module level.
use gtk4 as gtk;
use libadwaita as adw;

use adw::prelude::*;
use gio::ActionEntry;

use crate::window;

/// Build the `adw::Application`, register actions, and enter the `GLib`
/// main loop.
pub fn run(app_id: &str) -> glib::ExitCode {
    // Register the embedded GResource bundle produced by build.rs
    // before any code tries to load `.ui` templates from it
    // (window.rs / controls_view.rs).
    gio::resources_register_include!("obsbot.gresource")
        .expect("failed to register embedded GResource");

    let app = adw::Application::builder()
        .application_id(app_id)
        .resource_base_path("/io/github/domatix/ObsbotCamControl/")
        .build();

    let app_id_owned = app_id.to_owned();
    app.connect_startup(move |app| {
        // Sets the icon for every window the app creates so GTK can
        // paint it in the Wayland window-list / X11 WM_HINTS even
        // before the user has run `meson install`. GNOME Shell still
        // resolves the overview icon via the `.desktop` file (T-009);
        // both paths converge on the same hicolor entry T-010 installs.
        gtk::Window::set_default_icon_name(&app_id_owned);
        register_actions(app, &app_id_owned);
    });

    app.connect_activate(|app| {
        let window = window::build(app);
        window.present();
    });

    app.run()
}

fn register_actions(app: &adw::Application, app_id: &str) {
    let quit = ActionEntry::builder("quit")
        .activate(|app: &adw::Application, _, _| app.quit())
        .build();
    // Owned copy captured by the about-action closure (action entries
    // outlive this function so the borrow needs `'static`).
    let app_id_for_about = app_id.to_owned();
    let about = ActionEntry::builder("about")
        .activate(move |app: &adw::Application, _, _| {
            present_about_dialog(app, &app_id_for_about);
        })
        .build();
    app.add_action_entries([quit, about]);
    app.set_accels_for_action("app.quit", &["<primary>q"]);
}

/// Build and present the application's About dialog (T-106).
///
/// Uses [`adw::AboutDialog`] (HIG-preferred since libadwaita 1.5 — adapts
/// to mobile and desktop form factors) over the legacy
/// `AdwAboutWindow`. Metadata is pulled from the workspace `Cargo.toml`
/// at compile time via `env!("CARGO_PKG_*")`; the acknowledgement
/// section credits the reverse-engineering work cited in
/// `docs/PROTOCOL.md` §0.
fn present_about_dialog(app: &adw::Application, app_id: &str) {
    let dialog = adw::AboutDialog::builder()
        .application_name("Obsbot Cam Control")
        .application_icon(app_id)
        .version(env!("CARGO_PKG_VERSION"))
        .developer_name(env!("CARGO_PKG_AUTHORS"))
        .copyright("© 2026 Domatix and contributors")
        .license_type(gtk::License::Gpl30)
        .website(env!("CARGO_PKG_HOMEPAGE"))
        .issue_url(concat!(env!("CARGO_PKG_REPOSITORY"), "/issues"))
        .developers(vec![env!("CARGO_PKG_AUTHORS").to_string()])
        .build();

    // PROTOCOL.md §0 — these projects are load-bearing for our
    // understanding of the device. Credit them prominently so users
    // who land here can follow the upstream trail.
    dialog.add_acknowledgement_section(
        Some("Reverse-engineering references"),
        &[
            "Aaron Brown — aaronsb/obsbot-camera-control (Qt6 reference)",
            "taxfromdk — obsbot_tiny_reversing",
        ],
    );

    dialog.present(app.active_window().as_ref());
}
