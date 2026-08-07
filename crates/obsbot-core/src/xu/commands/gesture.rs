// SPDX-License-Identifier: GPL-3.0-or-later
//
// Copyright (C) 2026 Domatix and contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! On-device gesture control — selector `0x02` 36-byte frames (T-229).
//!
//! Not derived from any FOSS project: neither `cgevans/tiny2` nor
//! `OpenFoxes/Tiny4Linux` nor `taxfromdk/obsbot_tiny_reversing` maps
//! gestures at all, and Tiny4Linux's README points users at OBSBOT
//! Center inside a Windows VM as the only route. The frames below were
//! captured on 2026-08-06 by observing the `UVCIOC_CTRL_QUERY` traffic
//! the vendor SDK emits, using an `LD_PRELOAD` shim over `ioctl` while
//! calling `aiSetGestureCtrlIndividualR`. Only the observed bytes were
//! kept; no vendor code or header text is reproduced here, and the SDK
//! is not linked (`ADR-0002` stands).
//!
//! Why it matters: `PROTOCOL.md` quirk Q10 established that the zoom a
//! gesture triggers never reaches `zoom_absolute`, so it cannot be
//! constrained from the V4L2 side. Turning the gesture itself off is
//! the only lever, and this is it.
//!
//! ## Frame shape
//!
//! The usual selector-`0x02` layout (`command02::build`). The
//! `function_group` selects which gesture, and the `command` carries the
//! on/off value in its third byte:
//!
//! ```text
//! function_group     gesture
//! 0a 04 c4 30 01 00  target selection
//! 0a 04 44 31 01 00  zoom
//! 0a 04 44 33 01 00  dynamic zoom
//! 0a 04 c4 33 01 00  dynamic zoom direction
//!
//! command            value
//! e6 3f 00 00 00 00  off
//! 27 ff 01 00 00 00  on
//! ```
//!
//! ## On the sequence/checksum pairs
//!
//! Bytes 2-3 are a sequence number and bytes 6-7 a checksum that varies
//! with it. Fourteen captured frames were tested against every common
//! CRC-16 variant (CCITT, XMODEM, KERMIT, MODBUS, ARC, MAXIM, USB, CMS,
//! DNP, X.25) over six candidate byte ranges in both endiannesses, and
//! none reproduces it, so the algorithm stays unknown.
//!
//! We therefore replay captured `(sequence, checksum)` pairs verbatim,
//! which is what every other command in this module already does —
//! `sleep`, `preset` and `tracking_speed` all carry fixed pairs lifted
//! from Tiny4Linux. The firmware accepts a replayed pair repeatedly, so
//! it evidently does not enforce monotonic sequence numbers.

use std::fs::File;

use crate::xu::command02::build;
use crate::xu::errors::XuError;
use crate::xu::transport::{set_cur, BUNIT_ID, SELECTOR_FRAME};

/// Which on-device gesture to turn on or off.
///
/// **[`Self::Zoom`] is the L-shaped hand pose** — the one that makes the
/// camera zoom in and out on its own mid-call. Confirmed on hardware
/// (Tiny 2 Lite, firmware 6.2.5.3, 2026-08-06): switching it off stops
/// the gesture, and switching off [`Self::DynamicZoom`] changes nothing
/// observable.
///
/// The naming comes from the vendor API and is not obvious, so do not
/// swap them back on the assumption that "dynamic" means the continuous
/// pose. It was tested the other way round first and was wrong.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Gesture {
    /// Target-selection gesture (pick who to track).
    TargetSelection,
    /// The L-shaped hand pose that zooms in and out. This is the one
    /// users want off.
    Zoom,
    /// Vendor's "dynamic zoom". No observable effect on a Tiny 2 Lite;
    /// kept because it is a distinct switch on the device.
    DynamicZoom,
    /// Direction of the dynamic zoom gesture.
    DynamicZoomDirection,
}

impl Gesture {
    /// `function_group` bytes identifying this gesture on the wire.
    const fn function_group(self) -> [u8; 6] {
        match self {
            Self::TargetSelection => [0x0a, 0x04, 0xc4, 0x30, 0x01, 0x00],
            Self::Zoom => [0x0a, 0x04, 0x44, 0x31, 0x01, 0x00],
            Self::DynamicZoom => [0x0a, 0x04, 0x44, 0x33, 0x01, 0x00],
            Self::DynamicZoomDirection => [0x0a, 0x04, 0xc4, 0x33, 0x01, 0x00],
        }
    }

    /// Captured `(sequence, checksum)` for this gesture and value. See
    /// the module docs for why these are replayed rather than computed.
    const fn seq_and_checksum(self, on: bool) -> ([u8; 2], [u8; 2]) {
        match (self, on) {
            (Self::TargetSelection, false) => ([0x08, 0x00], [0x2b, 0x5a]),
            (Self::TargetSelection, true) => ([0x09, 0x00], [0x7a, 0x9f]),
            (Self::Zoom, false) => ([0x0a, 0x00], [0x2a, 0x90]),
            (Self::Zoom, true) => ([0x0b, 0x00], [0x7b, 0x55]),
            (Self::DynamicZoom, false) => ([0x0c, 0x00], [0x4b, 0x4e]),
            (Self::DynamicZoom, true) => ([0x0d, 0x00], [0x1a, 0x8b]),
            (Self::DynamicZoomDirection, false) => ([0x0e, 0x00], [0x8b, 0x44]),
            (Self::DynamicZoomDirection, true) => ([0x0f, 0x00], [0xda, 0x81]),
        }
    }
}

/// `command` bytes for the off value, shared by every gesture.
const COMMAND_OFF: [u8; 6] = [0xe6, 0x3f, 0x00, 0x00, 0x00, 0x00];
/// `command` bytes for the on value, shared by every gesture.
const COMMAND_ON: [u8; 6] = [0x27, 0xff, 0x01, 0x00, 0x00, 0x00];

/// Build the 36-byte frame that turns `gesture` on or off.
#[must_use]
pub fn payload(gesture: Gesture, on: bool) -> [u8; 36] {
    let (seq, checksum) = gesture.seq_and_checksum(on);
    let command = if on { COMMAND_ON } else { COMMAND_OFF };
    build(gesture.function_group(), seq, checksum, command, None)
}

/// Turn an on-device gesture on or off.
///
/// # Errors
/// Propagates [`XuError::Io`] from the underlying ioctl, and
/// [`XuError::LengthMismatch`] if the device reports an unexpected
/// selector length.
pub fn set_gesture(camera: &File, gesture: Gesture, on: bool) -> Result<(), XuError> {
    set_cur(camera, BUNIT_ID, SELECTOR_FRAME, &payload(gesture, on))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every frame captured on 2026-08-06, byte for byte. If any of
    /// these drift, the capture and the code have diverged.
    #[test]
    fn frames_match_the_capture() {
        let cases: [(Gesture, bool, [u8; 20]); 8] = [
            (
                Gesture::TargetSelection,
                false,
                [
                    0xaa, 0x25, 0x08, 0x00, 0x0c, 0x00, 0x2b, 0x5a, 0x0a, 0x04, 0xc4, 0x30, 0x01,
                    0x00, 0xe6, 0x3f, 0x00, 0x00, 0x00, 0x00,
                ],
            ),
            (
                Gesture::TargetSelection,
                true,
                [
                    0xaa, 0x25, 0x09, 0x00, 0x0c, 0x00, 0x7a, 0x9f, 0x0a, 0x04, 0xc4, 0x30, 0x01,
                    0x00, 0x27, 0xff, 0x01, 0x00, 0x00, 0x00,
                ],
            ),
            (
                Gesture::Zoom,
                false,
                [
                    0xaa, 0x25, 0x0a, 0x00, 0x0c, 0x00, 0x2a, 0x90, 0x0a, 0x04, 0x44, 0x31, 0x01,
                    0x00, 0xe6, 0x3f, 0x00, 0x00, 0x00, 0x00,
                ],
            ),
            (
                Gesture::Zoom,
                true,
                [
                    0xaa, 0x25, 0x0b, 0x00, 0x0c, 0x00, 0x7b, 0x55, 0x0a, 0x04, 0x44, 0x31, 0x01,
                    0x00, 0x27, 0xff, 0x01, 0x00, 0x00, 0x00,
                ],
            ),
            (
                Gesture::DynamicZoom,
                false,
                [
                    0xaa, 0x25, 0x0c, 0x00, 0x0c, 0x00, 0x4b, 0x4e, 0x0a, 0x04, 0x44, 0x33, 0x01,
                    0x00, 0xe6, 0x3f, 0x00, 0x00, 0x00, 0x00,
                ],
            ),
            (
                Gesture::DynamicZoom,
                true,
                [
                    0xaa, 0x25, 0x0d, 0x00, 0x0c, 0x00, 0x1a, 0x8b, 0x0a, 0x04, 0x44, 0x33, 0x01,
                    0x00, 0x27, 0xff, 0x01, 0x00, 0x00, 0x00,
                ],
            ),
            (
                Gesture::DynamicZoomDirection,
                false,
                [
                    0xaa, 0x25, 0x0e, 0x00, 0x0c, 0x00, 0x8b, 0x44, 0x0a, 0x04, 0xc4, 0x33, 0x01,
                    0x00, 0xe6, 0x3f, 0x00, 0x00, 0x00, 0x00,
                ],
            ),
            (
                Gesture::DynamicZoomDirection,
                true,
                [
                    0xaa, 0x25, 0x0f, 0x00, 0x0c, 0x00, 0xda, 0x81, 0x0a, 0x04, 0xc4, 0x33, 0x01,
                    0x00, 0x27, 0xff, 0x01, 0x00, 0x00, 0x00,
                ],
            ),
        ];
        for (gesture, on, expected) in cases {
            let frame = payload(gesture, on);
            assert_eq!(
                &frame[..20],
                &expected[..],
                "frame for {gesture:?} on={on} does not match the capture",
            );
            assert_eq!(&frame[20..], &[0u8; 16], "appendix must stay zero");
        }
    }

    #[test]
    fn the_value_lives_in_the_third_command_byte() {
        for gesture in [
            Gesture::TargetSelection,
            Gesture::Zoom,
            Gesture::DynamicZoom,
            Gesture::DynamicZoomDirection,
        ] {
            assert_eq!(payload(gesture, false)[16], 0x00);
            assert_eq!(payload(gesture, true)[16], 0x01);
        }
    }

    #[test]
    fn each_gesture_has_its_own_function_group() {
        let groups = [
            Gesture::TargetSelection.function_group(),
            Gesture::Zoom.function_group(),
            Gesture::DynamicZoom.function_group(),
            Gesture::DynamicZoomDirection.function_group(),
        ];
        for (i, a) in groups.iter().enumerate() {
            for b in groups.iter().skip(i + 1) {
                assert_ne!(a, b, "two gestures share a function group");
            }
        }
    }
}
