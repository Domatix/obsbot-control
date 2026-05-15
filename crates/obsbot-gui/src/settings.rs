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

//! Per-camera `GSettings` persistence (T-105).
//!
//! Saves the last-set V4L2 control value per `(camera serial, control
//! name)` pair so subsequent app launches can restore each camera's
//! state. The on-disk schema is shipped in `data/io.github.domatix.
//! ObsbotCamControl.gschema.xml` and installed via meson under
//! `$datadir/glib-2.0/schemas/`; for `cargo run` (no `meson install`)
//! the `build.rs` shim recompiles the schema into `OUT_DIR/schemas/`
//! and this module loads it directly via
//! [`gio::SettingsSchemaSource::from_directory`] — no environment-
//! variable manipulation needed (which is good, because the GUI crate
//! forbids `unsafe_code` and `env::set_var` is unsafe since
//! Rust 1.84).
//!
//! Schema shape: a single key `control-values` of type `a{si}` — a
//! flat dictionary keyed by `"<serial>\x1f<control-name>"` (the ASCII
//! Unit Separator is reserved and never appears in V4L2 names or
//! OBSBOT serials). Values are `i32` because V4L2 standard control
//! values are `__s32`; booleans encode as 0 / 1 and menus encode as
//! their integer ID.

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use gio::prelude::*;
use glib::object::{IsA, ObjectExt};
use gtk4 as gtk;
use libadwaita as adw;

use gtk::prelude::WidgetExt;
use obsbot_core::{read_controls, write_control, ControlValue};

use crate::i18n::gettext;

const APP_ID: &str = "io.github.domatix.ObsbotCamControl";
const KEY: &str = "control-values";
/// Duration in seconds before a write-failure toast auto-dismisses.
/// `adw::Toast` interprets `0` as "never auto-dismiss"; we want users
/// to actually notice the message but not be hostage to it.
const TOAST_TIMEOUT_SECS: u32 = 5;
/// ASCII Unit Separator — reserved in V4L2 control names and OBSBOT
/// serial strings, safe as an in-key separator.
const KEY_SEP: char = '\x1f';

fn dict_key(serial: &str, control_name: &str) -> String {
    format!("{serial}{KEY_SEP}{control_name}")
}

thread_local! {
    /// Weak ref to the `AdwToastOverlay` wrapping the currently-active
    /// controls page (T-108). Set by `controls_view::build_controls_
    /// page`; cleared implicitly when the previous page widget drops
    /// and the weak upgrade returns `None`. Stored as
    /// `glib::WeakRef` (vs `std::rc::Weak`) because gtk-rs widgets
    /// are GObject-refcounted, not Rc-refcounted.
    static TOAST_OVERLAY: RefCell<Option<glib::WeakRef<adw::ToastOverlay>>> =
        const { RefCell::new(None) };

    /// Registry of `(control_id, widget)` pairs for the currently-
    /// active controls page (T-111). Populated by each row builder
    /// (`control_row`-derived widgets in `controls_view.rs`,
    /// `wb_group`, `exposure_group`, and the focus / zoom rows in
    /// `ptz_pad.rs`) and consumed by [`refresh_sensitivity`] after
    /// every successful Boolean / Menu write so the kernel's V4L2
    /// `INACTIVE` flag flips propagate to the widgets without
    /// requiring per-control ad-hoc listeners. Cleared at the start
    /// of every new controls-page build by
    /// [`reset_row_registry`].
    static REGISTERED_ROWS: RefCell<Vec<(u32, gtk::Widget)>> =
        const { RefCell::new(Vec::new()) };

    /// `/dev/videoN` path of the currently-active controls page
    /// (T-111). Companion to [`REGISTERED_ROWS`]; used by
    /// [`refresh_sensitivity`] to re-read controls without
    /// threading the path through every write callback.
    static ACTIVE_VIDEO_PATH: RefCell<Option<PathBuf>> =
        const { RefCell::new(None) };
}

/// Bind the toast surface used by [`surface_error`] (T-108). Called
/// once per controls-page build; later binds supersede earlier ones,
/// so navigating to a different camera replaces the target without
/// leaving a stale strong reference.
pub fn bind_toast_overlay(overlay: &adw::ToastOverlay) {
    TOAST_OVERLAY.with(|cell| {
        *cell.borrow_mut() = Some(overlay.downgrade());
    });
}

/// Reset the per-page sensitivity refresh state for a freshly-built
/// controls page (T-111). Clears the row registry and stores the
/// camera's video path so [`refresh_sensitivity`] knows which device
/// to re-`read_controls` from.
pub fn reset_row_registry(video_path: Option<PathBuf>) {
    REGISTERED_ROWS.with(|cell| cell.borrow_mut().clear());
    ACTIVE_VIDEO_PATH.with(|cell| *cell.borrow_mut() = video_path);
}

/// Register a single row's `(control_id, widget)` pair (T-111). Called
/// by each row builder right after `set_sensitive(ctrl.is_active)`;
/// every write that flips an INACTIVE flag downstream then refreshes
/// this widget via [`refresh_sensitivity`].
pub fn register_row(ctrl_id: u32, row: &impl IsA<gtk::Widget>) {
    REGISTERED_ROWS.with(|cell| {
        cell.borrow_mut().push((ctrl_id, row.clone().upcast()));
    });
}

/// Re-read the camera's controls and update every registered row's
/// `set_sensitive` flag to match the current `is_active` from the
/// kernel (T-111). Skipped silently if no path is bound (cargo run
/// before navigating to a camera) or `read_controls` fails (device
/// just disconnected / busy). Called from [`write_and_save`] after a
/// successful Boolean / Menu write — Integer writes (slider drags)
/// don't trigger this path because they don't gate other controls
/// in the UVC standard control set.
fn refresh_sensitivity() {
    let path = ACTIVE_VIDEO_PATH.with(|cell| cell.borrow().clone());
    let Some(path) = path else { return };
    let Ok(controls) = read_controls(&path) else {
        return;
    };
    let active_by_id: HashMap<u32, bool> = controls.iter().map(|c| (c.id, c.is_active)).collect();
    REGISTERED_ROWS.with(|cell| {
        for (id, widget) in cell.borrow().iter() {
            if let Some(&active) = active_by_id.get(id) {
                widget.set_sensitive(active);
            }
        }
    });
}

/// Show `msg` on the most-recently-bound toast overlay (T-108).
///
/// Falls through to `eprintln!` when no overlay is bound or the
/// previously-bound overlay has been dropped (e.g. the user navigated
/// away from the controls page just before a delayed write callback
/// fires). The fall-through keeps the diagnostic visible during dev
/// runs without requiring a toast surface.
pub fn surface_error(msg: &str) {
    let shown = TOAST_OVERLAY.with(|cell| {
        let Some(weak) = cell.borrow().as_ref().cloned() else {
            return false;
        };
        let Some(overlay) = weak.upgrade() else {
            return false;
        };
        let toast = adw::Toast::builder()
            .title(msg)
            .timeout(TOAST_TIMEOUT_SECS)
            .build();
        overlay.add_toast(toast);
        true
    });
    if !shown {
        eprintln!("warning: {msg}");
    }
}

/// Resolve the [`gio::Settings`] handle for the app, loading the
/// compiled schema from `OUT_DIR/schemas/` (set by `build.rs`).
/// Returns `None` if the schema source can not be opened — callers
/// then degrade gracefully (warning + persistence disabled).
fn settings_handle() -> Option<gio::Settings> {
    let schema_dir: PathBuf = PathBuf::from(env!("OBSBOT_DEV_SCHEMA_DIR"));
    let source = gio::SettingsSchemaSource::from_directory(&schema_dir, None, false).ok()?;
    let schema = source.lookup(APP_ID, false)?;
    Some(gio::Settings::new_full(
        &schema,
        gio::SettingsBackend::NONE,
        None,
    ))
}

/// Load every saved control value for the given camera serial.
/// Returns an empty map when the schema cannot be opened or no entries
/// match.
pub fn load_for_camera(serial: &str) -> HashMap<String, i32> {
    let Some(settings) = settings_handle() else {
        return HashMap::new();
    };
    let prefix = format!("{serial}{KEY_SEP}");
    let map: HashMap<String, i32> = settings.get(KEY);
    map.into_iter()
        .filter_map(|(k, v)| k.strip_prefix(&prefix).map(|n| (n.to_string(), v)))
        .collect()
}

/// Persist one control's value under `(serial, control_name)`.
///
/// Failures (schema not found, dconf write rejected) are logged to
/// stderr and swallowed — persistence is a best-effort feature and
/// must never break the live write path.
pub fn save_for_camera(serial: &str, control_name: &str, value: i32) {
    let Some(settings) = settings_handle() else {
        eprintln!("warning: GSettings schema not loadable; persistence disabled");
        return;
    };
    let mut map: HashMap<String, i32> = settings.get(KEY);
    map.insert(dict_key(serial, control_name), value);
    if let Err(err) = settings.set(KEY, &map) {
        eprintln!("warning: GSettings save failed: {err}");
    }
}

/// Write a control to the V4L2 device, then persist the value if a
/// `serial` is available. The on-disk format stores the raw integer
/// representation: Integer / Menu store the value directly, Boolean
/// stores `0` / `1`.
///
/// Persistence failures never propagate up — the V4L2 write is the
/// authoritative half of the contract, the `GSettings` update is the
/// best-effort half.
pub fn write_and_save(path: &Path, id: u32, value: ControlValue, serial: Option<&str>, name: &str) {
    // T-111: only Boolean / Menu writes are "gates" that could flip
    // the V4L2 INACTIVE flag of dependent controls (WB Auto switch,
    // Auto Exposure dropdown, etc.). Integer writes — slider drags
    // running at ~100Hz — don't gate anything in the UVC standard
    // control set, so we skip the extra `read_controls` ioctl on
    // them. This keeps the perf cost of the refresh path bounded.
    let needs_refresh = matches!(value, ControlValue::Boolean(_) | ControlValue::Menu(_));
    if let Err(err) = write_control(path, id, value) {
        // T-108: surface the failure as an in-app toast so the user
        // sees the message without having to read the terminal. The
        // helper falls through to `eprintln!` when no toast overlay
        // is bound (cargo run before navigating into a camera).
        // GSettings save failures (further below) stay on stderr —
        // they are transparently recovered next session and are not
        // user-actionable.
        let msg = gettext("Failed to set {name}: {error}")
            .replace("{name}", name)
            .replace("{error}", &err.to_string());
        surface_error(&msg);
        return;
    }
    if needs_refresh {
        refresh_sensitivity();
    }
    let Some(serial) = serial else { return };
    let int_value = match value {
        ControlValue::Integer(v) | ControlValue::Menu(v) => {
            i32::try_from(v).unwrap_or(if v >= 0 { i32::MAX } else { i32::MIN })
        }
        ControlValue::Boolean(b) => i32::from(b),
        // `ControlValue` is `#[non_exhaustive]`; any future variant
        // (compound payloads, string menus, etc.) is dropped from
        // persistence rather than crashing the GUI.
        _ => return,
    };
    save_for_camera(serial, name, int_value);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `dict_key` keeps serials and control names disjoint via the
    /// reserved ASCII Unit Separator — adding a colon-y control name
    /// to a colon-y serial must still round-trip cleanly.
    #[test]
    fn dict_key_separates_serial_and_name() {
        let k = dict_key("Tiny2L:00:11:22", "Power Line Frequency");
        assert!(k.starts_with("Tiny2L:00:11:22\x1f"));
        let (serial, name) = k.split_once(KEY_SEP).unwrap();
        assert_eq!(serial, "Tiny2L:00:11:22");
        assert_eq!(name, "Power Line Frequency");
    }

    /// Schema / runtime alignment (T-105fix): the compiled schema in
    /// `OUT_DIR/schemas/` (produced by `build.rs` from `data/`) must
    /// declare the same key name and type that `settings.rs` writes
    /// against. If a future schema edit drifts away from
    /// `(KEY, a{si})`, this catches it without needing the GUI to be
    /// launched.
    ///
    /// Uses [`settings_handle`] (the same loader used in production)
    /// and exercises set/get on the live `gio::Settings` object —
    /// `from_directory` + `Settings::new_full` with `SettingsBackend::
    /// NONE` keeps the test self-contained, no dconf side-effects.
    #[test]
    fn schema_round_trip_with_runtime_key() {
        // Skip gracefully if the test runner lacks GLib type init
        // (some sandboxed CI). Calling `gio::Settings::*` before
        // `glib::MainContext` exists triggers warnings rather than
        // panics, but the round-trip itself needs no main loop.
        let Some(settings) = settings_handle() else {
            // Build did not produce a compiled schema — should not
            // happen because `build.rs` always runs, but bail if so.
            panic!(
                "settings_handle() returned None — compiled schema \
                 not loadable from OBSBOT_DEV_SCHEMA_DIR"
            );
        };

        // Start from a clean slate inside the in-memory backend.
        let empty: HashMap<String, i32> = HashMap::new();
        settings
            .set(KEY, &empty)
            .expect("schema must accept an empty a{si} for `control-values`");

        // Write a representative composite key + read it back.
        let mut map: HashMap<String, i32> = HashMap::new();
        map.insert(dict_key("Tiny2L:00:11:22", "Brightness"), 75);
        map.insert(dict_key("Tiny2L:00:11:22", "Auto Exposure"), 1);
        settings
            .set(KEY, &map)
            .expect("schema must accept a{si} writes against `control-values`");

        let read: HashMap<String, i32> = settings.get(KEY);
        assert_eq!(
            read.len(),
            2,
            "expected 2 entries after the write, got {}",
            read.len()
        );
        assert_eq!(
            read.get(&dict_key("Tiny2L:00:11:22", "Brightness"))
                .copied(),
            Some(75),
        );
        assert_eq!(
            read.get(&dict_key("Tiny2L:00:11:22", "Auto Exposure"))
                .copied(),
            Some(1),
        );

        // `load_for_camera` filters by serial prefix — verify it
        // returns the per-camera subset stripped of the prefix.
        let per_camera = load_for_camera("Tiny2L:00:11:22");
        assert_eq!(per_camera.get("Brightness").copied(), Some(75));
        assert_eq!(per_camera.get("Auto Exposure").copied(), Some(1));
        assert_eq!(
            per_camera.len(),
            2,
            "load_for_camera should return exactly the matching entries",
        );
    }
}
