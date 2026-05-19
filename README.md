# Obsbot Cam Control

A native GNOME application to control OBSBOT cameras on Linux. Built with
GTK 4, libadwaita, and Rust. Targets GNOME Circle inclusion.

- **App ID**: `io.github.domatix.ObsbotCamControl`
- **License**: [GPL-3.0-or-later](LICENSE) (SPDX: `GPL-3.0-or-later`)
- **Repo**: hosted under the [`Domatix`](https://github.com/Domatix)
  GitHub organization.

> ⚠️ **Status**: pre-alpha. Project scaffolding only. No functionality yet.

## Goals

- Full feature parity with OBSBOT Center (the official macOS/Windows app) for
  the OBSBOT **Tiny 2 family** (regular Tiny 2 and Tiny 2 Lite), starting
  with a free, reverse-engineered protocol stack.
- Strict adherence to the GNOME Human Interface Guidelines and GNOME Circle
  criteria.
- 100% free software. No proprietary SDKs, no closed blobs.
- Flatpak as primary distribution channel.

## Status

**v0.3.1 — Live preview + smooth PTZ** (released 2026-05-19).
Native build adds the v0.4 Live Preview pipeline behind a
`live-preview` Cargo feature flag (`gstreamer1.0-gtk4` system
package required) — a `v4l2src ! videoconvert !
gtk4paintablesink` chain renders the camera feed inside a sticky
revealer above the controls page, with a header-bar toggle and
an `AdwBanner` discoverability hint. The PTZ pad gains
press-and-hold smooth motion (≈ 20°/s, 1° per 50 ms tick after a
200 ms long-press threshold) plus keyboard arrow navigation
(Left / Right / Up / Down + Home recenter) that respects focused
sliders. Also rolls up the `T-105fix` schema/runtime alignment
so per-camera settings persistence works end-to-end. Flatpak
artifact still tracks v0.3.0 until the GStreamer plugin module
lands in the manifest.

**v0.3.0 — Vendor XU & AI tracking** (released 2026-05-15). The
GUI surfaces OBSBOT-specific controls via reverse-engineered USB
Extension Units: 10 AI auto-framing modes, HDR, Field of View
(Wide / Normal / Narrow), Tracking speed (Standard / Sport),
Sleep / Wake power state, 3 preset-recall slots, and a "Show XU
status" diagnostic dialog. See
[`docs/ROADMAP.md`](docs/ROADMAP.md) for what's already shipped
and what's queued for v0.4 (Live Preview milestone wrap-up:
snapshot, filters, Flatpak GStreamer module) and beyond.

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
    org.gnome.Platform//48 \
    org.gnome.Sdk//48 \
    org.freedesktop.Sdk.Extension.rust-stable//24.08
flatpak-builder --user --install --force-clean \
    build-flatpak build-aux/io.github.domatix.ObsbotCamControl.json
flatpak run io.github.domatix.ObsbotCamControl
```

The manifest at
[`build-aux/io.github.domatix.ObsbotCamControl.json`](build-aux/io.github.domatix.ObsbotCamControl.json)
targets the GNOME 48 runtime with the rust-stable SDK extension.
Flathub submission is a v1.0 goal — for v0.1 the manifest exists for
local-build verification and to seed T-015 CI (when the repository
goes public).

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
`obsbot-cam-control-0.1.0-1-x86_64.pkg.tar.zst` under
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
