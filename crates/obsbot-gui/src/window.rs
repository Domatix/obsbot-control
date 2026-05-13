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

//! Main `AdwApplicationWindow`: header bar + a placeholder status page
//! that T-013 replaces with the real diagnostics view.

// gtk-rs idiom: alias the canonical crate names to their conventional
// short forms at the module level.
use gtk4 as gtk;
use libadwaita as adw;

use adw::prelude::*;

/// Build the top-level window. T-007 only wires a header bar with the
/// app name and a placeholder status page; T-013 replaces the page with
/// the diagnostics view.
pub fn build(app: &adw::Application) -> adw::ApplicationWindow {
    let header = adw::HeaderBar::new();

    let status = adw::StatusPage::builder()
        .icon_name("camera-web-symbolic")
        .title("Obsbot Cam Control")
        .description("Scaffolding only — camera detection and controls arrive in T-013 and v0.2.")
        .build();

    let layout = gtk::Box::new(gtk::Orientation::Vertical, 0);
    layout.append(&header);
    layout.append(&status);
    status.set_vexpand(true);

    adw::ApplicationWindow::builder()
        .application(app)
        .title("Obsbot Cam Control")
        .default_width(720)
        .default_height(540)
        .content(&layout)
        .build()
}
