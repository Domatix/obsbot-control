// SPDX-License-Identifier: GPL-3.0-or-later
//
// Build script for the Obsbot Cam Control GUI crate (T-099).
//
// Two stages:
//
// 1. Compile the Blueprint templates under `resources/*.blp` into
//    `*.ui` files under OUT_DIR using the `blueprint-compiler`
//    binary (Debian: apt install blueprint-compiler; Arch: pacman
//    -S blueprint-compiler; Flatpak: build-aux/io.github.domatix.
//    ObsbotCamControl.json's modules list adds it before building
//    the crate).
//
// 2. Pack the produced `.ui` files into a single GResource bundle
//    at `OUT_DIR/obsbot.gresource` via `glib_build_tools::
//    compile_resources`. The binary registers it at startup via
//    `gio::resources_register_include!`.

use std::path::PathBuf;
use std::process::Command;

const TEMPLATES: &[&str] = &["window", "controls-view"];

fn main() {
    let manifest_dir =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not set"));
    let resources_dir = manifest_dir.join("resources");

    // Stage 1 — Blueprint → .ui.
    for name in TEMPLATES {
        let input = resources_dir.join(format!("{name}.blp"));
        let output = out_dir.join(format!("{name}.ui"));
        println!("cargo:rerun-if-changed={}", input.display());

        let status = Command::new("blueprint-compiler")
            .arg("compile")
            .arg("--output")
            .arg(&output)
            .arg(&input)
            .status()
            .unwrap_or_else(|err| {
                panic!(
                    "blueprint-compiler must be installed and on PATH \
                     (Debian: apt install blueprint-compiler; \
                     Arch: pacman -S blueprint-compiler): {err}"
                )
            });

        assert!(
            status.success(),
            "blueprint-compiler failed for {} (exit: {status})",
            input.display(),
        );
    }

    // Stage 2 — .ui → GResource bundle.
    let gresource_xml = resources_dir.join("obsbot.gresource.xml");
    println!("cargo:rerun-if-changed={}", gresource_xml.display());

    glib_build_tools::compile_resources(
        &[&out_dir],
        gresource_xml.to_str().expect("non-UTF-8 manifest path"),
        "obsbot.gresource",
    );
}
