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

//! Translations for the V4L2 control names and menu labels.
//!
//! `read_controls` hands back the driver's own labels ("White Balance,
//! Automatic", "Auto Mode"), which are English runtime data: gettext
//! cannot see them, so a translated app would still show an English
//! Image tab. Everything the camera advertises is mapped here by
//! control ID (stable) rather than by label text (which the driver
//! could reword), with the kernel label as fallback for unknown
//! controls so nothing ever disappears from the UI.
//!
//! The English label doubles as the gettext message id, so adding a
//! language needs no code change, only a catalog entry.

use crate::i18n::gettext;

/// Localised title for a V4L2 control.
#[must_use]
pub fn control_name(id: u32, kernel_name: &str) -> String {
    match id {
        // V4L2_CID_USER_CLASS_BASE + n (image controls).
        0x0098_0900 => gettext("Brightness"),
        0x0098_0901 => gettext("Contrast"),
        0x0098_0902 => gettext("Saturation"),
        0x0098_0903 => gettext("Hue"),
        0x0098_090c => gettext("White Balance, Automatic"),
        0x0098_090e => gettext("Red Balance"),
        0x0098_090f => gettext("Blue Balance"),
        0x0098_0913 => gettext("Gain"),
        0x0098_0918 => gettext("Power Line Frequency"),
        0x0098_091a => gettext("White Balance Temperature"),
        0x0098_091b => gettext("Sharpness"),
        0x0098_091c => gettext("Backlight Compensation"),
        // V4L2_CID_CAMERA_CLASS_BASE + n (exposure and PTZ).
        0x009a_0901 => gettext("Auto Exposure"),
        0x009a_0902 => gettext("Exposure Time, Absolute"),
        0x009a_0908 => gettext("Pan, Absolute"),
        0x009a_0909 => gettext("Tilt, Absolute"),
        0x009a_090a => gettext("Focus, Absolute"),
        0x009a_090c => gettext("Focus, Automatic Continuous"),
        0x009a_090d => gettext("Zoom, Absolute"),
        0x009a_090f => gettext("Zoom, Continuous"),
        0x009a_0920 => gettext("Pan, Speed"),
        0x009a_0921 => gettext("Tilt, Speed"),
        _ => kernel_name.to_string(),
    }
}

/// Localised labels for the menu options this camera advertises.
///
/// Options travel as plain strings inside [`ControlKind::Menu`], so the
/// match is keyed by both the control and the driver's label.
#[must_use]
pub fn menu_option(control_id: u32, kernel_label: &str) -> String {
    match (control_id, kernel_label) {
        (0x0098_0918, "Disabled") => gettext("Disabled"),
        (0x0098_0918, "50 Hz") => gettext("50 Hz"),
        (0x0098_0918, "60 Hz") => gettext("60 Hz"),
        (0x009a_0901, "Auto Mode") => gettext("Auto Mode"),
        (0x009a_0901, "Manual Mode") => gettext("Manual Mode"),
        (0x009a_0901, "Aperture Priority Mode") => gettext("Aperture Priority Mode"),
        _ => kernel_label.to_string(),
    }
}
