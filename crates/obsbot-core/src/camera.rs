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

//! The [`Camera`] trait and the supporting value types the GUI and CLI
//! consume.
//!
//! Shape matches `ARCHITECTURE.md §3.1`. Default trait methods return
//! [`Error::Unsupported`] so a backend only overrides what it actually
//! implements.

use std::path::PathBuf;

use crate::Error;
use crate::Result;

/// Identifying metadata for a connected camera.
///
/// Filled by the enumerator (T-011); consumers treat it as read-only.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CameraInfo {
    /// Vendor name string (e.g. `"Remo Tech Co., Ltd."`).
    pub vendor: String,
    /// Product name string as reported by the device
    /// (e.g. `"OBSBOT Tiny 2 Lite"`).
    pub product: String,
    /// USB vendor ID.
    pub vid: u16,
    /// USB product ID.
    pub pid: u16,
    /// USB `iSerial` string descriptor, if the device advertises one.
    ///
    /// **Note** — Tiny 2 Lite firmware 5.10 reports `iSerial = 0`
    /// (no string), so this is `None` on that model. Per-device
    /// persistence cannot key off serial alone for the Lite. See
    /// `PROTOCOL.md §5`.
    pub serial: Option<String>,
    /// Firmware version string, if known.
    pub firmware: Option<String>,
    /// Primary V4L2 capture device path (e.g. `/dev/video0`), if discovered.
    pub video_path: Option<PathBuf>,
}

/// Bit-set describing which optional features the connected device exposes.
///
/// Populated by the backend at open time from the device's V4L2 control
/// enumeration plus XU capabilities; the GUI uses it to hide controls a
/// given device does not actually support.
//
// Many `bool` fields are intentional and prescribed by
// `ARCHITECTURE.md §3.1` ("Capabilities is a struct of bools"). The
// pedantic `clippy::struct_excessive_bools` lint suggests refactoring
// into a state machine, which is the wrong shape here: these are
// independent feature flags, not interlocking states.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct Capabilities {
    // ---- PTZ ----
    /// Pan (absolute) supported.
    pub pan: bool,
    /// Tilt (absolute) supported.
    pub tilt: bool,
    /// Zoom (absolute) supported.
    pub zoom: bool,
    /// Pan speed (signed velocity) supported.
    pub pan_speed: bool,
    /// Tilt speed (signed velocity) supported.
    pub tilt_speed: bool,

    // ---- Image controls ----
    /// Brightness slider.
    pub brightness: bool,
    /// Contrast slider.
    pub contrast: bool,
    /// Saturation slider.
    pub saturation: bool,
    /// Hue slider.
    pub hue: bool,
    /// Sharpness slider.
    pub sharpness: bool,
    /// Gamma slider (typically vendor-XU-only on Tiny 2 family).
    pub gamma: bool,
    /// Backlight compensation.
    pub backlight_compensation: bool,
    /// Gain slider.
    pub gain: bool,

    // ---- White balance / exposure / focus ----
    /// Auto white balance toggle.
    pub white_balance_auto: bool,
    /// Manual white balance temperature (Kelvin) supported.
    pub white_balance_manual: bool,
    /// Auto exposure mode menu.
    pub exposure_auto: bool,
    /// Manual exposure time supported.
    pub exposure_manual: bool,
    /// Continuous autofocus toggle.
    pub focus_auto: bool,
    /// Manual focus position supported.
    pub focus_manual: bool,

    // ---- Misc V4L2 ----
    /// Anti-flicker menu (Disabled / 50 Hz / 60 Hz).
    pub anti_flicker: bool,

    // ---- Vendor XU (OBSBOT) ----
    /// HDR toggle.
    pub hdr: bool,
    /// Field-of-view discrete selector.
    pub fov_mode: bool,
    /// Auto-framing mode selector.
    pub auto_framing: bool,
    /// Face-priority auto-exposure toggle.
    pub face_auto_exposure: bool,
    /// Face-priority auto-focus toggle.
    pub face_auto_focus: bool,
    /// Gesture-control toggle.
    pub gesture_control: bool,
}

/// Anti-flicker frequency setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum AntiFlicker {
    /// Anti-flicker disabled.
    Disabled,
    /// 50 Hz mains (Europe, most of Asia).
    Hz50,
    /// 60 Hz mains (Americas).
    Hz60,
}

/// Auto-exposure mode.
///
/// Mirrors the V4L2 `auto_exposure` menu observed on Tiny 2 Lite firmware
/// 5.10. Note that UVC reserves value `2` for "Shutter Priority", which
/// this firmware does **not** expose; a future device that does will
/// require a new variant rather than an `Other(u8)` catch-all (kept under
/// `#[non_exhaustive]` so adding it is non-breaking).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ExposureMode {
    /// Auto-exposure ("Auto Mode").
    Auto,
    /// Manual exposure.
    Manual,
    /// Aperture-priority auto-exposure.
    AperturePriority,
}

/// Field-of-view discrete setting for vendor cameras that support it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Fov {
    /// Widest FOV the device supports.
    Wide,
    /// Mid FOV (typical default).
    Medium,
    /// Narrowest FOV the device supports.
    Narrow,
}

/// Auto-framing mode for vendor cameras that support it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum AutoFramingMode {
    /// Auto-framing disabled.
    Off,
    /// Track a single subject.
    Single,
    /// Frame a group of people.
    Group,
    /// Track the subject's upper body.
    UpperBody,
}

/// A connected camera and the operations the GUI/CLI can perform on it.
///
/// Default trait methods return [`Error::Unsupported`]; backends override
/// only the methods they actually implement. The GUI uses
/// [`Camera::capabilities`] to decide which controls to surface, and
/// distinguishes "not supported" from runtime errors by matching the
/// returned [`Error`] variant.
pub trait Camera: Send + Sync {
    // ---- Identity & capabilities (required) ----

    /// Static identification info.
    fn info(&self) -> CameraInfo;

    /// Bit-set describing which controls this device exposes.
    fn capabilities(&self) -> Capabilities;

    // ---- Image controls ----

    /// Current brightness in the device-advertised range.
    ///
    /// # Errors
    /// Returns [`Error::Unsupported`] unless the backend overrides this.
    fn brightness(&self) -> Result<i32> {
        Err(Error::Unsupported)
    }
    /// Set brightness.
    ///
    /// # Errors
    /// Returns [`Error::Unsupported`] unless the backend overrides this.
    fn set_brightness(&self, _value: i32) -> Result<()> {
        Err(Error::Unsupported)
    }

    /// Current contrast.
    ///
    /// # Errors
    /// See [`Self::brightness`].
    fn contrast(&self) -> Result<i32> {
        Err(Error::Unsupported)
    }
    /// Set contrast.
    ///
    /// # Errors
    /// See [`Self::brightness`].
    fn set_contrast(&self, _value: i32) -> Result<()> {
        Err(Error::Unsupported)
    }

    /// Current saturation.
    ///
    /// # Errors
    /// See [`Self::brightness`].
    fn saturation(&self) -> Result<i32> {
        Err(Error::Unsupported)
    }
    /// Set saturation.
    ///
    /// # Errors
    /// See [`Self::brightness`].
    fn set_saturation(&self, _value: i32) -> Result<()> {
        Err(Error::Unsupported)
    }

    /// Current hue.
    ///
    /// # Errors
    /// See [`Self::brightness`].
    fn hue(&self) -> Result<i32> {
        Err(Error::Unsupported)
    }
    /// Set hue.
    ///
    /// # Errors
    /// See [`Self::brightness`].
    fn set_hue(&self, _value: i32) -> Result<()> {
        Err(Error::Unsupported)
    }

    /// Current sharpness.
    ///
    /// # Errors
    /// See [`Self::brightness`].
    fn sharpness(&self) -> Result<i32> {
        Err(Error::Unsupported)
    }
    /// Set sharpness.
    ///
    /// # Errors
    /// See [`Self::brightness`].
    fn set_sharpness(&self, _value: i32) -> Result<()> {
        Err(Error::Unsupported)
    }

    /// Current gamma.
    ///
    /// # Errors
    /// See [`Self::brightness`]. Gamma is typically XU-only on the Tiny 2
    /// family; see `PROTOCOL.md §2.3` quirk Q3.
    fn gamma(&self) -> Result<i32> {
        Err(Error::Unsupported)
    }
    /// Set gamma.
    ///
    /// # Errors
    /// See [`Self::gamma`].
    fn set_gamma(&self, _value: i32) -> Result<()> {
        Err(Error::Unsupported)
    }

    /// Current backlight-compensation value.
    ///
    /// # Errors
    /// See [`Self::brightness`].
    fn backlight_compensation(&self) -> Result<i32> {
        Err(Error::Unsupported)
    }
    /// Set backlight-compensation value.
    ///
    /// # Errors
    /// See [`Self::brightness`].
    fn set_backlight_compensation(&self, _value: i32) -> Result<()> {
        Err(Error::Unsupported)
    }

    /// Current sensor gain.
    ///
    /// # Errors
    /// See [`Self::brightness`].
    fn gain(&self) -> Result<i32> {
        Err(Error::Unsupported)
    }
    /// Set sensor gain.
    ///
    /// # Errors
    /// See [`Self::brightness`].
    fn set_gain(&self, _value: i32) -> Result<()> {
        Err(Error::Unsupported)
    }

    // ---- White balance ----

    /// Whether auto white-balance is currently engaged.
    ///
    /// # Errors
    /// See [`Self::brightness`].
    fn white_balance_auto(&self) -> Result<bool> {
        Err(Error::Unsupported)
    }
    /// Engage or release auto white-balance.
    ///
    /// # Errors
    /// See [`Self::brightness`].
    fn set_white_balance_auto(&self, _on: bool) -> Result<()> {
        Err(Error::Unsupported)
    }

    /// Manual white-balance colour temperature, in Kelvin.
    ///
    /// # Errors
    /// See [`Self::brightness`]. May also return `Error::Unsupported`
    /// when [`Self::white_balance_auto`] is currently `true`
    /// (the device freezes the manual control in that mode).
    fn white_balance_temperature(&self) -> Result<i32> {
        Err(Error::Unsupported)
    }
    /// Set manual white-balance colour temperature, in Kelvin.
    ///
    /// # Errors
    /// See [`Self::white_balance_temperature`].
    fn set_white_balance_temperature(&self, _kelvin: i32) -> Result<()> {
        Err(Error::Unsupported)
    }

    // ---- Exposure ----

    /// Current auto-exposure mode.
    ///
    /// # Errors
    /// See [`Self::brightness`].
    fn exposure_mode(&self) -> Result<ExposureMode> {
        Err(Error::Unsupported)
    }
    /// Set auto-exposure mode.
    ///
    /// # Errors
    /// See [`Self::brightness`].
    fn set_exposure_mode(&self, _mode: ExposureMode) -> Result<()> {
        Err(Error::Unsupported)
    }

    /// Manual exposure time, in the device's native units (Tiny 2 Lite:
    /// 1 unit = 100 µs).
    ///
    /// # Errors
    /// See [`Self::brightness`]. Returns `Error::Unsupported` while
    /// auto-exposure is engaged.
    fn exposure_time(&self) -> Result<i32> {
        Err(Error::Unsupported)
    }
    /// Set manual exposure time.
    ///
    /// # Errors
    /// See [`Self::exposure_time`].
    fn set_exposure_time(&self, _value: i32) -> Result<()> {
        Err(Error::Unsupported)
    }

    // ---- Focus ----

    /// Whether continuous autofocus is engaged.
    ///
    /// # Errors
    /// See [`Self::brightness`].
    fn focus_auto(&self) -> Result<bool> {
        Err(Error::Unsupported)
    }
    /// Engage or release continuous autofocus.
    ///
    /// # Errors
    /// See [`Self::brightness`].
    fn set_focus_auto(&self, _on: bool) -> Result<()> {
        Err(Error::Unsupported)
    }

    /// Manual focus position.
    ///
    /// # Errors
    /// See [`Self::brightness`]. Returns `Error::Unsupported` while
    /// autofocus is engaged.
    fn focus(&self) -> Result<i32> {
        Err(Error::Unsupported)
    }
    /// Set manual focus position.
    ///
    /// # Errors
    /// See [`Self::focus`].
    fn set_focus(&self, _value: i32) -> Result<()> {
        Err(Error::Unsupported)
    }

    // ---- Anti-flicker ----

    /// Anti-flicker setting.
    ///
    /// # Errors
    /// See [`Self::brightness`].
    fn anti_flicker(&self) -> Result<AntiFlicker> {
        Err(Error::Unsupported)
    }
    /// Set anti-flicker setting.
    ///
    /// # Errors
    /// See [`Self::brightness`].
    fn set_anti_flicker(&self, _v: AntiFlicker) -> Result<()> {
        Err(Error::Unsupported)
    }

    // ---- PTZ ----

    /// Current pan (UVC units: degrees × 3600).
    ///
    /// # Errors
    /// See [`Self::brightness`].
    fn pan(&self) -> Result<i32> {
        Err(Error::Unsupported)
    }
    /// Set pan.
    ///
    /// # Errors
    /// See [`Self::brightness`].
    fn set_pan(&self, _value: i32) -> Result<()> {
        Err(Error::Unsupported)
    }

    /// Current tilt (UVC units: degrees × 3600).
    ///
    /// # Errors
    /// See [`Self::brightness`].
    fn tilt(&self) -> Result<i32> {
        Err(Error::Unsupported)
    }
    /// Set tilt.
    ///
    /// # Errors
    /// See [`Self::brightness`].
    fn set_tilt(&self, _value: i32) -> Result<()> {
        Err(Error::Unsupported)
    }

    /// Current zoom (device-relative units).
    ///
    /// # Errors
    /// See [`Self::brightness`].
    fn zoom(&self) -> Result<i32> {
        Err(Error::Unsupported)
    }
    /// Set zoom.
    ///
    /// # Errors
    /// See [`Self::brightness`].
    fn set_zoom(&self, _value: i32) -> Result<()> {
        Err(Error::Unsupported)
    }

    /// Current pan velocity (signed; `-1` = idle on Tiny 2 family).
    ///
    /// # Errors
    /// See [`Self::brightness`].
    fn pan_speed(&self) -> Result<i32> {
        Err(Error::Unsupported)
    }
    /// Set pan velocity.
    ///
    /// # Errors
    /// See [`Self::brightness`].
    fn set_pan_speed(&self, _value: i32) -> Result<()> {
        Err(Error::Unsupported)
    }

    /// Current tilt velocity (signed; `-1` = idle on Tiny 2 family).
    ///
    /// # Errors
    /// See [`Self::brightness`].
    fn tilt_speed(&self) -> Result<i32> {
        Err(Error::Unsupported)
    }
    /// Set tilt velocity.
    ///
    /// # Errors
    /// See [`Self::brightness`].
    fn set_tilt_speed(&self, _value: i32) -> Result<()> {
        Err(Error::Unsupported)
    }

    // ---- Vendor XU (OBSBOT-specific; may stay Unsupported on others) ----

    /// Whether HDR is enabled.
    ///
    /// # Errors
    /// Returns `Error::Unsupported` unless the device's XU exposes HDR.
    fn hdr_enabled(&self) -> Result<bool> {
        Err(Error::Unsupported)
    }
    /// Toggle HDR.
    ///
    /// # Errors
    /// See [`Self::hdr_enabled`].
    fn set_hdr_enabled(&self, _on: bool) -> Result<()> {
        Err(Error::Unsupported)
    }

    /// Current field-of-view setting.
    ///
    /// # Errors
    /// See [`Self::hdr_enabled`].
    fn fov(&self) -> Result<Fov> {
        Err(Error::Unsupported)
    }
    /// Set field-of-view setting.
    ///
    /// # Errors
    /// See [`Self::hdr_enabled`].
    fn set_fov(&self, _fov: Fov) -> Result<()> {
        Err(Error::Unsupported)
    }

    /// Current auto-framing mode.
    ///
    /// # Errors
    /// See [`Self::hdr_enabled`].
    fn auto_framing(&self) -> Result<AutoFramingMode> {
        Err(Error::Unsupported)
    }
    /// Set auto-framing mode.
    ///
    /// # Errors
    /// See [`Self::hdr_enabled`].
    fn set_auto_framing(&self, _mode: AutoFramingMode) -> Result<()> {
        Err(Error::Unsupported)
    }

    /// Whether face-priority auto-exposure is engaged.
    ///
    /// # Errors
    /// See [`Self::hdr_enabled`].
    fn face_auto_exposure(&self) -> Result<bool> {
        Err(Error::Unsupported)
    }
    /// Engage face-priority auto-exposure.
    ///
    /// # Errors
    /// See [`Self::hdr_enabled`].
    fn set_face_auto_exposure(&self, _on: bool) -> Result<()> {
        Err(Error::Unsupported)
    }

    /// Whether face-priority auto-focus is engaged.
    ///
    /// # Errors
    /// See [`Self::hdr_enabled`].
    fn face_auto_focus(&self) -> Result<bool> {
        Err(Error::Unsupported)
    }
    /// Engage face-priority auto-focus.
    ///
    /// # Errors
    /// See [`Self::hdr_enabled`].
    fn set_face_auto_focus(&self, _on: bool) -> Result<()> {
        Err(Error::Unsupported)
    }

    /// Whether gesture control is engaged.
    ///
    /// # Errors
    /// See [`Self::hdr_enabled`].
    fn gesture_control(&self) -> Result<bool> {
        Err(Error::Unsupported)
    }
    /// Engage gesture control.
    ///
    /// # Errors
    /// See [`Self::hdr_enabled`].
    fn set_gesture_control(&self, _on: bool) -> Result<()> {
        Err(Error::Unsupported)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DummyCamera;

    impl Camera for DummyCamera {
        fn info(&self) -> CameraInfo {
            CameraInfo {
                vendor: "Test Vendor".into(),
                product: "Dummy".into(),
                vid: 0,
                pid: 0,
                serial: None,
                firmware: None,
                video_path: None,
            }
        }

        fn capabilities(&self) -> Capabilities {
            Capabilities::default()
        }
    }

    #[test]
    fn default_methods_return_unsupported() {
        let cam = DummyCamera;
        assert!(matches!(cam.brightness(), Err(Error::Unsupported)));
        assert!(matches!(cam.set_pan(0), Err(Error::Unsupported)));
        assert!(matches!(cam.hdr_enabled(), Err(Error::Unsupported)));
        assert!(matches!(cam.fov(), Err(Error::Unsupported)));
    }

    #[test]
    fn capabilities_default_is_nothing_supported() {
        let caps = Capabilities::default();
        assert!(!caps.pan);
        assert!(!caps.hdr);
        assert!(!caps.auto_framing);
    }

    #[test]
    fn info_round_trips() {
        let cam = DummyCamera;
        let info = cam.info();
        assert_eq!(info.vid, 0);
        assert_eq!(info.product, "Dummy");
        assert!(info.serial.is_none());
    }
}
