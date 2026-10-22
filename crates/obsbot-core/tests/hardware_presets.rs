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

//! Hardware-dependent integration tests for the preset store (framed
//! commands plus the flat list and entry cursor).
//!
//! These tests are `#[ignore]`d by default: they need a Tiny 2 family
//! unit and they write to its preset slots. Run them with
//! `cargo test -p obsbot-core --test hardware_presets -- --ignored`.
//!
//! What they deliberately avoid, following `docs/PROTOCOL.md` Q12:
//! no streaming client is opened, and no absolute pan/tilt write is
//! issued. Each test saves the *current* pose, touches one slot, and
//! deletes it again through a Drop guard, so the camera keeps whatever
//! position it had and the store ends up as it started.
//!
//! Tests are serialised with a process-wide mutex: the runner may use
//! several threads and the device is a single shared resource.

use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard};

use obsbot_core::xu::commands::preset;
use obsbot_core::xu::commands::preset_read::{list_preset_slots, read_preset_entries};

/// Serialises the tests: one device, one command stream at a time.
static DEVICE: Mutex<()> = Mutex::new(());

fn lock() -> MutexGuard<'static, ()> {
    DEVICE
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// Path of the first `/dev/videoN` that answers a preset list read.
fn device_path() -> PathBuf {
    for n in 0..4 {
        let path = PathBuf::from(format!("/dev/video{n}"));
        if !path.exists() {
            continue;
        }
        if let Ok(file) = OpenOptions::new().read(true).write(true).open(&path) {
            if list_preset_slots(&file).is_ok() {
                return path;
            }
        }
    }
    panic!("no OBSBOT device answered the preset list read; is the camera plugged in?");
}

/// Deletes `slot` when dropped, so a failing assertion does not leave
/// the user's camera with a test preset in it.
///
/// Only ever constructed for a slot the test found free: the camera
/// store belongs to whoever is using it, and a test that overwrites an
/// occupied slot destroys a position nobody can recover.
struct SlotGuard<'a> {
    file: &'a File,
    slot: u8,
}

impl Drop for SlotGuard<'_> {
    fn drop(&mut self) {
        let _ = preset::delete_preset(self.file, self.slot);
    }
}

/// First slot (1-based, as the admin commands take it) that the flat
/// list reports as free, or `None` when the store is full.
fn free_slot(file: &File) -> Option<u8> {
    let used = list_preset_slots(file).expect("list preset slots");
    (1..=3).find(|slot| !used.contains(&(slot - 1)))
}

#[test]
#[ignore = "requires a Tiny 2 family camera plugged into the host"]
fn flat_list_reports_only_valid_slots() {
    let _serial = lock();
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(device_path())
        .expect("open device");

    let ids = list_preset_slots(&file).expect("list preset slots");
    assert!(
        ids.iter().all(|id| *id < 3),
        "the Lite has three slots, ids must be 0..2, got {ids:?}",
    );
}

#[test]
#[ignore = "requires a Tiny 2 family camera plugged into the host"]
fn save_rename_read_and_delete_roundtrip() {
    let _serial = lock();
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(device_path())
        .expect("open device");

    let Some(slot) = free_slot(&file) else {
        eprintln!(
            "skipping: all three preset slots are in use and this test will not overwrite \
             a stored preset; delete one and run it again"
        );
        return;
    };
    let _guard = SlotGuard { file: &file, slot };

    // Save the current pose so the test never moves the gimbal.
    preset::add_preset(
        &file,
        slot,
        preset::Pose {
            pan: 0.0,
            tilt: 0.0,
            roll: 0.0,
            zoom: 0.0,
        },
    )
    .expect("save preset");

    let ids = list_preset_slots(&file).expect("list after save");
    assert!(
        ids.contains(&(slot - 1)),
        "saved slot {slot} should appear in the flat list, got {ids:?}",
    );

    let name = "HardwareTest";
    preset::rename_preset(&file, slot, name).expect("rename preset");

    let entries = read_preset_entries(&file).expect("read entries");
    let entry = entries
        .iter()
        .find(|entry| entry.slot == slot - 1)
        .expect("saved slot should have an entry");
    assert_eq!(
        entry.name, name,
        "the entry cursor should return the name written by the framed command",
    );

    preset::delete_preset(&file, slot).expect("delete preset");
    let ids = list_preset_slots(&file).expect("list after delete");
    assert!(
        !ids.contains(&(slot - 1)),
        "deleted slot {slot} should be gone from the flat list, got {ids:?}",
    );
}

#[test]
#[ignore = "requires a Tiny 2 family camera plugged into the host"]
fn entry_cursor_can_be_walked_twice() {
    let _serial = lock();
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(device_path())
        .expect("open device");

    // The cursor is consuming: without the selector-0x0C echo reset the
    // second walk would come back empty even when presets are stored.
    let first = read_preset_entries(&file).expect("first walk");
    let second = read_preset_entries(&file).expect("second walk");
    assert_eq!(
        first.len(),
        second.len(),
        "a fresh walk must see the same presets: the reset happens inside read_preset_entries",
    );
}
