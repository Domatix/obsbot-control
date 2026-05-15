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

//! HDR toggle — selector `0x06` opcode `0x01`.
//!
//! Wire bytes: `[0x01, 0x01, value]` where `value ∈ {0x00, 0x01}`.
//! Status reflected at byte `0x06` of the 60-byte status struct.

use std::fs::File;

use crate::xu::errors::XuError;
use crate::xu::transport::{set_cur, BUNIT_ID, SELECTOR_OPCODE};

/// Opcode byte for HDR commands.
pub const OPCODE: u8 = 0x01;

/// Build the 3-byte SET_CUR payload for HDR on/off.
#[must_use]
pub fn payload(on: bool) -> [u8; 3] {
    [OPCODE, 0x01, u8::from(on)]
}

/// Toggle HDR on the camera.
///
/// # Errors
/// Propagates [`XuError`] from the transport layer.
pub fn set_hdr(camera: &File, on: bool) -> Result<(), XuError> {
    set_cur(camera, BUNIT_ID, SELECTOR_OPCODE, &payload(on))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payload_off() {
        assert_eq!(payload(false), [0x01, 0x01, 0x00]);
    }

    #[test]
    fn payload_on() {
        assert_eq!(payload(true), [0x01, 0x01, 0x01]);
    }
}
