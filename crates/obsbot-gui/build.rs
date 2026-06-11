// SPDX-License-Identifier: GPL-3.0-or-later
//
// Build script for the Obsbot Cam Control GUI crate.
//
// Four stages:
//
// 1. Compile the Blueprint templates under `resources/*.blp` into
//    `*.ui` files under OUT_DIR using the `blueprint-compiler`
//    binary (T-099).
//
// 2. Pack the produced `.ui` files into a single GResource bundle
//    at `OUT_DIR/obsbot.gresource` via `glib_build_tools::
//    compile_resources` (T-099). The binary registers it at startup
//    via `gio::resources_register_include!`.
//
// 3. Copy + compile the GSettings schema from `data/` into
//    `OUT_DIR/schemas/gschemas.compiled` so `cargo run` finds it
//    without needing `meson install` (T-105). The compiled-schema
//    directory is exposed to the binary as the
//    `OBSBOT_DEV_SCHEMA_DIR` rustc env var.
//
// 4. Re-export the `OBSBOT_LOCALEDIR` build-time env var (set by
//    `build-aux/cargo-build.sh` when meson drives the build) as a
//    `cargo:rustc-env` so `i18n::init()` can pick it up via
//    `option_env!` at compile time (T-107). Bare `cargo run` /
//    `cargo build` leaves it unset — `i18n::init()` then skips
//    `bindtextdomain` and English source strings flow through
//    unchanged.

use std::path::PathBuf;
use std::process::Command;

const TEMPLATES: &[&str] = &["window", "controls-view", "ptz-pad"];
/// Static (non-generated) resource files staged into `OUT_DIR` so the
/// `GResource` compiler — whose only source dir is `OUT_DIR` — can find
/// them next to the Blueprint-generated `.ui` files.
const STATIC_RESOURCES: &[&str] = &["style.css"];
const SCHEMA_FILENAME: &str = "io.github.domatix.ObsbotCamControl.gschema.xml";

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

    // Stage 1b — stage static (non-generated) resources into OUT_DIR
    // so `compile_resources` (whose only source dir is OUT_DIR) can
    // find them alongside the Blueprint-generated .ui files. The
    // custom stylesheet (T-212) is the only one for now.
    for name in STATIC_RESOURCES {
        let src = resources_dir.join(name);
        let dst = out_dir.join(name);
        println!("cargo:rerun-if-changed={}", src.display());
        std::fs::copy(&src, &dst).unwrap_or_else(|err| {
            panic!(
                "failed to stage resource {} → {}: {err}",
                src.display(),
                dst.display(),
            )
        });
    }

    // Stage 2 — .ui → GResource bundle.
    let gresource_xml = resources_dir.join("obsbot.gresource.xml");
    println!("cargo:rerun-if-changed={}", gresource_xml.display());

    glib_build_tools::compile_resources(
        &[&out_dir],
        gresource_xml.to_str().expect("non-UTF-8 manifest path"),
        "obsbot.gresource",
    );

    // Stage 3 — GSettings schema → compiled cache for cargo-run dev.
    // Source schema lives in <repo>/data/; manifest_dir is
    // <repo>/crates/obsbot-gui/ so we walk two parents up.
    let repo_root = manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .expect("manifest dir is at least two levels deep")
        .to_path_buf();
    let schema_src = repo_root.join("data").join(SCHEMA_FILENAME);
    let schema_dst_dir = out_dir.join("schemas");
    let schema_dst = schema_dst_dir.join(SCHEMA_FILENAME);

    println!("cargo:rerun-if-changed={}", schema_src.display());

    std::fs::create_dir_all(&schema_dst_dir).expect("create OUT_DIR/schemas");
    std::fs::copy(&schema_src, &schema_dst).unwrap_or_else(|err| {
        panic!(
            "failed to stage GSettings schema {} → {}: {err}",
            schema_src.display(),
            schema_dst.display(),
        )
    });

    let status = Command::new("glib-compile-schemas")
        .arg(&schema_dst_dir)
        .status()
        .unwrap_or_else(|err| {
            panic!(
                "glib-compile-schemas must be installed and on PATH \
                 (ships with glib-2.0; on Debian: apt install \
                 libglib2.0-bin): {err}"
            )
        });
    assert!(
        status.success(),
        "glib-compile-schemas failed for {} (exit: {status})",
        schema_dst_dir.display(),
    );

    println!(
        "cargo:rustc-env=OBSBOT_DEV_SCHEMA_DIR={}",
        schema_dst_dir.display(),
    );

    // Stage 4 — re-export OBSBOT_LOCALEDIR if meson set it.
    //
    // The meson wrapper `build-aux/cargo-build.sh` exports
    // `OBSBOT_LOCALEDIR=<install localedir>` before invoking cargo
    // build. We forward it via `cargo:rustc-env` so `option_env!` in
    // `src/i18n.rs` evaluates to `Some(...)` at compile time. Bare
    // `cargo build` / `cargo run` leaves the env var unset, the
    // option_env! returns `None`, and `i18n::init()` skips
    // `bindtextdomain` — source-language strings flow unchanged.
    println!("cargo:rerun-if-env-changed=OBSBOT_LOCALEDIR");
    if let Ok(localedir) = std::env::var("OBSBOT_LOCALEDIR") {
        println!("cargo:rustc-env=OBSBOT_LOCALEDIR={localedir}");
    }
}
