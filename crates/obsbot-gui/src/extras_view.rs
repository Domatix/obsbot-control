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

//! "Presets" group widget (T-302; the Sleep/Wake switch was dropped in
//! T-211 as it did not reliably drive the firmware, and the diagnostic
//! "Show XU status (hex dump)" row was removed in T-220).
//!
//! Hosts the Tiny4Linux-only XU surface: the three preset-position
//! rows, each with a recall, a save (overwrite with the current pose)
//! and a delete action.
//!
//! Quirk Q7 documented recall-only behavior for the original group.
//! The save/delete wire commands (`AI_ADD_GIMBAL_PRESET`,
//! `AI_DEL_GIMBAL_PRESET`) are now implemented in `obsbot-core`, so
//! the group exposes them. The group description reflects the live
//! preset list (`AI_GET_GIMBAL_PRESET_LIST`): the camera only answers
//! "list empty" vs "list non-empty" reliably, so per-slot occupancy is
//! not shown.
//!
//! The saved pose is the current `pan_absolute` / `tilt_absolute` /
//! `zoom_absolute` triplet read over V4L2. `POSE_SCALE` converts the
//! V4L2 integer units into the float units the preset command expects;
//! it was verified on a Tiny 2 Lite with `scripts/verify-recall.sh`
//! (save, move, recall, frame-compare). Do not change it without
//! re-running that check.

// Same rationale as `ai_effects_view`: doc-markdown false-positives
// on `Tiny4Linux`, `cgevans`, `GET_CUR` etc. are noise; the genuine
// code references are still backticked manually.
#![allow(clippy::doc_markdown)]

use std::cell::RefCell;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::rc::Rc;

use gtk4 as gtk;
use libadwaita as adw;

use adw::prelude::*;
use obsbot_core::xu::commands::preset::{self, Pose};
use obsbot_core::xu::commands::preset_read;
use obsbot_core::xu::v4l2_ptz::{
    V4L2_CID_PAN_ABSOLUTE, V4L2_CID_TILT_ABSOLUTE, V4L2_CID_ZOOM_ABSOLUTE,
};
use obsbot_core::xu::XuError;
use obsbot_core::{read_controls, CameraInfo, ControlDescriptor, ControlKind, TINY2_FAMILY};

use crate::i18n::gettext;
use crate::settings;

/// Unit conversion from the V4L2 integer triplet into the preset
/// command's float pose. Verified by `scripts/verify-recall.sh`.
const POSE_SCALE: f32 = 1.0;

/// Returns `true` if `(vid, pid)` is an OBSBOT Tiny 2 family unit.
fn is_tiny_2_family(vid: u16, pid: u16) -> bool {
    TINY2_FAMILY.iter().any(|&(v, p)| v == vid && p == pid)
}

/// Build the "Presets" preferences group, or `None` for cameras that
/// are not Tiny 2 family or have no `/dev/videoN` path. Symmetric with
/// `ai_effects_view::build_ai_effects_group`.
pub fn build_extras_group(cam: &CameraInfo) -> Option<adw::PreferencesGroup> {
    if !is_tiny_2_family(cam.vid, cam.pid) {
        return None;
    }
    let path = cam.video_path.as_deref()?;
    let file = match OpenOptions::new().read(true).write(true).open(path) {
        Ok(f) => Rc::new(f),
        Err(err) => {
            eprintln!(
                "warning: extras_view: open {} for XU writes failed: {err}; \
                 skipping Presets group",
                path.display()
            );
            return None;
        }
    };

    let group = adw::PreferencesGroup::builder()
        .title(gettext("Presets"))
        .build();

    let shared = Rc::new(State {
        group: group.clone(),
        file: Rc::clone(&file),
        path: path.to_path_buf(),
        rows: RefCell::new(Vec::new()),
        ids: RefCell::new(Vec::new()),
        names: RefCell::new(Vec::new()),
    });
    for index in 0..3 {
        let row = preset_row(&file, index, &shared);
        group.add(&row);
        shared.rows.borrow_mut().push((index, row));
    }

    refresh_state(&shared);

    Some(group)
}

/// Live preset state shared by the row actions so every save and
/// delete re-renders the group description and the per-row subtitles.
struct State {
    group: adw::PreferencesGroup,
    file: Rc<File>,
    path: std::path::PathBuf,
    rows: RefCell<Vec<(i8, adw::ActionRow)>>,
    /// Stored slot ids from the last refresh (authoritative occupancy).
    ids: RefCell<Vec<u8>>,
    /// Entries from the last refresh, for names and dialog prefill.
    names: RefCell<Vec<preset_read::PresetEntry>>,
}

/// Update the group description and the per-row subtitles from the
/// camera: the framed list says whether anything is stored, and the
/// entry cursor (selector `0x0D`, see `docs/PROTOCOL.md`) supplies
/// per-slot occupancy and names. Failures leave the previous
/// rendering in place.
fn refresh_state(state: &State) {
    // The framed list reply is the plain list of stored slot ids.
    let ids = preset::list_presets(&state.file).unwrap_or_default();
    let description = if ids.is_empty() {
        gettext(
            "No presets stored yet. Use the save button on a row to store the current position.",
        )
    } else {
        gettext("Presets stored on the camera: {n}.").replace("{n}", &ids.len().to_string())
    };
    state.group.set_description(Some(&description));

    let entries = preset_read::read_preset_entries(&state.file).unwrap_or_default();
    state.ids.borrow_mut().clone_from(&ids);
    state.names.borrow_mut().clone_from(&entries);
    for (index, row) in state.rows.borrow().iter() {
        let slot_idx = u8::try_from(*index).unwrap_or(0);
        let subtitle = entries
            .iter()
            .find(|entry| entry.slot == slot_idx)
            .map_or_else(
                || {
                    if ids.contains(&slot_idx) {
                        gettext("Stored")
                    } else {
                        gettext("Empty")
                    }
                },
                |entry| {
                    if entry.name.is_empty() {
                        gettext("Stored")
                    } else {
                        entry.name.clone()
                    }
                },
            );
        row.set_subtitle(&subtitle);
    }
}

fn preset_row(file: &Rc<File>, index: i8, state: &Rc<State>) -> adw::ActionRow {
    // User-visible numbering is 1-based ("Preset 1") but the
    // wire-level slot index is 0-based.
    let title = gettext("Preset {n}").replace("{n}", &(index + 1).to_string());

    let row = adw::ActionRow::builder()
        .title(&title)
        .activatable(true)
        .build();

    let recall = gtk::Button::builder()
        .icon_name("go-next-symbolic")
        .valign(gtk::Align::Center)
        .css_classes(vec!["flat"])
        .tooltip_text(gettext("Recall"))
        .build();
    row.add_suffix(&recall);

    let save = gtk::Button::builder()
        .icon_name("document-save-symbolic")
        .valign(gtk::Align::Center)
        .css_classes(vec!["flat"])
        .tooltip_text(gettext("Save current position"))
        .build();
    row.add_suffix(&save);

    let rename = gtk::Button::builder()
        .icon_name("document-edit-symbolic")
        .valign(gtk::Align::Center)
        .css_classes(vec!["flat"])
        .tooltip_text(gettext("Rename"))
        .build();
    row.add_suffix(&rename);

    let delete = gtk::Button::builder()
        .icon_name("user-trash-symbolic")
        .valign(gtk::Align::Center)
        .css_classes(vec!["flat"])
        .tooltip_text(gettext("Delete"))
        .build();
    row.add_suffix(&delete);

    {
        let file = Rc::clone(file);
        recall.connect_clicked(move |_| recall_with_feedback(&file, index));
    }
    {
        let file = Rc::clone(file);
        row.connect_activated(move |_| recall_with_feedback(&file, index));
    }
    {
        let state = Rc::clone(state);
        save.connect_clicked(move |_| ask_save(&state, index));
    }
    {
        let state = Rc::clone(state);
        rename.connect_clicked(move |_| {
            let slot_idx = u8::try_from(index).unwrap_or(0);
            if !state.ids.borrow().contains(&slot_idx) {
                let msg = gettext("Preset {n} is empty. Save a position first.")
                    .replace("{n}", &(index + 1).to_string());
                settings::surface_error(&msg);
                return;
            }
            ask_name(&state.file, index, &state);
        });
    }
    {
        let state = Rc::clone(state);
        delete.connect_clicked(move |_| {
            let slot_idx = u8::try_from(index).unwrap_or(0);
            if !state.ids.borrow().contains(&slot_idx) {
                let msg = gettext("Preset {n} is already empty.")
                    .replace("{n}", &(index + 1).to_string());
                settings::surface_error(&msg);
                return;
            }
            delete_with_feedback(&state.file, index);
            refresh_state(&state);
        });
    }

    row
}

/// Read the current gimbal pose (`pan_absolute` / `tilt_absolute` /
/// `zoom_absolute`) over V4L2 and convert it into the preset command's
/// float units. Returns `None` when any of the three controls is
/// unavailable or inactive.
///
/// Controls are matched by V4L2 CID, not by name: the kernel reports
/// label-style names ("Pan, Absolute"), so the `v4l2-ctl`-style
/// identifiers never appear in a [`ControlDescriptor`].
fn current_pose(path: &Path) -> Option<Pose> {
    let controls = read_controls(path).ok()?;
    Some(Pose {
        pan: axis_current(&controls, V4L2_CID_PAN_ABSOLUTE)?,
        tilt: axis_current(&controls, V4L2_CID_TILT_ABSOLUTE)?,
        roll: 0.0,
        zoom: axis_current(&controls, V4L2_CID_ZOOM_ABSOLUTE)?,
    })
}

// UVC pan/tilt/zoom absolute values stay far below 2^24, so the
// i64 -> f32 conversion is exact for every value the driver can
// report; the lint cannot know that bound.
#[allow(clippy::cast_precision_loss)]
fn axis_current(controls: &[ControlDescriptor], id: u32) -> Option<f32> {
    let ctrl = controls.iter().find(|c| c.id == id)?;
    match ctrl.kind {
        ControlKind::Integer { current, .. } => Some(current as f32 * POSE_SCALE),
        _ => None,
    }
}

/// Recall preset `index` and acknowledge the click with a toast. The
/// firmware gives no "slot empty" signal, so a successful send only
/// means the command was accepted — we surface "Recalling preset N…"
/// so the user knows the click registered even when an unprogrammed
/// slot leaves the camera still (the colleague-reported "nothing
/// happens" confusion). A transport failure still reports as an error.
fn recall_with_feedback(file: &File, index: i8) {
    match preset::recall_preset(file, index) {
        Ok(()) => {
            let msg = gettext("Recalling preset {n}…").replace("{n}", &(index + 1).to_string());
            settings::surface_error(&msg);
        }
        Err(err) => report_xu_error(&gettext("Preset recall"), &err),
    }
}

/// Overwrite `index` with the current pose. The delete answer is not
/// proof of removal (the firmware ACKs even unknown slots), so after
/// a save or delete the group description is refreshed from the
/// preset list, which is the only reliable signal.
/// Ask for a name and store the current pose in that slot, in one
/// step. The name is optional: an empty field stores the position
/// without a name, which the row then shows as "Stored".
fn ask_save(state: &Rc<State>, index: i8) {
    let Some(window) = gio::Application::default()
        .and_downcast::<gtk::Application>()
        .and_then(|app| app.active_window())
    else {
        return;
    };
    let title = gettext("Save preset {n}").replace("{n}", &(index + 1).to_string());
    let dialog = adw::AlertDialog::new(Some(&title), None);
    dialog.add_response("cancel", &gettext("Cancel"));
    dialog.add_response("save", &gettext("Save"));
    dialog.set_response_appearance("save", adw::ResponseAppearance::Suggested);
    dialog.set_default_response(Some("save"));
    dialog.set_close_response("cancel");

    let current = state
        .names
        .borrow()
        .iter()
        .find(|entry| entry.slot == u8::try_from(index).unwrap_or(0))
        .map(|entry| entry.name.clone())
        .unwrap_or_default();
    let entry = gtk::Entry::builder()
        .placeholder_text(gettext("Preset name"))
        .text(&current)
        .activates_default(true)
        .build();
    dialog.set_extra_child(Some(&entry));

    let state_for_dialog = Rc::clone(state);
    dialog.choose(&window, gio::Cancellable::NONE, move |response| {
        if response.as_str() != "save" {
            return;
        }
        let name = entry.text().trim().to_string();
        save_with_feedback(&state_for_dialog, index, &name);
        refresh_state(&state_for_dialog);
    });
}

/// Write the current pose into `index`, then apply `name` when the
/// user typed one. Both commands go to the camera; the refresh after
/// the caller returns reads them back.
fn save_with_feedback(state: &State, index: i8, name: &str) {
    let Some(pose) = current_pose(&state.path) else {
        settings::surface_error(&gettext("Could not read the current camera position."));
        return;
    };
    let slot = u8::try_from(index + 1).unwrap_or(1);
    if let Err(err) = preset::add_preset(&state.file, slot, pose) {
        report_xu_error(&gettext("Preset save"), &err);
        return;
    }
    if !name.is_empty() {
        if let Err(err) = preset::rename_preset(&state.file, slot, name) {
            report_xu_error(&gettext("Preset rename"), &err);
            return;
        }
    }
    let msg = gettext("Saved preset {n}.").replace("{n}", &(index + 1).to_string());
    settings::surface_error(&msg);
}

/// Ask for a name in a modal dialog, then rename the slot on the
/// camera. The camera stores the name base64-encoded; the cursor read
/// in [`refresh_state`] decodes it back.
fn ask_name(file: &Rc<File>, index: i8, state: &Rc<State>) {
    let Some(window) = gio::Application::default()
        .and_downcast::<gtk::Application>()
        .and_then(|app| app.active_window())
    else {
        return;
    };
    let title = gettext("Rename preset {n}").replace("{n}", &(index + 1).to_string());
    let dialog = adw::AlertDialog::new(Some(&title), None);
    dialog.add_response("cancel", &gettext("Cancel"));
    dialog.add_response("rename", &gettext("Rename"));
    dialog.set_response_appearance("rename", adw::ResponseAppearance::Suggested);
    dialog.set_default_response(Some("rename"));
    dialog.set_close_response("cancel");

    let current = state
        .names
        .borrow()
        .iter()
        .find(|entry| entry.slot == u8::try_from(index).unwrap_or(0))
        .map(|entry| entry.name.clone())
        .unwrap_or_default();
    let entry = gtk::Entry::builder()
        .placeholder_text(gettext("Preset name"))
        .text(&current)
        .activates_default(true)
        .build();
    dialog.set_extra_child(Some(&entry));

    let file_for_dialog = Rc::clone(file);
    let state_for_dialog = Rc::clone(state);
    dialog.choose(&window, gio::Cancellable::NONE, move |response| {
        if response.as_str() != "rename" {
            return;
        }
        let name = entry.text().trim().to_string();
        if name.is_empty() {
            return;
        }
        let slot = u8::try_from(index + 1).unwrap_or(1);
        match preset::rename_preset(&file_for_dialog, slot, &name) {
            Ok(()) => {
                let msg = gettext("Renamed preset {n}.").replace("{n}", &(index + 1).to_string());
                settings::surface_error(&msg);
                refresh_state(&state_for_dialog);
            }
            Err(err) => report_xu_error(&gettext("Preset rename"), &err),
        }
    });
}

fn delete_with_feedback(file: &File, index: i8) {
    let slot = u8::try_from(index + 1).unwrap_or(1);
    match preset::delete_preset(file, slot) {
        Ok(()) => {
            let msg = gettext("Deleted preset {n}.").replace("{n}", &(index + 1).to_string());
            settings::surface_error(&msg);
        }
        Err(err) => report_xu_error(&gettext("Preset delete"), &err),
    }
}

fn report_xu_error(control: &str, err: &XuError) {
    let msg = gettext("Failed to set {name}: {error}")
        .replace("{name}", control)
        .replace("{error}", &err.to_string());
    settings::surface_error(&msg);
}
