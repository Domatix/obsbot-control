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

//! Preset position recall — selector `0x02` frame,
//! `function_group = [0x0a, 0x04, 0xc4, 0x39, 0x14, 0x00]`.
//!
//! Tiny4Linux-only, recall only (no save opcode in either FOSS
//! source — see `PROTOCOL.md §3.2 Q7`). Three slots, 0-indexed in
//! source / 1-based for the UI ("Preset 1 / 2 / 3"). The payload is
//! twenty bytes: a slot byte plus four IEEE-754 little-endian
//! `1.0_f32` floats; the camera rejects the recall without them.

use std::fs::File;

use crate::xu::command02::{build, PRESET_RECALL_APPENDIX};
use crate::xu::errors::XuError;
use crate::xu::transport::{set_cur, BUNIT_ID, SELECTOR_FRAME};

/// `function_group` for preset-recall frames.
pub const FUNCTION_GROUP: [u8; 6] = [0x0a, 0x04, 0xc4, 0x39, 0x14, 0x00];

/// Build the selector-0x02 payload to recall preset slot `index`.
///
/// `index` must be in `0..=2`.
///
/// # Errors
/// Returns [`XuError::InvalidPresetIndex`] if `index` is outside
/// `0..=2`.
pub fn payload(index: i8) -> Result<Vec<u8>, XuError> {
    let (seq, slot) = match index {
        0 => (0x0020, 0u8),
        1 => (0x001a, 1),
        2 => (0x0026, 2),
        other => return Err(XuError::InvalidPresetIndex(other)),
    };
    let cmd = [0u8, 0u8, slot, 0x00, 0x00, 0x00];
    Ok(build(
        FUNCTION_GROUP,
        seq,
        cmd,
        Some(PRESET_RECALL_APPENDIX),
    ))
}

/// Recall preset slot `index` (must be `0..=2`).
///
/// # Errors
/// See [`payload`] and the transport layer.
pub fn recall_preset(camera: &File, index: i8) -> Result<(), XuError> {
    let frame = payload(index)?;
    set_cur(camera, BUNIT_ID, SELECTOR_FRAME, &frame)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payload_preset_1_matches_capture() {
        let frame = payload(0).unwrap();
        assert_eq!(&frame[0..2], &[0xaa, 0x25]);
        assert_eq!(&frame[2..4], &[0x20, 0x00]);
        assert_eq!(&frame[6..8], &[0x6b, 0xdc], "header checksum");
        assert_eq!(&frame[8..12], &[0x0a, 0x04, 0xc4, 0x39]);
        // Payload: length 20, checksum, slot byte, four 1.0f32.
        assert_eq!(&frame[12..14], &[0x14, 0x00]);
        assert_eq!(&frame[14..16], &[0xd6, 0xfb], "payload checksum");
        let mut expected = Vec::from([0u8, 0, 0, 0]);
        expected.extend_from_slice(&PRESET_RECALL_APPENDIX);
        assert_eq!(&frame[16..36], &expected[..]);
    }

    #[test]
    fn payload_preset_2_seq_and_cks() {
        let frame = payload(1).unwrap();
        assert_eq!(&frame[2..4], &[0x1a, 0x00]);
        assert_eq!(&frame[6..8], &[0x4b, 0x03]);
        assert_eq!(&frame[14..16], &[0xeb, 0x2a]);
        assert_eq!(frame[16], 0x01, "slot byte");
    }

    #[test]
    fn payload_preset_3_seq_and_cks() {
        let frame = payload(2).unwrap();
        assert_eq!(&frame[2..4], &[0x26, 0x00]);
        assert_eq!(&frame[6..8], &[0x8b, 0xc3]);
        assert_eq!(&frame[14..16], &[0xaf, 0x19]);
        assert_eq!(frame[16], 0x02, "slot byte");
    }

    #[test]
    fn negative_index_rejected() {
        let err = payload(-1).unwrap_err();
        assert!(matches!(err, XuError::InvalidPresetIndex(-1)));
    }

    #[test]
    fn index_3_rejected() {
        let err = payload(3).unwrap_err();
        assert!(matches!(err, XuError::InvalidPresetIndex(3)));
    }
}
