// SPDX-License-Identifier: GPL-3.0-or-later
//
// Copyright (C) 2026 Domatix and contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! V4L2 standard Pan / Tilt / Zoom CIDs used by the Tiny 2 family.
//!
//! These are not XU controls — they live on the camera's
//! `INPUT_TERMINAL` (`bUnitID = 1`) per `PROTOCOL.md §1.1` and are
//! addressable through the standard V4L2 control API (`VIDIOC_G_CTRL`
//! / `VIDIOC_S_CTRL`). The T-101 PTZ pad already drives them via the
//! generic `obsbot-core::controls` write path; the constants live
//! here so the XU module surface is self-contained for callers that
//! want symbolic names instead of raw CID values.
//!
//! Source: `linux/v4l2-controls.h` `V4L2_CID_CAMERA_CLASS_BASE +
//! offset` (also documented inline in cgevans/tiny2's `src/usbio.rs`).

/// `V4L2_CID_CAMERA_CLASS_BASE = 0x009A_0900`.
pub const V4L2_CID_CAMERA_CLASS_BASE: u32 = 0x009A_0900;

/// Pan, absolute (degrees × 3600).
pub const V4L2_CID_PAN_ABSOLUTE: u32 = V4L2_CID_CAMERA_CLASS_BASE + 0x08;
/// Tilt, absolute (degrees × 3600).
pub const V4L2_CID_TILT_ABSOLUTE: u32 = V4L2_CID_CAMERA_CLASS_BASE + 0x09;
/// Pan, relative (-1, 0, +1).
pub const V4L2_CID_PAN_RELATIVE: u32 = V4L2_CID_CAMERA_CLASS_BASE + 0x0A;
/// Tilt, relative (-1, 0, +1).
pub const V4L2_CID_TILT_RELATIVE: u32 = V4L2_CID_CAMERA_CLASS_BASE + 0x0B;
/// Zoom, absolute (device-relative units).
pub const V4L2_CID_ZOOM_ABSOLUTE: u32 = V4L2_CID_CAMERA_CLASS_BASE + 0x0D;
/// Zoom, relative (-1, 0, +1).
pub const V4L2_CID_ZOOM_RELATIVE: u32 = V4L2_CID_CAMERA_CLASS_BASE + 0x0E;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cid_values_match_kernel_header() {
        // Cross-checked against linux/v4l2-controls.h on the user's
        // Debian trixie host (kernel 6.12); see also PROTOCOL.md §2.2
        // for the same values observed via v4l2-ctl --list-ctrls.
        assert_eq!(V4L2_CID_PAN_ABSOLUTE, 0x009A_0908);
        assert_eq!(V4L2_CID_TILT_ABSOLUTE, 0x009A_0909);
        assert_eq!(V4L2_CID_PAN_RELATIVE, 0x009A_090A);
        assert_eq!(V4L2_CID_TILT_RELATIVE, 0x009A_090B);
        assert_eq!(V4L2_CID_ZOOM_ABSOLUTE, 0x009A_090D);
        assert_eq!(V4L2_CID_ZOOM_RELATIVE, 0x009A_090E);
    }
}
