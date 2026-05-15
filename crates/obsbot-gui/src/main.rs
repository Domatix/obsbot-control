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

//! `obsbot-gui` — GTK4 + libadwaita interface for Obsbot Cam Control.
//!
//! T-007 lands the bootstrap: an `adw::Application` that opens an empty
//! `AdwApplicationWindow` titled "Obsbot Cam Control" and quits cleanly
//! on Ctrl+Q. The diagnostics view, hot-plug listener, and per-camera
//! controls arrive from T-013 onwards.

mod ai_effects_view;
mod application;
mod controls_view;
mod exposure_group;
mod extras_view;
mod i18n;
mod ptz_pad;
mod settings;
mod wb_group;
mod window;

/// The reverse-DNS App ID resolved in ADR-0012.
const APP_ID: &str = "io.github.domatix.ObsbotCamControl";

fn main() -> glib::ExitCode {
    // T-107: bind the gettext text-domain before any widget is built
    // so labels constructed via `i18n::gettext(...)` resolve against
    // the runtime locale catalogs from the very first frame.
    i18n::init();
    application::run(APP_ID)
}
