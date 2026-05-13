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
//! T-006 lands this as a minimal `--version`-aware scaffold. Subcommands
//! (`list`, etc.) arrive from T-012 onwards.

use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    name = "obsbot-cli",
    version,
    about = "Companion command-line interface for Obsbot Cam Control",
    long_about = None,
)]
struct Cli {}

fn main() {
    let _ = Cli::parse();
    println!("obsbot-cli v{}", env!("CARGO_PKG_VERSION"));
}
