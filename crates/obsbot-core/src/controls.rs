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

//! Read-only V4L2 control enumeration for the diagnostics view.
//!
//! Opens a `/dev/videoN` node via the `v4l` crate, walks the device's
//! advertised controls, queries the current value of each, and reshapes
//! the result into an obsbot-core-owned [`ControlDescriptor`] vector so
//! consumers (the GUI sub-page from T-013c, future CLI subcommands)
//! never have to depend on the `v4l` crate types directly. This is the
//! discovery layer for the [`Camera`](crate::Camera) trait's read paths
//! — backends that wire writes still go through the trait, not this
//! module.

use std::path::Path;

use v4l::control::{Description, Flags, Type, Value};
use v4l::Device;

use crate::Result;

/// One V4L2 control, reshaped for UI consumption.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlDescriptor {
    /// Human-readable name reported by the driver (e.g. `"Brightness"`).
    pub name: String,
    /// Which V4L2 class the control belongs to (User, Camera, …).
    pub class: ControlClass,
    /// Type-specific payload (current value plus range / options).
    pub kind: ControlKind,
}

/// V4L2 control class, reduced to the two we care about plus an escape
/// hatch for anything else (codec, image-source, etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ControlClass {
    /// `V4L2_CTRL_CLASS_USER` (`0x0098_0000`) — brightness, contrast, …
    User,
    /// `V4L2_CTRL_CLASS_CAMERA` (`0x009A_0000`) — PTZ, focus, exposure.
    Camera,
    /// Any other class (`0xXXXX_0000`).
    Other(u32),
}

/// Type-shaped view of a control's current value plus range or options.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ControlKind {
    /// Integer-valued control (UVC `Integer` / `Integer64`).
    Integer {
        /// Currently read-back value.
        current: i64,
        /// Inclusive minimum.
        min: i64,
        /// Inclusive maximum.
        max: i64,
        /// Step between valid values (always positive).
        step: u64,
    },
    /// Boolean toggle.
    Boolean {
        /// Currently read-back value.
        current: bool,
    },
    /// Menu of named items (regular or integer menu).
    Menu {
        /// Label for the currently selected item.
        current_label: String,
        /// All option labels in driver order.
        options: Vec<String>,
    },
    /// Anything else: surfaced as a name-only entry so the UI shows it
    /// exists without lying about its value. Carries the V4L2 type name
    /// as a debugging hint.
    Other(String),
}

/// Read every advertised, non-disabled, non-write-only V4L2 control
/// from `video_path`.
///
/// # Errors
/// Propagates any `io::Error` from opening the device or running
/// `VIDIOC_QUERY_EXT_CTRL` (wrapped in [`Error::Io`](crate::Error::Io)).
pub fn read_controls(video_path: &Path) -> Result<Vec<ControlDescriptor>> {
    let device = Device::with_path(video_path)?;
    let descriptions = device.query_controls()?;
    let mut out = Vec::with_capacity(descriptions.len());

    for desc in descriptions {
        if desc.typ == Type::CtrlClass {
            continue;
        }
        if desc.flags.contains(Flags::DISABLED) || desc.flags.contains(Flags::WRITE_ONLY) {
            continue;
        }

        out.push(ControlDescriptor {
            class: classify(desc.id),
            kind: build_kind(&device, &desc),
            name: desc.name,
        });
    }

    Ok(out)
}

/// Map a V4L2 control ID to its [`ControlClass`].
///
/// V4L2 packs the class into the high 16 bits of the ID — see
/// `V4L2_CTRL_ID2CLASS` in
/// `Documentation/userspace-api/media/v4l/vidioc-queryctrl.rst`.
fn classify(id: u32) -> ControlClass {
    const V4L2_CTRL_CLASS_USER: u32 = 0x0098_0000;
    const V4L2_CTRL_CLASS_CAMERA: u32 = 0x009A_0000;
    match id & 0xFFFF_0000 {
        V4L2_CTRL_CLASS_USER => ControlClass::User,
        V4L2_CTRL_CLASS_CAMERA => ControlClass::Camera,
        other => ControlClass::Other(other),
    }
}

/// Build the type-shaped [`ControlKind`] for a control, falling back to
/// the description's `default` if the current-value read fails (e.g. the
/// control is inactive while another mode is engaged — see
/// `PROTOCOL §2.3` Q1/Q2 for two such cases on the Tiny 2 family).
fn build_kind(device: &Device, desc: &Description) -> ControlKind {
    match desc.typ {
        Type::Integer | Type::Integer64 => ControlKind::Integer {
            current: read_integer(device, desc.id).unwrap_or(desc.default),
            min: desc.minimum,
            max: desc.maximum,
            step: desc.step,
        },
        Type::Boolean => ControlKind::Boolean {
            current: read_integer(device, desc.id).map_or(desc.default != 0, |v| v != 0),
        },
        Type::Menu | Type::IntegerMenu => {
            let current_index = read_integer(device, desc.id).unwrap_or(desc.default);
            let options: Vec<String> = desc
                .items
                .as_ref()
                .map(|items| items.iter().map(|(_, item)| item.to_string()).collect())
                .unwrap_or_default();
            let current_label = desc
                .items
                .as_ref()
                .and_then(|items| {
                    items
                        .iter()
                        .find(|(idx, _)| i64::from(*idx) == current_index)
                        .map(|(_, item)| item.to_string())
                })
                .unwrap_or_else(|| current_index.to_string());
            ControlKind::Menu {
                current_label,
                options,
            }
        }
        other => ControlKind::Other(format!("{other:?}")),
    }
}

/// Convenience wrapper: read the control by ID, coerce `Value::Integer`
/// out of it. Returns `None` if the read fails or the value is not an
/// integer-shaped variant.
fn read_integer(device: &Device, id: u32) -> Option<i64> {
    match device.control(id).ok()?.value {
        Value::Integer(v) => Some(v),
        Value::Boolean(b) => Some(i64::from(b)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_user_class_id() {
        // V4L2_CID_BRIGHTNESS = V4L2_CID_BASE + 0 = 0x00980900.
        assert_eq!(classify(0x0098_0900), ControlClass::User);
        // V4L2_CID_USER_CLASS itself.
        assert_eq!(classify(0x0098_0001), ControlClass::User);
    }

    #[test]
    fn classify_camera_class_id() {
        // V4L2_CID_PAN_ABSOLUTE = V4L2_CID_CAMERA_CLASS_BASE + 8.
        assert_eq!(classify(0x009A_0908), ControlClass::Camera);
        assert_eq!(classify(0x009A_0001), ControlClass::Camera);
    }

    #[test]
    fn classify_unknown_class_id() {
        // V4L2_CTRL_CLASS_CODEC = 0x00990000.
        assert_eq!(classify(0x0099_0123), ControlClass::Other(0x0099_0000));
    }
}
