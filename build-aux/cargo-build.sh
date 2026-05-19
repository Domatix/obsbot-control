#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-or-later
#
# Wrapper invoked by `meson.build` to drive `cargo build`. Splits out
# the few branches we need (debug vs release, package, binary name)
# so meson.build stays declarative.
#
# Usage:
#   cargo-build.sh <manifest> <target-dir> <profile> <package> <bin-name> <out-file> <localedir> [feature]
#
# Arguments:
#   manifest    Absolute path to the workspace Cargo.toml.
#   target-dir  Absolute path Meson reserved for cargo's --target-dir.
#   profile     'debug' or 'release'.
#   package     Cargo package to build (e.g. obsbot-gui).
#   bin-name    The binary cargo produces inside target-dir/<profile>/.
#   out-file    Where Meson expects the binary to land (@OUTPUT@).
#   localedir   Install-time localedir (T-107). Exported as
#               OBSBOT_LOCALEDIR for build.rs to re-emit via
#               cargo:rustc-env so i18n.rs::init() can bind the
#               textdomain at runtime.
#   feature     Optional cargo feature to enable (T-203). Empty
#               string = no extra feature (default). Currently the
#               only value the meson.build wires is 'live-preview'
#               when -Dlive-preview=true.

set -euo pipefail

if [ "$#" -lt 7 ] || [ "$#" -gt 8 ]; then
    echo "usage: $0 <manifest> <target-dir> <profile> <package> <bin-name> <out-file> <localedir> [feature]" >&2
    exit 64
fi

manifest="$1"
target_dir="$2"
profile="$3"
package="$4"
bin_name="$5"
out_file="$6"
localedir="$7"
feature="${8:-}"

export OBSBOT_LOCALEDIR="$localedir"

args=(build --manifest-path="$manifest" --target-dir="$target_dir" -p "$package")
if [ -n "$feature" ]; then
    args+=(--features "$feature")
fi
case "$profile" in
    release)
        args+=(--release)
        subdir="release"
        ;;
    debug)
        subdir="debug"
        ;;
    *)
        echo "cargo-build.sh: unknown profile '$profile' (want debug|release)" >&2
        exit 64
        ;;
esac

cargo "${args[@]}"

src="$target_dir/$subdir/$bin_name"
if [ ! -f "$src" ]; then
    echo "cargo-build.sh: expected binary not found at $src" >&2
    exit 65
fi

install -m 755 "$src" "$out_file"
