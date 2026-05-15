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

//! AI tracking mode — selector `0x06` opcode `0x16`.
//!
//! Wire bytes: `[0x16, 0x02, m, n]`. The `(m, n)` tuple per
//! [`crate::xu::enums::AiMode::to_wire`] — 10 modes.
//!
//! Status reflected at bytes `0x18` (m) and `0x1c` (n) of the
//! 60-byte status struct. Quirk Q4 (`PROTOCOL.md §3.2`): cgevans's
//! setter writes `m=3` for [`crate::xu::enums::AiMode::Hand`] but
//! the decoder reads `m=6`. We mirror the setter (write `m=3`); the
//! status decoder accepts both for the round-trip case.

use std::fs::File;

use crate::xu::enums::AiMode;
use crate::xu::errors::XuError;
use crate::xu::transport::{set_cur, BUNIT_ID, SELECTOR_OPCODE};

/// Opcode byte for AI tracking mode commands.
pub const OPCODE: u8 = 0x16;

/// Build the 4-byte SET_CUR payload for an AI tracking mode.
#[must_use]
pub fn payload(mode: AiMode) -> [u8; 4] {
    let (m, n) = mode.to_wire();
    [OPCODE, 0x02, m, n]
}

/// Set the AI tracking mode on the camera.
///
/// # Errors
/// Propagates [`XuError`] from the transport layer.
pub fn set_ai_mode(camera: &File, mode: AiMode) -> Result<(), XuError> {
    set_cur(camera, BUNIT_ID, SELECTOR_OPCODE, &payload(mode))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payload_no_tracking() {
        assert_eq!(payload(AiMode::NoTracking), [0x16, 0x02, 0x00, 0x00]);
    }

    #[test]
    fn payload_upper_body() {
        assert_eq!(payload(AiMode::UpperBody), [0x16, 0x02, 0x02, 0x01]);
    }

    #[test]
    fn payload_group() {
        assert_eq!(payload(AiMode::Group), [0x16, 0x02, 0x01, 0x00]);
    }

    #[test]
    fn q4_payload_hand_mirrors_cgevans_setter() {
        // The setter writes m=3 verbatim from cgevans even though
        // the status decoder accepts (3,0) and (6,0) as Hand.
        assert_eq!(payload(AiMode::Hand), [0x16, 0x02, 0x03, 0x00]);
    }
}
