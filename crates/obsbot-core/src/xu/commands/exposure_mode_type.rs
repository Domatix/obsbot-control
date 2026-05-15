// SPDX-License-Identifier: GPL-3.0-or-later
//
// Portions of this file are derived from EUPL-1.2 source:
//   - cgevans/tiny2        (https://github.com/cgevans/tiny2)
//   - OpenFoxes/Tiny4Linux (https://github.com/OpenFoxes/Tiny4Linux)
// "Licensed under the EUPL"
//
// Copyright (C) 2026 Domatix and contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Exposure-mode toggle (Auto ↔ Manual) — selector `0x02`
//! 36-byte frame, `function_group = [0x0a, 0x02, 0x82, 0x29, 0x05,
//! 0x00]`.
//!
//! Two fixed 36-byte payloads. cgevans's labelling adopted per
//! quirk Q5 (`PROTOCOL.md §3.2`). After putting the camera in
//! Auto, follow with [`crate::xu::commands::face_ae::set_face_ae`]
//! to choose the metering style (Global vs Face).

use std::fs::File;

use crate::xu::command02::build;
use crate::xu::enums::ExposureMode;
use crate::xu::errors::XuError;
use crate::xu::transport::{set_cur, BUNIT_ID, SELECTOR_FRAME};

/// `function_group` for exposure-mode-type frames.
pub const FUNCTION_GROUP: [u8; 6] = [0x0a, 0x02, 0x82, 0x29, 0x05, 0x00];

/// Build the 36-byte selector-0x02 frame for the given mode.
///
/// Per cgevans's labelling (Q5): Auto sends
/// `seq=[0x15,0x00] cks=[0xa8,0x9e] cmd=[0xf9,0x27,0x01,0x32,0x00,0x00]`;
/// Manual sends
/// `seq=[0x16,0x00] cks=[0x58,0x91] cmd=[0xb2,0xaf,0x02,0x04,0x00,0x00]`.
#[must_use]
pub fn payload(mode: ExposureMode) -> [u8; 36] {
    let (seq, cks, cmd) = match mode {
        ExposureMode::Auto => (
            [0x15, 0x00],
            [0xa8, 0x9e],
            [0xf9, 0x27, 0x01, 0x32, 0x00, 0x00],
        ),
        ExposureMode::Manual => (
            [0x16, 0x00],
            [0x58, 0x91],
            [0xb2, 0xaf, 0x02, 0x04, 0x00, 0x00],
        ),
    };
    build(FUNCTION_GROUP, seq, cks, cmd, None)
}

/// Toggle the camera between auto and manual exposure modes.
///
/// # Errors
/// Propagates [`XuError`] from the transport layer.
pub fn set_exposure_mode(camera: &File, mode: ExposureMode) -> Result<(), XuError> {
    set_cur(camera, BUNIT_ID, SELECTOR_FRAME, &payload(mode))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// cgevans's `AUTO_EXP_CMD` is the first 18 bytes of our 36-byte
    /// Auto payload (the remaining 18 are the zero appendix +
    /// command tail, which cgevans's shorter form encodes only
    /// implicitly).
    const CGEVANS_AUTO_18B: [u8; 18] = [
        0xaa, 0x25, 0x15, 0x00, 0x0c, 0x00, 0xa8, 0x9e, 0x0a, 0x02, 0x82, 0x29, 0x05, 0x00, 0xf9,
        0x27, 0x01, 0x32,
    ];

    /// cgevans's `MANUAL_EXP_CMD` ditto.
    const CGEVANS_MANUAL_18B: [u8; 18] = [
        0xaa, 0x25, 0x16, 0x00, 0x0c, 0x00, 0x58, 0x91, 0x0a, 0x02, 0x82, 0x29, 0x05, 0x00, 0xb2,
        0xaf, 0x02, 0x04,
    ];

    #[test]
    fn auto_payload_first_18_bytes_match_cgevans() {
        let frame = payload(ExposureMode::Auto);
        assert_eq!(&frame[0..18], &CGEVANS_AUTO_18B);
        // Bytes 18..20 are the remaining command tail (cgevans's
        // 18-byte form truncates here; ours is the full 36-byte
        // structured frame).
        assert_eq!(&frame[18..20], &[0x00, 0x00]);
        // Bytes 20..36 are the zero appendix.
        assert_eq!(&frame[20..36], &[0u8; 16]);
    }

    #[test]
    fn manual_payload_first_18_bytes_match_cgevans() {
        let frame = payload(ExposureMode::Manual);
        assert_eq!(&frame[0..18], &CGEVANS_MANUAL_18B);
        assert_eq!(&frame[18..20], &[0x00, 0x00]);
        assert_eq!(&frame[20..36], &[0u8; 16]);
    }
}
