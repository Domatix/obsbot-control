// SPDX-License-Identifier: GPL-3.0-or-later
//
// Portions of this file are derived from EUPL-1.2 source:
//   - cgevans/tiny2 (https://github.com/cgevans/tiny2)
// "Licensed under the EUPL"
//
// Copyright (C) 2026 Domatix and contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Field of View — selector `0x06` opcode `0x04`.
//!
//! Wire bytes: `[0x04, 0x01, value]` where `0x01 = Wide (86°)`,
//! `0x02 = Normal (78°)`, `0x03 = Narrow (65°)`. Not in Tiny4Linux;
//! cgevans/tiny2 is the only source.

use std::fs::File;

use crate::xu::enums::FovMode;
use crate::xu::errors::XuError;
use crate::xu::transport::{set_cur, BUNIT_ID, SELECTOR_OPCODE};

/// Opcode byte for FOV commands.
pub const OPCODE: u8 = 0x04;

/// Build the 3-byte SET_CUR payload for FOV selection.
#[must_use]
pub fn payload(mode: FovMode) -> [u8; 3] {
    [OPCODE, 0x01, mode.to_wire()]
}

/// Set the field-of-view on the camera.
///
/// # Errors
/// Propagates [`XuError`] from the transport layer.
pub fn set_fov(camera: &File, mode: FovMode) -> Result<(), XuError> {
    set_cur(camera, BUNIT_ID, SELECTOR_OPCODE, &payload(mode))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payloads() {
        assert_eq!(payload(FovMode::Wide), [0x04, 0x01, 0x01]);
        assert_eq!(payload(FovMode::Normal), [0x04, 0x01, 0x02]);
        assert_eq!(payload(FovMode::Narrow), [0x04, 0x01, 0x03]);
    }
}
