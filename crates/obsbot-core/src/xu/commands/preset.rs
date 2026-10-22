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

//! Preset position recall — selector `0x02` frame,
//! `function_group = [0x0a, 0x04, 0xc4, 0x39, 0x14, 0x00]`.
//!
//! Tiny4Linux-only, recall only (no save opcode in either FOSS
//! source — see `PROTOCOL.md §3.2 Q7`). Three slots, 0-indexed in
//! source / 1-based for the UI ("Preset 1 / 2 / 3"). The payload is
//! twenty bytes: a slot byte plus four IEEE-754 little-endian
//! `1.0_f32` floats; the camera rejects the recall without them.

use std::fs::File;

use crate::xu::command02::{build, PRESET_RECALL_APPENDIX};
use crate::xu::errors::XuError;
use crate::xu::transport::{get_cur, set_cur, BUNIT_ID, SELECTOR_FRAME};

/// `function_group` for preset-recall frames.
pub const FUNCTION_GROUP: [u8; 6] = [0x0a, 0x04, 0xc4, 0x39, 0x14, 0x00];

/// Build the selector-0x02 payload to recall preset slot `index`.
///
/// `index` must be in `0..=2`.
///
/// # Errors
/// Returns [`XuError::InvalidPresetIndex`] if `index` is outside
/// `0..=2`.
pub fn payload(index: i8) -> Result<Vec<u8>, XuError> {
    let (seq, slot) = match index {
        0 => (0x0020, 0u8),
        1 => (0x001a, 1),
        2 => (0x0026, 2),
        other => return Err(XuError::InvalidPresetIndex(other)),
    };
    let cmd = [0u8, 0u8, slot, 0x00, 0x00, 0x00];
    Ok(build(
        FUNCTION_GROUP,
        seq,
        cmd,
        Some(PRESET_RECALL_APPENDIX),
    ))
}

/// Recall preset slot `index` (must be `0..=2`).
///
/// # Errors
/// See [`payload`] and the transport layer.
pub fn recall_preset(camera: &File, index: i8) -> Result<(), XuError> {
    let frame = payload(index)?;
    set_cur(camera, BUNIT_ID, SELECTOR_FRAME, &frame)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payload_preset_1_matches_capture() {
        let frame = payload(0).unwrap();
        assert_eq!(&frame[0..2], &[0xaa, 0x25]);
        assert_eq!(&frame[2..4], &[0x20, 0x00]);
        assert_eq!(&frame[6..8], &[0x6b, 0xdc], "header checksum");
        assert_eq!(&frame[8..12], &[0x0a, 0x04, 0xc4, 0x39]);
        // Payload: length 20, checksum, slot byte, four 1.0f32.
        assert_eq!(&frame[12..14], &[0x14, 0x00]);
        assert_eq!(&frame[14..16], &[0xd6, 0xfb], "payload checksum");
        let mut expected = Vec::from([0u8, 0, 0, 0]);
        expected.extend_from_slice(&PRESET_RECALL_APPENDIX);
        assert_eq!(&frame[16..36], &expected[..]);
    }

    #[test]
    fn payload_preset_2_seq_and_cks() {
        let frame = payload(1).unwrap();
        assert_eq!(&frame[2..4], &[0x1a, 0x00]);
        assert_eq!(&frame[6..8], &[0x4b, 0x03]);
        assert_eq!(&frame[14..16], &[0xeb, 0x2a]);
        assert_eq!(frame[16], 0x01, "slot byte");
    }

    #[test]
    fn payload_preset_3_seq_and_cks() {
        let frame = payload(2).unwrap();
        assert_eq!(&frame[2..4], &[0x26, 0x00]);
        assert_eq!(&frame[6..8], &[0x8b, 0xc3]);
        assert_eq!(&frame[14..16], &[0xaf, 0x19]);
        assert_eq!(frame[16], 0x02, "slot byte");
    }

    #[test]
    fn negative_index_rejected() {
        let err = payload(-1).unwrap_err();
        assert!(matches!(err, XuError::InvalidPresetIndex(-1)));
    }

    #[test]
    fn index_3_rejected() {
        let err = payload(3).unwrap_err();
        assert!(matches!(err, XuError::InvalidPresetIndex(3)));
    }

    fn verify_checksums(frame: &[u8]) {
        use crate::xu::crc16::crc16;
        let hlen = usize::from(u16::from_le_bytes([frame[4], frame[5]]));
        let mut hdr = frame[..hlen].to_vec();
        let want = u16::from_le_bytes([hdr[6], hdr[7]]);
        hdr[6] = 0;
        hdr[7] = 0;
        assert_eq!(crc16(&hdr), want, "header checksum");
        if hlen < frame.len() {
            let plen = usize::from(u16::from_le_bytes([frame[hlen], frame[hlen + 1]]));
            let mut blk = frame[hlen..hlen + plen + 4].to_vec();
            let want = u16::from_le_bytes([blk[2], blk[3]]);
            blk[2] = 0;
            blk[3] = 0;
            assert_eq!(crc16(&blk), want, "payload checksum");
        }
    }

    #[test]
    fn add_frame_layout_and_checksums() {
        let pose = Pose {
            pan: 10.0,
            tilt: -5.0,
            roll: 0.0,
            zoom: 2.0,
        };
        let frame = add_payload(0x0200, 1, pose).unwrap();
        assert_eq!(&frame[8..12], &[0x0a, 0x04, 0x44, 0x39], "cmd 0x3944");
        assert_eq!(frame.len(), 12 + 4 + 24);
        assert_eq!(&frame[12..14], &[24, 0], "payload length");
        assert_eq!(&frame[16..20], &[0, 0, 0, 0], "slot index 0");
        verify_checksums(&frame);
    }

    #[test]
    fn delete_frame_layout() {
        let frame = delete_payload(0x0201, 2).unwrap();
        assert_eq!(&frame[8..12], &[0x0a, 0x04, 0x84, 0x39], "cmd 0x3984");
        assert_eq!(frame.len(), 12 + 4 + 4);
        assert_eq!(&frame[16..20], &[1, 0, 0, 0], "slot index 1");
        verify_checksums(&frame);
    }

    #[test]
    fn rename_accepts_the_longest_name() {
        let name = "x".repeat(MAX_PRESET_NAME_LEN);
        let frame = rename_payload(0x0203, 1, &name).expect("longest name fits");
        assert_eq!(frame.len(), 12 + 4 + 4 + MAX_PRESET_NAME_LEN);
        verify_checksums(&frame);
    }

    #[test]
    fn rename_rejects_a_name_that_does_not_fit() {
        let name = "x".repeat(MAX_PRESET_NAME_LEN + 1);
        let err = rename_payload(0x0204, 1, &name).unwrap_err();
        assert!(
            matches!(err, XuError::PresetNameTooLong { len, max }
                if len == MAX_PRESET_NAME_LEN + 1 && max == MAX_PRESET_NAME_LEN),
            "got {err:?}",
        );
    }

    #[test]
    fn rename_frame_layout() {
        let frame = rename_payload(0x0202, 3, "Desk").unwrap();
        assert_eq!(&frame[8..12], &[0x0a, 0x04, 0x84, 0x3a], "cmd 0x3A84");
        assert_eq!(&frame[16..20], &[2, 0, 0, 0], "slot index 2");
        assert_eq!(&frame[20..24], b"Desk");
        verify_checksums(&frame);
    }

    #[test]
    fn admin_slots_are_bounded() {
        let pose = Pose {
            pan: 0.0,
            tilt: 0.0,
            roll: 0.0,
            zoom: 1.0,
        };
        assert!(matches!(
            add_payload(1, 0, pose),
            Err(XuError::InvalidPresetSlot(0))
        ));
        assert!(matches!(
            delete_payload(1, 4),
            Err(XuError::InvalidPresetSlot(4))
        ));
    }
}

/// Camera pose stored in a preset, in the units the firmware expects for
/// this command family (the same floats the vendor application sends).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pose {
    /// Pan angle.
    pub pan: f32,
    /// Tilt angle.
    pub tilt: f32,
    /// Roll angle.
    pub roll: f32,
    /// Zoom factor.
    pub zoom: f32,
}

/// Wire command for creating a preset in a slot.
const CMD_ADD: u16 = 0x3944;
/// Wire command for updating an existing preset.
const CMD_UPDATE: u16 = 0x3e04;
/// Wire command for deleting a preset.
const CMD_DELETE: u16 = 0x3984;
/// Wire command for renaming a preset.
const CMD_SET_NAME: u16 = 0x3a84;
/// Wire command that lists the stored presets.
const CMD_LIST: u16 = 0x3b44;

/// Longest preset name a rename frame can carry: the payload is
/// `slot(4) + name` inside a block of `60 - 16` bytes, so 40 is the
/// ceiling, and the vendor SDK caps names below 50 as well.
pub const MAX_PRESET_NAME_LEN: usize = 40;

/// Sequence numbers are advisory: the device echoes them back. A shared
/// counter keeps them distinct across calls.
static NEXT_SEQ: std::sync::atomic::AtomicU16 = std::sync::atomic::AtomicU16::new(0x0100);

fn next_seq() -> u16 {
    NEXT_SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

/// Four-byte function group: sender host, receiver AI, command.
const fn function_group(cmd: u16) -> [u8; 4] {
    [0x0a, 0x04, (cmd & 0xff) as u8, (cmd >> 8) as u8]
}

fn slot_index(slot: u8) -> Result<u32, XuError> {
    if (1..=3).contains(&slot) {
        Ok(u32::from(slot - 1))
    } else {
        Err(XuError::InvalidPresetSlot(slot))
    }
}

fn pose_bytes(pose: Pose) -> Vec<u8> {
    let mut out = Vec::with_capacity(20);
    for value in [pose.pan, pose.tilt, pose.roll, pose.zoom, -1000.0] {
        out.extend_from_slice(&value.to_le_bytes());
    }
    out
}

/// Build the frame that creates preset `slot` with `pose`.
///
/// # Errors
/// [`XuError::InvalidPresetSlot`] when `slot` is outside `1..=3`.
pub fn add_payload(seq: u16, slot: u8, pose: Pose) -> Result<Vec<u8>, XuError> {
    let mut payload = slot_index(slot)?.to_le_bytes().to_vec();
    payload.extend_from_slice(&pose_bytes(pose));
    Ok(crate::xu::frame::encode(
        seq,
        function_group(CMD_ADD),
        &payload,
    ))
}

/// Build the frame that updates preset `slot` in place.
///
/// # Errors
/// [`XuError::InvalidPresetSlot`] when `slot` is outside `1..=3`.
pub fn update_payload(seq: u16, slot: u8, pose: Pose) -> Result<Vec<u8>, XuError> {
    let mut payload = slot_index(slot)?.to_le_bytes().to_vec();
    payload.extend_from_slice(&pose_bytes(pose));
    Ok(crate::xu::frame::encode(
        seq,
        function_group(CMD_UPDATE),
        &payload,
    ))
}

/// Build the frame that deletes preset `slot`.
///
/// # Errors
/// [`XuError::InvalidPresetSlot`] when `slot` is outside `1..=3`.
pub fn delete_payload(seq: u16, slot: u8) -> Result<Vec<u8>, XuError> {
    Ok(crate::xu::frame::encode(
        seq,
        function_group(CMD_DELETE),
        &slot_index(slot)?.to_le_bytes(),
    ))
}

/// Build the frame that renames preset `slot`.
///
/// # Errors
/// [`XuError::InvalidPresetSlot`] when `slot` is outside `1..=3`, and
/// [`XuError::PresetNameTooLong`] when the name exceeds
/// [`MAX_PRESET_NAME_LEN`] bytes (the frame would not fit the
/// selector and `set_cur` would reject it with a bare length error).
pub fn rename_payload(seq: u16, slot: u8, name: &str) -> Result<Vec<u8>, XuError> {
    if name.len() > MAX_PRESET_NAME_LEN {
        return Err(XuError::PresetNameTooLong {
            len: name.len(),
            max: MAX_PRESET_NAME_LEN,
        });
    }
    let mut payload = slot_index(slot)?.to_le_bytes().to_vec();
    payload.extend_from_slice(name.as_bytes());
    Ok(crate::xu::frame::encode(
        seq,
        function_group(CMD_SET_NAME),
        &payload,
    ))
}

/// Create preset `slot` with `pose`.
///
/// # Errors
/// See [`add_payload`] and the transport layer.
pub fn add_preset(camera: &File, slot: u8, pose: Pose) -> Result<(), XuError> {
    let frame = add_payload(next_seq(), slot, pose)?;
    set_cur(camera, BUNIT_ID, SELECTOR_FRAME, &frame)
}

/// Update preset `slot` in place.
///
/// # Errors
/// See [`update_payload`] and the transport layer.
pub fn update_preset(camera: &File, slot: u8, pose: Pose) -> Result<(), XuError> {
    let frame = update_payload(next_seq(), slot, pose)?;
    set_cur(camera, BUNIT_ID, SELECTOR_FRAME, &frame)
}

/// Delete preset `slot`.
///
/// # Errors
/// See [`delete_payload`] and the transport layer.
pub fn delete_preset(camera: &File, slot: u8) -> Result<(), XuError> {
    let frame = delete_payload(next_seq(), slot)?;
    set_cur(camera, BUNIT_ID, SELECTOR_FRAME, &frame)
}

/// Rename preset `slot`.
///
/// # Errors
/// See [`rename_payload`] and the transport layer.
pub fn rename_preset(camera: &File, slot: u8, name: &str) -> Result<(), XuError> {
    let frame = rename_payload(next_seq(), slot, name)?;
    set_cur(camera, BUNIT_ID, SELECTOR_FRAME, &frame)
}

/// Ask the camera for the list of stored presets.
///
/// The reply payload is the plain list of stored slot ids (0-based, in
/// store order, no count byte): the vendor server sets the packet
/// payload length to the preset count and copies one id per preset, so
/// `[]` means nothing stored, `[1]` is one preset in slot 2, and
/// `[2, 1]` is two presets. Confirmed against the camera firmware (the
/// AI module's `rpcHandlerCenter` dispatch for index `0xED`) and
/// against every capture taken on the Tiny 2 Lite.
///
/// The device fills the reply asynchronously, hence the short settle
/// before reading the selector back.
///
/// # Errors
/// Propagates transport errors and reports a malformed reply as
/// [`XuError::Io`].
pub fn list_presets(camera: &File) -> Result<Vec<u8>, XuError> {
    let frame = crate::xu::frame::encode_get(next_seq(), 0x04, CMD_LIST);
    set_cur(camera, BUNIT_ID, SELECTOR_FRAME, &frame)?;
    std::thread::sleep(std::time::Duration::from_millis(250));
    let mut buf = [0u8; crate::xu::status::STATUS_LEN];
    get_cur(camera, BUNIT_ID, SELECTOR_FRAME, &mut buf)?;
    match crate::xu::frame::parse_reply(&buf) {
        Ok(reply) => Ok(reply.payload),
        Err(err) => Err(XuError::Io(std::io::Error::other(format!(
            "malformed reply: {err:?}"
        )))),
    }
}
