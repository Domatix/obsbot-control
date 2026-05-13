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

//! Exposure group widget (T-104).
//!
//! Symmetric to [`crate::wb_group`] but for Camera-class exposure
//! controls: `auto_exposure` (menu) + `exposure_time_absolute` (int).
//! Per `PROTOCOL §2.2`, the kernel marks `exposure_time_absolute` as
//! `INACTIVE` while `auto_exposure ∈ {0=Auto, 3=Aperture Priority}`;
//! the T-102 [`ControlDescriptor::is_active`] propagation grey-outs
//! the slider automatically — no explicit listener needed here.

use std::path::Path;

use libadwaita as adw;

use adw::prelude::*;
use obsbot_core::ControlDescriptor;

use crate::controls_view::control_row;

const CID_AUTO_EXPOSURE: u32 = 0x009a_0901;
const CID_EXPOSURE_TIME_ABSOLUTE: u32 = 0x009a_0902;

/// Control IDs claimed by the exposure group widget. Consumers filter
/// these from the generic per-class render so they appear only inside
/// the dedicated group.
pub const EXPOSURE_GROUP_IDS: &[u32] = &[CID_AUTO_EXPOSURE, CID_EXPOSURE_TIME_ABSOLUTE];

/// Build the "Exposure" preferences group. Returns `None` when neither
/// auto-exposure nor manual exposure time is advertised.
pub fn build_exposure_group(
    controls: &[ControlDescriptor],
    path: &Path,
) -> Option<adw::PreferencesGroup> {
    let ordered_ids = [CID_AUTO_EXPOSURE, CID_EXPOSURE_TIME_ABSOLUTE];

    let present: Vec<&ControlDescriptor> = ordered_ids
        .iter()
        .filter_map(|id| controls.iter().find(|c| c.id == *id))
        .collect();

    if present.is_empty() {
        return None;
    }

    let group = adw::PreferencesGroup::builder()
        .title("Exposure")
        .description(
            "Choose Manual to drive the exposure time yourself; in Auto / Aperture Priority \
             the camera firmware picks it and freezes the slider.",
        )
        .build();

    for ctrl in present {
        let row = control_row(ctrl, path);
        row.set_sensitive(ctrl.is_active);
        group.add(&row);
    }

    Some(group)
}
