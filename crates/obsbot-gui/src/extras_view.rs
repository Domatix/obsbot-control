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

use std::fs::{File, OpenOptions};
use std::path::Path;
use std::rc::Rc;

use gtk4 as gtk;
use libadwaita as adw;

use adw::prelude::*;
use obsbot_core::xu::commands::preset::{self, Pose};
use obsbot_core::xu::XuError;
use obsbot_core::{read_controls, CameraInfo, ControlKind, TINY2_FAMILY};

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
    refresh_description(&group, &file);

    for index in 0..3 {
        group.add(&preset_row(&file, path, index));
    }

    Some(group)
}

/// Update the group description from the live preset list. The list
/// payload's per-slot encoding is not decoded yet (see
/// `docs/PROTOCOL.md`), so only emptiness is surfaced.
fn refresh_description(group: &adw::PreferencesGroup, file: &File) {
    let stored = preset::list_presets(file).is_ok_and(|p| !p.is_empty());
    let description = if stored {
        gettext("Presets stored on the camera.")
    } else {
        gettext(
            "No presets stored yet. Use the save button on a row to store the current position.",
        )
    };
    group.set_description(Some(&description));
}

fn preset_row(file: &Rc<File>, path: &Path, index: i8) -> adw::ActionRow {
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
        let file = Rc::clone(file);
        let path = path.to_path_buf();
        save.connect_clicked(move |_| save_with_feedback(&file, &path, index));
    }
    {
        let file = Rc::clone(file);
        delete.connect_clicked(move |_| delete_with_feedback(&file, index));
    }

    row
}

/// Read the current gimbal pose (`pan_absolute` / `tilt_absolute` /
/// `zoom_absolute`) over V4L2 and convert it into the preset command's
/// float units. Returns `None` when any of the three controls is
/// unavailable or inactive.
fn current_pose(path: &Path) -> Option<Pose> {
    let controls = read_controls(path).ok()?;
    // UVC pan/tilt/zoom absolute values stay far below 2^24, so the
    // i64 -> f32 conversion is exact for every value the driver can
    // report; the lint cannot know that bound.
    #[allow(clippy::cast_precision_loss)]
    let value = |name: &str| -> Option<f32> {
        controls.iter().find_map(|ctrl| {
            if ctrl.name != name {
                return None;
            }
            match ctrl.kind {
                ControlKind::Integer { current, .. } => Some(current as f32 * POSE_SCALE),
                _ => None,
            }
        })
    };
    Some(Pose {
        pan: value("pan_absolute")?,
        tilt: value("tilt_absolute")?,
        roll: 0.0,
        zoom: value("zoom_absolute")?,
    })
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
        Err(err) => report_xu_error("Preset recall", &err),
    }
}

/// Overwrite `index` with the current pose. The delete answer is not
/// proof of removal (the firmware ACKs even unknown slots), so after
/// a save or delete the group description is refreshed from the
/// preset list, which is the only reliable signal.
fn save_with_feedback(file: &File, path: &Path, index: i8) {
    let Some(pose) = current_pose(path) else {
        settings::surface_error(&gettext("Could not read the current camera position."));
        return;
    };
    let slot = u8::try_from(index + 1).unwrap_or(1);
    match preset::add_preset(file, slot, pose) {
        Ok(()) => {
            let msg = gettext("Saved preset {n}.").replace("{n}", &(index + 1).to_string());
            settings::surface_error(&msg);
        }
        Err(err) => report_xu_error("Preset save", &err),
    }
}

fn delete_with_feedback(file: &File, index: i8) {
    let slot = u8::try_from(index + 1).unwrap_or(1);
    match preset::delete_preset(file, slot) {
        Ok(()) => {
            let msg = gettext("Deleted preset {n}.").replace("{n}", &(index + 1).to_string());
            settings::surface_error(&msg);
        }
        Err(err) => report_xu_error("Preset delete", &err),
    }
}

fn report_xu_error(control: &str, err: &XuError) {
    let msg = gettext("Failed to set {name}: {error}")
        .replace("{name}", control)
        .replace("{error}", &err.to_string());
    settings::surface_error(&msg);
}
