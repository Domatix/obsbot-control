#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-or-later
#
# Produces the convenience `.deb` artifact described in [[docs/DECISIONS.md
# ADR-0015]] / [[docs/PLAN.md T-016]]. This is NOT a Debian-policy-grade
# package; Flatpak via Flathub remains the supported distribution channel.
#
# Sequence:
#   1. `meson setup builddir` if it does not exist (otherwise `--reconfigure`)
#      so `configure_file()` produces builddir/data/*.desktop and
#      *.metainfo.xml with @APP_ID@ / @VERSION@ substituted. cargo-deb's
#      asset list in `crates/obsbot-gui/Cargo.toml` points at those
#      substituted files.
#   2. `cargo deb -p obsbot-gui --output build-aux/dist/` builds the
#      release binary and assembles the .deb.
#
# Usage: `./build-aux/build-deb.sh` (no arguments).

set -euo pipefail

here=$(cd "$(dirname "$0")" && pwd)
repo=$(cd "$here/.." && pwd)
dist="$here/dist"
builddir="$repo/builddir"

if ! cargo deb --version >/dev/null 2>&1; then
    echo "build-deb.sh: cargo-deb is not installed. Run \`cargo install cargo-deb --locked --version '^2.10'\` first." >&2
    exit 64
fi

if ! command -v meson >/dev/null 2>&1; then
    echo "build-deb.sh: meson is not installed. Install meson and rerun." >&2
    exit 64
fi

mkdir -p "$dist"

if [ ! -d "$builddir" ]; then
    meson setup "$builddir" "$repo"
else
    meson setup --reconfigure "$builddir" "$repo" >/dev/null
fi

for f in \
    "$builddir/data/io.github.domatix.ObsbotCamControl.desktop" \
    "$builddir/data/io.github.domatix.ObsbotCamControl.metainfo.xml"
do
    if [ ! -f "$f" ]; then
        echo "build-deb.sh: meson did not produce $f. Re-run \`meson setup --reconfigure $builddir\`." >&2
        exit 65
    fi
done

cd "$repo"
cargo deb -p obsbot-gui --output "$dist/"

echo
echo "build-deb.sh: artifact ready under $dist/"
ls -1 "$dist"/*.deb
