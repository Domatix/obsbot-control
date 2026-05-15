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

//! "AI and effects" group widget (T-301).
//!
//! Mounts the vendor XU surface for the Tiny 2 family inside a
//! single `AdwPreferencesGroup` placed at the **top** of the
//! per-camera controls page (above the PTZ pad). The widgets:
//!
//! * `AdwComboRow` — AI tracking mode (10 entries).
//! * `AdwComboRow` — Tracking speed (Standard / Sport).
//! * `AdwComboRow` — Field of view (Wide / Normal / Narrow).
//! * `AdwSwitchRow` — HDR on/off.
//!
//! Writes go through `obsbot_core::xu::commands::*` on the GTK main
//! thread (the ioctls are sub-millisecond on the user's hardware;
//! profiling will dictate whether to lift the path off-thread).
//! Failures surface as toasts via `settings::surface_error` (T-108).
//!
//! The group is **hidden entirely** for non-Tiny-2-family cameras
//! (per ADR-0014 the project is best-effort on other OBSBOT models,
//! and the XU bytes here are not portable to other vendor cameras).
//!
//! **Scope changes during T-301 live validation** (see commit log
//! for the fix that landed them):
//!
//! * `Exposure mode` row removed — redundant with the V4L2
//!   standard `auto_exposure` menu in the Exposure group (T-104),
//!   which already exposes 3 values (Auto Mode / Manual / Aperture
//!   Priority), greys the exposure-time slider via the kernel
//!   INACTIVE flag, and uses kernel-supplied labels (no risk of
//!   the Q5 label-swap surfacing). The underlying XU encoder
//!   (`obsbot_core::xu::commands::exposure_mode_type`) stays
//!   available for any future caller.
//! * `Face metering` row removed — only meters correctly when
//!   the camera is in auto-exposure *via the XU frame*, not
//!   via the V4L2 standard route the user actually takes. The
//!   AI tracking modes UpperBody / CloseUp already cover the
//!   "frame my face" intent.
//!
//! Per-camera persistence of these values is **not yet wired** —
//! the existing T-105 schema (`cameras a{sa{si}}`) and the
//! `settings::write_and_save` path (which reads a different
//! `control-values` key) are mismatched in this branch's `main`,
//! and untangling them is out of T-301's scope. Values are
//! re-hydrated from `xu::get_status()` on every page open, which
//! is correct (the camera firmware is the source of truth) but
//! does not survive a power-cycle of the camera. A follow-on
//! task can add proper XU persistence once T-105's plumbing is
//! sorted out.

// Same rationale as `extras_view`: doc-markdown false-positives
// on `Tiny4Linux`, `cgevans`, `UpperBody`, etc. would force
// noisy edits across prose. Genuine code references in doc
// comments are still backticked manually.
#![allow(clippy::doc_markdown)]

use std::fs::{File, OpenOptions};
use std::rc::Rc;

use gtk4 as gtk;
use libadwaita as adw;

use adw::prelude::*;
use obsbot_core::xu::commands::{set_ai_mode, set_fov, set_hdr, set_tracking_speed};
use obsbot_core::xu::{get_status, AiMode, FovMode, Status, TrackingSpeed, XuError};
use obsbot_core::{CameraInfo, TINY2_FAMILY};

use crate::i18n::gettext;
use crate::settings;

/// AI mode order in the dropdown (user-facing). Independent of the
/// wire `(m, n)` encoding; ordered by how OBSBOT Center presents the
/// list, then by approximate frequency-of-use.
const AI_MODE_ORDER: &[AiMode] = &[
    AiMode::NoTracking,
    AiMode::NormalTracking,
    AiMode::UpperBody,
    AiMode::CloseUp,
    AiMode::Headless,
    AiMode::LowerBody,
    AiMode::Group,
    AiMode::DeskMode,
    AiMode::Whiteboard,
    AiMode::Hand,
];

/// FOV combo order.
const FOV_ORDER: &[FovMode] = &[FovMode::Wide, FovMode::Normal, FovMode::Narrow];

/// Tracking speed combo order (T-302).
const TRACKING_SPEED_ORDER: &[TrackingSpeed] = &[TrackingSpeed::Standard, TrackingSpeed::Sport];

/// Returns `true` if `(vid, pid)` is an OBSBOT Tiny 2 family unit.
fn is_tiny_2_family(vid: u16, pid: u16) -> bool {
    TINY2_FAMILY.iter().any(|&(v, p)| v == vid && p == pid)
}

/// Build the "AI & Effects" preferences group, or `None` for cameras
/// that are not Tiny 2 family or have no `/dev/videoN` path.
///
/// The group's contained widgets share a single open `File` handle
/// (wrapped in `Rc<File>` so closures can clone-and-capture without
/// re-opening per signal). If the open fails — typically a
/// permission error or a device that disappeared between
/// enumeration and page-build — the entire group is omitted; the
/// rest of the controls page still renders. The caller logs the
/// open failure to stderr.
pub fn build_ai_effects_group(cam: &CameraInfo) -> Option<adw::PreferencesGroup> {
    if !is_tiny_2_family(cam.vid, cam.pid) {
        return None;
    }
    let path = cam.video_path.as_deref()?;
    let file = match OpenOptions::new().read(true).write(true).open(path) {
        Ok(f) => Rc::new(f),
        Err(err) => {
            eprintln!(
                "warning: ai_effects_view: open {path:?} for XU writes failed: {err}; \
                 skipping AI and effects group"
            );
            return None;
        }
    };

    // Best-effort hydration. Failure here is non-fatal — non-Tiny-2
    // descriptors that happen to share the VID/PID would land here
    // (none known); we render the group with widgets in their
    // dropdown-default state and let the user discover via the
    // surface_error toast on first interaction.
    let baseline = get_status(&file).ok();

    let group = adw::PreferencesGroup::builder()
        // GNOME HIG: prefer "and" over "&" — and libadwaita treats
        // the title as Pango markup so a literal "&" would break the
        // entity parser ("entity does not end with a semicolon").
        .title(gettext("AI and effects"))
        .description(gettext("Vendor controls for OBSBOT Tiny 2 family cameras."))
        .build();

    group.add(&ai_mode_row(&file, baseline.as_ref()));
    group.add(&tracking_speed_row(&file, baseline.as_ref()));
    group.add(&fov_row(&file));
    group.add(&hdr_row(&file, baseline.as_ref()));

    Some(group)
}

fn ai_mode_row(file: &Rc<File>, baseline: Option<&Status>) -> adw::ComboRow {
    let labels: Vec<String> = AI_MODE_ORDER.iter().map(|m| ai_mode_label(*m)).collect();
    let label_refs: Vec<&str> = labels.iter().map(String::as_str).collect();
    let model = gtk::StringList::new(&label_refs);

    let selected_idx = baseline
        .and_then(|s| {
            AI_MODE_ORDER
                .iter()
                .position(|m| *m == s.ai_mode)
                .and_then(|i| u32::try_from(i).ok())
        })
        .unwrap_or(0);

    let row = adw::ComboRow::builder()
        .title(gettext("AI tracking"))
        .subtitle(gettext("Auto-frames a face, group, hand, or other subject"))
        .model(&model)
        .selected(selected_idx)
        .build();

    let file_for_cb = Rc::clone(file);
    row.connect_selected_notify(move |row| {
        let Ok(idx) = usize::try_from(row.selected()) else {
            return;
        };
        let Some(&mode) = AI_MODE_ORDER.get(idx) else {
            return;
        };
        if let Err(err) = set_ai_mode(&file_for_cb, mode) {
            report_xu_error("AI tracking mode", &err);
        }
    });
    row
}

fn fov_row(file: &Rc<File>) -> adw::ComboRow {
    // FOV is not surfaced in the GET_CUR status struct, so we cannot
    // hydrate the current selection from the camera — we mark the
    // default as Wide and let the first user interaction send the
    // wire bytes. (The widget then reflects whatever the user picked
    // for the rest of the session; a future enhancement would issue
    // a GET_CUR on selector 0x06 op 0x04 to read the current FOV
    // back, but neither cgevans nor Tiny4Linux ship a getter for it.)
    //
    // Caveat — PROTOCOL.md §3.2 Q8 — selecting "Narrow (65°)" on
    // the user's Tiny 2 Lite (3564:fef9, bcdDevice 5.10) produced
    // no visible crop change during T-301 live validation, while
    // Wide and Normal worked. The wire byte we send (`[0x04, 0x01,
    // 0x03]`) is identical to cgevans's setter; we suspect the
    // Lite's hardware lacks the Narrow optical path the regular
    // Tiny 2 ships with. Kept in the dropdown so regular-Tiny-2
    // owners can use it — the subtitle warns about the Lite case.
    let labels = [
        gettext("Wide (86°)"),
        gettext("Normal (78°)"),
        gettext("Narrow (65°)"),
    ];
    let label_refs: Vec<&str> = labels.iter().map(String::as_str).collect();
    let model = gtk::StringList::new(&label_refs);

    let row = adw::ComboRow::builder()
        .title(gettext("Field of view"))
        .subtitle(gettext(
            "Lens angle of view (digital crop). Narrow (65°) does not apply on \
             Tiny 2 Lite — see PROTOCOL §3.2 Q8.",
        ))
        .model(&model)
        .selected(0)
        .build();

    let file_for_cb = Rc::clone(file);
    row.connect_selected_notify(move |row| {
        let Ok(idx) = usize::try_from(row.selected()) else {
            return;
        };
        let Some(&mode) = FOV_ORDER.get(idx) else {
            return;
        };
        if let Err(err) = set_fov(&file_for_cb, mode) {
            report_xu_error("Field of view", &err);
        }
    });
    row
}

fn hdr_row(file: &Rc<File>, baseline: Option<&Status>) -> adw::SwitchRow {
    let row = adw::SwitchRow::builder()
        .title(gettext("HDR"))
        .subtitle(gettext(
            "High Dynamic Range — improves contrast in mixed-lighting scenes",
        ))
        .active(baseline.is_some_and(|s| s.hdr_on))
        .build();

    let file_for_cb = Rc::clone(file);
    row.connect_active_notify(move |row| {
        if let Err(err) = set_hdr(&file_for_cb, row.is_active()) {
            report_xu_error("HDR", &err);
        }
    });
    row
}

// `exposure_mode_row` and `face_ae_row` were removed during the
// T-301 live validation pass (PROTOCOL.md §3.2 quirks Q5 +
// follow-on). Rationale:
//
// * The V4L2 standard control `auto_exposure` (User-class menu in
//   the "Exposure" group lower on the page, landed by T-104)
//   already exposes the same firmware state with three values
//   (Auto Mode / Manual / Aperture Priority), greys out the
//   "Exposure Time, Absolute" slider via the kernel INACTIVE
//   flag, and uses the kernel's own labels — no risk of the Q5
//   label-swap surfacing again.
// * `set_face_ae` (selector 0x06 op 0x03) only meters correctly
//   when the camera is in auto-exposure *via the XU selector-0x02
//   frame*, not when V4L2 puts the camera in auto. Without an
//   in-app guarantee that the user took the XU path, the
//   Face-AE toggle could silently no-op. The AI tracking modes
//   `UpperBody` / `CloseUp` already cover the "frame my face"
//   intent end-users care about.
//
// The byte-level encoders for both stay available in
// `obsbot-core::xu::commands::{exposure_mode_type, face_ae}` for
// any future caller; they are just not surfaced in this widget.

fn tracking_speed_row(file: &Rc<File>, baseline: Option<&Status>) -> adw::ComboRow {
    let labels = [gettext("Standard"), gettext("Sport")];
    let label_refs: Vec<&str> = labels.iter().map(String::as_str).collect();
    let model = gtk::StringList::new(&label_refs);

    let selected_idx = baseline
        .and_then(|s| {
            TRACKING_SPEED_ORDER
                .iter()
                .position(|t| *t == s.tracking_speed)
                .and_then(|i| u32::try_from(i).ok())
        })
        .unwrap_or(0);

    let row = adw::ComboRow::builder()
        .title(gettext("Tracking speed"))
        .subtitle(gettext(
            "Standard is smooth and low-acceleration; Sport reacts faster.",
        ))
        .model(&model)
        .selected(selected_idx)
        .build();

    let file_for_cb = Rc::clone(file);
    row.connect_selected_notify(move |row| {
        let Ok(idx) = usize::try_from(row.selected()) else {
            return;
        };
        let Some(&mode) = TRACKING_SPEED_ORDER.get(idx) else {
            return;
        };
        if let Err(err) = set_tracking_speed(&file_for_cb, mode) {
            report_xu_error("Tracking speed", &err);
        }
    });
    row
}

fn ai_mode_label(mode: AiMode) -> String {
    match mode {
        AiMode::NoTracking => gettext("No tracking"),
        AiMode::NormalTracking => gettext("Normal tracking"),
        AiMode::UpperBody => gettext("Upper body"),
        AiMode::CloseUp => gettext("Close-up"),
        AiMode::Headless => gettext("Headless"),
        AiMode::LowerBody => gettext("Lower body"),
        AiMode::Group => gettext("Group"),
        AiMode::DeskMode => gettext("Desk mode"),
        AiMode::Whiteboard => gettext("Whiteboard"),
        AiMode::Hand => gettext("Hand"),
    }
}

/// Format an XU error as a single line and route it to the T-108
/// toast helper.
fn report_xu_error(control: &str, err: &XuError) {
    let msg = gettext("Failed to set {name}: {error}")
        .replace("{name}", control)
        .replace("{error}", &err.to_string());
    settings::surface_error(&msg);
}
