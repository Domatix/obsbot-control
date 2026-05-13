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

//! Discovery of connected OBSBOT cameras via Linux sysfs.
//!
//! Walks `/sys/class/video4linux/*`, follows each entry's `device`
//! symlink to its parent USB device, and filters by VID/PID against
//! [`TINY2_FAMILY`]. Returns one [`CameraInfo`] per *USB device* — a
//! single Tiny 2 family unit advertises two `/dev/videoN` nodes (one
//! capture stream, one metadata stream), so the per-device dedup keeps
//! the camera count honest. See `PROTOCOL.md §2` for the per-node
//! enumeration the user captured against the Tiny 2 Lite.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::CameraInfo;

/// Remo Tech Co., Ltd. — the USB-IF vendor entity OBSBOT ships under.
pub const VID_OBSBOT: u16 = 0x3564;

/// `(VID, PID)` pairs recognised as belonging to the OBSBOT Tiny 2 family.
///
/// A flat constant per [[ADR-0014]]: a future model is appended with a
/// single line, no code-path branching anywhere else in the codebase.
pub const TINY2_FAMILY: &[(u16, u16)] = &[
    (VID_OBSBOT, 0xfef8), // OBSBOT Tiny 2 (regular)
    (VID_OBSBOT, 0xfef9), // OBSBOT Tiny 2 Lite
];

/// Default sysfs class root the production code scans.
const SYSFS_VIDEO4LINUX: &str = "/sys/class/video4linux";

/// Discover Tiny 2 family cameras connected to this machine.
///
/// Scans `/sys/class/video4linux`. Returns the empty vector if the
/// directory is missing or unreadable — the caller (the GUI's
/// hot-plug listener, the CLI's `list` command) treats "no cameras"
/// the same regardless of cause, so we log the failure via `tracing`
/// rather than propagating it through the return type.
#[must_use]
pub fn enumerate_cameras() -> Vec<CameraInfo> {
    enumerate_cameras_in(Path::new(SYSFS_VIDEO4LINUX))
}

/// Test-friendly variant of [`enumerate_cameras`] that targets an
/// arbitrary sysfs-class-video4linux-shaped tree. Unit tests assemble a
/// `tempfile::TempDir` matching real sysfs (with the `device → ../usbN/
/// busN-port/busN-port:1.0` relative-symlink convention) and point this
/// at it.
#[must_use]
pub fn enumerate_cameras_in(sysfs_video_root: &Path) -> Vec<CameraInfo> {
    let entries = match fs::read_dir(sysfs_video_root) {
        Ok(e) => e,
        Err(err) => {
            tracing::warn!(
                target: "obsbot_core::enumerate",
                path = %sysfs_video_root.display(),
                error = %err,
                "cannot read sysfs video4linux directory",
            );
            return Vec::new();
        }
    };

    // Keyed on the canonicalised USB device path so the dedup is robust
    // across the two video nodes a Tiny 2 family camera advertises.
    let mut by_device: HashMap<PathBuf, CameraInfo> = HashMap::new();
    // Preserve the order in which devices first appeared so callers
    // get a deterministic ordering.
    let mut order: Vec<PathBuf> = Vec::new();

    for entry in entries.flatten() {
        let Some(info_key) = collect_one(&entry.path()) else {
            continue;
        };
        let (usb_device_dir, video_node, info) = info_key;
        if let Some(existing) = by_device.get_mut(&usb_device_dir) {
            // Prefer the lower-numbered /dev/videoN as the primary
            // capture path. On Tiny 2 Lite the metadata node lives
            // at video1; this picks video0.
            if existing.video_path.as_ref().is_none_or(|p| p > &video_node) {
                existing.video_path = Some(video_node);
            }
        } else {
            order.push(usb_device_dir.clone());
            by_device.insert(usb_device_dir, info);
        }
    }

    order
        .into_iter()
        .filter_map(|k| by_device.remove(&k))
        .collect()
}

/// Resolve one `/sys/class/video4linux/<name>` entry to its parent USB
/// device and read out the descriptor attributes. Returns `None` if the
/// entry does not point at a `TINY2_FAMILY` device.
fn collect_one(video_entry: &Path) -> Option<(PathBuf, PathBuf, CameraInfo)> {
    let video_name = video_entry.file_name()?.to_str()?.to_owned();

    // From the v4l device's "device" symlink (which points at the USB
    // *interface*, e.g. .../1-7:1.0), go up one level to reach the USB
    // *device* (.../1-7). `fs::canonicalize` resolves the relative
    // symlink chain natively.
    let usb_device_dir = fs::canonicalize(video_entry.join("device").join("..")).ok()?;

    let vid = parse_hex_u16(&read_attr(&usb_device_dir, "idVendor")?)?;
    let pid = parse_hex_u16(&read_attr(&usb_device_dir, "idProduct")?)?;
    if !TINY2_FAMILY.contains(&(vid, pid)) {
        return None;
    }

    let info = CameraInfo {
        vendor: read_attr(&usb_device_dir, "manufacturer").unwrap_or_default(),
        product: read_attr(&usb_device_dir, "product").unwrap_or_default(),
        vid,
        pid,
        // Tiny 2 Lite firmware 5.10 advertises iSerial = 0; the kernel
        // surfaces missing string descriptors as a missing attribute
        // file rather than an empty one. See PROTOCOL.md §5.
        serial: read_attr(&usb_device_dir, "serial"),
        firmware: read_attr(&usb_device_dir, "bcdDevice"),
        video_path: Some(PathBuf::from("/dev").join(&video_name)),
    };

    Some((usb_device_dir, PathBuf::from("/dev").join(video_name), info))
}

/// Read a single trimmed sysfs attribute, returning `None` if the file
/// is missing or unreadable.
fn read_attr(dir: &Path, name: &str) -> Option<String> {
    fs::read_to_string(dir.join(name))
        .ok()
        .map(|s| s.trim().to_owned())
}

/// Parse a 4-character hex string ("3564" → `0x3564`).
fn parse_hex_u16(s: &str) -> Option<u16> {
    u16::from_str_radix(s.trim(), 16).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs as unix_fs;

    /// Build a sysfs-shaped tempdir with a single USB device on a given
    /// (VID, PID), the given list of `/dev/video*`-named nodes, and the
    /// given USB attributes. Returns `(tempdir, root_path)`.
    ///
    /// Layout produced (mirrors real sysfs):
    ///   tempdir/sys/devices/usbN/portN/
    ///       idVendor, idProduct, manufacturer, product, bcdDevice
    ///       portN:1.0/
    ///           video4linux/
    ///               videoX/   (just an empty directory standing in for
    ///                          the kernel's video device node folder)
    ///   tempdir/sys/class/video4linux/
    ///       videoX -> ../../devices/usbN/portN/portN:1.0/video4linux/videoX
    fn build_mock_sysfs(
        port: &str,
        videos: &[&str],
        vid_hex: &str,
        pid_hex: &str,
        attrs: &[(&str, &str)],
    ) -> (tempfile::TempDir, PathBuf) {
        let td = tempfile::tempdir().expect("create tempdir");
        let root = td.path();

        let usb_device = root.join("sys").join("devices").join("usb1").join(port);
        let usb_interface = usb_device.join(format!("{port}:1.0"));
        let kernel_v4l = usb_interface.join("video4linux");
        fs::create_dir_all(&kernel_v4l).expect("mkdir kernel v4l dir");

        // USB-device attribute files.
        fs::write(usb_device.join("idVendor"), format!("{vid_hex}\n")).unwrap();
        fs::write(usb_device.join("idProduct"), format!("{pid_hex}\n")).unwrap();
        for (k, v) in attrs {
            fs::write(usb_device.join(k), format!("{v}\n")).unwrap();
        }

        // Per-video-node directory under the kernel side.
        for name in videos {
            fs::create_dir_all(kernel_v4l.join(name)).unwrap();
        }

        // /sys/class/video4linux/<name> symlinks, mirroring the real
        // relative-symlink convention.
        let class_dir = root.join("sys").join("class").join("video4linux");
        fs::create_dir_all(&class_dir).unwrap();
        for name in videos {
            let target = PathBuf::from("..")
                .join("..")
                .join("devices")
                .join("usb1")
                .join(port)
                .join(format!("{port}:1.0"))
                .join("video4linux")
                .join(name);
            unix_fs::symlink(target, class_dir.join(name)).unwrap();
        }

        // Each video node also needs a `device` symlink pointing back
        // at the USB interface (real sysfs uses `../../../<port>:1.0`,
        // see `readlink /sys/class/video4linux/video0/device`).
        // Production code does `canonicalize(device/..)` to obtain the
        // USB *device* dir from this interface link.
        for name in videos {
            let device_link = kernel_v4l.join(name).join("device");
            let target = PathBuf::from("..")
                .join("..")
                .join("..")
                .join(format!("{port}:1.0"));
            unix_fs::symlink(target, device_link).unwrap();
        }

        (td, class_dir)
    }

    #[test]
    fn detects_tiny2_lite_with_dual_video_nodes() {
        let (_td, class_dir) = build_mock_sysfs(
            "1-7",
            &["video0", "video1"],
            "3564",
            "fef9",
            &[
                ("manufacturer", "Remo Tech Co., Ltd."),
                ("product", "OBSBOT Tiny 2 Lite"),
                ("bcdDevice", "0510"),
            ],
        );

        let cams = enumerate_cameras_in(&class_dir);
        assert_eq!(cams.len(), 1, "two video nodes must dedup to one camera");

        let cam = &cams[0];
        assert_eq!(cam.vid, 0x3564);
        assert_eq!(cam.pid, 0xfef9);
        assert_eq!(cam.vendor, "Remo Tech Co., Ltd.");
        assert_eq!(cam.product, "OBSBOT Tiny 2 Lite");
        assert_eq!(cam.firmware.as_deref(), Some("0510"));
        // Lite firmware 5.10 has no `serial` attribute file.
        assert!(cam.serial.is_none());
        // video0 wins over video1 as the primary capture path.
        assert_eq!(cam.video_path.as_deref(), Some(Path::new("/dev/video0")));
    }

    #[test]
    fn detects_regular_tiny2() {
        let (_td, class_dir) = build_mock_sysfs(
            "1-3",
            &["video2"],
            "3564",
            "fef8",
            &[
                ("manufacturer", "Remo Tech Co., Ltd."),
                ("product", "OBSBOT Tiny 2"),
                ("bcdDevice", "0400"),
                ("serial", "ABCDEF123456"),
            ],
        );

        let cams = enumerate_cameras_in(&class_dir);
        assert_eq!(cams.len(), 1);

        let cam = &cams[0];
        assert_eq!(cam.vid, 0x3564);
        assert_eq!(cam.pid, 0xfef8);
        assert_eq!(cam.serial.as_deref(), Some("ABCDEF123456"));
        assert_eq!(cam.video_path.as_deref(), Some(Path::new("/dev/video2")));
    }

    #[test]
    fn rejects_non_obsbot_camera() {
        // A generic Logitech webcam (`046d:0892`, made-up but plausible)
        // must not show up in our list.
        let (_td, class_dir) = build_mock_sysfs(
            "1-4",
            &["video0"],
            "046d",
            "0892",
            &[
                ("manufacturer", "Logitech"),
                ("product", "HD Webcam"),
                ("bcdDevice", "0010"),
            ],
        );

        assert!(enumerate_cameras_in(&class_dir).is_empty());
    }

    #[test]
    fn missing_root_returns_empty() {
        let phantom = Path::new("/nonexistent/sysfs/class/video4linux");
        assert!(enumerate_cameras_in(phantom).is_empty());
    }

    #[test]
    fn parses_known_hex() {
        assert_eq!(parse_hex_u16("3564"), Some(0x3564));
        assert_eq!(parse_hex_u16("fef9\n"), Some(0xfef9));
        assert_eq!(parse_hex_u16("zzzz"), None);
    }
}
