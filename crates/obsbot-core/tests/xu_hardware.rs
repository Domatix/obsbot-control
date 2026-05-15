// SPDX-License-Identifier: GPL-3.0-or-later
//
// Copyright (C) 2026 Domatix and contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Hardware-gated integration test for `obsbot-core::xu`.
//!
//! Requires the user's Tiny 2 / Tiny 2 Lite plugged in at
//! `/dev/video0` (or wherever `obsbot_core::enumerate_cameras` finds
//! it). The test is `#[ignore]`d by default; run explicitly with:
//!
//! ```text
//! cargo test -p obsbot-core --test xu_hardware -- --ignored
//! ```
//!
//! Only reads + a single HDR round-trip — no AI mode change, no
//! preset recall, no exposure flip; those have visible consequences
//! and belong in the user-driven T-303 validation matrix.

use std::fs::OpenOptions;

use obsbot_core::enumerate::enumerate_cameras;
use obsbot_core::xu::commands::set_hdr;
use obsbot_core::xu::{get_status, AiMode};

#[test]
#[ignore = "hardware-gated; run with --ignored when Tiny 2 is plugged in"]
fn hdr_round_trip_against_real_tiny_2() {
    let cams = enumerate_cameras();
    let cam = cams
        .into_iter()
        .find(|c| c.video_path.is_some())
        .expect("no OBSBOT Tiny 2 family camera detected");

    let path = cam.video_path.expect("camera has a video path");
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&path)
        .unwrap_or_else(|e| panic!("opening {path:?} failed: {e}"));

    // Baseline.
    let baseline = get_status(&file).expect("get_status baseline");
    eprintln!(
        "baseline: hdr_on={} sleep={:?} ai_mode={:?} tracking_speed={:?}",
        baseline.hdr_on, baseline.sleep, baseline.ai_mode, baseline.tracking_speed
    );

    // Toggle HDR to the opposite of baseline, confirm, restore.
    let toggled = !baseline.hdr_on;
    set_hdr(&file, toggled).expect("set_hdr toggled");
    let after = get_status(&file).expect("get_status after toggle");
    assert_eq!(after.hdr_on, toggled, "HDR did not flip");

    // Sanity: AI mode is one of the known variants (every Tiny 2
    // family unit reports something here, even when idle).
    let _: AiMode = after.ai_mode;

    // Restore baseline so the user's camera ends where it started.
    set_hdr(&file, baseline.hdr_on).expect("set_hdr restore");
    let restored = get_status(&file).expect("get_status restore");
    assert_eq!(
        restored.hdr_on, baseline.hdr_on,
        "HDR did not return to baseline"
    );
}
