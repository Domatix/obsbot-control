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

//! Face Auto-Exposure metering style — selector `0x06` opcode `0x03`.
//!
//! Wire bytes: `[0x03, 0x01, value]` where `0x00 = Global` and
//! `0x01 = Face`. Only meaningful when the camera is in auto-exposure
//! mode (send the Auto frame from
//! [`crate::xu::commands::exposure_mode_type`] first).

use std::fs::File;

use crate::xu::enums::FaceAeMode;
use crate::xu::errors::XuError;
use crate::xu::transport::{set_cur, BUNIT_ID, SELECTOR_OPCODE};

/// Opcode byte for Face AE commands.
pub const OPCODE: u8 = 0x03;

/// Build the 3-byte SET_CUR payload for Face AE selection.
#[must_use]
pub fn payload(mode: FaceAeMode) -> [u8; 3] {
    [OPCODE, 0x01, mode.to_wire()]
}

/// Select Face AE metering on the camera.
///
/// # Errors
/// Propagates [`XuError`] from the transport layer.
pub fn set_face_ae(camera: &File, mode: FaceAeMode) -> Result<(), XuError> {
    set_cur(camera, BUNIT_ID, SELECTOR_OPCODE, &payload(mode))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payload_global() {
        assert_eq!(payload(FaceAeMode::Global), [0x03, 0x01, 0x00]);
    }

    #[test]
    fn payload_face() {
        assert_eq!(payload(FaceAeMode::Face), [0x03, 0x01, 0x01]);
    }
}
