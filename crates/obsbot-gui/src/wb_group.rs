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

//! White-balance group widget (T-103).
//!
//! Assembles `white_balance_automatic`, `white_balance_temperature`,
//! `red_balance`, and `blue_balance` into a single
//! [`adw::PreferencesGroup`] titled "White balance" with a description
//! explaining the auto / manual relationship. The four IDs get filtered
//! out of [`crate::controls_view::render_controls`]'s generic loop so
//! they appear exactly once.
//!
//! The widgets themselves come from
//! [`crate::controls_view::control_row`] unchanged — the T-100 scale +
//! spin + reset trio (Integer) and the T-100 `AdwSwitchRow` (Boolean)
//! are reused as-is. Greying-out while WB Auto is on continues to be
//! driven by the V4L2 `INACTIVE` flag → `ControlDescriptor.is_active`
//! propagation from T-102.

use std::path::Path;

use libadwaita as adw;

use adw::prelude::*;
use obsbot_core::ControlDescriptor;

use crate::controls_view::control_row;
use crate::i18n::gettext;

const CID_WHITE_BALANCE_AUTOMATIC: u32 = 0x0098_090c;
const CID_RED_BALANCE: u32 = 0x0098_090e;
const CID_BLUE_BALANCE: u32 = 0x0098_090f;
const CID_WHITE_BALANCE_TEMPERATURE: u32 = 0x0098_091a;

/// Control IDs claimed by the WB group widget. Consumers filter these
/// from the generic per-class render so they appear only inside the
/// dedicated group.
pub const WB_GROUP_IDS: &[u32] = &[
    CID_WHITE_BALANCE_AUTOMATIC,
    CID_WHITE_BALANCE_TEMPERATURE,
    CID_RED_BALANCE,
    CID_BLUE_BALANCE,
];

/// Build the "White balance" preferences group. Returns `None` when
/// none of the four WB controls is present — non-OBSBOT cameras that
/// only advertise one of them (or none) skip the group entirely.
pub fn build_wb_group(
    controls: &[ControlDescriptor],
    path: &Path,
    serial: Option<&str>,
) -> Option<adw::PreferencesGroup> {
    // Display order: Auto switch first (it gates everything else),
    // then temperature, then red / blue balance.
    let ordered_ids = [
        CID_WHITE_BALANCE_AUTOMATIC,
        CID_WHITE_BALANCE_TEMPERATURE,
        CID_RED_BALANCE,
        CID_BLUE_BALANCE,
    ];

    let present: Vec<&ControlDescriptor> = ordered_ids
        .iter()
        .filter_map(|id| controls.iter().find(|c| c.id == *id))
        .collect();

    if present.is_empty() {
        return None;
    }

    let group = adw::PreferencesGroup::builder()
        .title(gettext("White balance"))
        .build();

    for ctrl in present {
        let row = control_row(ctrl, path, serial);
        row.set_sensitive(ctrl.is_active);
        crate::settings::register_row(ctrl.id, &row);
        group.add(&row);
    }

    Some(group)
}
