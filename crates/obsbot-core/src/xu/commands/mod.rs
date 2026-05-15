// SPDX-License-Identifier: GPL-3.0-or-later
//
// Copyright (C) 2026 Domatix and contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Per-opcode and per-frame command helpers.
//!
//! Each submodule owns one wire-level command. Selector-0x06
//! opcodes go in [`hdr`], [`face_ae`], [`fov`], [`ai_mode`].
//! Selector-0x02 36-byte frames go in [`exposure_mode_type`],
//! [`sleep`], [`tracking_speed`], [`preset`].

pub mod ai_mode;
pub mod exposure_mode_type;
pub mod face_ae;
pub mod fov;
pub mod hdr;
pub mod preset;
pub mod sleep;
pub mod tracking_speed;

pub use ai_mode::set_ai_mode;
pub use exposure_mode_type::set_exposure_mode;
pub use face_ae::set_face_ae;
pub use fov::set_fov;
pub use hdr::set_hdr;
pub use preset::recall_preset;
pub use sleep::set_sleep;
pub use tracking_speed::set_tracking_speed;
