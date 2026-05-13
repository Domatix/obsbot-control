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
    let app = adw::Application::builder()
        .application_id(app_id)
        .resource_base_path("/io/github/domatix/ObsbotCamControl/")
        .build();

    let icon_name = app_id.to_owned();
    app.connect_startup(move |app| {
        // Sets the icon for every window the app creates so GTK can
        // paint it in the Wayland window-list / X11 WM_HINTS even
        // before the user has run `meson install`. GNOME Shell still
        // resolves the overview icon via the `.desktop` file (T-009);
        // both paths converge on the same hicolor entry T-010 installs.
        gtk::Window::set_default_icon_name(&icon_name);
        register_actions(app);
    });

    app.connect_activate(|app| {
        let window = window::build(app);
        window.present();
    });

    app.run()
}

fn register_actions(app: &adw::Application) {
    let quit = ActionEntry::builder("quit")
        .activate(|app: &adw::Application, _, _| app.quit())
        .build();
    app.add_action_entries([quit]);
    app.set_accels_for_action("app.quit", &["<primary>q"]);
}
