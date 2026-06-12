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

## ADR-0011 — License: GPL-3.0-or-later

**Date**: 2026-05-12
**Status**: accepted (supersedes the license half of [[ADR-0009]])
**Context**: T-002 required choosing an OSI-approved license compatible with
the GNOME Circle criteria ([[ADR-0001]]), with the no-proprietary-derivatives
spirit of [[ADR-0002]], and with redistribution by Debian main, Fedora, and
Arch. Four alternatives were weighed: GPL-3.0-or-later, GPL-2.0-or-later,
LGPL-3.0-or-later, and permissive (MIT/Apache-2.0). All four are OSI/FSF
approved and redistributable in the target distros; the differentiating
factors were copyleft strength, GNOME Circle precedent, and patent /
anti-Tivoization clauses.
**Decision**: Release the entire project under **GPL-3.0-or-later** (SPDX:
`GPL-3.0-or-later`). The full license text is committed at the repo root
as `LICENSE` (verbatim text from gnu.org). New Rust source files must carry
an SPDX header (`// SPDX-License-Identifier: GPL-3.0-or-later`) plus the
short GPL boilerplate; new documentation files keep the project-level
licensing implicit (no per-file header needed for prose).
**Consequence**:
- AppStream metainfo will declare `project_license` as `GPL-3.0-or-later`.
- Any third-party Rust crate added to the workspace must be license-compatible
  with GPLv3+ (Apache-2.0, MIT, BSD, MPL-2.0, LGPLv3+ — all OK; GPLv2-only,
  CDDL, ISC-with-advertising — case-by-case). License compatibility is
  checked at crate-add time, not retroactively.
- Forks that distribute modified versions must release source under the same
  license; this is the desired outcome per [[ADR-0002]].
- A `CONTRIBUTING.md` (later task, v0.6 area) will state explicitly that
  contributions are accepted under the same license and that no CLA is
  required (Circle criterion).

---

## ADR-0012 — App identity, hosting, and copyright

**Date**: 2026-05-12
**Status**: accepted (supersedes the namespace half of [[ADR-0009]])
**Context**: T-002 required choosing the project's public identity:
reverse-DNS App ID, hosting URL, and binary/display naming. Constraints:
the App ID prefix must follow Flathub conventions for the chosen host;
the trademark "OBSBOT" (Remo Tech Co., Ltd.) is used descriptively under
nominative-fair-use but should be qualified to avoid confusion with the
manufacturer's own products; Cargo crate names were already fixed in
[[ARCHITECTURE.md §2]] (`obsbot-core`, `obsbot-cli`, `obsbot-gui`) and are
out of scope for this ADR.
**Decision**:
- **Hosting**: GitHub organization `Domatix` (https://github.com/Domatix).
  The repo URL will be `https://github.com/Domatix/obsbot-control` (folder
  name preserved as the umbrella project name; the public-facing app name
  differs intentionally — see below).
- **Reverse-DNS prefix**: `io.github.domatix` (Flathub rule: the
  user/organization segment is lowercased; "Domatix" → "domatix"; no hyphens
  to escape).
- **App ID** (Flatpak, AppStream, D-Bus path, GSettings schema base,
  `.desktop` filename): **`io.github.domatix.ObsbotCamControl`**.
- **Display name** (GNOME Shell, header bar, About dialog): **"Obsbot Cam
  Control"** (HIG title-case; the "Cam" qualifier helps distinguish from
  OBSBOT's own products and from other OBSBOT-related tools).
- **GUI binary name**: deferred to T-007 implementation; default
  `obsbot-cam-control` (kebab-case of the App ID's last segment), settable
  via `[[bin]] name = ...` in `crates/obsbot-gui/Cargo.toml`. The crate name
  itself stays `obsbot-gui`.
- **CLI binary name**: `obsbot-cli` (short, idiomatic for a CLI companion;
  unchanged from the crate name).
- **Copyright holder line** (used in source-file headers, About dialog,
  AppStream metainfo): `Copyright © 2026 Domatix and contributors`. This
  attributes the project to the Domatix organization while leaving room for
  external contributors. Individual file `Authors:` lines, where applicable,
  record specific authorship.
- **Trademark disclaimer**: `README.md` and AppStream metainfo carry the
  disclaimer "Unofficial third-party application, not affiliated with or
  endorsed by OBSBOT/Remo Tech Co., Ltd." (already partially present in
  `README.md`; refined at T-009 metainfo writing).
- **Local folder, repo name, crate names**: unchanged (`obsbot-control`,
  `obsbot-core`, `obsbot-cli`, `obsbot-gui`). The umbrella project name
  ("obsbot-control") and the user-facing app name ("Obsbot Cam Control")
  are intentionally distinct: the former is a dev/repository handle, the
  latter is the product identity.
**Consequence**:
- Every `<app-id>` placeholder in the docs is now resolved to
  `io.github.domatix.ObsbotCamControl`; every `io.github.<ns>` to
  `io.github.domatix`. T-002's project-wide replacement applies.
- GSettings schemas live under `io.github.domatix.ObsbotCamControl`
  (`...preferences`, `...state`, etc.).
- Filenames generated at T-009/T-010/T-014:
  `data/io.github.domatix.ObsbotCamControl.desktop.in`,
  `data/io.github.domatix.ObsbotCamControl.metainfo.xml.in`,
  `data/io.github.domatix.ObsbotCamControl.gschema.xml`,
  `data/icons/scalable/apps/io.github.domatix.ObsbotCamControl.svg`,
  `build-aux/io.github.domatix.ObsbotCamControl.json`.
- If Flathub objects to the use of "Obsbot" in the App ID during submission,
  renaming is a single ADR-superseding event and a few hours of
  search-and-replace; the repo URL and crate names absorb no churn because
  they don't use the trademark.

---

## ADR-0013 — T-004 validation: `cargo metadata` instead of `cargo check --workspace`

**Date**: 2026-05-13
**Status**: accepted (amends T-004 acceptance criteria in [[PLAN.md]])
**Context**: T-004 was specified with acceptance criteria
"`cargo check --workspace` passes (even if crates are empty)" and
"`cargo fmt --check` passes". After installing the toolchain (rustc
1.85.0 from Debian trixie) and writing the root `Cargo.toml`, both
commands fail with `manifest is virtual, and the workspace has no
members` because `members = ["crates/*"]` expands to the empty set —
the `crates/` directory only contains a `.gitkeep`. The original
criteria implicitly assumed at least one stub member existed, which is
not the case until T-005. Cargo's behavior here is by design: a
virtual workspace with zero members is a hard error for `cargo check`
and `cargo fmt`, not a warning.

Three alternatives were considered:
1. Merge T-004 and T-005 into a single commit so a member exists at
   validation time. Rejected: breaks task atomicity and reduces commit
   granularity ([[ADR-0008]] favors fine-grained commits).
2. Add a throwaway placeholder member crate to be deleted in T-005.
   Rejected: clutters history, easy to forget to delete.
3. Adapt T-004's validation to "manifest parses correctly" (via
   `cargo metadata --no-deps`) and move the `cargo check --workspace`
   + `cargo fmt --check` enforcement to T-005, which naturally
   introduces the first member crate.

**Decision**: Adopt option 3.
- T-004 acceptance criteria become:
  - `cargo metadata --no-deps --format-version 1` succeeds (manifest
    parses, workspace.dependencies resolve).
  - `[workspace.package]` and `[workspace.dependencies]` blocks
    follow [[ADR-0003]] (MSRV 1.83, edition 2021) and [[ARCHITECTURE
    §1]] (pinned versions of gtk4, libadwaita, gstreamer family, v4l,
    nusb, etc.).
  - Commit: `build: create cargo workspace (T-004)`.
- T-005 acceptance criteria gain (additive, not replacing existing):
  - `cargo check --workspace` passes after `obsbot-core` is added.
  - `cargo fmt --all --check` passes after `obsbot-core` is added.
  - `cargo clippy -p obsbot-core -- -D warnings` passes (already in
    T-005).

**Consequence**: T-004 closes cleanly today with toolchain validation
limited to manifest parsing. T-005 picks up the full lint/format gate
when there is real source code to lint, which is its natural home
anyway. No code change to `Cargo.toml` is needed; the manifest as
drafted already parses. `PLAN.md` is amended in-place to reflect the
new criteria, and the original criteria's intent is preserved (just
relocated by one task).

---

## ADR-0014 — Primary target: OBSBOT Tiny 2 family (regular + Lite)

**Date**: 2026-05-13
**Status**: accepted (amends [[SPEC.md §3, §5, §7]] and surrounding
documentation)
**Context**: SPEC.md was authored assuming the user's reference hardware was
the OBSBOT Tiny 2 (regular). During T-003's first hardware capture
(`lsusb` against the user's plugged-in camera), the actual device on
this machine was identified as **OBSBOT Tiny 2 Lite** (`3564:fef9`,
"Remo Tech Co., Ltd. OBSBOT Tiny 2 Lite", iProduct string, bcdDevice
5.10). The regular Tiny 2 ships as `3564:fef8` (cross-checked against
the linuxtv-commits 2025-12 kernel patch already cited in
[[PROTOCOL.md §6]]). The two siblings share:
- Vendor ID `0x3564` (Remo Tech Co., Ltd., OBSBOT's parent entity).
- USB-IF Video class (UVC 1.0 + UAC1 audio).
- Almost identical I/O profile (camera-sensor terminal with the same
  bmControls mask, identical processing-unit control set, one vendor
  extension unit on the same bUnitID = 2 — to be cross-checked against
  the regular Tiny 2's lsusb output as community captures appear).
The Lite is positioned by OBSBOT as a feature-reduced sibling of the
regular Tiny 2 (no LCD, weaker AI/NPU, lower max bitrate), but the
USB protocol layer is overwhelmingly common. Treating them as one
"family" target avoids artificial scope split, matches the available
development hardware, and lets the codebase serve both audiences from
day one. Three alternatives were weighed (recorded in PROGRESS for
2026-05-13 ~10:30Z): (A) family target chosen here; (B) flip primary
to Lite, downgrade regular to best-effort; (C) keep SPEC pointing at
regular Tiny 2 and treat the Lite as a development proxy. (B) burdens
the project narrative with an under-marketed model; (C) hides the
truth that development happens against the Lite, complicating bug
triage.

**Decision**: Expand the project's primary support target to the
**OBSBOT Tiny 2 family**, comprising:
- **OBSBOT Tiny 2** — VID `0x3564`, PID `0xfef8`. Primary, but not
  physically available to the project; relies on community testing.
- **OBSBOT Tiny 2 Lite** — VID `0x3564`, PID `0xfef9`. Primary AND
  the actual development hardware on the user's machine.

Both PIDs are first-class targets for USB enumeration ([[PLAN T-011]]),
metainfo, and roadmap milestones. Features that diverge between Lite
and regular (e.g., the LCD that the Lite lacks) are documented per
control as they're discovered; the GUI presents only those a given
unit advertises.

Other OBSBOT cameras (Meet 2, Meet SE, original Tiny, Tail Air,
etc.) remain best-effort: code must not actively reject them, but
features beyond the V4L2 standard set are unsupported until a
community report establishes selector compatibility.

**Consequence**:
- SPEC.md §3 (target users), §4 (in scope), §5 (out of scope), §7
  (constraints), §10 (references) updated to name the family.
- ROADMAP.md prose adjusted where it singled out the regular Tiny 2.
- README.md "Supported cameras" lists both PIDs and clarifies the Lite
  is the dev hardware.
- PROTOCOL.md §1 (Hardware identifiers) rewritten with both PIDs and
  observed descriptor data from the Lite; speculative Tiny 2 numbers
  remain marked as such until a regular-Tiny-2 capture lands.
- AppStream metainfo (T-009) and `.desktop` keywords (T-009) will
  enumerate both model names so software-center search hits both.
- USB enumeration (T-011) filters on the set `{0x3564:0xfef8,
  0x3564:0xfef9}`; the filter is a constant in `obsbot-core` so a
  future model can be appended without code-path branching.
- No churn to crate names, App ID, repo name, or GUI binary name —
  all three are family-neutral.

---

## ADR-0015 — Test artifacts: ship .deb and Arch packages alongside Flatpak

**Date**: 2026-05-13
**Status**: accepted (amends [[SPEC.md §4.5]] and [[ROADMAP.md v0.1]])
**Context**: SPEC.md §4.5 originally said "Flatpak as primary distribution
(target: Flathub). Deb/RPM packaging is a non-goal for v1.0 (community
can package)." A project stakeholder asked on 2026-05-13 that, once
there is a runnable build of the camera-control app, the toolchain
also emit a `.deb` (for the user's Debian trixie machine) and an
Arch package (`pkg.tar.zst` from a `PKGBUILD`) so the stakeholder on
Arch can sideload-test the same revision. The intent is **internal
test distribution**, not Debian-policy / AUR-grade upstreaming —
artifacts to ease iteration with non-Flatpak testers, not commitments
to long-lived package maintenance.

Three alternatives were weighed:
1. Ignore the request: contradicts a direct stakeholder need; forces
   the Arch tester to figure out their own packaging or sideload via
   Flatpak. Rejected.
2. Full upstream packaging (debian/ tree submitted to Debian, PKGBUILD
   submitted to AUR): large scope, contradicts the original §4.5 stance,
   and is the user-community responsibility per existing decision.
   Rejected for this iteration.
3. **Add a "test artifact" tier**: CI / release tooling produces a
   non-policy `.deb` (via `cargo-deb` reading metadata from
   `Cargo.toml`) and a non-policy Arch package (via `makepkg` against
   an in-tree `PKGBUILD`). Distributed as GitHub Release attachments
   or downloadable CI artifacts. No commitment to track upstream
   policy churn, no `apt`/`pacman` repository hosting.

**Decision**: Adopt option 3. Concretely:
- `cargo-deb` becomes a dev-dependency (or used via `cargo install
  cargo-deb` in CI). Metadata for the deb lives under
  `[package.metadata.deb]` in `crates/obsbot-gui/Cargo.toml` once
  T-007 lands, listing runtime deps on the libgtk-4-1, libadwaita-1-0,
  libgstreamer1.0-0, libgstreamer-plugins-base1.0-0, and a few
  v4l/uvc system libraries (final list pinned when T-007 actually
  builds against them).
- `build-aux/PKGBUILD` (Arch) lives in-tree, hand-maintained, depends
  on `gtk4 libadwaita gstreamer gst-plugins-base gst-plugins-good
  v4l-utils`. Source pulled from a git tag.
- Both artifacts produced by GitHub Actions at tag time (and on demand
  via `workflow_dispatch`). README documents the install / uninstall
  command for each.
- Scope explicitly **not** a Debian-policy compliant package and
  **not** an AUR submission: the README "Installation" section calls
  them "test packages", links to the Flathub install as the supported
  channel, and points downstream packagers to the in-tree
  `cargo-deb`/`PKGBUILD` files as starting points if they want to
  pursue policy compliance themselves.

**Consequence**:
- SPEC.md §4.5 amended to add the test-artifact tier alongside the
  Flatpak primary; the "community can package" sentence kept for
  policy-grade Debian/AUR submissions.
- ROADMAP.md v0.1 "Includes" gains ".deb test package" and "Arch
  test package" lines; the milestone's definition-of-done (CLAUDE.md
  §7) gains "test packages build successfully" criteria once T-016
  / T-017 below land.
- PLAN.md gains two new tasks at the end of v0.1, sequenced after
  T-013 (so there's a real GUI to install) and T-014 (Flatpak first,
  since Flathub is still primary): **T-016 — `.deb` test package via
  `cargo-deb`** and **T-017 — Arch `PKGBUILD` test package**.
- CI (T-015) gains two extra jobs to produce these on tag.
- No impact on T-005 (just landed), T-006 (about to start), T-007,
  or any task before T-013 — the change is additive at the v0.1 tail.
- If the .deb dependency list proves brittle across Debian releases,
  or the PKGBUILD breaks against gtk4-rs upgrades, the test-artifact
  status (rather than a distribution-grade promise) gives us licence
  to skip a broken target temporarily and call it out in the release
  notes.

---

## ADR-0016 — Split T-013 into T-013a/b/c; defer Blueprint to T-013d

**Date**: 2026-05-13
**Status**: accepted (amends [[PLAN.md T-013]])
**Context**: [[PLAN T-013]] as originally written bundles four
sub-deliverables behind one task ID: (i) a list of detected cameras in
the GUI, (ii) a hot-plug listener so unplugging removes the row, (iii)
read-only V4L2 control sub-page per camera, (iv) "Use Blueprint for
the UI definition". The closed atoms so far (T-005…T-012) have all
been ~5–15 minute units with one obvious commit. T-013 in its
original shape is a different scale (a backend hot-plug listener, a
V4L2 enumeration UI, a Blueprint pipeline) and rolling them all into
one `IN_PROGRESS` task would (a) violate the "atomic functional
change" granularity that has worked well so far ([[CLAUDE.md §2.1]]),
(b) leave `STATE.md` in an unusable shape if interrupted mid-task,
and (c) couple decisions that benefit from being weighed
independently (e.g. udev vs polling for hot-plug is unrelated to the
V4L2 enumeration approach).

Three alternatives weighed:
1. Leave T-013 as one atom. Rejected — multiple commits inside one
   PLAN task is the discipline-breaker [[CLAUDE.md §2.1]] flags.
2. Rewrite T-013 in place as a checklist. Rejected — would still
   keep one IN_PROGRESS state covering 4+ days of work, and the
   acceptance criteria already point at three separable mechanisms.
3. **Split via the suffixed atom pattern**: keep T-013 as a parent
   "split into…" marker, introduce T-013a (initial scan list),
   T-013b (hot-plug listener), T-013c (V4L2 control sub-page),
   T-013d (Blueprint pipeline). Each atom carries its own
   acceptance criteria, commit, and risk profile.

**Decision**: Adopt option 3. The split:
- **T-013a — Initial camera-list view in GUI**: depends on T-007 +
  T-011. Replace the T-007 placeholder `StatusPage` with an
  `AdwPreferencesPage` containing an `AdwPreferencesGroup` of
  `AdwActionRow`s, one per camera returned by
  `obsbot_core::enumerate_cameras()` at app startup. Empty-state
  remains an `AdwStatusPage` with a "plug in a Tiny 2 family unit"
  hint. Hand-coded GTK is acceptable per [[CLAUDE.md §5.3]]'s
  "unless dynamic" carve-out: the entire row list is dynamic, and
  the surrounding page shell is too small to justify standing up
  the Blueprint pipeline. Hot-plug remains out of scope.
- **T-013b — Hot-plug listener**: depends on T-013a. Plug in a
  camera while the app is running → the new row appears without
  user intervention; unplug → the row disappears. Mechanism choice
  (`udev` crate vs polling `enumerate_cameras` on a `glib::timeout`
  vs gio `FileMonitor` on `/sys/class/video4linux`) deferred to the
  task itself; first-pass instinct is polling (simplest, no extra
  dependency), revisit if it shows up in profiling.
- **T-013c — V4L2 control sub-page (read-only)**: depends on
  T-013a (the row to drill into) + a new `obsbot-core` enumeration
  for V4L2 controls reading from the device path. Tapping a camera
  row opens an `AdwNavigationPage` with the 24 controls from
  [[PROTOCOL §2]] grouped by category, each shown with its current
  value and advertised range. Read-only — write paths are
  T-100-series work.
- **T-013d — Blueprint pipeline**: depends on T-013c (the first
  static-shape UI that actually benefits from `.blp` markup is the
  V4L2 form, with a known set of named child widgets). Introduces
  `blueprint-compiler` as a build dependency, a meson `custom_
  target` to compile `.blp` → `.ui`, and a `gnome.compile_
  resources` call to bundle the `.ui` into a GResource that the
  binary loads via `gio::Resource::register_from_data` /
  `include_bytes!`. The T-013a/b code that hand-coded
  `AdwPreferencesPage` then migrates to a Blueprint template in a
  separate commit (no functional change), preserving the rule that
  every commit on `main` compiles and tests pass.

**Consequence**:
- PLAN.md T-013 is **superseded** by T-013a/b/c/d. The original
  T-013 stub remains as a marker pointing at the split (the
  acceptance criteria of the four sub-tasks together still satisfy
  the original three criteria of T-013).
- v0.1 milestone definition-of-done unchanged: the four sub-tasks
  together produce the same diagnostics view T-013 promised, so
  the milestone closes when T-013a/b/c/d are all DONE.
- T-014 (Flatpak) and T-016 / T-017 (test packages) originally
  depended on "T-013"; they now formally depend on T-013a (the
  point at which the GUI shows a real camera, regardless of
  whether hot-plug or V4L2 controls are wired). The PLAN.md
  cross-references will be updated as those tasks become active.
- The `video` group membership pending action [[STATE.md
  pending_user_actions]] graduates from "v0.1 prerequisite" to a
  prerequisite of T-013c specifically. T-013a / T-013b touch
  sysfs only and do not need group membership; the user is
  already in the `video` group on this machine as of the T-013
  start probe, so the action is effectively closed for T-013c
  too.
- A future model added to [[ADR-0014]]'s `TINY2_FAMILY` automatically
  flows through to T-013a/b's rendering with no GUI-side change
  (the row factory keys off `CameraInfo` fields, not on the model).

---

## ADR-0017 — Defer T-013d (Blueprint pipeline) to v0.2

**Date**: 2026-05-13
**Status**: accepted (amends [[ADR-0016]] §T-013d, amends [[PLAN.md T-013d]] and the v0.1 milestone DOD)
**Context**: [[ADR-0016]] introduced T-013d on the premise that "T-013c
would have many named children" once the V4L2 detail page landed,
making the Blueprint pipeline pay for itself. In practice T-013c's
detail page renders from a dynamic `Vec<ControlDescriptor>` (22 entries
on the user's Tiny 2 Lite, all built from the same row factory): zero
named children, zero static widget trees beyond the
`AdwToolbarView + AdwHeaderBar + AdwPreferencesPage` shell — five
builder calls in total. The "Blueprint pays for itself" condition
the ADR-0016 reasoning depended on did not materialise.

Setting up the Blueprint pipeline as a v0.1 task would mean:
* A new build dependency on `blueprint-compiler` (user-side
  `sudo apt install blueprint-compiler`, plus an addition to
  `Build-Depends` for T-016's `.deb` and `makedepends` for
  T-017's PKGBUILD).
* A new cargo build-dep on `glib-build-tools 0.20` and a
  `crates/obsbot-gui/build.rs` shim that runs `blueprint-compiler
  compile` and `glib_build_tools::compile_resources`.
* One or two `.blp` files migrating ~5 builder-chain lines from
  the existing hand-coded GTK in `window.rs` / `controls_view.rs`
  to `gtk::Builder::from_resource` + named-child lookups.
* Roughly 150 lines of new code across build.rs, .blp templates,
  GResource manifest, and the Rust-side Builder consumption.

The benefit is consistency with [[CLAUDE.md §5.3]] ("UI defined in
Blueprint compiled to .ui") for the small static shell — but §5.3
also carves out "unless dynamic", which is the dominant shape of
every UI the v0.1 milestone surfaces. [[CLAUDE.md Doing tasks]] is
explicit on the matching call: "Don't add features, refactor, or
introduce abstractions beyond what the task requires. […] Don't
design for hypothetical future requirements. Three similar lines is
better than a premature abstraction." Blueprint infrastructure with
one consumer is premature; it pays for itself the moment a
brightness/contrast slider form (T-100) or a PTZ pad (T-101)
introduces a static widget tree with ≥3 named children.

Three alternatives weighed:
1. Land T-013d now with the minimum-viable migration (one `.blp`
   for the window shell). Rejected — overhead exceeds benefit for
   v0.1, and the migration burns ~150 lines of glue we'll need to
   touch again the moment v0.2 introduces a real static form.
2. Land T-013d now with the full migration (every static widget
   tree to `.blp`). Rejected — even more glue, and most of the
   current static shells are 1-3 lines that don't benefit.
3. **Defer T-013d to v0.2** as the first task before any T-100+
   work that introduces static widget trees. Pipeline lands
   exactly when the first paying customer materialises.

**Decision**: Adopt option 3.

**Consequence**:
- PLAN.md T-013d state moves from `TODO` to `DEFERRED`, with a
  pointer at this ADR. The acceptance criteria are kept verbatim;
  they will be the criteria for whatever task absorbs T-013d in
  the v0.2 series.
- ROADMAP.md v0.1 definition-of-done amended: drop the T-013d
  requirement (the four sub-task split from ADR-0016 effectively
  becomes a three-sub-task split: T-013a/b/c). v0.1 closes when
  T-013a/b/c + T-014/T-015/T-016/T-017 are all DONE.
- [[CLAUDE.md §5.3]] "UI defined in Blueprint" still holds as a
  forward-looking rule. The v0.2 series MUST land the pipeline
  before the first T-100+ task that introduces a static widget
  tree; this is added to the v0.2 hints in PLAN.md so it is the
  first task at the top of the v0.2 backlog.
- T-013c's hand-coded `window.rs` shell (`AdwApplicationWindow →
  AdwNavigationView → AdwNavigationPage("cameras") → AdwToolbarView
  → AdwHeaderBar + AdwBin(body_slot)`) and `controls_view.rs`
  shell (`AdwToolbarView → AdwHeaderBar + AdwPreferencesPage`)
  remain in code for the duration of v0.1. Both are tagged as
  "Blueprint candidates" in the inline doc-comments so the v0.2
  pipeline task knows which trees to migrate first.
- No commit is reverted; the rule from [[CLAUDE.md §2.1]]
  ("never commit work-in-progress that breaks main") stays
  satisfied — T-013a/b/c on `main` continue to compile, test, and
  run as user-confirmed.

---

## ADR-0018 — Tag v0.1.0 with T-015 (CI) BLOCKED; defer to v0.1.1 / v0.2

**Date**: 2026-05-13
**Status**: accepted (amends [[CLAUDE.md §7]] and [[ADR-0017]])

**Context**: With T-017 closing in this session, every technical
task originally scoped under the v0.1 milestone is `DONE`
(possibly DONE-with-caveat) **except T-015**:

* **DONE**: T-001..T-012, T-013a, T-013b, T-013c, T-014, T-016
* **DONE-with-caveat**: T-010 (icon visual confirmation deferred
  to a real distro install + a fresh GNOME session), T-017
  (downstream `makepkg` + `pacman -U/-R` deferred to the Arch
  stakeholder per [[ADR-0015]])
* **SUPERSEDED**: T-013 (by [[ADR-0016]])
* **DEFERRED to v0.2**: T-013d (by [[ADR-0017]])
* **BLOCKED**: T-015 — the GitHub Actions CI workflows. The
  PLAN.md note on T-015 said "do not run until repo is on
  GitHub. Mark BLOCKED if still private when reached." Today's
  remote push to `github.com/Domatix/obsbot-control` (PRIVATE)
  partially unblocks the workflow side (Actions runs on private
  repos) but the README-status-badge and Flathub-prep parts of
  the original T-015 acceptance text both want the repo public.

[[CLAUDE.md §7]] requires "All its tasks in `PLAN.md` are DONE"
for a milestone to be cut. [[ADR-0017]] re-affirmed this by
listing T-015 explicitly as a v0.1 gate. The strict reading
therefore says: do not tag v0.1.0 yet.

Three alternatives weighed:

1. **Wait for T-015 before tagging v0.1.0.** Honours
   CLAUDE.md §7 strictly. Costs: indefinite delay (the public-
   repo move is a separate user decision with no current
   timeline), and the natural "everything technical is done"
   moment passes without a versioned snapshot — which makes
   future bisection / "what was v0.1" reconstruction harder.

2. **Tag v0.1.0 now and treat T-015 as a v0.1.1 patch.** The
   v0.1.0 tag captures the technical feature-completeness
   (enumeration, diagnostics, Flatpak, .deb, Arch package). The
   subsequent v0.1.1 (or v0.2 absorption) adds CI when the
   public repo enables it. Costs: one extra patch tag, and a
   small departure from §7's literal reading.

3. **Move T-015 out of the v0.1 milestone entirely.** Reclassify
   it as "infrastructure work, not version-gated" and rewrite
   ADR-0017's v0.1 DoD list. Costs: muddles the milestone story
   (the very-first-CI is usually a v0.1 deliverable) and
   requires touching multiple ADRs to keep them coherent.

**Decision**: Adopt option **2**.

Concretely:

* Tag `v0.1.0` on `main` at the SHA that lands T-017's commit
  plus this ADR's docs commit. The tag's annotated message
  records the feature-completeness framing and explicitly notes
  T-015 is deferred.
* PLAN.md T-015's `State` stays `TODO` with the `BLOCKED`-on-
  public-repo annotation; the task is **NOT** moved out of the
  v0.1 section. When it lands it ships as v0.1.1, or it gets
  re-tagged as the first v0.2 deliverable — whichever sequence
  the public-repo split (the user mentioned in the 2026-05-13
  remote-online PROGRESS entry) implies.
* CLAUDE.md §7's checklist is updated to add a "(or: explicitly
  deferred via ADR)" clause on the "All tasks DONE" criterion —
  this ADR is the canonical example.

**Consequence**:

* `v0.1.0` exists immediately as a snapshot of "Tiny 2 family
  enumeration + V4L2 diagnostics shipped via three distribution
  channels" — the value the milestone was always about. Future
  `git log v0.1.0..main` queries become useful.
* The README badge from T-015 will land in v0.1.1 when public.
  Flathub-prep work also lands then; the manifest itself is
  already shipped in v0.1.0.
* Patch tags `v0.1.1+` carry the post-tag fixes the Arch
  stakeholder or the next dev-session may produce (e.g., a real
  `makepkg` run surfacing a PKGBUILD bug from T-017's deferred
  acceptance). This keeps the test-artifact tier honest without
  re-cutting `v0.1.0`.
* No deletion of work; this is purely a scoping decision about
  when to attach the version label. The codebase shape stays
  exactly as committed.
* If the Arch stakeholder, on their first `makepkg` run, finds
  a real PKGBUILD issue we missed in static validation, the fix
  lands in v0.1.1 alongside whatever CI work T-015 brings —
  symmetric with the existing T-010 caveat structure.

---

## ADR-0019 — Re-scope T-102 from "Zoom slider" to "Menu writes + INACTIVE grey-out"

**Status**: Accepted, 2026-05-13.

**Context**:

The v0.2 PLAN backlog hints listed:

- T-101 PTZ pad widget in GUI
- T-102 Zoom slider
- T-103 White balance widget
- T-104 Exposure widget
- T-105 Per-camera GSettings persistence

After landing T-100 and reading PROTOCOL §2.2 in full, two issues
with the original T-102 framing emerge:

1. **Zoom is already inside the PTZ pad scope.** ROADMAP v0.2 lists
   "PTZ pad with absolute and continuous pan/tilt/zoom" as a single
   bullet — the natural place for the zoom slider is inside the pad
   widget, next to the directional buttons. A standalone T-102
   "Zoom slider" task would either duplicate work or be a no-op.
2. **Menus are an unhandled control kind.** The Tiny 2 family
   exposes `power_line_frequency` (User-class menu, anti-flicker)
   and `auto_exposure` (Camera-class menu, T-104 needs it). T-100's
   write path only handles Integer / Boolean. Without a generic
   menu-write path, T-103 and T-104 both have to ship their own
   ad-hoc menu handling — exactly the duplication ADR-0008
   warns against.

Additionally, T-100 left an explicit UX debt: V4L2 `INACTIVE` flag
(driver-side interlock; e.g. WB Temperature inactive while WB Auto
is on) is not surfaced — the user sees a slider that silently
no-ops. This debt is best repaid alongside the menu work because
the AdwComboRow / ComboBox bindings need the same `is_active`
boolean propagation as the existing scale/spin rows.

**Decision**:

T-102 is re-scoped to "Menu writes + INACTIVE grey-out":

* Extend `ControlKind::Menu` to carry the option IDs (current shape
  is label-only; writes need the underlying integer index).
* Add `ControlValue::Menu(i64)` to write any menu by its integer
  index; `write_control` accepts it via the same path as
  Integer / Boolean.
* GUI uses `AdwComboRow` for User-class menus (anti-flicker
  naturally appears); Camera-class menus are handled explicitly
  in T-104 (auto_exposure).
* `ControlDescriptor` gains an `is_active: bool` field derived
  from `Description.flags::INACTIVE`. The GUI calls
  `widget.set_sensitive(is_active)` on the resulting row so
  inactive controls grey out automatically.

T-101 is augmented to explicitly include the zoom slider inside
the PTZ pad widget. PLAN.md is updated accordingly. ROADMAP v0.2's
bullet list does not change — the milestone still ships
"PTZ + WB + Exposure + GSettings + Anti-flicker" (anti-flicker is
now a free side-effect of T-102's menu infra). Task IDs are
re-mapped; user-visible deliverables are unchanged.

**Consequence**:

* Anti-flicker selector ships in T-102 alongside the menu infra,
  earlier than where the original hint placed it.
* The INACTIVE grey-out lands in v0.2 rather than v0.6 polish.
* T-103 / T-104 become smaller because the heavy lifting moves
  upstream into T-102.
* `ControlDescriptor` grows an `is_active` field; downstream
  pattern-matches will need adjustments. The only such consumer
  is `controls_view.rs`, which this milestone owns.
* PROTOCOL §2.3's quirk Q2 (`zoom_continuous` overrange) is
  now formally inside T-101 scope: drop `zoom_continuous` from
  the surfaced controls until a stakeholder asks for it.

---

## ADR-0020 — Pivot to AI tracking via FOSS lineage; collapse v0.4 + v0.5; swap with v0.3

**Status**: Accepted, 2026-05-14.

**Context**:

Up to this session, the ROADMAP staged the milestones as:

- v0.3 — Live Preview (T-200 seeded)
- v0.4 — Vendor XU (HDR, FOV, Face AE, LED, mic, gesture, voice;
  **prerequisite**: USB capture of OBSBOT Center on a Windows VM
  per [[PROTOCOL.md §3.1]])
- v0.5 — Auto-Framing & AI Features (face zones, auto-framing
  modes; marked "risky" because reverse-engineering effort was
  unbounded)

After the user shipped the v0.2.0 controls and explicitly asked
to pivot to *"the camera tracking, the best possible, without
needing to resort to a Windows VM"*, a re-investigation of the
FOSS ecosystem surfaced two repositories that PROTOCOL.md §6 did
not previously cite:

- **cgevans/tiny2** — Rust, EUPL-1.2, 51 GitHub stars, last push
  2026-03-29. `src/lib.rs` carries the complete Tiny 2 XU surface
  for HDR, Face AE, FOV (3 widths), and AI Tracking Mode
  (10 modes) on `bUnitID = 0x02`, `bSelector = 0x06`, plus
  Manual / Auto exposure 18-byte frames on the same unit's
  `bSelector = 0x02`, plus the GET_CUR 60-byte status decoder.
  Wraps `UVCIOC_CTRL_QUERY` directly via `nix`. No libusb.
- **OpenFoxes/Tiny4Linux** — Rust, EUPL-1.2, AUR-packaged active
  fork (2026-05-12). Adds Sleep / Wake, Tracking Speed
  (Standard / Sport), three Preset position recalls; modular
  factoring of the 36-byte `command02` frame.

Both projects are EUPL-1.2; the EUPL Appendix lists GPL-3 as a
**compatible licence** for derivative works, with the only
attribution requirement being the literal string
`"Licensed under the EUPL"` plus the SPDX line on ported files
(EUPL Article 5).

The full byte-level extraction is recorded in
[[docs/XU_INVESTIGATION_2026-05-14.md]]. Headlines:

- AI tracking — `[0x16, 0x02, m, n]`, 10 modes byte-identical
  between the two repos.
- HDR / Face AE / FOV — `[op, 0x01, value]`, three opcodes.
- Manual / Auto exposure — two 18-byte fixed frames.
- Sleep / Wake, Tracking Speed, Preset recall — three 36-byte
  frames (Tiny4Linux-only).
- GET_CUR status — 60 bytes, 5 decoded by the two repos
  (sleep, hdr, AI mode `m`, AI mode `n`, tracking speed); the
  remaining 55 are the discovery frontier.

**This invalidates** the Windows-VM + Wireshark prerequisite the
ROADMAP attached to v0.4 and the speculative framing of v0.5.
Everything the user asked for ("the best AI tracking") is
already reverse-engineered with byte-level fidelity in
EUPL-1.2 sources we can port. The only Wireshark+VM work still
on the table is *future* probing of the unmapped status bytes
and the unmapped selector-0x06 opcodes; it is no longer
blocking the milestone.

**Decision**:

Five linked changes:

1. **Swap v0.3 ↔ v0.4 priority.** Live Preview moves to v0.4;
   Vendor XU + AI tracking moves to v0.3. Rationale: the user
   explicitly requested AI tracking before preview, and the XU
   surface is the differentiator versus a generic UVC tool
   (anyone can ship a Preview pane; nobody else ships the
   Tiny 2 AI modes outside the proprietary SDK).

2. **Collapse old-v0.4 (Vendor XU) and old-v0.5 (Auto-Framing &
   AI Features) into a single new v0.3 milestone titled
   "Vendor XU & AI tracking"**. Justification: the XU surface,
   auto-framing modes, and AI tracking are decoded by the same
   `bUnitID=0x02 / bSelector=0x06 / op=0x16` path; separating
   them by milestone introduced redundant work without
   benefit. The "risky" framing of v0.5 is retired — the work
   is bounded by the cgevans + Tiny4Linux extraction, not by
   unbounded reverse engineering.

3. **Adopt EUPL-1.2 → GPL-3 attribution model.** Create a new
   `CREDITS.md` at repo root documenting the lineage
   (Tiny4Linux → cgevans → meet4k). Files in
   `crates/obsbot-core/src/xu/**` that contain ported bytes
   carry a dual SPDX block:

   ```rust
   // SPDX-License-Identifier: GPL-3.0-or-later
   //
   // Portions of this file are derived from EUPL-1.2 source:
   //   - cgevans/tiny2        (https://github.com/cgevans/tiny2)
   //   - OpenFoxes/Tiny4Linux (https://github.com/OpenFoxes/Tiny4Linux)
   // "Licensed under the EUPL"
   ```

   Files that are wholly original keep the plain GPL-3.0-or-later
   SPDX line they already use.

4. **Drop "Windows + Wireshark" as a milestone-level
   prerequisite.** PROTOCOL.md §1 (Status) and §3.1
   (per-selector decode) are amended to reflect that the bulk
   of the XU surface is already known from FOSS sources. The
   remaining `wireshark + Windows VM` capture is reframed as
   an *optional follow-on* for probing the unmapped status
   bytes and the unmapped selector-0x06 opcodes (`0x02`,
   `0x05`, `0x06`-`0x15`, `0x17`+), gated by user availability.

5. **Refresh PLAN.md.** T-200 (preview pane) moves from v0.3 to
   v0.4 (renumbering of the milestone bucket only; the task ID
   stays). New tasks T-300 / T-301 / T-302 / T-303 land under
   the new v0.3 with byte-level acceptance criteria pulled
   from the investigation report.

**Consequence**:

- **Scope**: SPEC.md §4.1 features that the new v0.3 covers —
  PTZ (already shipped in T-101 via V4L2 CIDs), HDR, FOV
  (Wide/Normal/Narrow), Face AE, Auto-framing modes (10 AI
  modes), Manual / Auto exposure, plus Tiny4Linux extras
  (Sleep/Wake, Tracking Speed, Preset recall). Features the
  new v0.3 does NOT yet cover — LED brightness, mic pickup
  pattern, Gesture control, Voice command toggle. These stay
  in SPEC scope but are deferred to a follow-on milestone
  pending either community discovery or a user-driven USB
  capture session against the proprietary app.
- **Reliability**: every byte we ship in v0.3 has been read
  out of EUPL-1.2 source code. Two known discrepancies
  flagged for hardware validation before tag-cut:
  (a) `AIMode::Hand` setter writes `m=3` while the status
  decoder reads `m=6` (likely typo in upstream cgevans);
  (b) `AUTO_EXP_CMD` vs `MANUAL_EXP_CMD` labels are swapped
  between cgevans and Tiny4Linux (cgevans's labelling is
  almost certainly the correct one based on the two-step
  Auto+FaceAE protocol). Both will be observed against the
  user's Tiny 2 Lite in T-303.
- **Licence hygiene**: the project remains OSI-clean. EUPL-1.2
  → GPL-3 is a documented compatibility (EUPL Appendix); no
  CLA, no proprietary blob, no telemetry. GNOME Circle
  eligibility per [[ADR-0001]] is preserved.
- **Roadmap timing**: v0.3 (XU + AI tracking) becomes the
  next shippable milestone, ~1-2 days of T-300 + 0.5-1 day
  of T-301 + 1 day of T-302 + validation in T-303. Live
  Preview (now v0.4) and Polish (unchanged v0.6) follow.
- **Task IDs**: T-200 moves to the v0.4 bucket; T-300/301/302/303
  are the new v0.3 tasks. No task ID changes.
- **Risk that does not disappear**: the unmapped 55 status
  bytes and the unmapped selector-0x06 opcodes remain
  discovery surface. We will ship a debug "Dump status" page
  in v0.3 (T-302) so the user can capture full hex dumps for
  future contributions without leaving the GUI.

This ADR supersedes the milestone framing in [[ROADMAP.md v0.4]]
and [[ROADMAP.md v0.5]] as they existed at commit `9651ce1`. It
does not supersede any prior ADR.

## ADR-0021 — Strip PTZ to pure single-step; remove press-and-hold (revert T-101a/b/c continuous mode)

**Status**: Accepted, 2026-06-02.

**Context**:

T-101a/b/c built a "continuous motion" model on top of the PTZ
pad: a `gtk::GestureClick` with a 200 ms long-press threshold
promoted a held button into a recurring `glib` timer that wrote
pan/tilt every 50 ms; the keyboard arrows ran an equivalent
per-key hold timer keyed by `gdk::Key`, with auto-repeat
suppression and a per-axis local accumulator
(`PtzAccumulators`); a `ptz-speed-fast` `GSettings` key scaled
the per-tick step and Shift+Arrow tripled it.

Testing the v0.3.2 Flatpak against the connected Tiny 2 Lite on
2026-06-02, the user reported the arrow behaviour "works terribly,
extremely buggy" and asked to reset it to the simplest possible
form: **one click / one key-press = exactly one move, no
press-and-hold, nothing that can error.**

**Decision**:

Remove the entire continuous-motion machinery and reduce the PTZ
pad to discrete single steps:

- On-screen directional buttons become plain `gtk::Button`s with
  a single `connect_clicked` → one `PAN_TILT_STEP` (5°) step per
  click. No `GestureClick`, no long-press, no repeat timer, no
  trailing-click suppression.
- Keyboard arrows fire one step per key-press event via a single
  `EventControllerKey` (Bubble phase preserved so focused sliders
  still consume their own arrows); `Home` recenters. No timers,
  no accumulator, no active-hold map. OS key auto-repeat simply
  issues more discrete steps — nothing runs on its own, so
  nothing can stick.
- Delete `PtzAccumulators`, `hold_tick`, `resolved_hold_step`,
  the `HOLD_*` / `LONG_PRESS_MS` / accelerator constants, and the
  now-orphaned `ptz-speed-fast` GSettings key +
  `settings::ptz_speed_fast()`.
- Extract the step arithmetic into a pure
  `next_position(current, sign, step, min, max)` and cover it
  with unit tests (it is the only logic worth testing; GTK
  signal wiring is not unit-testable).

**Consequences**:

- The whole class of bugs the user hit (sticky timers, drift,
  simultaneous-key races) is gone by construction — there is no
  state that outlives a single event.
- Lost features: smooth press-and-hold panning, the speed slider,
  and the Shift accelerator. These were never surfaced in a
  Preferences dialog anyway. If smooth panning is wanted again it
  should be re-introduced deliberately and tested on hardware,
  not as the default.
- This reverses the behaviour shipped in v0.3.1/v0.3.2 (T-101a/b/c).
  Tracked as task T-101d; the discrete tap path itself is
  unchanged from the original T-101 design.
- `cargo test` gains 4 `next_position` unit tests; all gates green
  default + with `obsbot-gui/live-preview`.

---

## ADR-0022 — Ship live-preview in the .deb artifact; GStreamer plugins as Recommends

**Status**: Accepted, 2026-06-05. Amends [[ADR-0015]] (the .deb
convenience artifact).

**Context**: while producing 0.4.0 hand-out artifacts for
colleague testing, the regenerated `.deb` turned out to exclude
the live preview: `obsbot-gui`'s `default = []` feature set
(chosen so plain `cargo build` works without GStreamer dev
packages) also governs what `cargo deb` compiles. A v0.4.0
package whose headline milestone is Live Preview, shipped without
the preview, would mislead testers. Debian 13 ships the required
runtime plugin as `gstreamer1.0-gtk4` (0.13.5); some still-common
distros (e.g. Ubuntu 24.04) do not ship it at all. The GUI
degrades gracefully when the element is missing —
`PreviewError::MissingElement` surfaces as a user-visible toast
and everything else keeps working.

**Decision**:

- Set `features = ["live-preview"]` in
  `[package.metadata.deb]` so the `.deb` binary always includes
  the preview pipeline.
- Declare the runtime plugin packages (`gstreamer1.0-gtk4`,
  `gstreamer1.0-plugins-good`, `gstreamer1.0-plugins-base`) as
  **Recommends**, not Depends. Linked libraries (e.g.
  `libgstreamer1.0-0`) still land in Depends via `$auto`.

**Consequences**:

- The `.deb` installs on distros without `gstreamer1.0-gtk4`
  (apt installs Recommends by default where available, but their
  absence is not fatal); there the app runs with preview
  disabled and a clear missing-element toast.
- On Debian 13+ the preview works out of the box.
- The dev-time default feature set is unchanged: plain
  `cargo build` / `cargo test` still need no GStreamer dev
  packages. Tracked as task T-206.

---

## ADR-0023 — Hand the project off to an incoming developer; T-017b Arch validation transfers with it

**Date**: 2026-06-05
**Status**: accepted
**Context**: The project is being handed to another developer who
will carry the remaining work. The persistent-memory method this
repo follows (CLAUDE.md → STATE.md → PLAN/PROGRESS/DECISIONS) was
built precisely so a cold reader can resume without tribal
knowledge; the handoff exercises that design. Two loose ends had to
be tied first: (1) an uncommitted working tree (the T-017b PKGBUILD
0.4.0 refresh) that would otherwise reach the new developer as an
unexplained diff; (2) the T-017b Arch validation itself, which
could not run on this Debian host (no container runtime; installing
one needs sudo the session cannot perform). An audit confirmed the
five local-only `feat/*` branches are all residue already in `main`
or in the v0.3.x tags — no orphaned work would be lost on a fresh
clone.
**Decision**:

- Commit the T-017b PKGBUILD refresh + docs now so `main` is the
  single source of truth and the working tree is clean for the
  clone. The PKGBUILD is corrected and ready (pkgver 0.4.0,
  `blueprint-compiler` in makedepends, `-Dlive-preview=true`,
  gstreamer runtime deps); only the *execution* of the validation
  is outstanding.
- Reassign T-017b's remaining acceptance criteria (makepkg →
  pacman -U → binary exec → pacman -R) to the incoming developer,
  who runs them on a real Arch host or container. This also serves
  the boss's request for an Arch build directly.
- Add `docs/HANDOFF.md` as the human-facing "start here" entry
  point, pointing at the existing machine-readable docs rather
  than duplicating them.
- Do not push the local `feat/*` branches; they are residue and
  `main` carries their content. Leave them on the original
  machine (branch-hygiene policy: delete only on explicit ask).

**Consequences**:

- The new developer clones, reads `CLAUDE.md` (which routes them
  to `STATE.md`), and resumes from a clean tree with T-017b as the
  first actionable task.
- The Arch validation moves closer to its natural environment (a
  real Arch stakeholder) instead of being forced through a
  containerised Debian workaround.
- `STATE.md` keeps T-017b `IN_PROGRESS` (PKGBUILD done, validation
  pending) so its state is unambiguous to the cold reader.

---

## ADR-0024 — Stop the preview on navigate-away and window close; do not rely on Drop

**Date**: 2026-06-10
**Status**: accepted
**Context**: Colleagues testing the 0.4.0 hand-out artifacts reported
that the camera sometimes stays on (LED lit) when nobody is actively
using it. Investigation of the preview lifecycle found the GStreamer
pipeline only returns to NULL — releasing the V4L2 capture node — in
two places: an explicit toggle-off (`controls_view.rs`), and
`PreviewPipeline::drop` (`preview.rs`). Neither covers the common
"stop using it" gestures:

- **Navigating back** to the camera list (the `←` button, or the
  T-110 `pop_to_tag` after a hot-plug REMOVE) leaves the pipeline in
  PLAYING. The page is popped, but `AdwNavigationView` retains the
  page and the `Rc<RefCell<Option<PreviewPipeline>>>` clones live on
  inside the header-bar toggle / snapshot / grayscale closures, so
  the `Drop`-based stop is non-deterministic and frequently does not
  fire promptly. The camera keeps capturing while the user is on the
  list.
- **Closing the window**: process exit closes the fd, so this case
  usually self-heals, but the ordering is not guaranteed and a future
  background-hold would break it.
- **`preview-default-on`** aggravates the above by auto-starting the
  pipeline on every controls-page open.

SPEC §4.3 (XDG Background Portal) is listed as scope but is **not**
implemented, so the app does not run in the background deliberately —
the bug is purely a missing stop, not a runaway background process.
The user was offered three scopes (diagnose-only / robust-stop /
robust-stop + visibility-pause) and chose **robust stop**.

**Decision**:

- Stop the pipeline **deterministically via signals**, not via
  `Drop`. Add `preview::register_active` / `preview::stop_active`
  (a `thread_local!` weak back-reference to the active page's
  pipeline slot). `controls_view::build_controls_page` wires
  `AdwNavigationPage::connect_hidden` → `pipeline.stop()` for the
  navigation cases, and `window::build` wires
  `connect_close_request` → `preview::stop_active()` for close.
- Keep the `Drop` impl as a backstop; it is no longer the primary
  release path.
- **Out of scope for now**: pausing/resuming the preview on window
  minimise or focus-loss. If the preview is left explicitly on and
  the window is merely minimised, the camera stays on — recorded as a
  follow-up (`preview-visibility-pause`), and the natural place for
  the eventual SPEC §4.3 Background Portal work.

**Consequences**:

- Releasing the camera no longer depends on GObject finalization
  timing under `AdwNavigationView`; the LED goes off as soon as the
  user leaves the page or closes the window.
- One more `thread_local!` (mirrors the T-108 toast-overlay and
  T-111 row-registry patterns already in `settings.rs`), gated on the
  `live-preview` feature so feature-off builds are unaffected.
- The LED-behaviour acceptance is hardware-only; T-207 stays
  `IN_PROGRESS` until the user / a colleague confirms it on a real
  unit.

---

## ADR-0025 — Auto-sleep must be deferred ~3 s after streaming; revert the inline T-208 sleep

**Date**: 2026-06-11
**Status**: accepted (supersedes the inline-sleep mechanism of
[[ADR-0024]] / T-208; the goal — power the camera down when unused —
is unchanged)
**Context**: Hands-on headless probing of the user's Tiny 2 Lite
(fw 5.10), driving `/dev/video0` directly, established three firmware
facts that the original T-208 (sleep inline in `PreviewPipeline::stop`)
did not account for:

- **The firmware ignores a Sleep frame for ~3 s after streaming
  stops.** Measured by retrying Sleep once per second after a capture:
  `get_status` read `Awake` at t=1 s and t=2 s, flipped to `Sleep` at
  t≈3 s. A *cold* Sleep (no preceding capture) works immediately. So
  T-208's inline sleep — fired the instant the stream stopped — never
  actually slept the camera.
- **Closing the V4L2 fd does not power the camera down** (confirmed
  earlier, ADR-0024); only the XU Sleep frame does.
- **Rapid open/close/sleep/wake churn hangs the camera** (accepts
  open + negotiates caps but delivers 0 buffers; a USB replug
  recovers). Sleeping on *every* stop made this worse.

**Decision**:

- Revert the inline sleep. `stop` instead **arms a deferred timer**
  that sends Sleep at t=3,4,5 s (retries absorb firmware jitter),
  skipping if another process holds the device. `start` cancels the
  pending timer and sends an explicit **Wake** before opening the
  stream (a prior auto-sleep may have powered the camera down).
- Window close hides the window for an instant-feeling close, keeps
  the app alive (hidden, not destroyed), fires Sleep after 4 s, then
  `app.quit()` — because we cannot block the close for 3 s without it
  feeling broken.
- Keep the `another_process_has_device` safeguard and the `Drop`
  backstop from T-207.

**Consequences**:

- The camera now actually sleeps (LED off, lens cover) a few seconds
  after the preview stops or the window closes, and wakes reliably on
  the next preview.
- Closing the app keeps the process alive ~4 s (window hidden) before
  it exits — a deliberate, bounded trade to honour "sleep on close".
  Not a persistent background app.
- Less sleep/wake churn than the inline design, lowering the
  camera-hang risk. The hang itself (firmware-level, replug-recover)
  remains a latent issue tracked separately.
- `SLEEP_DELAY_SECS` (3) is hardware-measured on one unit; other
  OBSBOT models/firmwares may differ and can tune the constant.

---

## ADR-0026 — GUI redesign (ViewSwitcher tabs + preview card + custom CSS) and removal of the "Camera awake" switch

**Status**: Accepted, 2026-06-11.

**Context**: A colleague testing the 0.4.1 hand-out gave three pieces
of feedback: (1) the self-view is not mirrored ("right hand on the left
of the screen"); (2) the "Camera awake" switch in the Power-state group
does not reliably drive the firmware sleep/wake; (3) the interface
should be "MUCHO MÁS CHULA". The user asked to redesign the GUI
*without touching functionality* and confirmed the direction via
AskUserQuestion (ViewSwitcher tabs + featured preview card + custom CSS).

**Decisions**:

- **T-210**: add a preview-only mirror toggle (a `videoflip` element +
  header `ToggleButton`). Preview-only — it does not alter what other
  apps capture; the raw UVC stream is unchanged.
- **T-211**: remove the "Camera awake" `AdwSwitchRow`. ROADMAP v0.3
  listed "Sleep / Wake camera power state" as a feature, but the manual
  switch never worked reliably and the T-208 auto-sleep-on-close
  ([[ADR-0024]] / [[ADR-0025]]) already covers the real goal (power the
  camera down when unused). The XU `set_sleep` backend stays in
  `obsbot-core`; only the unreliable UI affordance is dropped. A
  deliberate, documented step back from that ROADMAP v0.3 line item.
- **T-212**: reorganise the per-camera controls page into an
  `AdwViewStack` + `AdwViewSwitcher` (tabs: Image · Move · AI · Extras),
  promote the live preview to a rounded, shadowed card, and ship a
  custom `style.css`. The CSS is kept light and uses Adwaita's named
  colors (`@card_bg_color`, `@accent_bg_color`, …) so it adapts to
  theme/accent and stays HIG-friendly for the GNOME Circle goal
  ([[SPEC.md §6.2]]) — no hard-coded palette, no fighting the platform
  theme. All existing group/row builders are reused verbatim and merely
  redistributed across tabs, so no control loses its write wiring.

**Consequences**:

- The controls page scrolls less and reads as a modern, sectioned app;
  the preview is the visual centre of gravity.
- A custom stylesheet now ships in the GResource (loaded via
  `CssProvider` at `APPLICATION` priority). Future contributors must
  keep it theme-relative; any hard-coded color is a HIG regression.
- The only way to sleep the camera from the app is now the automatic
  on-close path; there is no manual sleep button. If a manual power
  control is ever wanted back, it needs a reliable firmware path first
  (the old one was a no-op on the test unit).
- Tabs are added only for groups the connected camera advertises, so a
  minimal UVC device still renders cleanly (empty tabs are hidden).

---

## ADR-0027 — Distribute the Arch `.pkg.tar.zst` via a GitHub Release asset, not the git tree

**Status**: Accepted, 2026-06-12.

**Context**: After the GUI session (T-210..T-217) the user built the Arch
package `obsbot-cam-control-0.4.1-1-x86_64.pkg.tar.zst` and asked to "push
everything" so a colleague could use it. The built package had been sitting
untracked in the repo root (as the prior 0.4.0 one did — STATE always marked
it "leave untracked; superseded"). The literal request would have committed a
342 KB binary blob into git history permanently.

**Decision**: Do **not** commit built packages to the git tree. Instead:

- Tag the release commit `v0.4.1` (annotated) and push the tag.
- Create a GitHub Release for `v0.4.1` and attach the `.pkg.tar.zst` as a
  release asset (changelog derived from the conventional commits since
  `v0.4.0`).
- The `.pkg` files stay out of version control; they are reproducible build
  artifacts (the `build-aux/PKGBUILD` builds the working tree directly).

Confirmed with the user via AskUserQuestion (chose "GitHub Release (tag
v0.4.1)" over committing the blob or keeping it local-only).

**Consequences**:

- Binary artifacts never bloat the git history; the repo stays source-only,
  consistent with the long-standing "leave the `.pkg` untracked" convention.
- The canonical place to fetch a prebuilt Arch package is now the GitHub
  Releases page; the colleague can still rebuild from `build-aux/PKGBUILD`.
- Future point releases follow the same flow: bump version → tag `vX.Y.Z` →
  `gh release create` with the artifact attached.
- The local `.pkg` files remain untracked working-tree artifacts (cleanup is
  the user's call; the auto-mode classifier blocked deleting user-created
  files and that is fine).

---

<!-- Append new ADRs above this line, never below. Newest ADRs go at the bottom
     of the list but new entries are added; do not edit old ones. -->
