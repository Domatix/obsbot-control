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

use crate::i18n::gettext;
use crate::settings;
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
        // T-212: load the custom stylesheet now that a GdkDisplay
        // exists (GTK is initialized by the time `startup` fires).
        load_css();
        register_actions(app, &app_id_owned);
    });

    app.connect_activate(|app| {
        let window = window::build(app);
        window.present();
    });

    app.run()
}

/// Load the custom stylesheet (T-212) from the embedded `GResource` and
/// apply it to the default display at `APPLICATION` priority — above
/// the Adwaita theme, below user overrides. Silently does nothing if no
/// display is available (headless), which never happens for a presented
/// window.
fn load_css() {
    let provider = gtk::CssProvider::new();
    provider.load_from_resource("/io/github/domatix/ObsbotCamControl/style.css");
    if let Some(display) = gtk::gdk::Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

/// Apply an appearance preference (T-215) to the libadwaita style
/// manager. `"light"` / `"dark"` force the scheme; anything else
/// (`"default"`) follows the system light/dark setting.
fn apply_color_scheme(value: &str) {
    let scheme = match value {
        "light" => adw::ColorScheme::ForceLight,
        "dark" => adw::ColorScheme::ForceDark,
        _ => adw::ColorScheme::Default,
    };
    adw::StyleManager::default().set_color_scheme(scheme);
}

/// Register the stateful `app.color-scheme` action (T-215) backing the
/// primary-menu appearance radios. Seeds from the saved `GSettings` value,
/// applies it immediately, and on each activation updates the style
/// manager + persists the choice.
fn register_color_scheme_action(app: &adw::Application) {
    let initial = settings::color_scheme();
    apply_color_scheme(&initial);

    let action = gio::SimpleAction::new_stateful(
        "color-scheme",
        Some(glib::VariantTy::STRING),
        &initial.to_variant(),
    );
    action.connect_activate(|action, param| {
        if let Some(value) = param.and_then(glib::Variant::str) {
            action.set_state(&value.to_variant());
            apply_color_scheme(value);
            settings::set_color_scheme(value);
        }
    });
    app.add_action(&action);
}

fn register_actions(app: &adw::Application, app_id: &str) {
    register_color_scheme_action(app);

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
        // Application name is the project's literal branding —
        // intentionally NOT routed through gettext (translators
        // should not rebrand the product). Same rationale as
        // GNOME's own apps' AboutDialog wiring.
        .application_name("Obsbot Cam Control")
        .application_icon(app_id)
        .version(env!("CARGO_PKG_VERSION"))
        .developer_name(env!("CARGO_PKG_AUTHORS"))
        .copyright(gettext("© 2026 Domatix and contributors"))
        .license_type(gtk::License::Gpl30)
        .website(env!("CARGO_PKG_HOMEPAGE"))
        .issue_url(concat!(env!("CARGO_PKG_REPOSITORY"), "/issues"))
        .developers(vec![env!("CARGO_PKG_AUTHORS").to_string()])
        .build();

    // PROTOCOL.md §0 — these projects are load-bearing for our
    // understanding of the device. Credit them prominently so users
    // who land here can follow the upstream trail. The section title
    // is translatable; the names themselves stay as-is (they identify
    // people, not concepts).
    dialog.add_acknowledgement_section(
        Some(&gettext("Reverse-engineering references")),
        &[
            "Aaron Brown — aaronsb/obsbot-camera-control (Qt6 reference)",
            "taxfromdk — obsbot_tiny_reversing",
        ],
    );

    dialog.present(app.active_window().as_ref());
}
