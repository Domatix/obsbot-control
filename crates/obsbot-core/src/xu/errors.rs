// SPDX-License-Identifier: GPL-3.0-or-later
//
// Copyright (C) 2026 Domatix and contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Errors returned by the XU transport, the per-command helpers, and
//! [`crate::xu::status::get_status`].
//!
//! Local to the `xu` module by design — the crate-wide [`crate::Error`]
//! stays minimal and converts at the boundary via the
//! `From<XuError> for crate::Error` impl below. Callers using the
//! `obsbot_core::Result` flavour can therefore use `?` against an
//! `xu::*` call.

use thiserror::Error;

use crate::xu::enums::EnumDecodeError;

/// Errors raised by the XU transport and the per-command helpers.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum XuError {
    /// Underlying I/O error from opening the device node or from the
    /// `UVCIOC_CTRL_QUERY` ioctl.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// The caller's payload is larger than the selector accepts.
    /// `set_cur` rejects this (the kernel would `EINVAL`); shorter
    /// payloads are silently zero-padded to the selector length
    /// before issuing the ioctl (see `transport::set_cur`).
    #[error("UVC payload too large on unit {unit:#x}, selector {selector:#x}: payload is {payload_len} bytes but the selector only accepts {selector_len} bytes")]
    LengthMismatch {
        /// The XU unit the length was queried for.
        unit: u8,
        /// The XU selector the length was queried for.
        selector: u8,
        /// Caller-supplied payload length, in bytes.
        payload_len: usize,
        /// Length the kernel reported via `UVC_GET_LEN`, in bytes.
        selector_len: u16,
    },

    /// A status byte did not decode to a known enum variant. See the
    /// per-enum [`EnumDecodeError`].
    #[error("XU status decode failed: {0}")]
    Decode(#[from] EnumDecodeError),

    /// A preset index is outside the camera's three-slot range.
    /// See `PROTOCOL.md §3.2 Q7`.
    #[error("preset index out of range (must be 0..=2, got {0})")]
    InvalidPresetIndex(i8),
}

impl From<XuError> for crate::Error {
    fn from(value: XuError) -> Self {
        match value {
            XuError::Io(io) => Self::Io(io),
            // All other XU errors collapse to a generic Io for now
            // (the crate-wide Error enum has no XU variants yet — when
            // the GUI wires up the XU surface in T-301 we will add a
            // dedicated `Error::Xu(XuError)` variant if a richer match
            // is required).
            other => Self::Io(std::io::Error::other(other.to_string())),
        }
    }
}
