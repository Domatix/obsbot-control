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
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! Error and result types returned by [`Camera`](crate::Camera) backends.

use std::path::PathBuf;

use thiserror::Error;

/// Result alias used throughout `obsbot-core`.
pub type Result<T> = core::result::Result<T, Error>;

/// Errors returned by [`Camera`](crate::Camera) implementations.
///
/// The default trait methods on [`Camera`](crate::Camera) return
/// [`Error::Unsupported`] so backends only override what they actually
/// implement; the GUI distinguishes this from runtime failures by matching
/// on the variant.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// The requested feature is not implemented by this backend or is not
    /// advertised by the connected device.
    #[error("feature not supported by the connected camera or its backend")]
    Unsupported,

    /// The supplied value is outside the device-advertised range.
    #[error("value {value} is outside the allowed range [{min}, {max}]")]
    OutOfRange {
        /// Value supplied by the caller.
        value: i64,
        /// Inclusive minimum advertised by the device.
        min: i64,
        /// Inclusive maximum advertised by the device.
        max: i64,
    },

    /// The device node is busy — typically opened by another process.
    #[error("device {0:?} is busy")]
    Busy(PathBuf),

    /// The device disappeared between calls (e.g. unplugged).
    #[error("device disappeared")]
    Disconnected,

    /// Underlying I/O error from `std::io` or one of the backend crates.
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
