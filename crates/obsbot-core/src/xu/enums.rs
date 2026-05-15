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

//! Typed value enums for the XU command surface.
//!
//! Every enum carries its **wire-level** encoding, not a high-level
//! UI value. Tables here mirror `PROTOCOL.md §3.2`; if a byte value
//! disagrees with the doc the doc is wrong and should be fixed (the
//! source of truth is `docs/XU_INVESTIGATION_2026-05-14.md`, which
//! quotes the upstream EUPL-1.2 code verbatim).

use thiserror::Error;

/// AI tracking mode — selector `0x06` opcode `0x16` payload `(m, n)`.
///
/// 10 variants per `cgevans/tiny2` and `OpenFoxes/Tiny4Linux`. The
/// `(m, n)` tuple encoding is in [`AiMode::to_wire`]; the
/// [`TryFrom<(u8, u8)>`] decoder accepts both `(3, 0)` and `(6, 0)`
/// as [`AiMode::Hand`] until quirk Q4 is hardware-validated (see
/// `PROTOCOL.md §3.2 Q4`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiMode {
    /// No AI tracking — the camera holds whatever position was last
    /// set manually.
    NoTracking,
    /// Track a single subject (default tracking).
    NormalTracking,
    /// Track and frame the subject's upper body.
    UpperBody,
    /// Close-up framing.
    CloseUp,
    /// Frame the subject without showing the head (e.g. for product
    /// demos).
    Headless,
    /// Frame the lower body.
    LowerBody,
    /// Desk-mode framing (e.g. focuses on a workspace surface).
    DeskMode,
    /// Whiteboard tracking.
    Whiteboard,
    /// Hand-tracking mode.
    Hand,
    /// Frame a group of subjects.
    Group,
}

impl AiMode {
    /// Wire-level `(m, n)` tuple for the selector-0x06 opcode-0x16
    /// payload (`[0x16, 0x02, m, n]`).
    ///
    /// Matches cgevans's setter byte-for-byte, including the
    /// suspected typo in [`AiMode::Hand`] (`m=3` instead of the
    /// `m=6` the decoder reads). Quirk tracked as Q4 in
    /// `PROTOCOL.md §3.2`.
    #[must_use]
    pub fn to_wire(self) -> (u8, u8) {
        match self {
            Self::NoTracking => (0x00, 0x00),
            Self::NormalTracking => (0x02, 0x00),
            Self::UpperBody => (0x02, 0x01),
            Self::CloseUp => (0x02, 0x02),
            Self::Headless => (0x02, 0x03),
            Self::LowerBody => (0x02, 0x04),
            Self::DeskMode => (0x05, 0x00),
            Self::Whiteboard => (0x04, 0x00),
            // ⚠ Q4 — cgevans writes m=3, decoder reads m=6. We mirror
            // the upstream setter; the decoder below accepts both.
            Self::Hand => (0x03, 0x00),
            Self::Group => (0x01, 0x00),
        }
    }
}

impl TryFrom<(u8, u8)> for AiMode {
    type Error = EnumDecodeError;

    fn try_from(value: (u8, u8)) -> Result<Self, Self::Error> {
        Ok(match value {
            (0x00, 0x00) => Self::NoTracking,
            (0x02, 0x00) => Self::NormalTracking,
            (0x02, 0x01) => Self::UpperBody,
            (0x02, 0x02) => Self::CloseUp,
            (0x02, 0x03) => Self::Headless,
            (0x02, 0x04) => Self::LowerBody,
            (0x05, 0x00) => Self::DeskMode,
            (0x04, 0x00) => Self::Whiteboard,
            // Q4: accept BOTH the setter's m=3 and the cgevans
            // decoder's m=6 until T-303 validates live.
            (0x03 | 0x06, 0x00) => Self::Hand,
            (0x01, 0x00) => Self::Group,
            other => return Err(EnumDecodeError::AiMode(other.0, other.1)),
        })
    }
}

/// Field of View — selector `0x06` opcode `0x04`.
///
/// Three discrete widths on Tiny 2. Tiny4Linux does not implement
/// this opcode; we follow cgevans's mapping byte-for-byte.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FovMode {
    /// Wide field of view (≈ 86°).
    Wide,
    /// Normal field of view (≈ 78°).
    Normal,
    /// Narrow field of view (≈ 65°).
    Narrow,
}

impl FovMode {
    /// Wire byte for the selector-0x06 opcode-0x04 payload.
    #[must_use]
    pub fn to_wire(self) -> u8 {
        match self {
            Self::Wide => 0x01,
            Self::Normal => 0x02,
            Self::Narrow => 0x03,
        }
    }
}

impl TryFrom<u8> for FovMode {
    type Error = EnumDecodeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0x01 => Self::Wide,
            0x02 => Self::Normal,
            0x03 => Self::Narrow,
            other => return Err(EnumDecodeError::FovMode(other)),
        })
    }
}

/// Auto-exposure metering style — selector `0x06` opcode `0x03`.
///
/// Only meaningful when the camera is in auto-exposure mode (i.e.
/// after the Auto frame from [`crate::xu::commands::exposure_mode_type`]
/// has been sent on selector `0x02`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaceAeMode {
    /// Global metering — meter the whole frame.
    Global,
    /// Face metering — meter the tracked face.
    Face,
}

impl FaceAeMode {
    /// Wire byte for the selector-0x06 opcode-0x03 payload.
    #[must_use]
    pub fn to_wire(self) -> u8 {
        match self {
            Self::Global => 0x00,
            Self::Face => 0x01,
        }
    }
}

impl TryFrom<u8> for FaceAeMode {
    type Error = EnumDecodeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0x00 => Self::Global,
            0x01 => Self::Face,
            other => return Err(EnumDecodeError::FaceAeMode(other)),
        })
    }
}

/// Exposure mode — selector `0x02` 36-byte frame.
///
/// Two variants: Auto and Manual. Per quirk Q5 (`PROTOCOL.md §3.2`),
/// cgevans's labelling is adopted as the canonical one — the
/// upstream `MANUAL_EXP_CMD` literal matches Tiny4Linux's `Auto` and
/// vice versa, but cgevans's Face-AE follow-up (which only makes
/// sense after putting the camera in auto) confirms cgevans's
/// labels.
///
/// This is distinct from [`crate::ExposureMode`] (the high-level
/// trait enum), which adds an `AperturePriority` variant the XU
/// path does not expose.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExposureMode {
    /// Auto exposure — let the camera meter (see
    /// [`FaceAeMode`] for metering style sub-selection).
    Auto,
    /// Manual exposure — the user controls exposure time via the
    /// V4L2 standard `exposure_time_absolute` control (already
    /// exposed by `obsbot-core::controls`).
    Manual,
}

/// Sleep / Wake state — selector `0x02` 36-byte frame, also
/// returned in GET_CUR status byte `0x02`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SleepState {
    /// Camera is awake and streaming.
    Awake,
    /// Camera is asleep (lens covered, low power).
    Sleep,
    /// Status byte returned a value neither `0x00` nor `0x01` —
    /// undocumented state, preserved verbatim for diagnostics.
    Unknown,
}

impl From<u8> for SleepState {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::Awake,
            0x01 => Self::Sleep,
            _ => Self::Unknown,
        }
    }
}

/// Tracking speed — selector `0x02` 36-byte frame, also returned
/// in GET_CUR status byte `0x21`.
///
/// Tiny4Linux ships two variants (Standard / Sport). The decoder
/// defaults the unmapped gap value `0x01` to Standard (Q6 in
/// `PROTOCOL.md §3.2`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackingSpeed {
    /// Standard tracking — smooth, low-acceleration.
    Standard,
    /// Sport tracking — fast, high-acceleration.
    Sport,
}

impl TrackingSpeed {
    /// Default-to-Standard decode for status byte `0x21`.
    #[must_use]
    pub fn from_status_byte(byte: u8) -> Self {
        match byte {
            0x02 => Self::Sport,
            // 0x00 = Standard; 0x01 is unmapped (Q6) and defaults
            // to Standard per Tiny4Linux's permissive decode.
            _ => Self::Standard,
        }
    }
}

/// Per-enum decode failure (a status byte or `(m, n)` tuple did
/// not match any known variant).
///
/// Wrapped into [`crate::xu::errors::XuError::Decode`] for the
/// caller-facing error chain.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum EnumDecodeError {
    /// Bytes did not match any [`AiMode`] variant.
    #[error("unknown AiMode tuple (m, n) = ({0:#x}, {1:#x})")]
    AiMode(u8, u8),
    /// Byte did not match any [`FovMode`] variant.
    #[error("unknown FovMode byte {0:#x}")]
    FovMode(u8),
    /// Byte did not match any [`FaceAeMode`] variant.
    #[error("unknown FaceAeMode byte {0:#x}")]
    FaceAeMode(u8),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ai_mode_round_trip_for_unambiguous_variants() {
        // All variants EXCEPT Hand round-trip cleanly (Hand is
        // tested separately because the setter writes m=3 but the
        // decoder also accepts m=6 — see q4_hand_decoder_accepts_both).
        for mode in [
            AiMode::NoTracking,
            AiMode::NormalTracking,
            AiMode::UpperBody,
            AiMode::CloseUp,
            AiMode::Headless,
            AiMode::LowerBody,
            AiMode::DeskMode,
            AiMode::Whiteboard,
            AiMode::Group,
        ] {
            let wire = mode.to_wire();
            let decoded = AiMode::try_from(wire).expect("variant decodes");
            assert_eq!(mode, decoded, "round-trip for {mode:?} wire={wire:?}");
        }
    }

    #[test]
    fn q4_hand_decoder_accepts_both_m3_and_m6() {
        // Q4 — cgevans's setter writes (3, 0) for Hand; the upstream
        // status decoder reads (6, 0) as Hand. Our decoder accepts
        // both until live validation in T-303.
        assert_eq!(AiMode::try_from((0x03, 0x00)).unwrap(), AiMode::Hand);
        assert_eq!(AiMode::try_from((0x06, 0x00)).unwrap(), AiMode::Hand);
        // Our setter writes the cgevans-faithful (3, 0).
        assert_eq!(AiMode::Hand.to_wire(), (0x03, 0x00));
    }

    #[test]
    fn ai_mode_unknown_tuple_errors() {
        let err = AiMode::try_from((0x77, 0x77)).unwrap_err();
        assert!(matches!(err, EnumDecodeError::AiMode(0x77, 0x77)));
    }

    #[test]
    fn fov_mode_round_trip() {
        for mode in [FovMode::Wide, FovMode::Normal, FovMode::Narrow] {
            assert_eq!(FovMode::try_from(mode.to_wire()).unwrap(), mode);
        }
        assert!(FovMode::try_from(0x00).is_err());
        assert!(FovMode::try_from(0x04).is_err());
    }

    #[test]
    fn face_ae_round_trip() {
        for mode in [FaceAeMode::Global, FaceAeMode::Face] {
            assert_eq!(FaceAeMode::try_from(mode.to_wire()).unwrap(), mode);
        }
        assert!(FaceAeMode::try_from(0x02).is_err());
    }

    #[test]
    fn sleep_state_decode() {
        assert_eq!(SleepState::from(0x00), SleepState::Awake);
        assert_eq!(SleepState::from(0x01), SleepState::Sleep);
        assert_eq!(SleepState::from(0x99), SleepState::Unknown);
    }

    #[test]
    fn tracking_speed_q6_gap_defaults_to_standard() {
        assert_eq!(
            TrackingSpeed::from_status_byte(0x00),
            TrackingSpeed::Standard
        );
        assert_eq!(TrackingSpeed::from_status_byte(0x02), TrackingSpeed::Sport);
        // Q6 — 0x01 is the unmapped gap; defaults to Standard.
        assert_eq!(
            TrackingSpeed::from_status_byte(0x01),
            TrackingSpeed::Standard
        );
        // Anything else also defaults to Standard.
        assert_eq!(
            TrackingSpeed::from_status_byte(0xff),
            TrackingSpeed::Standard
        );
    }
}
