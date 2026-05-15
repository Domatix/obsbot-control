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

//! `UVCIOC_CTRL_QUERY` wrapper, UVC request codes, and the one-byte
//! `bUnitID` constant for the Tiny 2 vendor XU.
//!
//! This is the only module in `obsbot-core` that issues unsafe calls
//! — the `nix::ioctl_readwrite!` macro generates an `unsafe fn` and
//! the kernel struct carries a raw pointer to the payload buffer. The
//! crate-wide lint is `unsafe_code = "deny"` (relaxed from `forbid`
//! in T-300 specifically for this module); the rest of the crate
//! remains free of `unsafe`.

// Scoped allow for the single ioctl call below; see module docs.
#![allow(unsafe_code)]

use std::fs::File;
use std::os::fd::AsRawFd;

use crate::xu::errors::XuError;

/// XU unit ID for the Tiny 2 family vendor Extension Unit.
///
/// Confirmed against the user's Tiny 2 Lite descriptor in
/// `PROTOCOL.md §1.1` (`EXTENSION_UNIT bUnitID=2`); the same value
/// works against the regular Tiny 2 per cgevans/tiny2 + Tiny4Linux's
/// hard-coded call sites.
pub const BUNIT_ID: u8 = 0x02;

/// Selector `0x06` — opcode-multiplexed `[op, len, payload]` block.
///
/// Carries HDR / Face AE / FOV / AI tracking on `SET_CUR`, and
/// returns the 60-byte global status struct on `GET_CUR`.
pub const SELECTOR_OPCODE: u8 = 0x06;

/// Selector `0x02` — structured 36-byte frames.
///
/// Carries the Auto/Manual exposure toggle, Sleep/Wake, Tracking
/// Speed, and Preset position recall.
pub const SELECTOR_FRAME: u8 = 0x02;

/// UVC class-specific request codes (USB Video Class 1.5 §4.2.1).
pub mod uvc {
    /// Write the current setting (host → device).
    pub const SET_CUR: u8 = 0x01;
    /// Read the current setting (device → host).
    pub const GET_CUR: u8 = 0x81;
    /// Read the minimum value (device → host).
    pub const GET_MIN: u8 = 0x82;
    /// Read the maximum value (device → host).
    pub const GET_MAX: u8 = 0x83;
    /// Read the resolution (smallest step, device → host).
    pub const GET_RES: u8 = 0x84;
    /// Read the payload length the device expects on this selector.
    pub const GET_LEN: u8 = 0x85;
    /// Read the capability info bitmap (device → host).
    pub const GET_INFO: u8 = 0x86;
    /// Read the default value (device → host).
    pub const GET_DEF: u8 = 0x87;
}

/// Kernel `struct uvc_xu_control_query` — `include/uapi/linux/uvcvideo.h`.
///
/// `_IOWR('u', 0x21, struct uvc_xu_control_query)` — a single struct
/// (not a buffer ioctl), so the matching `nix` macro is
/// `ioctl_readwrite!`, not `ioctl_readwrite_buf!`.
#[repr(C)]
struct UvcXuControlQuery {
    unit: u8,
    selector: u8,
    query: u8,
    size: u16,
    data: *mut u8,
}

// The macro generates `unsafe fn uvcioc_ctrl_query(fd, *mut T) ->
// nix::Result<i32>` without a docstring; the function is private to
// this module (only `xu_query` below calls it).
#[allow(missing_docs)]
mod raw_ioctl {
    use super::UvcXuControlQuery;
    nix::ioctl_readwrite!(uvcioc_ctrl_query, b'u', 0x21, UvcXuControlQuery);
}
use raw_ioctl::uvcioc_ctrl_query;

/// Issue a `UVCIOC_CTRL_QUERY` against the given unit / selector.
///
/// `payload` is borrowed mutably so the same buffer can be reused for
/// `SET_CUR` (caller-filled bytes go to the device) and `GET_CUR`
/// (kernel-filled bytes come back). The buffer length must already
/// match the selector's declared length — call [`get_len`] first if
/// the caller does not know the size at compile time.
///
/// # Errors
/// Returns [`XuError::Io`] wrapping the kernel `errno` if the ioctl
/// fails (`EINVAL` on length mismatch, `EPIPE` if the device stalled,
/// `ENODEV` if the camera disappeared, etc.).
pub fn xu_query(
    camera: &File,
    unit: u8,
    selector: u8,
    request: u8,
    payload: &mut [u8],
) -> Result<(), XuError> {
    let size: u16 = u16::try_from(payload.len()).map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("xu_query payload too large: {} bytes", payload.len()),
        )
    })?;

    let mut q = UvcXuControlQuery {
        unit,
        selector,
        query: request,
        size,
        data: payload.as_mut_ptr(),
    };

    // SAFETY: `q` is a valid `UvcXuControlQuery` with a `data`
    // pointer derived from `payload`, which lives at least as long
    // as this call; `size` matches `payload.len()`. The fd is owned
    // by `camera` (kept alive across the call by the `&File`
    // borrow). The kernel never aliases `data` beyond this call.
    let result = unsafe { uvcioc_ctrl_query(camera.as_raw_fd(), &raw mut q) };

    result.map_err(|errno| std::io::Error::from_raw_os_error(errno as i32))?;

    Ok(())
}

/// Query the kernel for the byte length of an XU selector.
///
/// Both cgevans/tiny2 and OpenFoxes/Tiny4Linux call this before every
/// `GET_CUR` / `SET_CUR`; we replicate the paranoia because the
/// kernel returns `EINVAL` on payload-size mismatch.
///
/// # Errors
/// Returns [`XuError::Io`] if the ioctl fails.
pub fn get_len(camera: &File, unit: u8, selector: u8) -> Result<u16, XuError> {
    let mut buf = [0u8; 2];
    xu_query(camera, unit, selector, uvc::GET_LEN, &mut buf)?;
    Ok(u16::from_le_bytes(buf))
}

/// Send a `SET_CUR` payload to the XU.
///
/// The Linux uvcvideo driver requires the `size` field of the
/// `uvc_xu_control_query` to exactly match the selector's
/// declared length (see `drivers/media/usb/uvc/uvc_v4l2.c` —
/// `EINVAL` otherwise). On the Tiny 2 family, both selectors
/// `0x02` and `0x06` declare 60 bytes via `UVC_GET_LEN`, while
/// the meaningful payloads are 3-4 bytes (selector `0x06`
/// opcodes) or 36 bytes (selector `0x02` structured frames).
///
/// This function therefore queries `GET_LEN` first, then
/// zero-pads `payload` up to that length before issuing the
/// ioctl. Payloads strictly larger than the selector length
/// are rejected via [`XuError::LengthMismatch`] — the firmware
/// has no defined meaning for the over-length tail.
///
/// # Errors
/// Returns [`XuError::LengthMismatch`] when `payload.len() >
/// GET_LEN(selector)`; otherwise propagates [`XuError::Io`]
/// from the underlying ioctl.
pub fn set_cur(camera: &File, unit: u8, selector: u8, payload: &[u8]) -> Result<(), XuError> {
    let selector_len = get_len(camera, unit, selector)?;
    let selector_len_usize = usize::from(selector_len);
    if payload.len() > selector_len_usize {
        return Err(XuError::LengthMismatch {
            unit,
            selector,
            payload_len: payload.len(),
            selector_len,
        });
    }
    // Zero-pad the payload to the selector's declared length —
    // the kernel requires `xqry->size == ctrl->info.size` exactly,
    // and the firmware ignores the trailing zero bytes for the
    // commands we currently send.
    let mut buf = vec![0u8; selector_len_usize];
    buf[..payload.len()].copy_from_slice(payload);
    xu_query(camera, unit, selector, uvc::SET_CUR, &mut buf)
}

/// Read a `GET_CUR` payload from the XU into the caller's buffer.
///
/// The buffer length must already match the selector's declared
/// length — the kernel validates this exactly the same way as
/// for `SET_CUR`, so callers should size their buffer to
/// `GET_LEN(selector)`. For the Tiny 2 family that is 60 bytes
/// (use [`crate::xu::status::STATUS_LEN`]).
///
/// # Errors
/// Returns [`XuError::LengthMismatch`] when `buf.len() !=
/// GET_LEN(selector)`; otherwise propagates [`XuError::Io`].
pub fn get_cur(camera: &File, unit: u8, selector: u8, buf: &mut [u8]) -> Result<(), XuError> {
    let selector_len = get_len(camera, unit, selector)?;
    if usize::from(selector_len) != buf.len() {
        return Err(XuError::LengthMismatch {
            unit,
            selector,
            payload_len: buf.len(),
            selector_len,
        });
    }
    xu_query(camera, unit, selector, uvc::GET_CUR, buf)
}
