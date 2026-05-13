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

<!-- Append new ADRs above this line, never below. Newest ADRs go at the bottom
     of the list but new entries are added; do not edit old ones. -->
