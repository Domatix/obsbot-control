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

//! V4L2 control enumeration and writes for the GUI / CLI.
//!
//! Opens a `/dev/videoN` node via the `v4l` crate, walks the device's
//! advertised controls, queries the current value of each, and reshapes
//! the result into an obsbot-core-owned [`ControlDescriptor`] vector so
//! consumers (the GUI sub-page, future CLI subcommands) never have to
//! depend on the `v4l` crate types directly. T-100 layers a
//! [`write_control`] helper on top for setting Integer / Boolean values
//! back to the driver. This is the discovery + write layer for the
//! [`Camera`](crate::Camera) trait's V4L2 paths.

use std::path::Path;

use v4l::control::{Control, Description, Flags, Type, Value};
use v4l::Device;

use crate::Result;

/// One V4L2 control, reshaped for UI consumption.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlDescriptor {
    /// Raw V4L2 control ID (e.g. `V4L2_CID_BRIGHTNESS = 0x0098_0900`).
    /// Stable across reads; pass it to [`write_control`] to set a value.
    pub id: u32,
    /// Human-readable name reported by the driver (e.g. `"Brightness"`).
    pub name: String,
    /// Which V4L2 class the control belongs to (User, Camera, …).
    pub class: ControlClass,
    /// Whether the driver currently considers this control writable.
    /// Mirrors the inverse of `V4L2_CTRL_FLAG_INACTIVE` — when `false`,
    /// the kernel silently ignores writes (e.g. `white_balance_
    /// temperature` while `white_balance_automatic = 1`). UI consumers
    /// should grey out inactive controls; backends can still observe
    /// the value via [`read_controls`].
    pub is_active: bool,
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
        /// Driver-advertised default value (used by GUI "reset" buttons).
        default: i64,
    },
    /// Boolean toggle.
    Boolean {
        /// Currently read-back value.
        current: bool,
        /// Driver-advertised default value.
        default: bool,
    },
    /// Menu of named items (regular or integer menu).
    Menu {
        /// Currently read-back value (the menu option's integer ID).
        current: i64,
        /// Driver-advertised default value. May fall outside `options`
        /// — see `PROTOCOL §2.3` quirk Q1 for the `power_line_
        /// frequency` example where the kernel reports `default=3`
        /// despite a `{0,1,2}` menu. UI consumers should not assume
        /// the default is selectable.
        default: i64,
        /// All option `(id, label)` pairs in driver order.
        options: Vec<(i64, String)>,
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
            id: desc.id,
            class: classify(desc.id),
            is_active: !desc.flags.contains(Flags::INACTIVE),
            kind: build_kind(&device, &desc),
            name: desc.name,
        });
    }

    Ok(out)
}

/// Value payload accepted by [`write_control`].
///
/// Mirrors the subset of `v4l::control::Value` variants we know how to
/// drive from the GUI today: Integer covers `V4L2_CTRL_TYPE_INTEGER`
/// and `V4L2_CTRL_TYPE_INTEGER64` (treated identically by the kernel
/// at write-time per `Documentation/userspace-api/media/v4l/vidioc-
/// g-ext-ctrls.rst`), Boolean covers `V4L2_CTRL_TYPE_BOOLEAN`, and
/// Menu covers `V4L2_CTRL_TYPE_MENU` / `INTEGER_MENU` (the V4L2
/// driver stores a menu's selection as an `__s32` value, same
/// write semantics as Integer).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ControlValue {
    /// Integer-valued write. Caller is responsible for staying inside
    /// the descriptor's `min`/`max`/`step` envelope; the kernel
    /// silently clamps out-of-range integers on most drivers.
    Integer(i64),
    /// Boolean toggle.
    Boolean(bool),
    /// Menu selection by the option's integer ID (as advertised in
    /// `ControlKind::Menu::options`).
    Menu(i64),
}

impl From<ControlValue> for Value {
    fn from(value: ControlValue) -> Self {
        match value {
            ControlValue::Integer(v) | ControlValue::Menu(v) => Value::Integer(v),
            ControlValue::Boolean(b) => Value::Boolean(b),
        }
    }
}

/// Write a single V4L2 control to the device at `video_path`.
///
/// Opens the node read-write (the underlying `v4l` crate's
/// `Device::with_path` does `O_RDWR | O_NONBLOCK`), then issues
/// `VIDIOC_S_EXT_CTRLS` for the (`id`, `value`) pair. Wraps any
/// `io::Error` raised by the open / ioctl as
/// [`Error::Io`](crate::Error::Io).
///
/// # Errors
/// * The user lacks `rw` on `video_path` (typical fix: `video` group
///   membership — see `T-013` notes).
/// * The driver rejects the value (out of range / control inactive
///   in the current pipeline state — see `PROTOCOL §2.3`).
/// * The device disappeared between calls.
pub fn write_control(video_path: &Path, id: u32, value: ControlValue) -> Result<()> {
    let device = Device::with_path(video_path)?;
    device.set_control(Control {
        id,
        value: value.into(),
    })?;
    Ok(())
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
            default: desc.default,
        },
        Type::Boolean => ControlKind::Boolean {
            current: read_integer(device, desc.id).map_or(desc.default != 0, |v| v != 0),
            default: desc.default != 0,
        },
        Type::Menu | Type::IntegerMenu => {
            let current = read_integer(device, desc.id).unwrap_or(desc.default);
            let options: Vec<(i64, String)> = desc
                .items
                .as_ref()
                .map(|items| {
                    items
                        .iter()
                        .map(|(idx, item)| (i64::from(*idx), item.to_string()))
                        .collect()
                })
                .unwrap_or_default();
            ControlKind::Menu {
                current,
                default: desc.default,
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

    #[test]
    fn control_value_maps_to_v4l_integer() {
        let v: Value = ControlValue::Integer(42).into();
        assert_eq!(v, Value::Integer(42));
    }

    #[test]
    fn control_value_maps_to_v4l_boolean() {
        let on: Value = ControlValue::Boolean(true).into();
        let off: Value = ControlValue::Boolean(false).into();
        assert_eq!(on, Value::Boolean(true));
        assert_eq!(off, Value::Boolean(false));
    }

    #[test]
    fn control_value_menu_maps_to_v4l_integer() {
        // V4L2 stores menu selections as `__s32`; ControlValue::Menu
        // must therefore round-trip through Value::Integer (not a
        // dedicated Value::Menu variant, which v4l 0.14 does not have).
        let v: Value = ControlValue::Menu(2).into();
        assert_eq!(v, Value::Integer(2));
    }
}
