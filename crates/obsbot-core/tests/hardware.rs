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

//! Hardware-dependent integration tests for `obsbot-core::enumerate` and
//! `obsbot-core::controls`.
//!
//! These tests are `#[ignore]`d by default — they require a Tiny 2 family
//! unit physically connected to the host. The user runs them explicitly
//! with `cargo test -p obsbot-core -- --ignored`.
//!
//! CI skips them because hosted runners do not have OBSBOT hardware
//! attached; T-015 wires `cargo test` for the non-ignored set only.

use std::path::Path;

use obsbot_core::{
    enumerate_cameras, read_control, read_controls, write_control, ControlClass, ControlKind,
    ControlValue, TINY2_FAMILY, VID_OBSBOT,
};

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

#[test]
#[ignore = "requires a Tiny 2 family camera plugged into the host"]
fn reads_v4l2_controls_from_connected_unit() {
    let cams = enumerate_cameras();
    assert!(
        !cams.is_empty(),
        "expected at least one Tiny 2 family camera; is one plugged in?",
    );

    let path = cams[0]
        .video_path
        .as_deref()
        .expect("video_path must be set after enumeration");

    let controls =
        read_controls(path).expect("read_controls should succeed on a Tiny 2 family unit");

    // Tiny 2 Lite firmware 5.10 exposes 22 controls in the V4L2 surface
    // (12 User + 10 Camera) — cross-checked against `v4l2-ctl
    // --list-ctrls` 2026-05-13. PROTOCOL.md §2's "13 + 11 = 24"
    // tabulation overcounts by 2 (it appears to have counted the class
    // headers as controls); the v4l2 query enumeration is authoritative.
    // Use `>=` so a future firmware that exposes more does not regress.
    assert!(
        controls.len() >= 22,
        "expected ≥22 controls, got {}",
        controls.len(),
    );

    // Sanity: at least one User and one Camera class entry must show up.
    let has_user = controls.iter().any(|c| c.class == ControlClass::User);
    let has_camera = controls.iter().any(|c| c.class == ControlClass::Camera);
    assert!(has_user, "no User-class controls surfaced");
    assert!(has_camera, "no Camera-class controls surfaced");

    // Brightness (User, integer) should be present on every Tiny 2
    // family unit per PROTOCOL §2.1.
    let brightness = controls
        .iter()
        .find(|c| c.name.eq_ignore_ascii_case("Brightness"))
        .expect("Brightness control is part of the Tiny 2 family V4L2 surface");
    assert_eq!(brightness.class, ControlClass::User);
    assert!(
        matches!(brightness.kind, ControlKind::Integer { .. }),
        "Brightness must be an integer-typed control",
    );
    // V4L2_CID_BRIGHTNESS, used by the T-100 round-trip test below.
    assert_eq!(brightness.id, 0x0098_0900);
}

#[test]
#[ignore = "requires a Tiny 2 family camera plugged into the host"]
fn writes_v4l2_brightness_round_trip() {
    let cams = enumerate_cameras();
    let path = cams
        .first()
        .and_then(|c| c.video_path.as_deref())
        .expect("a connected Tiny 2 family camera with a /dev/videoN node");

    let controls = read_controls(path).expect("read_controls succeeds on the connected unit");
    let brightness = controls
        .iter()
        .find(|c| c.name.eq_ignore_ascii_case("Brightness"))
        .expect("Brightness control must be present");
    let ControlKind::Integer {
        current,
        min,
        max,
        step,
        ..
    } = brightness.kind
    else {
        panic!(
            "Brightness must be Integer-typed; got {:?}",
            brightness.kind
        );
    };

    // Pick a target one step away that still sits inside [min, max].
    let step_i64 = i64::try_from(step).expect("step fits in i64");
    let target = if current + step_i64 <= max {
        current + step_i64
    } else {
        current - step_i64
    };
    assert!(target >= min && target <= max, "target out of envelope");

    // Write target, read back, restore.
    write_control(path, brightness.id, ControlValue::Integer(target))
        .expect("brightness write must succeed");
    let after = read_controls(path).expect("re-read after write succeeds");
    let after_brightness = after
        .iter()
        .find(|c| c.id == brightness.id)
        .expect("brightness still enumerates after write");
    let ControlKind::Integer {
        current: read_back, ..
    } = after_brightness.kind
    else {
        panic!("brightness type changed mid-test");
    };
    assert_eq!(
        read_back, target,
        "read-back ({read_back}) must match the written target ({target})",
    );

    // Restore original — leave the camera in the state we found it.
    write_control(path, brightness.id, ControlValue::Integer(current))
        .expect("brightness restore must succeed");
}

#[test]
#[ignore = "requires a Tiny 2 family camera plugged into the host"]
fn writes_v4l2_zoom_absolute_round_trip() {
    // Mirror of the brightness round-trip for the T-101 PTZ pad's
    // zoom slider. zoom_absolute = V4L2_CID_ZOOM_ABSOLUTE = 0x009a090d
    // per PROTOCOL §2.2.
    let cams = enumerate_cameras();
    let path = cams
        .first()
        .and_then(|c| c.video_path.as_deref())
        .expect("a connected Tiny 2 family camera with a /dev/videoN node");

    let controls = read_controls(path).expect("read_controls succeeds on the connected unit");
    let zoom = controls
        .iter()
        .find(|c| c.id == 0x009a_090d)
        .expect("zoom_absolute must be present on Tiny 2 family");
    let ControlKind::Integer {
        current, min, max, ..
    } = zoom.kind
    else {
        panic!("zoom_absolute must be Integer-typed; got {:?}", zoom.kind);
    };

    let target = if current + 5 <= max {
        current + 5
    } else if current - 5 >= min {
        current - 5
    } else {
        // Pathological tiny range — pick the other endpoint.
        if current == max {
            min
        } else {
            max
        }
    };

    write_control(path, zoom.id, ControlValue::Integer(target)).expect("zoom write must succeed");
    let after = read_controls(path).expect("re-read after write succeeds");
    let after_zoom = after
        .iter()
        .find(|c| c.id == zoom.id)
        .expect("zoom_absolute still enumerates after write");
    let ControlKind::Integer {
        current: read_back, ..
    } = after_zoom.kind
    else {
        panic!("zoom_absolute type changed mid-test");
    };
    assert_eq!(read_back, target, "zoom read-back mismatch");

    // Restore — keep the camera frame at its original framing.
    write_control(path, zoom.id, ControlValue::Integer(current))
        .expect("zoom restore must succeed");
}

#[test]
#[ignore = "requires a Tiny 2 family camera plugged into the host"]
fn reads_single_v4l2_control_just_in_time() {
    // Covers the T-101 hot-fix path: the PTZ pad needs to read
    // `pan_absolute` / `tilt_absolute` just-in-time on every click
    // so its delta lands relative to the camera's real position
    // rather than a stale page-open snapshot (see ptz_pad.rs
    // module-level doc-block). This test exercises the singular
    // `read_control` against `pan_absolute` and cross-checks the
    // result against the bulk `read_controls` walk — they must
    // agree at a moment when the camera is otherwise still.
    let cams = enumerate_cameras();
    let path = cams
        .first()
        .and_then(|c| c.video_path.as_deref())
        .expect("a connected Tiny 2 family camera with a /dev/videoN node");

    let bulk = read_controls(path).expect("read_controls succeeds on the connected unit");
    let pan_descriptor = bulk
        .iter()
        .find(|c| c.id == 0x009a_0908)
        .expect("pan_absolute must be present on the Tiny 2 family");
    let ControlKind::Integer {
        current: bulk_current,
        ..
    } = pan_descriptor.kind
    else {
        panic!("pan_absolute must be Integer-typed");
    };

    let single = read_control(path, 0x009a_0908).expect("read_control(pan_absolute) must succeed");
    let ControlValue::Integer(single_current) = single else {
        panic!("read_control(pan_absolute) must return Integer; got {single:?}");
    };

    // The two reads happen within microseconds of each other against
    // a still camera, so they must agree exactly.
    assert_eq!(
        single_current, bulk_current,
        "singular read_control (={single_current}) must match the bulk \
         read_controls value (={bulk_current}) for pan_absolute",
    );
}

#[test]
#[ignore = "requires a Tiny 2 family camera plugged into the host"]
fn writes_v4l2_power_line_frequency_round_trip() {
    // Exercises the T-102 menu-write path on a known menu control.
    // V4L2_CID_POWER_LINE_FREQUENCY = 0x00980918 per PROTOCOL §2.1.
    // Per PROTOCOL §2.3 Q1 the kernel reports default=3 for this
    // control even though the menu only declares {0,1,2}; the test
    // should therefore restore by reading the *current* value at
    // start, not by trusting the reported default.
    let cams = enumerate_cameras();
    let path = cams
        .first()
        .and_then(|c| c.video_path.as_deref())
        .expect("a connected Tiny 2 family camera with a /dev/videoN node");

    let controls = read_controls(path).expect("read_controls succeeds on the connected unit");
    let plf = controls
        .iter()
        .find(|c| c.id == 0x0098_0918)
        .expect("power_line_frequency must be present on Tiny 2 family");
    let ControlKind::Menu {
        current, options, ..
    } = &plf.kind
    else {
        panic!(
            "power_line_frequency must be Menu-typed; got {:?}",
            plf.kind
        );
    };
    assert!(
        options.len() >= 3,
        "power_line_frequency must advertise at least Disabled/50/60 Hz"
    );

    // Pick a *different* menu option than current.
    let target = options
        .iter()
        .map(|(id, _)| *id)
        .find(|id| *id != *current)
        .expect("power_line_frequency has at least two options");

    write_control(path, plf.id, ControlValue::Menu(target))
        .expect("power_line_frequency write must succeed");
    let after = read_controls(path).expect("re-read after write succeeds");
    let after_plf = after
        .iter()
        .find(|c| c.id == plf.id)
        .expect("power_line_frequency still enumerates after write");
    let ControlKind::Menu {
        current: read_back, ..
    } = after_plf.kind
    else {
        panic!("power_line_frequency type changed mid-test");
    };
    assert_eq!(read_back, target, "power_line_frequency read-back mismatch");

    // Restore.
    write_control(path, plf.id, ControlValue::Menu(*current))
        .expect("power_line_frequency restore must succeed");
}
