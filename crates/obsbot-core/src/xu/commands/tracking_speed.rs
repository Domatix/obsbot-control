// SPDX-License-Identifier: GPL-3.0-or-later
//
// Portions of this file are derived from EUPL-1.2 source:
//   - OpenFoxes/Tiny4Linux (https://github.com/OpenFoxes/Tiny4Linux)
// "Licensed under the EUPL"
//
// Copyright (C) 2026 Domatix and contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Tracking speed — selector `0x02` frame,
//! `function_group = [0x0a, 0x04, 0xc4, 0x0c, 0x01, 0x00]`.
//!
//! Tiny4Linux-only. Status reflected at byte `0x21` of the 60-byte
//! GET_CUR status struct. Two variants: Standard / Sport. Quirk Q6
//! flags `0x21 == 0x01` as an unmapped gap that may or may not be a
//! third "Headroom" mode.

use std::fs::File;

use crate::xu::command02::build;
use crate::xu::enums::TrackingSpeed;
use crate::xu::errors::XuError;
use crate::xu::transport::{set_cur, BUNIT_ID, SELECTOR_FRAME};

/// `function_group` for tracking-speed frames.
pub const FUNCTION_GROUP: [u8; 6] = [0x0a, 0x04, 0xc4, 0x0c, 0x01, 0x00];

/// Build the selector-0x02 frame for the given tracking speed.
///
/// The checksum bytes of the captured command block are no longer
/// used: [`build`] recomputes both checksums from the frame contents.
#[must_use]
pub fn payload(speed: TrackingSpeed) -> Vec<u8> {
    let (seq, cmd) = match speed {
        TrackingSpeed::Standard => (0x0020, [0xe6, 0x3f, 0x00, 0x00, 0x00, 0x00]),
        TrackingSpeed::Sport => (0x0021, [0x67, 0xfe, 0x02, 0x00, 0x00, 0x00]),
    };
    build(FUNCTION_GROUP, seq, cmd, None)
}

/// Set the camera's tracking speed.
///
/// # Errors
/// Propagates [`XuError`] from the transport layer.
pub fn set_tracking_speed(camera: &File, speed: TrackingSpeed) -> Result<(), XuError> {
    set_cur(camera, BUNIT_ID, SELECTOR_FRAME, &payload(speed))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payload_standard_matches_capture() {
        assert_eq!(
            payload(TrackingSpeed::Standard),
            hex("aa2520000c00abcb0a04c40c0100e63f00")
        );
    }

    #[test]
    fn payload_sport_matches_capture() {
        assert_eq!(
            payload(TrackingSpeed::Sport),
            hex("aa2521000c00fa0e0a04c40c010067fe02")
        );
    }

    fn hex(s: &str) -> Vec<u8> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).expect("hex"))
            .collect()
    }
}
