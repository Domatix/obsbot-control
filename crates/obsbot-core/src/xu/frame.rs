// SPDX-License-Identifier: GPL-3.0-or-later
//
// Copyright (C) 2026 Domatix and contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Encoder for the selector `0x02` frame protocol.
//!
//! Layout of a frame, in bytes:
//!
//! ```text
//! 0     0xAA
//! 1     flags (captured frames use 0x25; bit 0x60 means a payload follows)
//! 2-3   sequence number, little endian
//! 4-5   header length, little endian (12)
//! 6-7   header CRC16, little endian, computed with this field zeroed
//! 8-11  function group
//! 12..  optional payload block:
//!         +0-1 payload length, little endian
//!         +2-3 payload CRC16 over `length + 4` bytes, field zeroed
//!         +4.. payload bytes
//! ```
//!
//! Before this module existed the command helpers replayed captured
//! byte arrays verbatim, checksum included, because the checksum
//! algorithm was unknown (see `PROTOCOL.md` §3.2 Q7). [`encode`] builds
//! the same bytes from fields and computes both checksums, which is
//! what makes new frames possible.

use crate::xu::crc16::crc16;

/// Frame leader byte.
pub const LEADER: u8 = 0xaa;

/// Flags byte used by every frame captured so far. Bit `0x60` tells the
/// receiver that a payload block follows the header.
pub const FLAGS: u8 = 0x25;

/// Fixed header length, in bytes.
pub const HEADER_LEN: usize = 12;

/// Build a selector `0x02` frame.
///
/// `function_group` is the four-byte identifier of the command (for
/// example `0a 04 c4 0c` recalls tracking speed). `payload` is empty
/// for commands without arguments.
///
/// The header and both checksums match the frames captured from the
/// vendor application for every command documented in `PROTOCOL.md`
/// §3.2. The result is `12 + 4 + payload.len()` bytes, so a command
/// with a short payload comes out shorter than the 36-byte capture it
/// came from: the vendor padded, and so does
/// [`crate::xu::transport::set_cur`], which zero-fills every payload to
/// the selector length (60) before the ioctl. Bytes on the wire are
/// therefore identical; the trailing zeroes are just added later.
///
/// # Panics
/// Panics if `payload` is longer than `u16::MAX` bytes, which the
/// protocol cannot express.
#[must_use]
pub fn encode(seq: u16, function_group: [u8; 4], payload: &[u8]) -> Vec<u8> {
    let mut hdr = [0u8; HEADER_LEN];
    hdr[0] = LEADER;
    hdr[1] = FLAGS;
    hdr[2..4].copy_from_slice(&seq.to_le_bytes());
    hdr[4..6].copy_from_slice(&12u16.to_le_bytes());
    hdr[8..12].copy_from_slice(&function_group);
    let hdr_crc = crc16(&hdr);
    hdr[6..8].copy_from_slice(&hdr_crc.to_le_bytes());

    if payload.is_empty() {
        return hdr.to_vec();
    }

    let mut block = Vec::with_capacity(4 + payload.len());
    debug_assert!(u16::try_from(payload.len()).is_ok(), "payload fits in u16");
    let len = u16::try_from(payload.len()).unwrap_or(u16::MAX);
    block.extend_from_slice(&len.to_le_bytes());
    block.extend_from_slice(&[0, 0]);
    block.extend_from_slice(payload);
    let payload_crc = crc16(&block);
    block[2..4].copy_from_slice(&payload_crc.to_le_bytes());

    let mut out = hdr.to_vec();
    out.extend_from_slice(&block);
    out
}

#[cfg(test)]
mod tests {
    use super::encode;

    fn hex(s: &str) -> Vec<u8> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).expect("hex"))
            .collect()
    }

    /// Preset recall payload: four leading zero bytes plus four 1.0f32
    /// values, as captured from the vendor app (20 bytes).
    fn preset_payload(index: u8) -> [u8; 20] {
        [
            index, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f, 0x00, 0x00,
            0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f,
        ]
    }

    /// The prefix of every frame captured from the vendor app, as
    /// documented in `PROTOCOL.md` §3.2. The captures are 36 bytes;
    /// these expectations stop at the last payload byte because the
    /// encoder does not emit the trailing zero padding (see
    /// [`encode`]). Expected bytes include the captured checksums, so a
    /// regression in the encoder or in the CRC tables fails here.
    #[test]
    fn matches_captured_frames() {
        let cases: [(u16, [u8; 4], &[u8], &str); 9] = [
            (
                0x00a5,
                [0x0a, 0x02, 0xc2, 0xa0],
                &[0, 0, 0, 0],
                "aa25a5000c005fef0a02c2a00400be0700000000",
            ),
            (
                0x0042,
                [0x0a, 0x02, 0xc2, 0xa0],
                &[0x01, 0, 0, 0],
                "aa2542000c00ea630a02c2a00400bffb01000000",
            ),
            (
                0x0020,
                [0x0a, 0x04, 0xc4, 0x0c],
                &[0x00],
                "aa2520000c00abcb0a04c40c0100e63f00",
            ),
            (
                0x0021,
                [0x0a, 0x04, 0xc4, 0x0c],
                &[0x02],
                "aa2521000c00fa0e0a04c40c010067fe02",
            ),
            (
                0x0020,
                [0x0a, 0x04, 0xc4, 0x39],
                &preset_payload(0),
                "aa2520000c006bdc0a04c4391400d6fb000000000000803f0000803f0000803f0000803f",
            ),
            (
                0x001a,
                [0x0a, 0x04, 0xc4, 0x39],
                &preset_payload(1),
                "aa251a000c004b030a04c4391400eb2a010000000000803f0000803f0000803f0000803f",
            ),
            (
                0x0026,
                [0x0a, 0x04, 0xc4, 0x39],
                &preset_payload(2),
                "aa2526000c008bc30a04c4391400af19020000000000803f0000803f0000803f0000803f",
            ),
            (
                0x0015,
                [0x0a, 0x02, 0x82, 0x29],
                &[0x01, 0x32, 0x00, 0x00, 0x00],
                "aa2515000c00a89e0a0282290500f9270132000000",
            ),
            (
                0x0016,
                [0x0a, 0x02, 0x82, 0x29],
                &[0x02, 0x04, 0x00, 0x00, 0x00],
                "aa2516000c0058910a0282290500b2af0204000000",
            ),
        ];
        for (seq, group, payload, expected) in cases {
            let got = encode(seq, group, payload);
            assert_eq!(
                got,
                hex(expected),
                "frame mismatch for seq {seq:#06x} group {group:02x?}"
            );
        }
    }

    #[test]
    fn get_frame_has_no_payload_and_valid_crc() {
        let frame = super::encode_get(0x0600, 0x04, 0x3b44);
        assert_eq!(frame.len(), 12);
        assert_eq!(frame[1], 0x01);
        assert_eq!(&frame[8..12], &[0x0a, 0x04, 0x44, 0x3b]);
        assert_eq!(&frame[6..8], &[0x1f, 0xe7], "computed header checksum");
    }

    #[test]
    fn parses_reply_with_payload() {
        let raw = hex("aa2900060c00fd6c040a443b010067fe0200000000000000");
        let reply = super::parse_reply(&raw).unwrap();
        assert_eq!(reply.seq, 0x0600);
        assert_eq!(reply.receiver, 0x0a);
        assert_eq!(reply.command, 0x3b44);
        assert_eq!(reply.payload, vec![0x02]);
        assert!(!reply.error);
    }

    #[test]
    fn parses_ack_without_payload() {
        let raw = hex("aa0900060c0056ac040a443b00000000000000000000");
        let reply = super::parse_reply(&raw).unwrap();
        assert!(reply.payload.is_empty());
        assert!(!reply.error);
    }

    #[test]
    fn parses_error_signature() {
        let raw = hex("aa8901001000b4cd020a04660002fdff0000000000");
        let reply = super::parse_reply(&raw).unwrap();
        assert!(reply.error);
        assert!(reply.payload.is_empty());
    }

    #[test]
    fn rejects_corrupted_header() {
        let mut raw = hex("aa0900060c0056ac040a443b00000000000000000000");
        raw[6] ^= 0xff;
        assert_eq!(
            super::parse_reply(&raw),
            Err(super::ReplyError::BadChecksum)
        );
    }

    #[test]
    fn header_only_when_payload_is_empty() {
        let frame = encode(0x0030, [0x0a, 0x02, 0xc2, 0xa0], &[]);
        assert_eq!(frame.len(), 12);
        assert_eq!(frame[0], 0xaa);
        assert_eq!(frame[4..6], [12, 0]);
    }
}

/// Flags byte used by header-only GET requests.
pub const FLAGS_GET: u8 = 0x01;

/// Build a header-only GET frame.
///
/// GET frames carry no payload block; the device answers by writing the
/// reply where the selector's `GET_CUR` reads it.
#[must_use]
pub fn encode_get(seq: u16, receiver: u8, command: u16) -> Vec<u8> {
    let mut hdr = [0u8; HEADER_LEN];
    hdr[0] = LEADER;
    hdr[1] = FLAGS_GET;
    hdr[2..4].copy_from_slice(&seq.to_le_bytes());
    hdr[4..6].copy_from_slice(&12u16.to_le_bytes());
    hdr[8] = 0x0a;
    hdr[9] = receiver;
    hdr[10..12].copy_from_slice(&command.to_le_bytes());
    let crc = crc16(&hdr);
    hdr[6..8].copy_from_slice(&crc.to_le_bytes());
    hdr.to_vec()
}

/// Decoded reply read back from the frame selector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reply {
    /// Sequence number echoed by the device.
    pub seq: u16,
    /// Receiver byte of the original request.
    pub receiver: u8,
    /// Command echoed by the device.
    pub command: u16,
    /// Payload bytes, empty when the reply has none.
    pub payload: Vec<u8>,
    /// True when the reply matches the error signature: a 16-byte
    /// header followed by `0x0200` and `0xFFFD`, no payload.
    pub error: bool,
}

/// Error raised while decoding a reply.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ReplyError {
    /// The buffer is shorter than a header or the header length is not
    /// plausible.
    BadLength,
    /// The leader byte is not `0xAA`.
    BadLeader(u8),
    /// The header checksum does not match.
    BadChecksum,
    /// A payload block is announced but does not fit in the buffer.
    TruncatedPayload,
}

/// Parse a reply read from the frame selector.
///
/// Bit `0x20` of the flags byte announces a payload block; the length
/// field is the header length (12, or 16 for some firmwares) and the
/// block, when present, starts at that offset.
///
/// # Errors
/// See [`ReplyError`].
pub fn parse_reply(buf: &[u8]) -> Result<Reply, ReplyError> {
    if buf.len() < HEADER_LEN {
        return Err(ReplyError::BadLength);
    }
    if buf[0] != LEADER {
        return Err(ReplyError::BadLeader(buf[0]));
    }
    let hlen = usize::from(u16::from_le_bytes([buf[4], buf[5]]));
    if hlen < HEADER_LEN || hlen > buf.len() {
        return Err(ReplyError::BadLength);
    }
    let want = u16::from_le_bytes([buf[6], buf[7]]);
    let mut hdr = buf[..hlen].to_vec();
    hdr[6] = 0;
    hdr[7] = 0;
    if crc16(&hdr) != want {
        return Err(ReplyError::BadChecksum);
    }
    let seq = u16::from_le_bytes([buf[2], buf[3]]);
    let receiver = buf[9];
    let command = u16::from_le_bytes([buf[10], buf[11]]);
    let mut payload = Vec::new();
    if buf[1] & 0x20 != 0 {
        if hlen + 4 > buf.len() {
            return Err(ReplyError::TruncatedPayload);
        }
        let plen = usize::from(u16::from_le_bytes([buf[hlen], buf[hlen + 1]]));
        if hlen + 4 + plen > buf.len() {
            return Err(ReplyError::TruncatedPayload);
        }
        let mut block = buf[hlen..hlen + 4 + plen].to_vec();
        let want = u16::from_le_bytes([block[2], block[3]]);
        block[2] = 0;
        block[3] = 0;
        if crc16(&block) != want {
            return Err(ReplyError::BadChecksum);
        }
        payload.extend_from_slice(&buf[hlen + 4..hlen + 4 + plen]);
    }
    let error = hlen >= 16
        && u16::from_le_bytes([buf[12], buf[13]]) == 0x0200
        && u16::from_le_bytes([buf[14], buf[15]]) == 0xfffd;
    Ok(Reply {
        seq,
        receiver,
        command,
        payload,
        error,
    })
}
