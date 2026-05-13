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

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use gio::prelude::*;

use obsbot_core::{write_control, ControlValue};

const APP_ID: &str = "io.github.domatix.ObsbotCamControl";
const KEY: &str = "control-values";
/// ASCII Unit Separator — reserved in V4L2 control names and OBSBOT
/// serial strings, safe as an in-key separator.
const KEY_SEP: char = '\x1f';

fn dict_key(serial: &str, control_name: &str) -> String {
    format!("{serial}{KEY_SEP}{control_name}")
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
    if let Err(err) = write_control(path, id, value) {
        eprintln!(
            "warning: failed to write {name} ({id:#010x}) on {}: {err}",
            path.display(),
        );
        return;
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
}
