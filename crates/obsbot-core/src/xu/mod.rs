// SPDX-License-Identifier: GPL-3.0-or-later
//
// Copyright (C) 2026 Domatix and contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! UVC Extension Unit (XU) command surface for the OBSBOT Tiny 2 family.
//!
//! Everything in this module operates against `bUnitID = 0x02` of the
//! Tiny 2 vendor XU (GUID `9a1e7291-6843-4683-6d92-39bc7906ee49`,
//! confirmed in `PROTOCOL.md §3.1`). The vendor surface is multiplexed
//! across two selectors:
//!
//! * `bSelector = 0x06` — opcode-multiplexed `[op, len, payload]`,
//!   carries HDR / Face Auto-Exposure / Field-of-View / AI tracking
//!   mode, and returns the 60-byte global status struct on `GET_CUR`.
//! * `bSelector = 0x02` — structured 36-byte frames, carries the
//!   Auto/Manual exposure toggle, Sleep/Wake, Tracking Speed, and
//!   Preset position recall (3 slots).
//!
//! The byte-level encoding is documented in
//! [`PROTOCOL.md §3.2`](https://github.com/Domatix/obsbot-control/blob/main/docs/PROTOCOL.md#32-tiny-2-xu-command-table--known-surface-foss-extracted).
//! It originates from two free-software reverse-engineering efforts
//! ported here under EUPL-1.2 → GPL-3 compatibility; see
//! [`CREDITS.md`](https://github.com/Domatix/obsbot-control/blob/main/CREDITS.md)
//! and `DECISIONS.md` ADR-0020 for the lineage and licence rationale.
//!
//! ## Calling conventions
//!
//! All command helpers take an `&File` opened against the camera's
//! V4L2 capture device (`/dev/videoN`). Open it with
//! [`std::fs::OpenOptions::new().read(true).write(true).open(path)`].
//! Permissions: stock `uvcvideo` rules — user in the `video` group, or
//! a matching udev rule. No libusb, no SDK, no telemetry.
//!
//! ## Module layout
//!
//! - [`transport`] — `UVCIOC_CTRL_QUERY` ioctl wrapper, UVC request
//!   codes, the only place that uses `unsafe`.
//! - [`enums`] — typed value enums ([`AiMode`], [`FovMode`],
//!   [`FaceAeMode`], [`SleepState`], [`TrackingSpeed`]) with
//!   `to_wire` / `TryFrom` round-trips.
//! - [`errors`] — [`XuError`] (kept distinct from
//!   [`crate::Error`]; converts at the crate boundary).
//! - [`command02`] — the 36-byte selector-0x02 frame builder.
//! - [`status`] — `get_status` returning the decoded 60-byte
//!   [`status::Status`] struct.
//! - [`v4l2_ptz`] — the standard V4L2 Pan/Tilt/Zoom CID constants
//!   used by the existing T-101 PTZ pad.
//! - [`commands`] — one file per command:
//!   [`commands::hdr`], [`commands::face_ae`], [`commands::fov`],
//!   [`commands::ai_mode`] on selector `0x06`;
//!   [`commands::exposure_mode_type`], [`commands::sleep`],
//!   [`commands::tracking_speed`], [`commands::preset`] on
//!   selector `0x02`.
//!
//! [`AiMode`]: enums::AiMode
//! [`FovMode`]: enums::FovMode
//! [`FaceAeMode`]: enums::FaceAeMode
//! [`SleepState`]: enums::SleepState
//! [`TrackingSpeed`]: enums::TrackingSpeed

// `clippy::doc_markdown` flags every prose mention of foreign words
// like `Tiny4Linux`, `cgevans`, `bon`, `Camset` as "item missing
// backticks". These are not code references — they are project /
// author names cited in attribution prose. Backticking them across
// the module would harm readability. Genuine code identifiers in
// doc comments are still backticked / linked manually.
#![allow(clippy::doc_markdown)]

pub mod command02;
pub mod commands;
pub mod enums;
pub mod errors;
pub mod status;
pub mod transport;
pub mod v4l2_ptz;

pub use enums::{AiMode, ExposureMode, FaceAeMode, FovMode, SleepState, TrackingSpeed};
pub use errors::XuError;
pub use status::{get_status, Status, STATUS_LEN};
pub use transport::{BUNIT_ID, SELECTOR_FRAME, SELECTOR_OPCODE};
