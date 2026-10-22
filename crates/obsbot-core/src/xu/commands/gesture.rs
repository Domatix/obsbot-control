// SPDX-License-Identifier: GPL-3.0-or-later
//
// Copyright (C) 2026 Domatix and contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! On-device gesture switches — selector `0x02` frames.
//!
//! Captured on 2026-08-06 by observing the vendor SDK's
//! `UVCIOC_CTRL_QUERY` traffic. Not mapped by any FOSS project. See
//! `PROTOCOL.md` §3.2 Q11.

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

    /// Sequence number captured for this gesture and value.
    const fn sequence(self, on: bool) -> u16 {
        match (self, on) {
            (Self::TargetSelection, false) => 0x0008,
            (Self::TargetSelection, true) => 0x0009,
            (Self::Zoom, false) => 0x000a,
            (Self::Zoom, true) => 0x000b,
            (Self::DynamicZoom, false) => 0x000c,
            (Self::DynamicZoom, true) => 0x000d,
            (Self::DynamicZoomDirection, false) => 0x000e,
            (Self::DynamicZoomDirection, true) => 0x000f,
        }
    }
}

/// `command` bytes for the off value, shared by every gesture.
const COMMAND_OFF: [u8; 6] = [0xe6, 0x3f, 0x00, 0x00, 0x00, 0x00];
/// `command` bytes for the on value, shared by every gesture.
const COMMAND_ON: [u8; 6] = [0x27, 0xff, 0x01, 0x00, 0x00, 0x00];

/// Build the frame that turns `gesture` on or off.
///
/// The checksum bytes in `COMMAND_OFF`/`COMMAND_ON` are the captured
/// ones; [`build`] recomputes both checksums from the frame contents.
#[must_use]
pub fn payload(gesture: Gesture, on: bool) -> Vec<u8> {
    let command = if on { COMMAND_ON } else { COMMAND_OFF };
    build(
        gesture.function_group(),
        gesture.sequence(on),
        command,
        None,
    )
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

    fn hex(s: &str) -> Vec<u8> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).expect("hex"))
            .collect()
    }

    /// Header and payload checksums captured on 2026-08-06. The frame
    /// is rebuilt from fields, so this pins the recomputed checksums to
    /// the captured ones.
    #[test]
    fn frames_match_the_capture() {
        let cases: [(Gesture, bool, &str); 8] = [
            (
                Gesture::TargetSelection,
                false,
                "aa2508000c002b5a0a04c4300100e63f00",
            ),
            (
                Gesture::TargetSelection,
                true,
                "aa2509000c007a9f0a04c430010027ff01",
            ),
            (Gesture::Zoom, false, "aa250a000c002a900a0444310100e63f00"),
            (Gesture::Zoom, true, "aa250b000c007b550a044431010027ff01"),
            (
                Gesture::DynamicZoom,
                false,
                "aa250c000c004b4e0a0444330100e63f00",
            ),
            (
                Gesture::DynamicZoom,
                true,
                "aa250d000c001a8b0a044433010027ff01",
            ),
            (
                Gesture::DynamicZoomDirection,
                false,
                "aa250e000c008b440a04c4330100e63f00",
            ),
            (
                Gesture::DynamicZoomDirection,
                true,
                "aa250f000c00da810a04c433010027ff01",
            ),
        ];
        for (gesture, on, expected) in cases {
            assert_eq!(
                payload(gesture, on),
                hex(expected),
                "frame for {gesture:?} on={on} does not match the capture",
            );
        }
    }

    #[test]
    fn the_value_lives_in_the_payload_byte() {
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
