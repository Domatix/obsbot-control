#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-or-later
#
# Produces the convenience `.pkg.tar.zst` artifact described in
# [[docs/DECISIONS.md ADR-0015]] / [[docs/PLAN.md T-017]]. NOT an
# AUR-grade package; Flatpak via Flathub remains the supported
# distribution channel.
#
# Must run on an Arch (or Arch-derivative) host. Mirrors the shape
# of `build-deb.sh`:
#   1. cd build-aux/ (where PKGBUILD lives)
#   2. makepkg -f (build-aux/PKGBUILD has source=(), so there is
#      nothing to fetch or verify: it builds the tree in place)
#   3. mv the resulting *.pkg.tar.zst into build-aux/dist/
#
# Usage: `./build-aux/build-arch.sh` (no arguments).

set -euo pipefail

here=$(cd "$(dirname "$0")" && pwd)
dist="$here/dist"

if [ -r /etc/os-release ]; then
    # shellcheck disable=SC1091
    . /etc/os-release
fi
is_arch="no"
case "${ID:-}:${ID_LIKE:-}" in
    arch:*|*:*arch*|cachyos:*|manjaro:*|endeavouros:*|*arch*)
        is_arch="yes"
        ;;
esac

if [ "$is_arch" != "yes" ]; then
    cat >&2 <<EOF
build-arch.sh: this script must run on an Arch (or Arch-derivative)
host. The current OS is reported as '${ID:-unknown}' (ID_LIKE='${ID_LIKE:-}').

Per [[ADR-0015]], the .pkg.tar.zst is a convenience test artifact for
the Arch stakeholder; the supported distribution channels on other
hosts are the Flatpak (build-aux/io.github.domatix.obsbot-control.json)
or the .deb (build-aux/build-deb.sh).

To build on a non-Arch host you would need a containerised Arch
environment, e.g.:

    docker run --rm -it -v "\$PWD":/repo -w /repo archlinux:latest \\
        bash -c 'pacman -Sy --noconfirm base-devel rust meson clang pkgconf \\
                                       gtk4 libadwaita git && \\
                 useradd -m -G wheel builder && \\
                 echo "builder ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers && \\
                 chown -R builder /repo && \\
                 su builder -c "./build-aux/build-arch.sh"'

The container path is intentionally not invoked from this script —
keep the wrapper simple, let the user opt in explicitly.
EOF
    exit 64
fi

if ! command -v makepkg >/dev/null 2>&1; then
    echo "build-arch.sh: makepkg not found; install pacman's 'base-devel' group." >&2
    exit 64
fi

mkdir -p "$dist"

cd "$here"
makepkg --force --noconfirm

mv ./*.pkg.tar.zst "$dist/"

echo
echo "build-arch.sh: artifact ready under $dist/"
ls -1 "$dist"/*.pkg.tar.zst
