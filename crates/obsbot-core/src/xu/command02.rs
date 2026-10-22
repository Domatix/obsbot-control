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

//! Command helpers for XU selector `0x02`.
//!
//! The per-command files under `commands/` (`exposure_mode_type`,
//! `sleep`, `tracking_speed`, `preset`, `gesture`) describe their
//! command as captured from the vendor application: a six-byte
//! function group and a six-byte command block. Once the frame format
//! is spelled out (`PROTOCOL.md` §3.2, "Frame layout and checksum
//! decoded"), those two blocks decompose as:
//!
//! ```text
//! function_group[0..4]   function group
//! function_group[4..6]   payload length, little endian
//! command[0..2]          payload CRC16 as captured (recomputed by encode)
//! command[2..6]          first four payload bytes
//! appendix (optional)    further payload bytes
//! ```
//!
//! [`build`] keeps the captured constants readable while producing the
//! frame through [`crate::xu::frame::encode`], so both checksums are
//! computed instead of replayed.

use crate::xu::frame::encode;

/// Compose a selector-0x02 frame from the captured constants of a
/// command.
///
/// `function_group` is the six-byte block from `PROTOCOL.md` §3.2;
/// its last two bytes are the payload length. `command` is the
/// six-byte block; its first two bytes are the captured payload
/// checksum, which is ignored here because [`encode`] recomputes it.
/// `appendix` supplies payload bytes past the first four; commands
/// without appendix pad with zeroes up to the declared length.
#[must_use]
pub fn build(
    function_group: [u8; 6],
    seq: u16,
    command: [u8; 6],
    appendix: Option<[u8; 16]>,
) -> Vec<u8> {
    let len = usize::from(u16::from_le_bytes([function_group[4], function_group[5]]));
    let mut data = [0u8; 20];
    data[0..4].copy_from_slice(&command[2..6]);
    if let Some(app) = appendix {
        data[4..20].copy_from_slice(&app);
    }
    encode(
        seq,
        [
            function_group[0],
            function_group[1],
            function_group[2],
            function_group[3],
        ],
        &data[..len],
    )
}

/// 16-byte appendix for Preset recall frames: four IEEE-754
/// little-endian `1.0_f32` floats. The camera rejects the recall
/// with a zero appendix; this constant must be passed as-is.
pub const PRESET_RECALL_APPENDIX: [u8; 16] = {
    // 1.0_f32 little-endian = [0x00, 0x00, 0x80, 0x3f]; repeated 4x.
    let one_le = 1.0_f32.to_le_bytes();
    let mut out = [0u8; 16];
    let mut i = 0;
    while i < 4 {
        let base = i * 4;
        out[base] = one_le[0];
        out[base + 1] = one_le[1];
        out[base + 2] = one_le[2];
        out[base + 3] = one_le[3];
        i += 1;
    }
    out
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_appendix_default_matches_explicit_zeros() {
        let with_default = build(
            [0x0a, 0x02, 0x82, 0x29, 0x05, 0x00],
            0x0015,
            [0xf9, 0x27, 0x01, 0x32, 0x00, 0x00],
            None,
        );
        let with_explicit_zeros = build(
            [0x0a, 0x02, 0x82, 0x29, 0x05, 0x00],
            0x0015,
            [0xf9, 0x27, 0x01, 0x32, 0x00, 0x00],
            Some([0u8; 16]),
        );
        assert_eq!(with_default, with_explicit_zeros);
    }

    #[test]
    fn frame_leader_and_header_are_fixed() {
        let f = build([0, 0, 0, 0, 1, 0], 0xabcd, [0, 0, 0, 0, 0, 0], None);
        assert_eq!(&f[0..2], &[0xaa, 0x25]);
        assert_eq!(&f[2..4], &[0xcd, 0xab]);
        assert_eq!(&f[4..6], &[0x0c, 0x00]);
    }

    #[test]
    fn preset_recall_appendix_is_four_ones() {
        // Cross-check: each 4-byte slice must be 1.0_f32 LE.
        for chunk in PRESET_RECALL_APPENDIX.chunks(4) {
            let arr: [u8; 4] = chunk.try_into().unwrap();
            assert!((f32::from_le_bytes(arr) - 1.0).abs() < f32::EPSILON);
        }
        // Spot-check the raw bytes documented in PROTOCOL.md §3.2.
        assert_eq!(
            PRESET_RECALL_APPENDIX,
            [
                0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f, 0x00, 0x00,
                0x80, 0x3f,
            ]
        );
    }
}
