// SPDX-License-Identifier: GPL-3.0-or-later
//
// Copyright (C) 2026 Domatix and contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

//! Reading the preset store back: flat slot list and the entry
//! cursor, with base64-encoded names.
//!
//! Two XU unit-2 selectors form the read path (both hardware-verified
//! on a Tiny 2 Lite, documented in `docs/PROTOCOL.md`):
//!
//! - selector `0x0C` — flat occupancy list `[count:u8, slotIdx...]`;
//! - selector `0x0D` — entry cursor: each `GET_CUR` returns one entry
//!   `[status, slotIdx, pitch i16, yaw i16, zoom x100, base64 name]`
//!   and **advances the cursor**. Echo-writing the `0x0C` block back
//!   resets it.
//!
//! The framed `AI_GET_GIMBAL_PRESET_ID_NAME` (`0x3B04`) exists in the
//! vendor opcode table but this firmware answers it with a structured
//! error (`0x0200 0x000a`); the cursor is the working read path.

use std::fs::File;
use std::time::Duration;

use crate::xu::transport::{get_cur, get_len, set_cur, BUNIT_ID};
use crate::xu::XuError;

/// XU unit 2 selector carrying the flat preset slot list.
const SEL_PRESET_LIST: u8 = 0x0c;
/// XU unit 2 selector walking one preset entry per read.
const SEL_PRESET_ENTRY: u8 = 0x0d;
/// Cursor status byte meaning "no more entries". The empty store
/// answers `0x03`, which repeats; both end the walk.
const ENTRY_EXHAUSTED: u8 = 0x02;
/// Cursor status byte carrying a real entry.
const ENTRY_VALID: u8 = 0x00;
/// Upper bound on slots walked before giving up (3 on the Lite;
/// headroom for other models).
const MAX_ENTRIES: usize = 8;
/// Bytes before the base64 name in an entry: status, slot, two unused
/// bytes, pitch, yaw and zoom.
const ENTRY_NAME_OFFSET: usize = 10;

/// One preset as reported by the entry cursor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresetEntry {
    /// 0-based slot index.
    pub slot: u8,
    /// Gimbal pitch in centidegrees (sign inverted vs `tilt_absolute`).
    pub pitch_centideg: i16,
    /// Gimbal yaw (pan) in centidegrees.
    pub yaw_centideg: i16,
    /// Zoom factor times 100 (100 = 1.0x).
    pub zoom_x100: u16,
    /// Decoded preset name.
    pub name: String,
}

/// Read the flat occupancy list (selector `0x0C`): the 0-based slot
/// indices of the stored presets.
///
/// # Errors
/// Propagates transport errors; a zeroed block (the idle state)
/// yields an empty vector.
pub fn list_preset_slots(camera: &File) -> Result<Vec<u8>, XuError> {
    let len = get_len(camera, BUNIT_ID, SEL_PRESET_LIST)?;
    if len == 0 {
        return Ok(Vec::new());
    }
    let mut buf = vec![0u8; usize::from(len)];
    get_cur(camera, BUNIT_ID, SEL_PRESET_LIST, &mut buf)?;
    let Some(&first) = buf.first() else {
        return Ok(Vec::new());
    };
    let count = usize::from(first).min(buf.len().saturating_sub(1));
    if count == 0 {
        return Ok(Vec::new());
    }
    Ok(buf[1..=count].to_vec())
}

/// Parse one entry from the cursor read.
///
/// `Ok(None)` when the status byte says the walk is over: `0x02` is
/// exhausted and `0x03` is what an empty store returns (it repeats, so
/// treating it as an entry fabricates ghost presets in slot 0). Any
/// other status is not an entry either.
///
/// # Errors
/// [`XuError::Io`] when the buffer is shorter than the fixed fields or
/// the name is not valid base64.
fn parse_entry(buf: &[u8]) -> Result<Option<PresetEntry>, XuError> {
    // Only status 0x00 is an entry. 0x02 is exhausted, 0x03 is what
    // the empty store returns (it repeats), and an empty buffer means
    // the read returned nothing.
    let status = buf.first().copied().unwrap_or(ENTRY_EXHAUSTED);
    if status != ENTRY_VALID {
        return Ok(None);
    }
    let fields = buf.get(..ENTRY_NAME_OFFSET).ok_or_else(|| {
        XuError::Io(std::io::Error::other(format!(
            "preset entry is {} bytes, need at least {ENTRY_NAME_OFFSET}",
            buf.len()
        )))
    })?;
    let name_field = &buf[ENTRY_NAME_OFFSET..];
    let end = name_field
        .iter()
        .position(|&b| b == 0)
        .unwrap_or(name_field.len());
    let name = decode_base64(&name_field[..end])
        .map_err(|err| XuError::Io(std::io::Error::other(format!("preset name: {err}"))))?;
    Ok(Some(PresetEntry {
        slot: fields[1],
        pitch_centideg: i16::from_le_bytes([fields[4], fields[5]]),
        yaw_centideg: i16::from_le_bytes([fields[6], fields[7]]),
        zoom_x100: u16::from_le_bytes([fields[8], fields[9]]),
        name,
    }))
}

/// Walk the preset entry cursor and collect every stored preset with
/// its pose and name.
///
/// Echoes the `0x0C` block back first to reset the cursor — without
/// that reset a cursor left exhausted by a previous walk answers
/// `0x02` immediately. The buffer is sized from `GET_LEN` on the entry
/// selector, like the flat-list read: `get_cur` requires an exact
/// length and a hardcoded 60 only works on firmware where the selector
/// happens to be that long.
///
/// # Errors
/// Propagates transport errors and reports malformed base64 names as
/// [`XuError::Io`].
pub fn read_preset_entries(camera: &File) -> Result<Vec<PresetEntry>, XuError> {
    let len = get_len(camera, BUNIT_ID, SEL_PRESET_LIST)?;
    if len == 0 {
        return Ok(Vec::new());
    }
    let mut block = vec![0u8; usize::from(len)];
    get_cur(camera, BUNIT_ID, SEL_PRESET_LIST, &mut block)?;
    set_cur(camera, BUNIT_ID, SEL_PRESET_LIST, &block)?;
    std::thread::sleep(Duration::from_millis(400));

    let entry_len = usize::from(get_len(camera, BUNIT_ID, SEL_PRESET_ENTRY)?);
    if entry_len < ENTRY_NAME_OFFSET {
        return Err(XuError::Io(std::io::Error::other(format!(
            "preset entry selector is {entry_len} bytes, need at least {ENTRY_NAME_OFFSET}"
        ))));
    }
    let mut buf = vec![0u8; entry_len];
    let mut out = Vec::new();
    for _ in 0..MAX_ENTRIES {
        buf.fill(0);
        get_cur(camera, BUNIT_ID, SEL_PRESET_ENTRY, &mut buf)?;
        let Some(entry) = parse_entry(&buf)? else {
            break;
        };
        out.push(entry);
        std::thread::sleep(Duration::from_millis(200));
    }
    Ok(out)
}

/// Decode standard-alphabet base64 with `=` padding. Hand-rolled to
/// keep `obsbot-core` dependency-free; the camera only produces this
/// one alphabet.
fn decode_base64(input: &[u8]) -> Result<String, &'static str> {
    fn value(b: u8) -> Option<u8> {
        match b {
            b'A'..=b'Z' => Some(b - b'A'),
            b'a'..=b'z' => Some(b - b'a' + 26),
            b'0'..=b'9' => Some(b - b'0' + 52),
            b'+' => Some(62),
            b'/' => Some(63),
            _ => None,
        }
    }
    let mut out = Vec::with_capacity(input.len() * 3 / 4);
    let mut acc: u32 = 0;
    let mut nbits = 0u32;
    for &b in input {
        if b == b'=' {
            break;
        }
        let v = value(b).ok_or("invalid base64 byte")?;
        acc = (acc << 6) | u32::from(v);
        nbits += 6;
        if nbits >= 8 {
            nbits -= 8;
            // Only the low byte is wanted; `acc` keeps growing across
            // iterations, so mask explicitly.
            #[allow(clippy::cast_possible_truncation, reason = "masked to 8 bits")]
            out.push(((acc >> nbits) & 0xff) as u8);
        }
    }
    String::from_utf8(out).map_err(|_| "name is not UTF-8")
}

#[cfg(test)]
mod tests {
    use super::{decode_base64, parse_entry};
    use crate::xu::XuError;

    #[test]
    fn parses_a_valid_entry() {
        // Entry captured on a Tiny 2 Lite: slot 1, pose, name "TestA".
        let mut buf = vec![0x00, 0x01, 0x00, 0x00, 0x10, 0x00, 0x20, 0x00, 0x64, 0x00];
        buf.extend_from_slice(b"VGVzdEE=");
        buf.resize(60, 0);
        let entry = parse_entry(&buf).unwrap().expect("one entry");
        assert_eq!(entry.slot, 1);
        assert_eq!(entry.pitch_centideg, 16);
        assert_eq!(entry.yaw_centideg, 32);
        assert_eq!(entry.zoom_x100, 100);
        assert_eq!(entry.name, "TestA");
    }

    #[test]
    fn exhausted_and_empty_store_statuses_end_the_walk() {
        assert!(parse_entry(&[0x02; 60]).unwrap().is_none());
        // The empty store answers 0x03 and repeats it for every read.
        assert!(parse_entry(&[0x03; 60]).unwrap().is_none());
        assert!(parse_entry(&[]).unwrap().is_none());
    }

    #[test]
    fn a_short_entry_is_an_error() {
        let err = parse_entry(&[0x00, 0x01, 0x00]).unwrap_err();
        assert!(matches!(err, XuError::Io(_)), "got {err:?}");
    }

    #[test]
    fn decodes_camera_captured_names() {
        // Captured on a Tiny 2 Lite after renaming a preset "TestA".
        assert_eq!(decode_base64(b"VGVzdEE=").unwrap(), "TestA");
        // And a capture where a rename to "Diez" was read back as
        // "Dieg". Not reproducible (seven later round-trips were
        // byte-exact), kept as a regression fixture for the decoder.
        assert_eq!(decode_base64(b"RGllZw==").unwrap(), "Dieg");
    }

    #[test]
    fn decodes_empty_and_padded_inputs() {
        assert_eq!(decode_base64(b"").unwrap(), "");
        assert_eq!(decode_base64(b"QQ==").unwrap(), "A");
        assert_eq!(decode_base64(b"QUI=").unwrap(), "AB");
    }

    #[test]
    fn rejects_garbage() {
        assert!(decode_base64(b"!!").is_err());
        assert!(decode_base64(b"\xff\xfe").is_err());
    }
}
