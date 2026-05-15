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

//! Sleep / Wake toggle — selector `0x02` 36-byte frame,
//! `function_group = [0x0a, 0x02, 0xc2, 0xa0, 0x04, 0x00]`.
//!
//! Tiny4Linux-only (not in cgevans/tiny2). Status reflected at byte
//! `0x02` of the 60-byte GET_CUR status struct.

use std::fs::File;

use crate::xu::command02::build;
use crate::xu::enums::SleepState;
use crate::xu::errors::XuError;
use crate::xu::transport::{set_cur, BUNIT_ID, SELECTOR_FRAME};

/// `function_group` for sleep / wake frames.
pub const FUNCTION_GROUP: [u8; 6] = [0x0a, 0x02, 0xc2, 0xa0, 0x04, 0x00];

/// Build the 36-byte payload for Awake / Sleep.
///
/// [`SleepState::Unknown`] is treated as a no-op caller-side; the
/// transport function returns [`XuError::Io`] with `InvalidInput`
/// rather than send a junk frame to the camera.
///
/// # Errors
/// [`XuError::Io`] (`InvalidInput`) when given [`SleepState::Unknown`].
pub fn payload(state: SleepState) -> Result<[u8; 36], XuError> {
    let (seq, cks, cmd) = match state {
        SleepState::Awake => (
            [0xa5, 0x00],
            [0x5f, 0xef],
            [0xbe, 0x07, 0x00, 0x00, 0x00, 0x00],
        ),
        SleepState::Sleep => (
            [0x42, 0x00],
            [0xea, 0x63],
            [0xbf, 0xfb, 0x01, 0x00, 0x00, 0x00],
        ),
        SleepState::Unknown => {
            return Err(XuError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "SleepState::Unknown cannot be sent — it is a read-only diagnostic value",
            )))
        }
    };
    Ok(build(FUNCTION_GROUP, seq, cks, cmd, None))
}

/// Put the camera to sleep or wake it up.
///
/// # Errors
/// See [`payload`] and the transport layer.
pub fn set_sleep(camera: &File, state: SleepState) -> Result<(), XuError> {
    let frame = payload(state)?;
    set_cur(camera, BUNIT_ID, SELECTOR_FRAME, &frame)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payload_awake_matches_tiny4linux() {
        let frame = payload(SleepState::Awake).unwrap();
        assert_eq!(
            frame,
            [
                0xaa, 0x25, 0xa5, 0x00, 0x0c, 0x00, 0x5f, 0xef, 0x0a, 0x02, 0xc2, 0xa0, 0x04, 0x00,
                0xbe, 0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ]
        );
    }

    #[test]
    fn payload_sleep_matches_tiny4linux() {
        let frame = payload(SleepState::Sleep).unwrap();
        assert_eq!(
            frame,
            [
                0xaa, 0x25, 0x42, 0x00, 0x0c, 0x00, 0xea, 0x63, 0x0a, 0x02, 0xc2, 0xa0, 0x04, 0x00,
                0xbf, 0xfb, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ]
        );
    }

    #[test]
    fn payload_unknown_is_rejected() {
        let err = payload(SleepState::Unknown).unwrap_err();
        assert!(matches!(err, XuError::Io(_)));
    }
}
