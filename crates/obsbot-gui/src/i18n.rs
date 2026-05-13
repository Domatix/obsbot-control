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

//! gettext scaffolding (T-107).
//!
//! Thin shim around [`gettextrs`] so the rest of the GUI can route
//! user-facing strings through one re-export instead of importing the
//! crate directly. The translation domain is fixed by the binary name
//! (kebab-case of the App ID's last segment per ADR-0012); the install
//! localedir is plumbed from meson via the `OBSBOT_LOCALEDIR`
//! build-time env var (see `build.rs` stage 4).
//!
//! For bare `cargo run` (no meson involvement) `OBSBOT_LOCALEDIR` is
//! unset and [`init`] becomes a no-op past `setlocale` — source-
//! language strings flow through [`gettext`] unchanged.

use gettextrs::{setlocale, LocaleCategory};

/// gettext domain. Matches the installed binary name (ADR-0012).
const TEXTDOMAIN: &str = "obsbot-cam-control";

/// Initialise the translation domain. Call once at process start,
/// before any code constructs widget labels.
///
/// When `OBSBOT_LOCALEDIR` is baked in (i.e. the build was driven by
/// meson via `build-aux/cargo-build.sh`), this binds the textdomain
/// to the installed `.gmo` catalogs. Otherwise the call sequence
/// degrades to a `setlocale(LC_ALL, "")` only — `gettext()` then
/// returns the source-language string unchanged.
pub fn init() {
    setlocale(LocaleCategory::LcAll, "");

    if let Some(localedir) = option_env!("OBSBOT_LOCALEDIR") {
        let _ = gettextrs::bindtextdomain(TEXTDOMAIN, localedir);
        let _ = gettextrs::bind_textdomain_codeset(TEXTDOMAIN, "UTF-8");
        let _ = gettextrs::textdomain(TEXTDOMAIN);
    }
}

/// Look up `msgid` in the current text-domain. Returns a `String`
/// copy of the translated text, or the source `msgid` if no
/// translation is bound.
#[inline]
#[must_use]
pub fn gettext(msgid: &str) -> String {
    gettextrs::gettext(msgid)
}
