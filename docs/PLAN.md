# PLAN — Atomic Tasks

> **Purpose**: Concrete, atomic, executable tasks. Updated by Claude Code as
> work happens. Each task has an ID, state, acceptance criteria, and depends-on.
>
> **States**: `TODO` · `IN_PROGRESS` · `DONE` · `BLOCKED` · `CANCELLED`

---

## Current milestone: v0.1 — Scaffolding & Detection

### T-001 — Initialize git repository and validate scaffolding
- **State**: DONE
- **Started**: 2026-05-12T10:30:00Z
- **Completed**: 2026-05-12T10:30:00Z
- **Depends on**: —
- **Description**: Run `git init`, validate file structure against
  `ARCHITECTURE.md` §2, make the first commit containing the scaffolding.
- **Acceptance criteria**:
  - `git log` shows one commit with message `chore: initial scaffolding (T-001)`.
  - `git status` is clean.
  - All files listed in `ARCHITECTURE.md` §2 either exist or have a justified
    absence noted in `DECISIONS.md`.
- **Outcome**: git repository initialized on branch `main`. Scaffolding
  validated against ARCHITECTURE §2; absent files justified in
  [[ADR-0010]] mapping each gap to its later PLAN task. Initial commit
  `chore: initial scaffolding (T-001)` created with the entire scaffolding
  tree (docs, CLAUDE.md, README.md, .gitignore, and four `.gitkeep`
  placeholders).
- **Notes**: this is also a test of the Conventional Commits + task-id rule.

### T-002 — Decide and document app namespace and license
- **State**: TODO
- **Depends on**: T-001
- **Description**: Pick the reverse-DNS namespace for the app (e.g.
  `io.github.<username>.ObsbotControl`) and the OSI license (recommend GPL-3.0
  or MIT). Stop and ask the user — both decisions need their input.
- **Acceptance criteria**:
  - `DECISIONS.md` contains an ADR with rationale for the namespace.
  - `DECISIONS.md` contains an ADR with rationale for the license.
  - `LICENSE` file added at repo root.
  - All placeholders `io.github.<ns>` replaced project-wide.
  - Commit: `chore: set namespace and license (T-002)`.

### T-003 — Capture and document Tiny 2 USB descriptor
- **State**: TODO
- **Depends on**: T-001
- **Description**: With the user's help (they must run the commands; Claude
  cannot touch hardware), capture `lsusb -v` and `v4l2-ctl --all` and
  `v4l2-ctl --list-ctrls-menus` output for the connected Tiny 2. Identify XU
  unit IDs, GUIDs, and existing V4L2 controls.
- **Acceptance criteria**:
  - `docs/PROTOCOL.md` has a "Tiny 2 USB descriptor" section with the trimmed
    relevant `lsusb -v` output (focus on Video Control interface, Extension
    Units).
  - `docs/PROTOCOL.md` has a table of V4L2 controls exposed and their ranges.
  - All XU unit IDs and GUIDs documented.
  - Commit: `docs: document Tiny 2 USB descriptor (T-003)`.
- **Notes**: STOP and ask the user to run the commands. Provide them exactly
  what to paste.

### T-004 — Set up Cargo workspace
- **State**: TODO
- **Depends on**: T-002
- **Description**: Create `Cargo.toml` at root declaring the three crates
  (`obsbot-core`, `obsbot-cli`, `obsbot-gui`) as a workspace. Add shared
  workspace dependencies block with pinned versions (`gtk4`, `libadwaita`,
  `gstreamer`, `tracing`, etc.).
- **Acceptance criteria**:
  - `cargo check --workspace` passes (even if crates are empty).
  - `cargo fmt --check` passes.
  - Commit: `build: create cargo workspace (T-004)`.

### T-005 — Stub `obsbot-core` crate
- **State**: TODO
- **Depends on**: T-004
- **Description**: Create `crates/obsbot-core/` with `lib.rs` exporting the
  `Camera` trait shape from `ARCHITECTURE.md` §3.1 (no implementations yet,
  trait methods can be stubs returning `Err(Unsupported)`). Add `CameraInfo`,
  `Capabilities`, `error::Error` types.
- **Acceptance criteria**:
  - `cargo test -p obsbot-core` passes (compiles, zero tests).
  - `cargo clippy -p obsbot-core -- -D warnings` passes.
  - Public API documented with `///` comments.
  - Commit: `feat(core): scaffold Camera trait and types (T-005)`.

### T-006 — Stub `obsbot-cli` crate
- **State**: TODO
- **Depends on**: T-005
- **Description**: Create `crates/obsbot-cli/` with a `main.rs` that prints
  "obsbot-cli vX.Y.Z" using `clap` and exits.
- **Acceptance criteria**:
  - `cargo run -p obsbot-cli -- --version` prints version from `Cargo.toml`.
  - Commit: `feat(cli): scaffold CLI binary (T-006)`.

### T-007 — Stub `obsbot-gui` crate
- **State**: TODO
- **Depends on**: T-005
- **Description**: Create `crates/obsbot-gui/` with an `adw::Application` that
  opens an empty `adw::ApplicationWindow` with a header bar saying "OBSBOT
  Control".
- **Acceptance criteria**:
  - `cargo run -p obsbot-gui` opens the window on the user's machine.
  - Closes cleanly on Ctrl+Q and on window close button.
  - Commit: `feat(gui): scaffold libadwaita application (T-007)`.

### T-008 — Set up Meson build system
- **State**: TODO
- **Depends on**: T-007
- **Description**: Create top-level `meson.build` that orchestrates the cargo
  build, processes the `.desktop.in` and `.metainfo.xml.in` templates,
  compiles the GSettings schema and installs everything to the prefix.
- **Acceptance criteria**:
  - `meson setup builddir && meson compile -C builddir` succeeds.
  - `meson install -C builddir --destdir /tmp/install-test` produces a
    correctly-laid-out filesystem under `/tmp/install-test`.
  - Commit: `build: set up Meson orchestration (T-008)`.

### T-009 — Create AppStream metainfo and desktop file
- **State**: TODO
- **Depends on**: T-002 (namespace), T-008
- **Description**: Write `<app-id>.metainfo.xml.in` with description, summary
  (≤ 35 chars), categories, license, content rating. Write `<app-id>.desktop.in`
  with name, comment, exec, icon, categories.
- **Acceptance criteria**:
  - `appstreamcli validate` passes with zero errors.
  - `desktop-file-validate` passes.
  - Commit: `feat: AppStream metainfo and desktop file (T-009)`.

### T-010 — Add icon (placeholder OK)
- **State**: TODO
- **Depends on**: T-009
- **Description**: Add a placeholder icon (scalable SVG) at the correct path
  (`data/icons/scalable/apps/<app-id>.svg`) and a symbolic version. A
  better-designed icon is a later concern; this just needs to be a recognizable
  camera shape in Adwaita style.
- **Acceptance criteria**:
  - Icon renders in GNOME Shell after `cargo run -p obsbot-gui`.
  - Symbolic icon respects current accent color.
  - Commit: `feat: add app icon (T-010)`.

### T-011 — Implement USB enumeration for Tiny 2
- **State**: TODO
- **Depends on**: T-005, T-003
- **Description**: In `obsbot-core`, implement `enumerate_cameras() ->
  Vec<CameraInfo>` that scans `/sys/class/video4linux/*` and filters by
  Tiny 2's VID/PID (from T-003).
- **Acceptance criteria**:
  - `cargo test -p obsbot-core` includes a unit test using a mock filesystem.
  - On the user's machine, an integration test marked `#[ignore]` succeeds
    when the camera is connected and reports the correct device path.
  - Commit: `feat(core): USB enumeration for Tiny 2 (T-011)`.

### T-012 — Wire enumeration into CLI
- **State**: TODO
- **Depends on**: T-006, T-011
- **Description**: Add `obsbot-cli list` subcommand that prints detected
  cameras.
- **Acceptance criteria**:
  - On the user's machine, `cargo run -p obsbot-cli -- list` prints the
    detected Tiny 2.
  - Output format documented in `--help`.
  - Commit: `feat(cli): list command (T-012)`.

### T-013 — Diagnostics view in GUI
- **State**: TODO
- **Depends on**: T-007, T-011
- **Description**: Replace the empty window with an `AdwPreferencesPage`
  showing one row per detected camera and a sub-page per camera listing its
  V4L2 controls (read-only). Use Blueprint for the UI definition.
- **Acceptance criteria**:
  - On the user's machine, plugging in Tiny 2 makes it appear in the list.
  - Unplugging removes it (hotplug listener via `udev` or polling).
  - V4L2 controls are listed with their current value and range.
  - Commit: `feat(gui): diagnostics view (T-013)`.

### T-014 — Initial Flatpak manifest
- **State**: TODO
- **Depends on**: T-008, T-009, T-010
- **Description**: Create `build-aux/<app-id>.json` for `flatpak-builder`.
  Permissions: `--device=all`, `--share=ipc`, `--socket=wayland`,
  `--socket=fallback-x11`. Runtime: GNOME 48.
- **Acceptance criteria**:
  - `flatpak-builder --user --install --force-clean build-flatpak
    build-aux/<app-id>.json` succeeds.
  - `flatpak run <app-id>` opens the diagnostics window from T-013.
  - Commit: `build: initial Flatpak manifest (T-014)`.

### T-015 — Set up CI (deferred until repo is public)
- **State**: TODO
- **Depends on**: T-014
- **Description**: GitHub Actions workflows: one for `cargo fmt + clippy +
  test`, one for the Flatpak build. Run on push and PR.
- **Acceptance criteria**:
  - Both workflows green on `main`.
  - Badge added to `README.md`.
  - Commit: `ci: GitHub Actions for build and lint (T-015)`.
- **Notes**: do not run until repo is on GitHub. Mark `BLOCKED` if still
  private when reached.

---

## Backlog (future milestones)

The detailed task breakdown for v0.2 onwards will be filled in when the
current milestone is near completion. This avoids stale plans.

Hints of what will come:

**v0.2 hints**: T-100 series.
- T-100 Implement V4L2 brightness/contrast/saturation/hue.
- T-101 PTZ pad widget in GUI.
- T-102 Zoom slider.
- T-103 White balance widget.
- T-104 Exposure widget.
- T-105 Per-camera GSettings persistence.
- T-106 About dialog.

**v0.3 hints**: T-200 series (GStreamer preview).
**v0.4 hints**: T-300 series (XU vendor features).
**v0.5 hints**: T-400 series (Auto-framing).
**v0.6 hints**: T-500 series (Polish).
