// SPDX-License-Identifier: GPL-3.0-or-later
//
// Copyright (C) 2026 Domatix and contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! CRC used by the selector `0x02` frame protocol.
//!
//! The firmware computes it with a table-driven routine
//! (`libremocpp_core`, `getcrc16`); the tables implement the standard
//! CRC-16/USB variant: polynomial `0xA001` (reflected `0x8005`),
//! initial value `0xFFFF`, final XOR `0xFFFF`. The result was checked
//! against every frame captured in `PROTOCOL.md` §3.2, header and
//! payload.

/// Compute the selector `0x02` frame CRC over `buf`.
///
/// The caller must have zeroed the checksum field in a copy of the
/// buffer before calling, which is what both the sender and the
/// camera-side validator do.
#[must_use]
pub fn crc16(buf: &[u8]) -> u16 {
    let mut crc: u16 = 0xffff;
    for &b in buf {
        crc ^= u16::from(b);
        for _ in 0..8 {
            crc = if crc & 1 != 0 {
                (crc >> 1) ^ 0xa001
            } else {
                crc >> 1
            };
        }
    }
    crc ^ 0xffff
}

#[cfg(test)]
mod tests {
    use super::crc16;

    #[test]
    fn known_header_checksum() {
        // Sleep -> Sleep frame header with the checksum field zeroed.
        let hdr = [
            0xaa, 0x25, 0xa5, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x0a, 0x02, 0xc2, 0xa0,
        ];
        assert_eq!(crc16(&hdr).to_le_bytes(), [0x5f, 0xef]);
    }

    #[test]
    fn known_payload_checksum() {
        // Sleep -> Sleep payload block, checksum field zeroed.
        let block = [0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        assert_eq!(crc16(&block).to_le_bytes(), [0xbe, 0x07]);
    }
}
