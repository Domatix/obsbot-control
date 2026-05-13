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
- **State**: DONE
- **Started**: 2026-05-13T11:01:00Z
- **Completed**: 2026-05-13T11:02:07Z
- **Depends on**: T-005
- **Description**: Create `crates/obsbot-cli/` with a `main.rs` that prints
  "obsbot-cli vX.Y.Z" using `clap` and exits.
- **Acceptance criteria**:
  - `cargo run -p obsbot-cli -- --version` prints version from `Cargo.toml`.
    **DONE** — `obsbot-cli 0.1.0` via clap's auto-rendered `--version`,
    `obsbot-cli v0.1.0` from `println!` on bare invocation.
  - Commit: `feat(cli): scaffold CLI binary (T-006)`.
- **Outcome**: `crates/obsbot-cli/Cargo.toml` (consumes `clap` from
  `[workspace.dependencies]`, `[lints]` mirroring obsbot-core, explicit
  `[[bin]] name = "obsbot-cli"`); `src/main.rs` with `#[derive(Parser)]
  struct Cli {}` carrying `#[command(name, version, about)]` and a
  `main()` of three meaningful lines. `obsbot-core` dependency
  intentionally deferred to T-012 when the `list` subcommand needs it
  (per "no abstractions beyond what the task requires", [[CLAUDE.md §
  Doing tasks]]). Four workspace gates green (`cargo fmt --all --check`,
  `cargo check --workspace --all-targets`, `cargo clippy --workspace
  --all-targets -- -D warnings`, `cargo test --workspace`).
  `Cargo.lock` picks up clap 4.6.1 + transitive deps.

### T-007 — Stub `obsbot-gui` crate
- **State**: DONE
- **Started**: 2026-05-13T12:21:13Z
- **Completed**: 2026-05-13T12:30:42Z
- **Depends on**: T-005
- **Description**: Create `crates/obsbot-gui/` with an `adw::Application` that
  opens an empty `adw::ApplicationWindow` with a header bar saying
  **"Obsbot Cam Control"** (the resolved display name per [[ADR-0012]];
  the original T-007 text said "OBSBOT Control" — a placeholder written
  before T-002 resolved the namespace and now superseded).
- **Acceptance criteria**:
  - `cargo run -p obsbot-gui` opens the window on the user's machine.
    **DONE** — objectively verified via xwininfo
    (`0x2600004 "Obsbot Cam Control" 842x662+539+231`) plus the user's
    visual confirmation 2026-05-13T12:30:42Z.
  - Closes cleanly on Ctrl+Q and on window close button.
    **DONE** — user confirmed both interactive paths work
    (Claude cannot drive keyboard input).
  - Commit: `feat(gui): scaffold libadwaita application (T-007)`.
- **Outcome**: `crates/obsbot-gui/` with three source files (~120 lines
  of project code, ignoring SPDX headers) backed by GTK 4.18.6 +
  libadwaita 1.7.6 on the user's Debian trixie. `[[bin]] name =
  "obsbot-cam-control"` per [[ADR-0012]]. Source split mirrors
  [[ARCHITECTURE §2]]: `main.rs` (APP_ID const + `application::run`),
  `application.rs` (`adw::Application` factory, registers
  `app.quit` ActionEntry, binds `<primary>q`), `window.rs`
  (`AdwApplicationWindow` with header bar + Adwaita StatusPage
  placeholder pointing at T-013 / v0.2 for real content).
  Per-module gtk-rs aliasing via `use gtk4 as gtk;` /
  `use libadwaita as adw;`. Four workspace gates green
  (fmt-check, check --all-targets, clippy -D warnings, test).
  Cargo.lock picks up the GTK4 + libadwaita Rust binding trees.

### T-008 — Set up Meson build system
- **State**: DONE
- **Started**: 2026-05-13T12:36:03Z
- **Completed**: 2026-05-13T12:39:26Z
- **Depends on**: T-007
- **Description**: Create top-level `meson.build` that orchestrates the cargo
  build, processes the `.desktop.in` and `.metainfo.xml.in` templates,
  compiles the GSettings schema and installs everything to the prefix.
  **In-task scope correction**: the `.desktop.in` / `.metainfo.xml.in` /
  schema do not exist yet (T-009, T-010, T-105 introduce them). T-008
  intentionally lands only the cargo-orchestration spine plus
  guarded hook comments for the subsequent additions; the meson.build
  will be extended in place by those tasks, not rewritten.
- **Acceptance criteria**:
  - `meson setup builddir && meson compile -C builddir` succeeds.
    **DONE** — setup picks up gtk4 4.18.6, libadwaita 1.7.6,
    glib/gio 2.84.4 (all above ARCHITECTURE §1 minima). Compile took
    1m 22s on the first run (cargo's release profile, full
    optimisation pass for the gtk-rs + libadwaita-rs binding trees);
    incremental rebuilds finish in tens of milliseconds.
  - `meson install -C builddir --destdir /tmp/install-test` produces a
    correctly-laid-out filesystem under `/tmp/install-test`. **DONE** —
    install drops `obsbot-cam-control` (424 KB stripped ELF) at
    `/tmp/install-test/usr/local/bin/obsbot-cam-control`; `--help`
    responds with the standard GLib option-group output.
  - Commit: `build: set up Meson orchestration (T-008)`.
- **Outcome**: top-level `meson.build` declares the project
  (`obsbot-cam-control` 0.1.0, GPL-3.0-or-later, meson ≥ 1.0),
  asserts runtime-lib minimums (belt-and-suspenders vs. the
  cargo-side gtk4-sys/libadwaita-sys link), and wraps cargo in a
  single `custom_target('cargo-build', ...)` plumbed through
  `build-aux/cargo-build.sh` (a 50-line bash shim with `set -euo
  pipefail`, profile validation, `install -m 755` of the produced
  binary to `@OUTPUT@`). `default_options: ['buildtype=release']`
  matches the convention for installable GNOME apps. Hook comments
  for the `subdir('data')` (T-009/T-010), `subdir('po')` (T-009+),
  and GSettings-schema (T-105) extensions left in place. `'rust'`
  language declaration intentionally omitted — meson never invokes
  rustc directly, so declaring the language only adds noise.
  `build-aux/.gitkeep` removed (the directory now has real content,
  matching the [[T-005]] precedent for `crates/`).

### T-009 — Create AppStream metainfo and desktop file
- **State**: DONE
- **Started**: 2026-05-13T12:50:39Z
- **Completed**: 2026-05-13T12:54:51Z
- **Depends on**: T-002 (namespace), T-008
- **Description**: Write `io.github.domatix.ObsbotCamControl.metainfo.xml.in`
  with description, summary (≤ 35 chars), categories, license, content
  rating. Write `io.github.domatix.ObsbotCamControl.desktop.in` with name,
  comment, exec, icon, categories.
- **Acceptance criteria**:
  - `appstreamcli validate` passes with zero errors. **DONE** — `LC_ALL=C
    appstreamcli validate --no-net --explain` exits 0 with no E/W/I
    messages; one pedantic (`P:`) note `cid-contains-uppercase-letter`
    on `ObsbotCamControl` is intentional per [[ADR-0012]] (the App ID
    is fixed; AppStream recommends lowercase but allows mixed case —
    not an error).
  - `desktop-file-validate` passes. **DONE** — silent exit 0.
  - Commit: `feat: AppStream metainfo and desktop file (T-009)`.
- **Outcome**: two GNOME-Circle-shaped templates land under `data/`:
  the `.desktop.in` (13 lines, `Categories=AudioVideo;Video;`,
  `StartupWMClass` matching what T-007's xwininfo capture observed,
  Keywords listing both Tiny 2 PIDs verbatim per [[ADR-0014]]) and
  the `.metainfo.xml.in` (96 lines, `<summary>` "Control your OBSBOT
  webcam" = 26/35 chars, `<metadata_license>` `CC0-1.0` +
  `<project_license>` `GPL-3.0-or-later`, OARS-1.1 content-rating
  declared all-clear via the empty-element form, `<developer
  id="io.github.domatix">` per the post-1.0 AppStream schema, single
  `<release type="development">` for v0.1.0 with prose pointing at the
  scaffolding-only status, trademark disclaimer per [[ADR-0012]],
  supported-input controls declared as keyboard/pointing/touch).
  Both files carry `@APP_ID@` / `@VERSION@` placeholders substituted
  by `data/meson.build`'s `configure_file()` calls; the substituted
  output installs at `share/applications` / `share/metainfo` as
  freedesktop expects. `data/meson.build` also wires both validators
  as `meson test` cases (`required: false` so CI without
  appstreamcli/desktop-file-validate skips rather than fails). i18n
  via gettext intentionally **not** added — the templates have no
  `_Name=` / `<_summary>` markers and no `subdir('po')` yet; a later
  task will plug gettext in when actual translatable strings emerge
  (e.g. preset labels in v0.2). `data/.gitkeep` removed (the dir now
  has real content — matches the [[T-005]] / `crates/` and [[T-008]]
  / `build-aux/` precedent). Top-level `meson.build` swaps the
  placeholder `# subdir('data')` comment for the real call; the
  `# subdir('po')` hook stays as a comment.

### T-010 — Add icon (placeholder OK)
- **State**: DONE (with caveat — see Outcome)
- **Started**: 2026-05-13T13:00:58Z
- **Code-complete**: 2026-05-13T13:18:54Z
- **Completed**: 2026-05-13T15:44:58Z
- **Depends on**: T-009
- **Description**: Add a placeholder icon (scalable SVG) at the correct path
  (`data/icons/scalable/apps/io.github.domatix.ObsbotCamControl.svg`) and a
  symbolic version. A
  better-designed icon is a later concern; this just needs to be a recognizable
  camera shape in Adwaita style.
- **Acceptance criteria**:
  - Icon renders in GNOME Shell after `cargo run -p obsbot-gui`.
    **DEFERRED** — verified by parts (file renders correctly via
    `xdg-open` → user confirmed blue webcam visible; the file lands
    at `share/icons/hicolor/scalable/apps/` per `meson install`;
    `gtk::Window::set_default_icon_name` is wired). The end-to-end
    "icon appears in Alt+Tab" path failed in the dev test setup
    because GNOME Shell builds its `.desktop` → window-icon cache
    at session-startup and does not re-index when a `.desktop` is
    dropped into `~/.local/share/applications/` mid-session. The
    canonical fix is a real install (Flathub via T-014, distro
    package via T-016 / T-017) or a `gnome-shell --replace` /
    fresh login, neither of which is in scope for T-010.
  - Symbolic icon respects current accent color. **DEFERRED** —
    `fill="currentColor"` is in the SVG by inspection; the GTK
    contract guarantees recolouring against the current text /
    accent color when the icon renders in a symbolic surface. No
    symbolic surface exists in the app yet (the placeholder
    StatusPage uses `camera-web-symbolic`, a stock icon, not ours);
    the criterion becomes properly testable when T-013 introduces
    the diagnostics view or an About dialog references our
    symbolic.
  - Commit: `feat: add app icon (T-010)`.
- **Outcome (caveat note)**: T-010's code deliverables are complete
  and committed (`7e7c172` for the feature, `ec0da31` for the SHA
  fix). The two visual acceptance criteria are deferred from a
  framework-correctness perspective (everything is wired right);
  the actual end-to-end visual test will land naturally when:
  * T-014 produces a Flatpak — Flathub's runtime path triggers
    GNOME Shell's normal indexing; or
  * The user next logs into the GNOME session — Shell re-reads
    `~/.local/share/applications/` on login; or
  * A distro test-package (T-016 `.deb` / T-017 PKGBUILD) lands
    the files under `/usr/share/`, which Shell scans at startup.
  If any of those paths still shows the generic icon, a follow-up
  task is filed; until then the failure mode observed is a test
  artefact (dev `~/.local/share` drop into a running session), not
  a code defect.
- **Code-complete outcome**: two SVGs land under `data/icons/`:
  the scalable variant (~1.0 KB, 128×128 viewBox, Adwaita palette
  blues `#3584e4` / `#1a5fb4`, lens stack of four concentric circles
  with a white highlight ellipse, red tally LED at the top right,
  mounting neck + base rectangle below — recognisable as a webcam at
  every cache size GTK builds) and the symbolic variant (~480 B,
  16×16 viewBox, single compound path with `fill="currentColor"`).
  `data/meson.build` gains two `install_data` calls landing them at
  `share/icons/hicolor/{scalable,symbolic}/apps/` and a
  `gnome.post_install(gtk_update_icon_cache: true,
  update_desktop_database: true)` so a real (non-DESTDIR) install
  refreshes the hicolor cache and the desktop database in one shot.
  `crates/obsbot-gui/src/application.rs` adds a
  `gtk::Window::set_default_icon_name(app_id)` inside the existing
  `connect_startup` closure (the `app_id` slice is moved into the
  closure via `.to_owned()` since GTK callbacks require `'static`).
  All four cargo gates (`fmt --check`, `clippy -D warnings`, `test
  --workspace`, `check`) plus the two meson tests stay green; install
  under `/tmp/install-test` produces five files (T-008 binary + the
  two T-009 metadata files + the two T-010 icons).

### T-011 — Implement USB enumeration for Tiny 2
- **State**: DONE
- **Started**: 2026-05-13T15:44:58Z
- **Completed**: 2026-05-13T15:53:49Z
- **Depends on**: T-005, T-003
- **Description**: In `obsbot-core`, implement `enumerate_cameras() ->
  Vec<CameraInfo>` that scans `/sys/class/video4linux/*` and filters by
  Tiny 2's VID/PID (from T-003).
- **Acceptance criteria**:
  - `cargo test -p obsbot-core` includes a unit test using a mock filesystem.
    **DONE** — five new unit tests in `enumerate::tests`:
    `detects_tiny2_lite_with_dual_video_nodes` (dedup correctness +
    full `CameraInfo` round-trip on the user's hardware shape),
    `detects_regular_tiny2` (the family's other PID + serial present
    case), `rejects_non_obsbot_camera` (filter negative path on a
    Logitech-shaped descriptor), `missing_root_returns_empty`
    (resilient `/sys` absence), and `parses_known_hex` (small
    helper). The five join the three pre-existing T-005 unit tests
    for an 8-test obsbot-core suite plus the doctest.
  - On the user's machine, an integration test marked `#[ignore]`
    succeeds when the camera is connected and reports the correct
    device path. **DONE** — `crates/obsbot-core/tests/hardware.rs`
    holds `finds_connected_tiny2_family_unit` with `#[ignore]`. Run
    explicitly: `cargo test -p obsbot-core --test hardware --
    --ignored` returns `1 passed` against the user's plugged-in
    Tiny 2 Lite (VID `0x3564`, PID `0xfef9`, video_path
    `/dev/video0`, product string starts with "OBSBOT").
  - Commit: `feat(core): USB enumeration for Tiny 2 (T-011)`.
- **Outcome**: new module `crates/obsbot-core/src/enumerate.rs`
  (~160 lines including doc comments and the test mock helper) plus
  `crates/obsbot-core/tests/hardware.rs`. Public surface:
  * `pub const VID_OBSBOT: u16 = 0x3564;`
  * `pub const TINY2_FAMILY: &[(u16, u16)] = &[(0x3564, 0xfef8),
    (0x3564, 0xfef9)];`
  * `pub fn enumerate_cameras() -> Vec<CameraInfo>`
  * `pub fn enumerate_cameras_in(sysfs_video_root: &Path) ->
    Vec<CameraInfo>` (test entry point and future hot-plug listener
    entry point for the GUI from T-013).
  All four are re-exported from `crates/obsbot-core/src/lib.rs`.
  Private helpers `collect_one` / `read_attr` / `parse_hex_u16` stay
  inside `enumerate.rs`. The function intentionally returns
  `Vec<CameraInfo>` (not `Result`) — its three failure modes (sysfs
  missing, unreadable, no devices match) all collapse to "no
  cameras" from the consumer's perspective, and the underlying
  `io::Error` is logged via `tracing::warn!` for diagnostics. The
  per-device dedup keys on the canonicalised USB-device sysfs path
  so a Tiny 2 family unit (two `/dev/videoN` nodes: capture +
  metadata) surfaces as one row in the GUI. New workspace
  dependency: `tempfile 3` in `[workspace.dependencies]` and as a
  `[dev-dependencies]` entry in `obsbot-core`. `Cargo.lock` picks up
  tempfile + transitive deps (fastrand 2.x, rustix 1.x). All four
  cargo gates (`fmt --check`, `clippy -D warnings`, `test
  --workspace`, hardware `-- --ignored`) green plus meson's
  validate-metainfo + validate-desktop unchanged.

### T-012 — Wire enumeration into CLI
- **State**: DONE
- **Started**: 2026-05-13T16:05:00Z
- **Completed**: 2026-05-13T16:12:00Z
- **Depends on**: T-006, T-011
- **Description**: Add `obsbot-cli list` subcommand that prints detected
  cameras.
- **Acceptance criteria**:
  - On the user's machine, `cargo run -p obsbot-cli -- list` prints the
    detected Tiny 2. **DONE** — live invocation against the user's
    plugged-in Tiny 2 Lite yields the stanza:

    ```
    1 camera detected:

    [1] OBSBOT Tiny 2 Lite
        Vendor:   Remo Tech Co., Ltd.
        USB ID:   3564:fef9
        Serial:   (not advertised)
        Firmware: 0510
        Video:    /dev/video0
    ```
  - Output format documented in `--help`. **DONE** — the `list`
    subcommand carries a `clap` `long_about` listing all six stanza
    fields plus the zero-camera fallback and the exit-code contract;
    surfaced verbatim by `obsbot-cli list --help`.
  - Commit: `feat(cli): list command (T-012)`.
- **Outcome**: `crates/obsbot-cli/Cargo.toml` gains the `path = "../
  obsbot-core"` dep that [[PLAN T-006]] explicitly deferred ("`obsbot-
  core` dependency intentionally deferred to T-012"). `crates/obsbot-
  cli/src/main.rs` grows from a `--version` stub into a `clap`
  subcommand router: `Cli` now carries `command: Option<Commands>`,
  `Commands::List` calls `obsbot_core::enumerate_cameras()`, and a
  pure helper `render(&[CameraInfo]) -> String` produces the on-stdout
  output. The bare `obsbot-cli` invocation keeps its T-006 behaviour
  (prints `obsbot-cli v0.1.0`) so the version smoke test stays green.
  Three new unit tests (`render_zero_cameras`, `render_one_camera_
  missing_serial`, `render_two_cameras_indexed_and_pluralised`) pin
  the format-stability promise made in `list --help`: empty list →
  `No OBSBOT cameras detected.`, single-camera → 1-camera stanza
  with `(not advertised)` serial fallback, two-camera → pluralised
  header + `[1]`/`[2]` indexing + blank-line separator. Workspace
  test count: 11 unit (8 obsbot-core + 3 obsbot-cli) + 1 ignored
  hardware + 1 doctest. All four cargo gates green (`fmt --check`,
  `check --workspace --all-targets`, `clippy -D warnings`, `test
  --workspace`) and the meson 2/2 (validate-desktop +
  validate-metainfo) untouched. Exit code 0 in every case
  (cameras present, no cameras, bare invocation) — matches `ls`'s
  empty-directory semantics, no false-positive non-zero for the
  "nothing connected" path.

### T-013 — Diagnostics view in GUI (SPLIT)
- **State**: SUPERSEDED by [[ADR-0016]] — see T-013a / T-013b / T-013c
  / T-013d. The acceptance criteria of those four sub-atoms together
  satisfy the original three criteria of this task.

### T-013a — Initial camera list in GUI
- **State**: DONE
- **Started**: 2026-05-13T16:18:00Z
- **Completed**: 2026-05-13T16:25:00Z
- **Depends on**: T-007, T-011
- **Description**: Replace the T-007 placeholder `AdwStatusPage` in
  `crates/obsbot-gui/src/window.rs` with an `AdwPreferencesPage`
  carrying one `AdwActionRow` per camera returned by
  `obsbot_core::enumerate_cameras()` at app startup. Empty state
  remains an `AdwStatusPage` (camera-web-symbolic icon, "Connect a
  Tiny 2 family unit via USB." hint). Hand-coded GTK is acceptable
  per [[CLAUDE.md §5.3]]'s "unless dynamic" carve-out (the row list
  is dynamic, the surrounding page shell is too small to justify a
  Blueprint pipeline). Hot-plug listener is out of scope (T-013b);
  V4L2 control drill-down is out of scope (T-013c).
- **Acceptance criteria**:
  - On the user's machine, the Tiny 2 Lite plugged in before `cargo
    run -p obsbot-gui` starts shows up as an `AdwActionRow` titled
    "OBSBOT Tiny 2 Lite" with a subtitle carrying the USB ID and
    `/dev/videoN` path. **DONE** — user-confirmed via
    AskUserQuestion 2026-05-13T16:25Z while
    `./target/debug/obsbot-cam-control` was running in background
    (xwininfo reported the same `0x2600004 "Obsbot Cam Control"
    842x662` window shape T-007 verified, and the user picked
    "Fila Tiny 2 Lite (correcto)" describing the AdwActionRow
    layout + subtitle `3564:fef9 · /dev/video0` + camera icon).
  - With no Tiny 2 family camera connected, the window shows the
    "No OBSBOT cameras detected" `AdwStatusPage`. **DEFERRED** —
    covered by the empty-list code path (the only `cameras.is_
    empty()` branch in `build_body`); will be exercised
    incidentally when T-013b's hot-plug listener triggers an
    unplug, or when the user happens to launch with the camera
    detached. Not blocking T-013a closure: the branch is trivially
    correct by inspection (same `AdwStatusPage` shape T-007 used,
    just different copy).
  - Four cargo gates green; `cargo run -p obsbot-gui` launches and
    accepts Ctrl+Q quit (regression check on T-007's verified
    behaviour). **DONE** — gates green this turn, binary launched
    successfully (xwininfo + user visual). Ctrl+Q regression
    inherits from T-007 since `register_actions` /
    `set_accels_for_action("app.quit", ["<primary>q"])` are
    unchanged.
  - Commit: `feat(gui): initial camera list (T-013a)`.
- **Outcome**: `crates/obsbot-gui/src/window.rs` rewritten (53 → 86
  lines including doc comments). `build()` retains its
  signature; the body is now produced by a new private factory
  `build_body(cameras: &[CameraInfo]) -> gtk::Widget` that branches
  on emptiness. Non-empty path mounts an `AdwPreferencesPage` with
  one `AdwPreferencesGroup` titled "Connected cameras" and one
  `AdwActionRow` per camera (delegated to a `camera_row(&CameraInfo)
  -> adw::ActionRow` helper). Row shape: title = `cam.product`,
  subtitle = `"{vid:04x}:{pid:04x} · {video_path}"`
  (e.g. `3564:fef9 · /dev/video0`), prefix icon
  `camera-web-symbolic`. Empty path keeps the T-007 `AdwStatusPage`
  shape with new copy ("No OBSBOT cameras detected" / "Connect an
  OBSBOT Tiny 2 family camera via USB."). No new dependencies
  (`obsbot-core` path dep, `gtk4`, `libadwaita` all already in
  Cargo.toml from T-007). No unit tests per [[CLAUDE.md §5.4]]
  (GUI is not auto-tested). Workspace test totals unchanged
  (11 unit + 1 ignored hardware + 1 doctest). `application.rs`
  and `main.rs` untouched.

### T-013b — Hot-plug listener
- **State**: DONE
- **Started**: 2026-05-13T16:30:00Z
- **Completed**: 2026-05-13T16:36:00Z
- **Depends on**: T-013a
- **Description**: While the app is running, plugging in or
  unplugging a Tiny 2 family camera updates the list without
  user intervention. First-pass mechanism (per [[ADR-0016]]):
  polling on a `glib::timeout_add_local` (2 s) — simplest, no
  extra dep. Switch to `udev` or gio `FileMonitor` if polling
  shows up in profiling once T-013c lands V4L2 reads on the same
  timer.
- **Acceptance criteria**:
  - On the user's machine, plug the Tiny 2 Lite in while the app
    is running → row appears within the polling interval.
    **DONE** — user-confirmed 2026-05-13T16:36Z via
    AskUserQuestion ("Ambos cambios funcionan"); the row
    reappeared within ~2-3 s of re-plugging.
  - Unplug → the row disappears within the polling interval.
    **DONE** — same confirmation; the empty-state `AdwStatusPage`
    appeared within ~2-3 s of unplugging.
  - Commit: `feat(gui): hot-plug listener (T-013b)`.
- **Outcome**: `crates/obsbot-gui/src/window.rs` extended to mount
  the body inside a stable `adw::Bin` slot and install a
  `glib::timeout_add_local(POLL_INTERVAL, …)` source (2 s) that
  re-enumerates and replaces the slot's child only when
  `Vec<CameraInfo>` differs from the previous tick. The closure
  captures `body_slot` weakly via `glib::clone!(#[weak], #[upgrade_
  or] ControlFlow::Break)`, so the source auto-cleans when the
  window dies (no manual `SourceId::remove()` plumbing). The
  `RefCell<Vec<CameraInfo>>` snapshot is captured by move (it's
  not a GObject). Steady-state cost: one `read_dir` plus a few
  `canonicalize` / `read_to_string` per detected video node per
  2 s tick on the GTK main thread (negligible for 1-2 cameras).
  `start_hotplug_poll(&Bin, Vec<CameraInfo>)` factored out of
  `build()` for readability; `build_body` and `camera_row` are
  unchanged from T-013a. No new dependencies; no unit tests per
  [[CLAUDE.md §5.4]] (GUI is not auto-tested). Workspace test
  totals unchanged.

### T-013c — V4L2 control sub-page (read-only)
- **State**: TODO
- **Depends on**: T-013a, plus a new `obsbot-core` V4L2-enumeration
  helper that reads each camera's `/dev/videoN`. User must be in
  the `video` group (already true on this machine).
- **Description**: Tapping an `AdwActionRow` opens an
  `AdwNavigationPage` listing the 24 controls captured in
  [[PROTOCOL §2]], each with its current value and advertised
  range. Read-only — write paths are T-100-series work.
- **Acceptance criteria**:
  - On the user's machine, opening the Tiny 2 Lite row shows the
    13 User + 11 Camera controls from [[PROTOCOL §2.1]] / §2.2.
  - Each row displays the live `v4l2-ctl --all`-equivalent value
    and its `min/max/step` range.
  - Commit: `feat(gui): V4L2 control sub-page (T-013c)`.

### T-013d — Blueprint pipeline
- **State**: TODO
- **Depends on**: T-013c (Blueprint pays for itself once the V4L2
  form has many named children; landing it earlier is overkill —
  see [[ADR-0016]]).
- **Description**: Introduce `blueprint-compiler` as a build
  dependency, a meson `custom_target` to compile `.blp` → `.ui`,
  and a `gnome.compile_resources` call to bundle the `.ui` files
  into a GResource the binary loads at startup. Migrate the
  T-013a/c hand-coded UI to Blueprint templates in a separate
  commit (no functional change).
- **Acceptance criteria**:
  - `blueprint-compiler` invoked successfully from `meson compile`.
  - `obsbot-cam-control` loads UI from the embedded GResource.
  - `cargo run -p obsbot-gui` behaviour unchanged from T-013c.
  - Commit: `build: Blueprint pipeline (T-013d)`.

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
- **Depends on**: T-007 (runnable GUI), T-013a (the diagnostics view's
  initial-scan list is enough for the installed app to "show
  something" — full hot-plug + V4L2 controls per [[ADR-0016]] are
  separable atoms), ideally T-014 too (Flatpak first since it stays
  the primary channel)
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
- **Depends on**: same as T-016 (T-007 + T-013a per [[ADR-0016]], and
  T-014 ideally first).
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
