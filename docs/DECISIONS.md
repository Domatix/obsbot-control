# DECISIONS — Architecture Decision Records

> **Purpose**: Append-only record of significant decisions and plan changes.
> Each entry: date, context, decision, consequence. Never edit past entries;
> if a decision is reversed, add a new entry that supersedes.

---

## ADR-0001 — Target GNOME Circle from day one

**Date**: 2026-05-12
**Status**: accepted
**Context**: The user is building an OBSBOT camera control app and wants
visibility within the GNOME ecosystem. Aiming at GNOME Circle imposes hard
constraints from the start (libre license, no CLA, no proprietary
dependencies, Flatpak, HIG compliance) that are cheaper to honor from the
beginning than to retrofit.
**Decision**: Treat GNOME Circle criteria as a hard requirement throughout
the project, not a "nice to have at the end".
**Consequence**: Excludes the OBSBOT proprietary SDK entirely, including as
an optional feature. Mandates reverse-engineering of vendor protocol.
Mandates Flatpak packaging from v0.1.

---

## ADR-0002 — No dependency on OBSBOT's proprietary SDK (`libdev.so`)

**Date**: 2026-05-12
**Status**: accepted
**Context**: The reference project (`aaronsb/obsbot-camera-control`) and
several others link against `libdev.so`, OBSBOT's closed-source binary
distributed without a license. GNOME Circle requires OSI-approved licensing
for the whole project. A "hybrid build" (free by default, SDK optional)
adds complexity, splits the user base, and would still raise questions
during Circle review.
**Decision**: The project will not link against, depend on, or optionally
support `libdev.so`. All vendor features will be implemented via UVC
Extension Units, raw USB requests, and V4L2 controls.
**Consequence**: Features depending on SDK-only logic (typically auto-framing
edge cases, AI scene modes) may be deferred or never implemented if reverse
engineering is too costly. v0.5 (auto-framing) is explicitly marked as
"risky".

---

## ADR-0003 — Rust + GTK 4 + libadwaita as the technology stack

**Date**: 2026-05-12
**Status**: accepted
**Context**: The reference project is Qt6/C++. Going native GNOME requires
GTK 4 + libadwaita. Among supported languages (C, Python, Vala, Rust), Rust
offers memory safety, modern tooling, and the strongest current momentum in
the GNOME world (Amberol, Pika Backup, Fractal, Shortwave, etc.).
**Decision**: Rust (MSRV 1.83+), `gtk4-rs`, `libadwaita-rs`, `gstreamer-rs`.
Meson as top-level build system to match GNOME conventions.
**Consequence**: Higher learning curve for human contributors compared to
Python. Compile times higher. In exchange: safer concurrency, faster
runtime, no GIL.

---

## ADR-0004 — Async via `glib::MainContext`, not Tokio

**Date**: 2026-05-12
**Status**: accepted
**Context**: GTK runs on GLib's main loop. Two main-loop runtimes in the
same process is a recipe for deadlocks and missed events. Tokio is the
standard Rust async runtime but is fundamentally incompatible with GLib's
main loop without complex bridges.
**Decision**: Use `glib::MainContext::spawn_local`, futures from `glib` and
`gio`, and `async-channel` for cross-thread communication. No Tokio
dependency anywhere.
**Consequence**: Some Rust crates (notably network ones) that depend on
Tokio cannot be used directly; need `smol`/`async-std` alternatives or
manual thread wrapping. Acceptable for this project since we have no
network requirement.

---

## ADR-0005 — Background operation via XDG Portal, no system-tray icon

**Date**: 2026-05-12
**Status**: accepted
**Context**: The reference Qt project uses a system-tray icon for
minimize-to-tray. GNOME deprecated tray icons years ago; modern GNOME
exposes background-running apps through the XDG Background Portal and a
panel menu. Emulating tray icons (e.g. via AppIndicator) is anti-HIG and
would fail Circle review.
**Decision**: Implement background operation via
`org.freedesktop.portal.Background`. The app appears in GNOME Shell's
"Background Apps" menu when minimized.
**Consequence**: On non-GNOME desktops (KDE, XFCE) the background experience
will be less integrated. Acceptable for a Circle target.

---

## ADR-0006 — Persistent project memory in `docs/`, not chat context

**Date**: 2026-05-12
**Status**: accepted
**Context**: Development is delegated to Claude Code. Sessions are
ephemeral; context is finite and expensive. Knowledge that must survive
across sessions cannot live only in chat history.
**Decision**: All durable project knowledge lives in committed files in
`docs/`. Claude Code reads a small set of these (`STATE.md` + summaries)
at session start. Everything else is on-demand. See `CLAUDE.md` §0.
**Consequence**: Discipline cost: every meaningful action must update the
documentation. Benefit: any session can resume from any previous state
without context replay.

---

## ADR-0007 — Conventional Commits and task IDs in messages

**Date**: 2026-05-12
**Status**: accepted
**Context**: A long-running AI-assisted project produces many commits.
Without convention, the log becomes opaque and tooling (changelogs,
release notes, traceability) breaks.
**Decision**: All commits follow Conventional Commits. Each commit
references the task ID it advances (e.g. `(T-014)`). Pre-commit hooks
enforce `cargo fmt`, `cargo clippy -D warnings`, `cargo test`. See
`CLAUDE.md` §2.
**Consequence**: Friction in commit creation. Bigger payoff: searchable
history, automatic changelog generation, clear traceability of tasks to
code changes.

---

## ADR-0008 — Documents and commits are independent disciplines

**Date**: 2026-05-12
**Status**: accepted
**Context**: An early version of the workflow conflated "update progress"
with "commit", causing two failure modes: (a) docs only update at session
end, losing state on interruption; (b) commits batch many small changes,
making history coarse.
**Decision**: `PROGRESS.md` and `STATE.md` are updated continuously, at
every sub-step. Commits happen autonomously when atomic work is complete
and tests pass. Neither waits for the other.
**Consequence**: More frequent disk writes and commits. Net win: any
interruption leaves a recoverable state, and history is fine-grained.

---

## ADR-0009 — App namespace and license — DEFERRED

**Date**: 2026-05-12
**Status**: deferred
**Context**: The app's reverse-DNS namespace depends on where the repo is
hosted (`io.github.<username>...` for GitHub, `org.gnome.<name>` only for
gnome.org-hosted projects). The license choice (GPL-3.0 vs MIT vs LGPL)
has implications for whether other projects can incorporate the code.
**Decision**: Deferred until T-002. Both decisions will be made there with
explicit user input.
**Consequence**: All current files use `io.github.<ns>` as placeholder.
Search-and-replace at T-002.

---

## ADR-0010 — Scaffolding completeness validated at T-001

**Date**: 2026-05-12
**Status**: accepted
**Context**: T-001's acceptance criteria require that every file listed in
`ARCHITECTURE.md` §2 either exists in the repo or has a justified absence
recorded here. The repo at this point contains only documentation, the
top-level `CLAUDE.md`, `README.md`, `.gitignore`, and four empty directories
(`crates/`, `data/`, `po/`, `build-aux/`) preserved with `.gitkeep`. The
remaining files in §2 are not yet present.
**Decision**: Accept the current scaffolding as complete for T-001. Every
absent file from §2 maps to an explicit later task in `PLAN.md`:

| Absent path (per ARCHITECTURE §2)                                   | Will be created by |
|---------------------------------------------------------------------|--------------------|
| `Cargo.toml` (workspace root)                                       | T-004              |
| `meson.build`                                                       | T-008              |
| `LICENSE`                                                           | T-002              |
| `crates/obsbot-core/**`                                             | T-005              |
| `crates/obsbot-cli/**`                                              | T-006              |
| `crates/obsbot-gui/**` (incl. `build.rs`, widgets)                  | T-007              |
| `data/ui/` (Blueprint sources)                                      | T-007 / T-013      |
| `data/icons/scalable/apps/`, `data/icons/symbolic/apps/`            | T-010              |
| `data/<app-id>.desktop.in`, `data/<app-id>.metainfo.xml.in`         | T-009              |
| `data/<app-id>.gschema.xml`                                         | T-105 (v0.2)       |
| `data/resources.gresource.xml`                                      | T-007              |
| `po/POTFILES`, `po/LINGUAS`, `po/es.po`                             | T-009 onwards      |
| `build-aux/<app-id>.json` (Flatpak manifest)                        | T-014              |

The four `.gitkeep` files (in `crates/`, `data/`, `po/`, `build-aux/`) are
intentional: they preserve the §2 directory skeleton in the initial commit
and disappear naturally as those directories acquire real content in later
tasks. They do not need their own ADR.
**Consequence**: The initial commit faithfully represents the project's §2
layout-in-skeleton. Any future divergence from this table must be reflected
either by completing the referenced task or by adding a superseding ADR.

---

<!-- Append new ADRs above this line, never below. Newest ADRs go at the bottom
     of the list but new entries are added; do not edit old ones. -->
