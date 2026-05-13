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

//! Hardware-dependent integration tests for `obsbot-core::enumerate`.
//!
//! These tests are `#[ignore]`d by default — they require a Tiny 2 family
//! unit physically connected to the host. The user runs them explicitly
//! with `cargo test -p obsbot-core -- --ignored`.
//!
//! CI skips them because hosted runners do not have OBSBOT hardware
//! attached; T-015 wires `cargo test` for the non-ignored set only.

use std::path::Path;

use obsbot_core::{enumerate_cameras, TINY2_FAMILY, VID_OBSBOT};

#[test]
#[ignore = "requires a Tiny 2 family camera plugged into the host"]
fn finds_connected_tiny2_family_unit() {
    let cams = enumerate_cameras();
    assert!(
        !cams.is_empty(),
        "expected at least one Tiny 2 family camera on /sys/class/video4linux; \
         is the camera plugged in and is the user in the `video` group?",
    );

    let cam = &cams[0];
    assert_eq!(cam.vid, VID_OBSBOT, "VID should be OBSBOT's");
    assert!(
        TINY2_FAMILY.contains(&(cam.vid, cam.pid)),
        "(vid={:#06x}, pid={:#06x}) is not in TINY2_FAMILY",
        cam.vid,
        cam.pid,
    );

    let path = cam
        .video_path
        .as_deref()
        .expect("video_path must be set after enumeration");
    assert!(
        path.starts_with(Path::new("/dev")),
        "video_path {path:?} should be under /dev",
    );
    assert!(
        path.exists(),
        "video_path {path:?} should resolve to a real /dev node",
    );

    // Product string is firmware-dependent; just confirm it is non-empty
    // and starts with "OBSBOT" on the Tiny 2 family (defends against
    // surfacing a non-OBSBOT device under our VID by accident — never
    // happens on real Remo Tech hardware but worth pinning).
    assert!(
        cam.product.starts_with("OBSBOT"),
        "product string {:?} should start with `OBSBOT`",
        cam.product,
    );
}
