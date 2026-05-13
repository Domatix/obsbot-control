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

//! `obsbot-cli` — companion command-line interface for Obsbot Cam Control.
//!
//! T-006 lands the `--version`-aware scaffold; T-012 adds the `list`
//! subcommand that wraps [`obsbot_core::enumerate_cameras`].

use std::fmt::Write as _;

use clap::{Parser, Subcommand};
use obsbot_core::{enumerate_cameras, CameraInfo};

#[derive(Debug, Parser)]
#[command(
    name = "obsbot-cli",
    version,
    about = "Companion command-line interface for Obsbot Cam Control",
    long_about = None,
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// List OBSBOT cameras connected to this machine.
    #[command(long_about = LIST_LONG_ABOUT)]
    List,
}

const LIST_LONG_ABOUT: &str = "\
Print the OBSBOT cameras connected to this machine.

Discovery walks /sys/class/video4linux and filters by USB VID/PID against
the Tiny 2 family (3564:fef8 regular, 3564:fef9 Lite).

Output: one indexed stanza per camera with these fields:
  - Product:  USB iProduct string descriptor.
  - Vendor:   USB iManufacturer string descriptor.
  - USB ID:   <VID>:<PID> in lower-case hex.
  - Serial:   USB iSerial, or `(not advertised)` if absent
              (firmware 5.10 on Tiny 2 Lite reports iSerial=0).
  - Firmware: raw bcdDevice hex (e.g. `0510` for firmware 5.10).
  - Video:    Primary V4L2 capture device path (e.g. `/dev/video0`).

With no cameras connected, prints `No OBSBOT cameras detected.` to
stdout. Exit code is 0 in every case.";

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::List) => {
            print!("{}", render(&enumerate_cameras()));
        }
        None => {
            println!("obsbot-cli v{}", env!("CARGO_PKG_VERSION"));
        }
    }
}

/// Render the `list` subcommand's output. Pure function so unit tests
/// can pin the format without going through stdout.
fn render(cameras: &[CameraInfo]) -> String {
    if cameras.is_empty() {
        return String::from("No OBSBOT cameras detected.\n");
    }

    let mut out = String::new();
    let header = if cameras.len() == 1 {
        String::from("1 camera detected:\n\n")
    } else {
        format!("{} cameras detected:\n\n", cameras.len())
    };
    out.push_str(&header);

    for (idx, cam) in cameras.iter().enumerate() {
        let _ = writeln!(out, "[{}] {}", idx + 1, cam.product);
        let _ = writeln!(out, "    Vendor:   {}", cam.vendor);
        let _ = writeln!(out, "    USB ID:   {:04x}:{:04x}", cam.vid, cam.pid);
        let _ = writeln!(
            out,
            "    Serial:   {}",
            cam.serial.as_deref().unwrap_or("(not advertised)"),
        );
        let _ = writeln!(
            out,
            "    Firmware: {}",
            cam.firmware.as_deref().unwrap_or("(unknown)"),
        );
        let video = cam
            .video_path
            .as_ref()
            .map_or_else(|| String::from("(none)"), |p| p.display().to_string());
        let _ = writeln!(out, "    Video:    {video}");
        if idx + 1 < cameras.len() {
            out.push('\n');
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn tiny2_lite() -> CameraInfo {
        CameraInfo {
            vendor: "Remo Tech Co., Ltd.".into(),
            product: "OBSBOT Tiny 2 Lite".into(),
            vid: 0x3564,
            pid: 0xfef9,
            serial: None,
            firmware: Some("0510".into()),
            video_path: Some(PathBuf::from("/dev/video0")),
        }
    }

    fn tiny2_regular() -> CameraInfo {
        CameraInfo {
            vendor: "Remo Tech Co., Ltd.".into(),
            product: "OBSBOT Tiny 2".into(),
            vid: 0x3564,
            pid: 0xfef8,
            serial: Some("ABCDEF123456".into()),
            firmware: Some("0400".into()),
            video_path: Some(PathBuf::from("/dev/video2")),
        }
    }

    #[test]
    fn render_zero_cameras() {
        assert_eq!(render(&[]), "No OBSBOT cameras detected.\n");
    }

    #[test]
    fn render_one_camera_missing_serial() {
        let out = render(&[tiny2_lite()]);
        assert!(out.starts_with("1 camera detected:\n\n"));
        assert!(out.contains("[1] OBSBOT Tiny 2 Lite\n"));
        assert!(out.contains("    USB ID:   3564:fef9\n"));
        assert!(out.contains("    Serial:   (not advertised)\n"));
        assert!(out.contains("    Firmware: 0510\n"));
        assert!(out.contains("    Video:    /dev/video0\n"));
    }

    #[test]
    fn render_two_cameras_indexed_and_pluralised() {
        let out = render(&[tiny2_lite(), tiny2_regular()]);
        assert!(out.starts_with("2 cameras detected:\n\n"));
        assert!(out.contains("[1] OBSBOT Tiny 2 Lite\n"));
        assert!(out.contains("[2] OBSBOT Tiny 2\n"));
        // Second camera advertises a serial — fallback must not apply.
        assert!(out.contains("    Serial:   ABCDEF123456\n"));
        // Stanzas separated by a blank line.
        assert!(out.contains("\n\n[2] "));
    }
}
