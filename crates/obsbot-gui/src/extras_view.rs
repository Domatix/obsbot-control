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

//! "Power state & Presets" group widget (T-302).
//!
//! Hosts the Tiny4Linux-only XU surface: Sleep/Wake state and the
//! three preset-position recall buttons. Also offers a "Show XU status
//! (hex dump)" row that pops an [`adw::AlertDialog`] rendering the
//! full 60-byte GET_CUR payload from selector `0x06` — the discovery
//! frontier for the 55 still-undecoded bytes (see
//! `PROTOCOL.md §3.2`).
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
use obsbot_core::xu::commands::{recall_preset, set_sleep};
use obsbot_core::xu::{get_status, SleepState, Status, XuError, STATUS_LEN};
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
                "warning: extras_view: open {path:?} for XU writes failed: {err}; \
                 skipping Power & Presets group"
            );
            return None;
        }
    };

    let baseline = get_status(&file).ok();

    let group = adw::PreferencesGroup::builder()
        // GNOME HIG: prefer "and" over "&" — and libadwaita treats
        // the title as Pango markup so a literal "&" would break the
        // entity parser.
        .title(gettext("Power state and presets"))
        .description(gettext(
            "Preset positions are stored in the camera firmware. Use the OBSBOT Center \
             app on Windows or macOS, or the gesture controls on the camera, to program \
             a position into a slot before recalling it from here.",
        ))
        .build();

    group.add(&sleep_row(&file, baseline.as_ref()));
    group.add(&preset_row(&file, 0));
    group.add(&preset_row(&file, 1));
    group.add(&preset_row(&file, 2));
    group.add(&dump_status_row(&file));

    Some(group)
}

fn sleep_row(file: &Rc<File>, baseline: Option<&Status>) -> adw::SwitchRow {
    // `Sleep` = the camera is "off" (lens covered, low power); the
    // switch's "active" state therefore means awake (camera is
    // streaming). Default-off (Awake) if we couldn't read baseline.
    let initial_awake = baseline.is_none_or(|s| matches!(s.sleep, SleepState::Awake));

    let row = adw::SwitchRow::builder()
        .title(gettext("Camera awake"))
        .subtitle(gettext(
            "Turn off to put the camera into low-power mode (lens cover descends).",
        ))
        .active(initial_awake)
        .build();

    let file_for_cb = Rc::clone(file);
    row.connect_active_notify(move |row| {
        let state = if row.is_active() {
            SleepState::Awake
        } else {
            SleepState::Sleep
        };
        if let Err(err) = set_sleep(&file_for_cb, state) {
            report_xu_error("Sleep state", &err);
        }
    });
    row
}

fn preset_row(file: &Rc<File>, index: i8) -> adw::ActionRow {
    // User-visible numbering is 1-based ("Preset 1") but the
    // wire-level slot index is 0-based.
    let title = gettext("Preset {n}").replace("{n}", &(index + 1).to_string());

    let row = adw::ActionRow::builder()
        .title(&title)
        .subtitle(gettext("Move the camera to this preset position"))
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
    button.connect_clicked(move |_| {
        if let Err(err) = recall_preset(&file_for_button, index) {
            report_xu_error("Preset recall", &err);
        }
    });

    // The whole row is `activatable` so clicking the body works too;
    // mirror the button's behaviour for that path.
    let file_for_row = Rc::clone(file);
    row.connect_activated(move |_| {
        if let Err(err) = recall_preset(&file_for_row, index) {
            report_xu_error("Preset recall", &err);
        }
    });

    row
}

fn dump_status_row(file: &Rc<File>) -> adw::ActionRow {
    let row = adw::ActionRow::builder()
        .title(gettext("Show XU status (hex dump)"))
        .subtitle(gettext(
            "Open a diagnostic view of the 60-byte status the camera returns. \
             Useful for community discovery of still-undecoded bytes.",
        ))
        .activatable(true)
        .build();

    let view_button = gtk::Button::builder()
        .icon_name("dialog-information-symbolic")
        .valign(gtk::Align::Center)
        .css_classes(vec!["flat"])
        .tooltip_text(gettext("Show"))
        .build();
    row.add_suffix(&view_button);

    let file_for_button = Rc::clone(file);
    view_button.connect_clicked(move |btn| open_dump_dialog(&file_for_button, btn));

    let file_for_row = Rc::clone(file);
    row.connect_activated(move |r| open_dump_dialog(&file_for_row, r));

    row
}

/// Pop an `AdwAlertDialog` showing the 60-byte status payload as a
/// hex grid with offsets and decoded-field annotations, plus a "Copy"
/// button that places the raw hex on the clipboard.
fn open_dump_dialog(file: &Rc<File>, anchor: &impl IsA<gtk::Widget>) {
    let status = match get_status(file) {
        Ok(s) => s,
        Err(err) => {
            report_xu_error("XU status read", &err);
            return;
        }
    };

    let dialog = adw::AlertDialog::builder()
        .heading(gettext("XU status dump"))
        .body_use_markup(true)
        .body(format_status_body(&status))
        .build();
    dialog.add_response("close", &gettext("Close"));
    dialog.add_response("copy", &gettext("Copy hex"));
    dialog.set_default_response(Some("close"));

    // Capture the raw bytes for the clipboard handler.
    let hex_payload = status
        .raw
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect::<Vec<_>>()
        .chunks(STATUS_LEN.div_ceil(4))
        .map(|chunk| chunk.join(" "))
        .collect::<Vec<_>>()
        .join("\n");

    let anchor_widget = anchor.clone().upcast::<gtk::Widget>();
    dialog.connect_response(None, move |dlg, response| {
        if response == "copy" {
            let display = anchor_widget.display();
            display.clipboard().set_text(&hex_payload);
            settings::surface_error(&gettext("XU status hex copied to clipboard."));
        }
        dlg.close();
    });

    let parent = anchor
        .clone()
        .upcast::<gtk::Widget>()
        .root()
        .and_downcast::<gtk::Window>();
    dialog.present(parent.as_ref());
}

/// Render the 60-byte payload as monospace hex with offset markers,
/// plus the 5 decoded fields listed under it.
fn format_status_body(status: &Status) -> String {
    let mut out = String::new();
    out.push_str("<tt>");
    for (i, byte) in status.raw.iter().enumerate() {
        if i % 16 == 0 {
            if i != 0 {
                out.push('\n');
            }
            out.push_str(&format!("{i:02x}: "));
        } else if i % 8 == 0 {
            out.push(' ');
        }
        out.push_str(&format!("{byte:02x} "));
    }
    out.push_str("</tt>\n\n<b>Decoded</b>:\n");
    out.push_str(&format!("  Sleep state     (0x02): {:?}\n", status.sleep));
    out.push_str(&format!("  HDR on          (0x06): {}\n", status.hdr_on));
    out.push_str(&format!("  AI mode  (0x18/0x1c): {:?}\n", status.ai_mode));
    out.push_str(&format!(
        "  Tracking speed  (0x21): {:?}",
        status.tracking_speed
    ));
    out
}

fn report_xu_error(control: &str, err: &XuError) {
    let msg = gettext("Failed to set {name}: {error}")
        .replace("{name}", control)
        .replace("{error}", &err.to_string());
    settings::surface_error(&msg);
}
