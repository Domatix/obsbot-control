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

//! 60-byte global status struct returned by `GET_CUR` on selector
//! `0x06`.
//!
//! Five bytes decode to known fields per `PROTOCOL.md §3.2`; the
//! other 55 are returned by the camera but undecoded by either FOSS
//! source. We surface them as a raw array so a future debug "Dump
//! status" page (T-302) can give users an easy way to capture
//! camera state for community discovery.

use std::fs::File;

use crate::xu::enums::{AiMode, SleepState, TrackingSpeed};
use crate::xu::errors::XuError;
use crate::xu::transport::{get_cur, BUNIT_ID, SELECTOR_OPCODE};

/// Total length of the status struct on selector `0x06` GET_CUR.
pub const STATUS_LEN: usize = 60;

/// Byte offset where the sleep state lives (Tiny4Linux).
pub const STATUS_OFFSET_SLEEP: usize = 0x02;
/// Byte offset where the HDR flag lives (cgevans + Tiny4Linux).
pub const STATUS_OFFSET_HDR: usize = 0x06;
/// Byte offset where the AI mode `m` tuple-half lives.
pub const STATUS_OFFSET_AI_M: usize = 0x18;
/// Byte offset where the AI mode `n` tuple-half lives.
pub const STATUS_OFFSET_AI_N: usize = 0x1c;
/// Byte offset where the tracking speed lives (Tiny4Linux).
pub const STATUS_OFFSET_TRACKING_SPEED: usize = 0x21;

/// Decoded snapshot of the camera's vendor-XU state.
///
/// The `raw` field is preserved so callers can render the full
/// 60-byte dump for diagnostics. AI mode decode follows the
/// permissive Q4 rule (accepts both `m=3` and `m=6` for
/// [`AiMode::Hand`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Status {
    /// Sleep / wake state (status byte `0x02`).
    pub sleep: SleepState,
    /// HDR enabled (status byte `0x06`; non-zero means on).
    pub hdr_on: bool,
    /// AI tracking mode (status bytes `0x18` and `0x1c`).
    pub ai_mode: AiMode,
    /// Tracking speed (status byte `0x21`).
    pub tracking_speed: TrackingSpeed,
    /// Full 60-byte payload as returned by the camera; for the
    /// future debug "Dump status" page.
    pub raw: [u8; STATUS_LEN],
}

impl Status {
    /// Decode an already-captured 60-byte payload.
    ///
    /// Most callers want [`get_status`], which combines the ioctl
    /// and the decode; this entry point exists so unit tests can
    /// validate decode against fixture vectors without hardware.
    ///
    /// # Errors
    /// Returns [`XuError::Decode`] if the AI-mode tuple does not
    /// match any known variant (Q4 means `(3, 0)` and `(6, 0)` are
    /// both accepted; anything else is the error path).
    pub fn decode(raw: [u8; STATUS_LEN]) -> Result<Self, XuError> {
        let sleep = SleepState::from(raw[STATUS_OFFSET_SLEEP]);
        let hdr_on = raw[STATUS_OFFSET_HDR] != 0;
        let ai_mode = AiMode::try_from((raw[STATUS_OFFSET_AI_M], raw[STATUS_OFFSET_AI_N]))?;
        let tracking_speed = TrackingSpeed::from_status_byte(raw[STATUS_OFFSET_TRACKING_SPEED]);
        Ok(Self {
            sleep,
            hdr_on,
            ai_mode,
            tracking_speed,
            raw,
        })
    }
}

/// Read the 60-byte status struct from the camera and decode it.
///
/// # Errors
/// Propagates [`XuError::Io`] from the underlying ioctl,
/// [`XuError::LengthMismatch`] if the device reports a selector
/// length other than 60 (would indicate a non-Tiny-2-family
/// camera), and [`XuError::Decode`] if the AI mode bytes do not
/// match a known variant.
pub fn get_status(camera: &File) -> Result<Status, XuError> {
    let mut buf = [0u8; STATUS_LEN];
    get_cur(camera, BUNIT_ID, SELECTOR_OPCODE, &mut buf)?;
    Status::decode(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 57-byte sample from `OpenFoxes/Tiny4Linux`'s integration test
    /// (`src/libs/camera/status.rs::tests::integration::camera_status::
    /// decode_status`), padded with zeros to 60 bytes. Reflects:
    /// Awake (byte 0x02 = 0x00), HDR on (byte 0x06 = 0x01),
    /// UpperBody (bytes 0x18 / 0x1c = 0x02 / 0x01), Sport
    /// (byte 0x21 = 0x02).
    const FIXTURE_AWAKE_HDR_UPPERBODY_SPORT: [u8; STATUS_LEN] = [
        0x27, 0x00, 0x00, 0x01, 0x42, 0x00, 0x01, 0x01, 0x01, 0x01, 0x88, 0xff, 0x00, 0x00, 0x01,
        0x00, 0x00, 0x03, 0x00, 0x00, 0x01, 0x00, 0x21, 0x00, 0x02, 0x01, 0x03, 0x00, 0x01, 0x00,
        0x00, 0x1e, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    #[test]
    fn decodes_tiny4linux_fixture_vector() {
        let status = Status::decode(FIXTURE_AWAKE_HDR_UPPERBODY_SPORT).unwrap();
        assert_eq!(status.sleep, SleepState::Awake);
        assert!(status.hdr_on);
        assert_eq!(status.ai_mode, AiMode::UpperBody);
        assert_eq!(status.tracking_speed, TrackingSpeed::Sport);
        // Raw preserved verbatim.
        assert_eq!(status.raw, FIXTURE_AWAKE_HDR_UPPERBODY_SPORT);
    }

    #[test]
    fn permissive_hdr_decode() {
        // Tiny4Linux's test sample uses 0x01 for HDR-on; cgevans's
        // decoder is just `byte != 0`, so 0x02 must also map to true.
        let mut buf = [0u8; STATUS_LEN];
        buf[STATUS_OFFSET_HDR] = 0x02;
        let status = Status::decode(buf).unwrap();
        assert!(status.hdr_on);
    }

    #[test]
    fn unknown_ai_mode_tuple_fails_decode() {
        let mut buf = [0u8; STATUS_LEN];
        buf[STATUS_OFFSET_AI_M] = 0x77;
        buf[STATUS_OFFSET_AI_N] = 0x88;
        let err = Status::decode(buf).unwrap_err();
        assert!(matches!(err, XuError::Decode(_)));
    }

    #[test]
    fn q4_hand_decodes_from_either_m_value() {
        // Hand at m=6 (cgevans's status decoder convention).
        let mut buf_m6 = [0u8; STATUS_LEN];
        buf_m6[STATUS_OFFSET_AI_M] = 0x06;
        buf_m6[STATUS_OFFSET_AI_N] = 0x00;
        assert_eq!(Status::decode(buf_m6).unwrap().ai_mode, AiMode::Hand);

        // Hand at m=3 (cgevans's setter convention — Q4).
        let mut buf_m3 = [0u8; STATUS_LEN];
        buf_m3[STATUS_OFFSET_AI_M] = 0x03;
        buf_m3[STATUS_OFFSET_AI_N] = 0x00;
        assert_eq!(Status::decode(buf_m3).unwrap().ai_mode, AiMode::Hand);
    }

    #[test]
    fn offsets_match_protocol_md() {
        // Guard against accidental edits to the offset constants.
        assert_eq!(STATUS_OFFSET_SLEEP, 0x02);
        assert_eq!(STATUS_OFFSET_HDR, 0x06);
        assert_eq!(STATUS_OFFSET_AI_M, 0x18);
        assert_eq!(STATUS_OFFSET_AI_N, 0x1c);
        assert_eq!(STATUS_OFFSET_TRACKING_SPEED, 0x21);
        assert_eq!(STATUS_LEN, 60);
    }
}
