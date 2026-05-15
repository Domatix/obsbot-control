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

//! 36-byte frame builder for XU selector `0x02`.
//!
//! The OBSBOT firmware accepts a fixed-shape 36-byte structured
//! frame on selector `0x02`; the per-command files under
//! `commands/` (`exposure_mode_type`, `sleep`, `tracking_speed`,
//! `preset`) supply the variable fields. Layout per
//! `PROTOCOL.md §3.2`:
//!
//! ```text
//! byte  0..1   FRAME_ID         = [0xaa, 0x25]   (fixed)
//! byte  2..3   sequence_nr      (per-command)
//! byte  4..5   SEGMENT_SIZE     = [0x0c, 0x00]   (fixed)
//! byte  6..7   checksum         (per-command)
//! byte  8..13  function_group   (6 bytes — subsystem id)
//! byte 14..19  command          (6 bytes — op + value)
//! byte 20..35  appendix         (16 bytes — typically zero;
//!                                non-zero for Preset recall)
//! ```
//!
//! Tiny4Linux uses the `bon` crate to build this with named-argument
//! ergonomics. We reproduce the same layout in plain Rust to avoid
//! the dep (the layout never changes; named args are nice but not
//! load-bearing).

/// Fixed frame leader on every selector-0x02 write.
const FRAME_ID: [u8; 2] = [0xaa, 0x25];
/// Fixed segment size — declares "12 bytes of payload follow"
/// (function_group + command); always `[0x0c, 0x00]`.
const SEGMENT_SIZE: [u8; 2] = [0x0c, 0x00];

/// Compose the 36-byte frame for a selector-0x02 `SET_CUR`.
///
/// `appendix` is `None` ⇒ 16 zero bytes (the common case).
/// Pass `Some([..])` for Preset recalls, which require four
/// IEEE-754 little-endian `1.0_f32` floats (= `[0x00, 0x00, 0x80,
/// 0x3f]` repeated four times). See `PROTOCOL.md §3.2 Q7`.
#[must_use]
pub fn build(
    function_group: [u8; 6],
    sequence_nr: [u8; 2],
    checksum: [u8; 2],
    command: [u8; 6],
    appendix: Option<[u8; 16]>,
) -> [u8; 36] {
    let mut out = [0u8; 36];
    out[0..2].copy_from_slice(&FRAME_ID);
    out[2..4].copy_from_slice(&sequence_nr);
    out[4..6].copy_from_slice(&SEGMENT_SIZE);
    out[6..8].copy_from_slice(&checksum);
    out[8..14].copy_from_slice(&function_group);
    out[14..20].copy_from_slice(&command);
    if let Some(app) = appendix {
        out[20..36].copy_from_slice(&app);
    }
    out
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
            [0x15, 0x00],
            [0xa8, 0x9e],
            [0xf9, 0x27, 0x01, 0x32, 0x00, 0x00],
            None,
        );
        let with_explicit_zeros = build(
            [0x0a, 0x02, 0x82, 0x29, 0x05, 0x00],
            [0x15, 0x00],
            [0xa8, 0x9e],
            [0xf9, 0x27, 0x01, 0x32, 0x00, 0x00],
            Some([0u8; 16]),
        );
        assert_eq!(with_default, with_explicit_zeros);
    }

    #[test]
    fn frame_leader_and_segment_size_are_fixed() {
        let f = build([0; 6], [0xab, 0xcd], [0xef, 0x01], [0; 6], None);
        assert_eq!(&f[0..2], &[0xaa, 0x25]);
        assert_eq!(&f[2..4], &[0xab, 0xcd]);
        assert_eq!(&f[4..6], &[0x0c, 0x00]);
        assert_eq!(&f[6..8], &[0xef, 0x01]);
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
