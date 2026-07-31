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
//! recall buttons.
//!
//! Preset recall is **recall-only** per quirk Q7: programming a
//! preset position into the camera firmware requires the OBSBOT
//! Center app or the on-device gesture mechanism. The group's
//! subtitle states this explicitly so users do not expect a "save
//! preset" button.

// Same rationale as `ai_effects_view`: doc-markdown false-positives
// on `Tiny4Linux`, `cgevans`, `GET_CUR` etc. are noise; the genuine
// code references are still backticked manually.
#![allow(clippy::doc_markdown)]

use std::fs::{File, OpenOptions};
use std::rc::Rc;

use gtk4 as gtk;
use libadwaita as adw;

use adw::prelude::*;
use obsbot_core::xu::commands::recall_preset;
use obsbot_core::xu::XuError;
use obsbot_core::{CameraInfo, TINY2_FAMILY};

use crate::i18n::gettext;
use crate::settings;

/// Returns `true` if `(vid, pid)` is an OBSBOT Tiny 2 family unit.
fn is_tiny_2_family(vid: u16, pid: u16) -> bool {
    TINY2_FAMILY.iter().any(|&(v, p)| v == vid && p == pid)
}

/// Build the "Power state & Presets" preferences group, or `None`
/// for cameras that are not Tiny 2 family or have no `/dev/videoN`
/// path. Symmetric with `ai_effects_view::build_ai_effects_group`.
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
                 skipping Power & Presets group",
                path.display()
            );
            return None;
        }
    };

    let group = adw::PreferencesGroup::builder()
        .title(gettext("Presets"))
        .description(gettext("This app can only recall presets, not save them."))
        .build();

    group.add(&preset_row(&file, 0));
    group.add(&preset_row(&file, 1));
    group.add(&preset_row(&file, 2));

    Some(group)
}

fn preset_row(file: &Rc<File>, index: i8) -> adw::ActionRow {
    // User-visible numbering is 1-based ("Preset 1") but the
    // wire-level slot index is 0-based.
    let title = gettext("Preset {n}").replace("{n}", &(index + 1).to_string());

    let row = adw::ActionRow::builder()
        .title(&title)
        .activatable(true)
        .build();

    let button = gtk::Button::builder()
        .icon_name("go-next-symbolic")
        .valign(gtk::Align::Center)
        .css_classes(vec!["flat"])
        .tooltip_text(gettext("Recall"))
        .build();
    row.add_suffix(&button);

    let file_for_button = Rc::clone(file);
    button.connect_clicked(move |_| recall_with_feedback(&file_for_button, index));

    // The whole row is `activatable` so clicking the body works too;
    // mirror the button's behaviour for that path.
    let file_for_row = Rc::clone(file);
    row.connect_activated(move |_| recall_with_feedback(&file_for_row, index));

    row
}

/// Recall preset `index` and acknowledge the click with a toast. The
/// firmware gives no "slot empty" signal, so a successful send only
/// means the command was accepted — we surface "Recalling preset N…"
/// so the user knows the click registered even when an unprogrammed
/// slot leaves the camera still (the colleague-reported "nothing
/// happens" confusion). A transport failure still reports as an error.
fn recall_with_feedback(file: &File, index: i8) {
    match recall_preset(file, index) {
        Ok(()) => {
            let msg = gettext("Recalling preset {n}…").replace("{n}", &(index + 1).to_string());
            settings::surface_error(&msg);
        }
        Err(err) => report_xu_error("Preset recall", &err),
    }
}

fn report_xu_error(control: &str, err: &XuError) {
    let msg = gettext("Failed to set {name}: {error}")
        .replace("{name}", control)
        .replace("{error}", &err.to_string());
    settings::surface_error(&msg);
}
