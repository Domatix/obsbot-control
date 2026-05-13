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
- **State**: DONE
- **Started**: 2026-05-12T10:35:00Z
- **Completed**: 2026-05-12T10:55:00Z
- **Depends on**: T-001
- **Description**: Pick the reverse-DNS namespace for the app (resolved to
  `io.github.domatix.ObsbotCamControl`) and the OSI license (resolved to
  `GPL-3.0-or-later`). Both decisions taken with explicit user input.
- **Acceptance criteria**:
  - `DECISIONS.md` contains an ADR with rationale for the namespace.
  - `DECISIONS.md` contains an ADR with rationale for the license.
  - `LICENSE` file added at repo root.
  - All placeholders `io.github.<ns>` replaced project-wide.
  - Commit: `chore: set namespace and license (T-002)`.
- **Outcome**: [[ADR-0011]] records GPL-3.0-or-later; [[ADR-0012]] records
  the App ID `io.github.domatix.ObsbotCamControl`, GitHub org `Domatix`,
  display name "Obsbot Cam Control", and copyright line "© 2026 Domatix and
  contributors". `LICENSE` file installed at repo root with verbatim GNU
  GPL-3.0 text. All live `<ns>` and `<app-id>` placeholders in ARCHITECTURE,
  PLAN, SKILLS, GLOSSARY, README replaced; historical references inside
  past PROGRESS entries and ADR-0009/ADR-0010 left intact (append-only).

### T-003 — Capture and document Tiny 2 family USB descriptor
- **State**: DONE
- **Started**: 2026-05-12T11:00:00Z
- **Completed**: 2026-05-13T10:45:00Z
- **Depends on**: T-001
- **Description**: Capture `lsusb -v` and `v4l2-ctl --all` and
  `v4l2-ctl --list-ctrls-menus` output for the user's connected Tiny 2
  family unit. Identify XU unit IDs, GUIDs, and existing V4L2 controls.
  Title amended from "Tiny 2" to "Tiny 2 family" by [[ADR-0014]] after
  the first capture revealed the hardware on hand is a Tiny 2 Lite
  (`3564:fef9`), sibling of the regular Tiny 2 (`3564:fef8`).
- **Notes**: original "Claude cannot touch hardware" framing was wrong.
  `lsusb` is a read-only USB descriptor query that Claude runs directly
  on the user's machine. The hand-off in PROGRESS 2026-05-12T11:00Z was
  unnecessary. V4L2 capture still benefits from a one-time `usermod
  -aG video alvaro` so Claude can re-run it without sudo.
- **Acceptance criteria**:
  - `docs/PROTOCOL.md` has a "Hardware identifiers" section with the
    `lsusb -v` capture for each Tiny 2 family PID for which data is
    available (Lite = direct capture; regular Tiny 2 = community
    capture, marked speculative until then). **DONE** — PROTOCOL.md §1.1.
  - `docs/PROTOCOL.md` has a table of V4L2 controls exposed and their
    ranges (`/dev/video0` and `/dev/video1`). **DONE** — PROTOCOL.md §2
    (24 controls: 13 User + 11 Camera; metadata node carries none).
  - All XU unit IDs and GUIDs of the captured device documented.
    **DONE** — Unit 2, GUID `9a1e7291-…`, kernel-mount confirmed via
    media-graph "Extension 2 (Video Pixel Formatter)" entity.
  - Commit: `docs: capture Tiny 2 Lite USB descriptor (T-003)` (lsusb
    half, `19d8026`) and `docs: capture Tiny 2 Lite V4L2 controls
    (T-003)` (v4l2-ctl half, this commit).
- **Outcome**: complete USB + V4L2 picture of the Tiny 2 Lite captured
  on Debian trixie / kernel 6.12.73 / driver uvcvideo. Three quirks
  flagged for the v0.2 GUI design: power_line_frequency default outside
  menu range, zoom_continuous saturating beyond its advertised max,
  gamma absent from PU bmControls (XU-only candidate). [[ADR-0014]]
  scope decision (Tiny 2 family) supersedes the original "Tiny 2 only"
  framing. Per-selector XU semantics intentionally deferred to v0.4 /
  T-300+ Wireshark work.

### T-004 — Set up Cargo workspace
- **State**: DONE
- **Started**: 2026-05-12T11:00:00Z
- **Completed**: 2026-05-13T10:05:00Z
- **Depends on**: T-002
- **Description**: Create `Cargo.toml` at root declaring the three crates
  (`obsbot-core`, `obsbot-cli`, `obsbot-gui`) as a workspace. Add shared
  workspace dependencies block with pinned versions (`gtk4`, `libadwaita`,
  `gstreamer`, `tracing`, etc.).
- **Acceptance criteria** (amended by [[ADR-0013]] — original `cargo check
  --workspace` + `cargo fmt --check` criteria moved to T-005 because they
  require ≥1 member crate, which T-004 does not yet create):
  - `cargo metadata --no-deps --format-version 1` succeeds (manifest parses,
    `[workspace.dependencies]` resolve).
  - `[workspace.package]` honors [[ADR-0003]] (MSRV 1.83, edition 2021).
  - `[workspace.dependencies]` pins the runtime stack from `ARCHITECTURE §1`.
  - Commit: `build: create cargo workspace (T-004)`.
- **Outcome**: root `Cargo.toml` declares `[workspace] resolver = "2",
  members = ["crates/*"]`, shared `[workspace.package]` (version 0.1.0,
  edition 2021, rust-version 1.83, license GPL-3.0-or-later, Domatix
  authors/repo), `[workspace.dependencies]` pinning the runtime stack
  per [[ADR-0003]] + [[ARCHITECTURE §1]] (gtk4 0.9, libadwaita 0.7,
  glib/gio 0.20, gstreamer family 0.23, v4l 0.14, nusb 0.1, nix 0.29
  +ioctl, tracing 0.1, tracing-subscriber 0.3 +env-filter, thiserror 2,
  anyhow 1, clap 4 +derive, async-channel 2, gettext-rs 0.7
  +gettext-system), and a `[profile.release]` (lto thin, codegen-units
  1, strip symbols). `.gitignore` amended: `Cargo.lock` is committed
  (this workspace ships binaries; reproducible Flathub/distro builds
  require pinned lockfile — inline comment links Cargo FAQ).
  Validation: `cargo metadata --no-deps --format-version 1` exit 0 and
  `cargo verify-project` returns `{"success":"true"}` against rustc
  1.85.0 from Debian trixie. Acceptance via [[ADR-0013]]-amended
  criteria; full `cargo check --workspace` + `cargo fmt --check`
  enforcement deferred to T-005's first member crate.

### T-005 — Stub `obsbot-core` crate
- **State**: DONE
- **Started**: 2026-05-13T10:49:16Z
- **Completed**: 2026-05-13T10:52:32Z
- **Depends on**: T-004
- **Description**: Create `crates/obsbot-core/` with `lib.rs` exporting the
  `Camera` trait shape from `ARCHITECTURE.md` §3.1 (no implementations yet,
  trait methods can be stubs returning `Err(Unsupported)`). Add `CameraInfo`,
  `Capabilities`, `error::Error` types.
- **Acceptance criteria**:
  - `cargo check --workspace` passes (inherited from T-004 via [[ADR-0013]]). **DONE.**
  - `cargo fmt --all --check` passes (inherited from T-004 via [[ADR-0013]]). **DONE.**
  - `cargo test -p obsbot-core` passes (compiles, zero tests). **DONE** — 3 unit tests + 1 doc-test pass, none ignored (PLAN said "zero tests" as a floor; landed three sanity assertions instead — confirms default-Unsupported semantics, Capabilities default, and CameraInfo round-trip).
  - `cargo clippy -p obsbot-core -- -D warnings` passes. **DONE** with a single `#[allow(clippy::struct_excessive_bools)]` on `Capabilities` (justification comment cites [[ARCHITECTURE §3.1]]; lint suggests a state machine which is the wrong shape for independent feature flags).
  - Public API documented with `///` comments. **DONE** — crate-level + all public items + each trait method (with `# Errors` sections per clippy `missing_errors_doc`).
  - Commit: `feat(core): scaffold Camera trait and types (T-005)`.
- **Outcome**: `crates/obsbot-core/` created with `Cargo.toml` (consumes
  `thiserror` and `tracing` from `[workspace.dependencies]`,
  `lints.rust.unsafe_code = "forbid"`, `lints.clippy.pedantic = warn`),
  `src/error.rs` (`Error` non_exhaustive enum: Unsupported / OutOfRange
  / Busy(PathBuf) / Disconnected / `#[from] std::io::Error`),
  `src/camera.rs` (`CameraInfo`, `Capabilities` of 26 bool flags,
  enums `AntiFlicker` / `ExposureMode` / `Fov` / `AutoFramingMode`
  with `#[non_exhaustive]`, `Camera` trait with 2 required +
  ~50 defaulted methods returning `Err(Error::Unsupported)`), and
  `src/lib.rs` (re-exports + crate-level doctest). `Cargo.lock` now
  tracked (workspace ships binaries — already enforced by
  [[ADR-0013]] indirectly via the `.gitignore` change in T-004).
  `crates/.gitkeep` removed (directory has real content now).

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
- **Description**: Write `io.github.domatix.ObsbotCamControl.metainfo.xml.in`
  with description, summary (≤ 35 chars), categories, license, content
  rating. Write `io.github.domatix.ObsbotCamControl.desktop.in` with name,
  comment, exec, icon, categories.
- **Acceptance criteria**:
  - `appstreamcli validate` passes with zero errors.
  - `desktop-file-validate` passes.
  - Commit: `feat: AppStream metainfo and desktop file (T-009)`.

### T-010 — Add icon (placeholder OK)
- **State**: TODO
- **Depends on**: T-009
- **Description**: Add a placeholder icon (scalable SVG) at the correct path
  (`data/icons/scalable/apps/io.github.domatix.ObsbotCamControl.svg`) and a
  symbolic version. A
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
- **Description**: Create
  `build-aux/io.github.domatix.ObsbotCamControl.json` for `flatpak-builder`.
  Permissions: `--device=all`, `--share=ipc`, `--socket=wayland`,
  `--socket=fallback-x11`. Runtime: GNOME 48.
- **Acceptance criteria**:
  - `flatpak-builder --user --install --force-clean build-flatpak
    build-aux/io.github.domatix.ObsbotCamControl.json` succeeds.
  - `flatpak run io.github.domatix.ObsbotCamControl` opens the diagnostics
    window from T-013.
  - Commit: `build: initial Flatpak manifest (T-014)`.

### T-015 — Set up CI (deferred until repo is public)
- **State**: TODO
- **Depends on**: T-014
- **Description**: GitHub Actions workflows: one for `cargo fmt + clippy +
  test`, one for the Flatpak build, one each (or one matrix) for the
  test packages from T-016 / T-017. Run on push and PR; the test-package
  jobs additionally upload artifacts on tag pushes (`v*`) per
  [[ADR-0015]].
- **Acceptance criteria**:
  - Workflows green on `main`.
  - Badge added to `README.md`.
  - On a `v*` tag, GitHub Release attaches a `.deb` and a `.pkg.tar.zst`.
  - Commit: `ci: GitHub Actions for build and lint (T-015)`.
- **Notes**: do not run until repo is on GitHub. Mark `BLOCKED` if still
  private when reached.

### T-016 — Test-artifact: `.deb` via `cargo-deb`
- **State**: TODO
- **Depends on**: T-007 (runnable GUI), T-013 (diagnostics view so the
  installed app actually shows something), ideally T-014 too (Flatpak
  first since it stays the primary channel)
- **Description**: Add `[package.metadata.deb]` to
  `crates/obsbot-gui/Cargo.toml` declaring runtime depends
  (`libgtk-4-1`, `libadwaita-1-0`, `libgstreamer1.0-0`,
  `libgstreamer-plugins-base1.0-0`, plus whatever v4l/uvc system libs
  the final build links against). Add a `build-aux/build-deb.sh` (or
  Meson target) that runs `cargo deb -p obsbot-gui` and drops the
  artifact under `build-aux/dist/`. Document the install/uninstall
  command in `README.md`. Test on the user's Debian trixie machine.
  Scope per [[ADR-0015]]: convenience artifact, not Debian-policy.
- **Acceptance criteria**:
  - `cargo deb -p obsbot-gui` succeeds locally; artifact installs via
    `sudo apt install ./obsbot-cam-control_*_amd64.deb`.
  - After install, `obsbot-cam-control` launches and reaches the T-013
    diagnostics view against the user's Tiny 2 Lite.
  - `sudo apt remove obsbot-cam-control` leaves no stray files in
    `/usr/share/applications`, `/usr/share/icons/hicolor`,
    `/usr/share/glib-2.0/schemas`.
  - Commit: `build(deb): test-artifact .deb via cargo-deb (T-016)`.

### T-017 — Test-artifact: Arch `PKGBUILD` (`pkg.tar.zst`)
- **State**: TODO
- **Depends on**: same as T-016 (T-007 + T-013, and T-014 ideally first).
- **Description**: Add `build-aux/PKGBUILD` with `depends=(gtk4
  libadwaita gstreamer gst-plugins-base gst-plugins-good v4l-utils)`,
  `makedepends=(cargo rust meson)`. `source=()` points at a git tag.
  Add a `build-aux/build-arch.sh` that runs `makepkg -f` inside a
  container or a fakeroot (since we likely don't have a host Arch).
  Document the install command in `README.md`. The Arch stakeholder
  uses this artifact to sideload-test releases. Scope per
  [[ADR-0015]]: convenience artifact, not AUR-grade.
- **Acceptance criteria**:
  - `makepkg -f` (run by CI or a contributor on Arch) produces
    `obsbot-cam-control-*-x86_64.pkg.tar.zst`.
  - On an Arch test machine, `sudo pacman -U <package>` installs
    cleanly and `obsbot-cam-control` launches.
  - Commit: `build(arch): test-artifact PKGBUILD (T-017)`.

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
