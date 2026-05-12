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
  the OBSBOT Tiny 2, starting with a free, reverse-engineered protocol stack.
- Strict adherence to the GNOME Human Interface Guidelines and GNOME Circle
  criteria.
- 100% free software. No proprietary SDKs, no closed blobs.
- Flatpak as primary distribution channel.

## Supported cameras

- **OBSBOT Tiny 2** — primary development target.
- Other UVC-compliant cameras: basic V4L2 controls may work, untested.

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

> Not yet implemented. Build instructions will be added when the first
> milestone (v0.1) is reached.

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
