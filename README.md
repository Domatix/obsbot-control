# Obsbot Cam Control

[![CI](https://github.com/Domatix/obsbot-control/actions/workflows/ci.yml/badge.svg)](https://github.com/Domatix/obsbot-control/actions/workflows/ci.yml)

A native GNOME application to control OBSBOT cameras on Linux. Built with
GTK 4, libadwaita, and Rust. Targets GNOME Circle inclusion.

- **App ID**: `io.github.domatix.ObsbotCamControl`
- **License**: [GPL-3.0-or-later](LICENSE) (SPDX: `GPL-3.0-or-later`)
- **Repo**: hosted under the [`Domatix`](https://github.com/Domatix)
  GitHub organization.

> ⚠️ **Status**: alpha. Fully functional on both the native build and
> the Flatpak (V4L2 standard controls, vendor XU / AI tracking, live
> preview with snapshot, PTZ, per-camera persistence). Not yet
> submitted to Flathub.

## Goals

- Full feature parity with OBSBOT Center (the official macOS/Windows app) for
  the OBSBOT **Tiny 2 family** (regular Tiny 2 and Tiny 2 Lite), starting
  with a free, reverse-engineered protocol stack.
- Strict adherence to the GNOME Human Interface Guidelines and GNOME Circle
  criteria.
- 100% free software. No proprietary SDKs, no closed blobs.
- Flatpak as primary distribution channel.

## Status

**Current release: v0.4.2** (2026-06-18). What works today, validated
against the Tiny 2 Lite:

- **AI tracking** with ten framing modes (Normal, Upper body, Close-up,
  Headless, Lower body, Group, Desk mode, Whiteboard, Hand, or off),
  plus tracking speed (Standard / Sport) — via the reverse-engineered
  vendor XU protocol.
- **HDR**, **Field of View** (Wide / Normal / Narrow), face
  auto-exposure, and auto/manual exposure mode.
- **Live preview** inside the app (GStreamer `gtk4paintablesink`) with
  **snapshot to PNG** saved to your Pictures folder.
- **PTZ pad** with 8 directional buttons and a zoom slider (single-step
  moves; press-and-hold was removed after proving unreliable on
  hardware). Arrow keys + Home also drive the camera.
- **Image controls**: brightness, contrast, saturation, hue, sharpness,
  white balance, exposure time, anti-flicker — every V4L2 control the
  kernel advertises, with reset-to-default.
- **Preset recall** for the three firmware slots (programming slots is
  done via OBSBOT Center or on-device gestures).
- **Per-camera settings persistence** (GSettings), hot-plug handling
  with toasts, and automatic camera sleep when the app stops using it.
- Single-window UI in English; gettext scaffolding is in place and
  translations are welcome.

Full release notes for every version live in the [AppStream
metainfo](data/io.github.domatix.ObsbotCamControl.metainfo.xml.in);
the milestone roadmap in [`docs/ROADMAP.md`](docs/ROADMAP.md).

## Supported cameras

- **OBSBOT Tiny 2** (`3564:fef8`) — first-class target. Validation
  depends on community testing; the project does not own this unit.
- **OBSBOT Tiny 2 Lite** (`3564:fef9`) — first-class target and the
  hardware development happens against day to day.
- Other OBSBOT models (Meet, Meet 2, Meet SE, original Tiny, Tail
  Air, …) — best-effort: the app will list them and expose V4L2
  standard controls; vendor-specific features only as community
  captures unlock them. Original **Meet** is tracked explicitly as
  T-400 in `PLAN.md` for a future milestone.
- Non-OBSBOT cameras: ignored.

Rationale for the family scoping in [`docs/DECISIONS.md`](docs/DECISIONS.md)
(ADR-0014).

## Project documentation

The full project documentation lives in [`docs/`](docs/). Start with:

- [`docs/SPEC.md`](docs/SPEC.md) — what this project is.
- [`docs/ROADMAP.md`](docs/ROADMAP.md) — milestone overview.
- [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) — technical design.

## Working on this project

This project is developed with AI assistance (Claude Code). The conventions and
workflow are documented:

- [`CLAUDE.md`](CLAUDE.md) — operating instructions for the AI agent.
- [`docs/AI_WORKFLOW.md`](docs/AI_WORKFLOW.md) — how humans collaborate with the AI on this project.
- [`docs/SKILLS.md`](docs/SKILLS.md) — coding standards and methodology.

## Building

### Local meson build (Debian / Fedora / Arch)

```sh
meson setup builddir
meson compile -C builddir
sudo meson install -C builddir
```

Runtime dependencies: GTK 4 ≥ 4.14, libadwaita ≥ 1.6, glib/gio ≥
2.74, plus a Rust toolchain ≥ 1.83 to compile.

### Flatpak (local build)

```sh
sudo apt install flatpak flatpak-builder
flatpak remote-add --user --if-not-exists \
    flathub https://dl.flathub.org/repo/flathub.flatpakrepo
flatpak install --user flathub \
    org.gnome.Platform//50 \
    org.gnome.Sdk//50
flatpak-builder --user --install --force-clean \
    build-flatpak build-aux/io.github.domatix.ObsbotCamControl.json
flatpak run io.github.domatix.ObsbotCamControl
```

The manifest at
[`build-aux/io.github.domatix.ObsbotCamControl.json`](build-aux/io.github.domatix.ObsbotCamControl.json)
targets the GNOME 50 runtime and pulls the rust-stable and llvm20 SDK
extensions automatically on first build. It also compiles the
`gtk4paintablesink` GStreamer plugin and `blueprint-compiler`, which
the GNOME runtime does not ship. Flathub submission is a v0.6/v1.0
goal; CI builds the Flatpak on every push (T-015).

### Test packages (Debian `.deb`)

> **Scope** ([ADR-0015](docs/DECISIONS.md)): convenience artifact for
> testers on non-Flatpak hosts. Not a Debian-policy package. Flatpak
> via Flathub remains the supported channel.

One-time tool install:

```sh
cargo install cargo-deb --locked --version '^2.10'
```

Build the artifact:

```sh
./build-aux/build-deb.sh
```

The shim runs `meson setup` so the `.desktop` / AppStream files have
their `@APP_ID@` / `@VERSION@` placeholders substituted, then invokes
`cargo deb -p obsbot-gui`. The resulting package lands under
`build-aux/dist/` as `obsbot-cam-control_<version>_amd64.deb`.

Install / uninstall:

```sh
sudo apt install ./build-aux/dist/obsbot-cam-control_*_amd64.deb
obsbot-cam-control                              # launches the GUI
sudo apt remove obsbot-cam-control              # removes everything
```

### Test packages (Arch `pkg.tar.zst`)

> **Scope** ([ADR-0015](docs/DECISIONS.md)): same as the `.deb` —
> convenience artifact for the Arch tester, **not** an AUR-grade
> package. Build, install, remove; no commitment to track Arch
> packaging policy churn.

On an Arch (or Arch-derivative) host:

```sh
./build-aux/build-arch.sh
```

The shim runs `makepkg -f --skipchecksums` against
[`build-aux/PKGBUILD`](build-aux/PKGBUILD) and drops the resulting
`obsbot-cam-control-<version>-1-x86_64.pkg.tar.zst` under
`build-aux/dist/`.

Install / uninstall:

```sh
sudo pacman -U ./build-aux/dist/obsbot-cam-control-*-x86_64.pkg.tar.zst
obsbot-cam-control                              # launches the GUI
sudo pacman -R obsbot-cam-control               # removes everything
```

Build dependencies pulled from official repos:
`base-devel rust meson clang pkgconf gtk4 libadwaita`.

On a non-Arch host the shim exits with an error pointing at a
`docker run … archlinux:latest …` recipe — see the script for
the exact command.

## License

Copyright © 2026 Domatix and contributors.

This project is released under the **GNU General Public License v3.0 or
later** (`SPDX-License-Identifier: GPL-3.0-or-later`). See [`LICENSE`](LICENSE)
for the full text and [`docs/DECISIONS.md`](docs/DECISIONS.md) (ADR-0011)
for the rationale. The OBSBOT proprietary SDK is **not** used (ADR-0002).

## Acknowledgments

Inspired by [aaronsb/obsbot-camera-control](https://github.com/aaronsb/obsbot-camera-control),
a Qt6 application for the same purpose. This project is a from-scratch
reimplementation in Rust/GTK with the explicit goal of avoiding the proprietary
OBSBOT SDK.

This is an unofficial third-party application, not affiliated with or endorsed
by OBSBOT.
