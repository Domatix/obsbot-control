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

//! Device abstraction for OBSBOT cameras.
//!
//! `obsbot-core` exposes the [`Camera`] trait that the CLI and GUI
//! consume, together with the value types ([`CameraInfo`],
//! [`Capabilities`], the per-feature enums) and the [`Error`] / [`Result`]
//! pair used by every backend method.
//!
//! Backends (V4L2 standard controls, UVC Extension Units, raw USB) will
//! land in later tasks (T-011 enumeration, T-100+ V4L2 wiring, T-300+ XU
//! reverse-engineering); this crate currently only provides the shape so
//! `obsbot-cli` (T-006) and `obsbot-gui` (T-007) can be scaffolded
//! against a stable surface.
//!
//! ## Shape at a glance
//!
//! ```no_run
//! use obsbot_core::{Camera, CameraInfo, Capabilities, Error};
//!
//! struct DummyCam;
//!
//! impl Camera for DummyCam {
//!     fn info(&self) -> CameraInfo {
//!         CameraInfo {
//!             vendor: "Test".into(),
//!             product: "Dummy".into(),
//!             vid: 0,
//!             pid: 0,
//!             serial: None,
//!             firmware: None,
//!             video_path: None,
//!         }
//!     }
//!     fn capabilities(&self) -> Capabilities {
//!         Capabilities::default()
//!     }
//!     // Every other method defaults to `Err(Error::Unsupported)`.
//! }
//!
//! let cam: Box<dyn Camera> = Box::new(DummyCam);
//! assert!(matches!(cam.brightness(), Err(Error::Unsupported)));
//! ```

#![warn(missing_docs)]

pub mod camera;
pub mod controls;
pub mod enumerate;
pub mod error;

pub use camera::{
    AntiFlicker, AutoFramingMode, Camera, CameraInfo, Capabilities, ExposureMode, Fov,
};
pub use controls::{
    read_controls, write_control, ControlClass, ControlDescriptor, ControlKind, ControlValue,
};
pub use enumerate::{enumerate_cameras, enumerate_cameras_in, TINY2_FAMILY, VID_OBSBOT};
pub use error::{Error, Result};
