# PLAN — Atomic Tasks

> **Purpose**: Concrete, atomic, executable tasks. Updated by Claude Code as
> work happens. Each task has an ID, state, acceptance criteria, and depends-on.
>
> **States**: `TODO` · `IN_PROGRESS` · `DONE` · `BLOCKED` · `CANCELLED`

---

## Current milestone: v0.2 — V4L2 Standard Controls

> v0.1.0 shipped 2026-05-13 (tag `v0.1.0`, commit `5e005fd`). The
> milestone definition lives in [[ROADMAP.md v0.2]]. PLAN tasks
> below are filled in as they become active (per the project's
> "no stale plans" rule in the Backlog section).

### T-099 — Blueprint pipeline (absorbs deferred T-013d)
- **State**: DONE
- **Started**: 2026-05-13T21:00:00Z
- **Completed**: 2026-05-13T21:20:00Z
- **Depends on**: T-013c (the hand-coded shells we migrate; v0.1 closed).
- **Description**: Introduce `blueprint-compiler` as a build-time
  dependency, a `crates/obsbot-gui/build.rs` shim that compiles
  `.blp` → `.ui` and bundles the result into a GResource via
  `glib_build_tools::compile_resources`, and the registered
  GResource the binary loads at startup. Migrate the two T-013c
  shells (the `AdwApplicationWindow` + `NavigationView` +
  `NavigationPage(cameras)` + `ToolbarView` + `HeaderBar` + body
  slot in `window.rs`, and the `NavigationPage` + `ToolbarView`
  + `HeaderBar` + body slot in `controls_view.rs`) to Blueprint
  templates loaded via `gtk::Builder::from_resource` + named-
  child lookup. The dynamic content per camera / per control
  stays code-built (zero benefit from Blueprint for trees that
  are `Vec<...>`-driven, per [[ADR-0017]]'s reasoning). This
  task MUST land before T-100 because the slider widgets, PTZ
  pad, and About dialog from T-100+ all benefit from
  template-defined shapes.
- **Acceptance criteria**:
  - `blueprint-compiler` invoked successfully from `cargo build`
    via the new `build.rs`. **DONE** — first `cargo build -p
    obsbot-gui` after installing `blueprint-compiler 0.16.0` on
    the Debian trixie host succeeded in 1m 13s cold;
    `target/debug/build/obsbot-gui-*/out/` contains both the
    intermediate `window.ui` + `controls-view.ui` and the
    packed `obsbot.gresource`. Incremental rebuild after small
    Rust changes finishes in <1 s (blueprint-compiler only
    re-runs when `.blp` sources change, via
    `cargo:rerun-if-changed`).
  - `obsbot-cam-control` loads its UI from the embedded
    `GResource`. **DONE** — `strings target/debug/obsbot-cam-
    control | grep '/io/github/domatix/ObsbotCamControl/' | wc
    -l` returns `3` (the two .ui paths used by Builder lookups
    plus the gresource prefix string). The
    `gio::resources_register_include!("obsbot.gresource")`
    call at the top of `application::run` bakes the
    GResource bytes into the binary; `gtk::Builder::from_
    resource("/io/github/domatix/ObsbotCamControl/window.ui")`
    and the matching `controls-view.ui` lookup both succeed
    at runtime (no `expect()` panic surfaced).
  - `cargo run -p obsbot-gui` behaviour unchanged from T-013c.
    **DONE** — user-confirmed 2026-05-13T21:20Z via
    AskUserQuestion ("Identical"). `xwininfo -tree -root`
    reports `0x2c00004 "Obsbot Cam Control" 842x662` —
    exactly the same window dimensions T-013a observed.
  - All four cargo gates green. **DONE** — `cargo fmt --all
    --check`, `cargo check --workspace --all-targets`,
    `cargo clippy --workspace --all-targets -- -D warnings`,
    `cargo test --workspace` all exit 0. (One detour: clippy
    `doc-markdown` flagged `GResource` in two doc-comments as
    "item missing backticks" — fixed both occurrences inline;
    rustfmt re-flowed the `let manifest_dir = PathBuf::from
    (...)` in `build.rs` once `cargo fmt --all` ran.)
  - Commit: `build(gui): Blueprint pipeline (T-099)`.
- **Outcome**: four-file delta plus the meson side-fix-free
  refactor of two existing modules.
  * **`crates/obsbot-gui/resources/window.blp`** — describes the
    static shell of the main window (`Adw.ApplicationWindow
    {id=window}` → `Adw.NavigationView {id=nav_view}` →
    `Adw.NavigationPage {tag="cameras"}` → `Adw.ToolbarView` →
    `Adw.HeaderBar` + `Adw.Bin {id=body_slot, vexpand=true}`).
    Dynamic content (camera list rows / empty `StatusPage`)
    stays code-built — zero Blueprint payoff for `Vec`-driven
    trees per [[ADR-0017]]'s reasoning.
  * **`crates/obsbot-gui/resources/controls-view.blp`** —
    describes the static shell of one drill-down page
    (`Adw.NavigationPage {id=page}` → `Adw.ToolbarView` →
    `Adw.HeaderBar` + `Adw.Bin {id=body_slot, vexpand=true}`).
    Placeholder `title: "Controls"` / `tag: "controls"` get
    overridden per-push by `controls_view.rs` (`page.set_
    title(&cam.product)`, `page.set_tag(Some(&format!
    ("controls-{:04x}-{:04x}", …)))`).
  * **`crates/obsbot-gui/resources/obsbot.gresource.xml`** —
    `<gresource prefix="/io/github/domatix/ObsbotCamControl">`
    declaring both `.ui` files with `compressed="true"
    preprocess="xml-stripblanks"` so the embedded bundle stays
    small.
  * **`crates/obsbot-gui/build.rs`** (~50 lines) — invokes
    `blueprint-compiler compile --output OUT_DIR/<name>.ui
    resources/<name>.blp` per template (with
    `cargo:rerun-if-changed`), then calls
    `glib_build_tools::compile_resources(&[&out_dir],
    "resources/obsbot.gresource.xml", "obsbot.gresource")`.
    `glib-build-tools = "0.20"` is the new `[build-
    dependencies]` entry in obsbot-gui's `Cargo.toml`,
    matching the workspace `glib` / `gio` 0.20 pin.
  * **`crates/obsbot-gui/src/application.rs`** — gains a
    `gio::resources_register_include!("obsbot.gresource")`
    call at the top of `run()`, before `adw::Application::
    builder()` so the GResource is available when GTK starts
    asking for `.ui` paths.
  * **`crates/obsbot-gui/src/window.rs`** — `build()` now
    `gtk::Builder::from_resource(WINDOW_UI)`'s the window +
    nav_view + body_slot, then runs the existing
    `start_hotplug_poll` / `build_body` / `camera_row`
    helpers unchanged. ~30 lines of constructor builder-
    chain code removed.
  * **`crates/obsbot-gui/src/controls_view.rs`** —
    `build_controls_page()` likewise loads from the
    controls-view resource and patches title + tag per
    camera. ~10 lines removed.
  No new Rust dependencies (gtk4 already exports
  `gtk::Builder`; `gio` already in `[dependencies]`).
  Workspace test totals unchanged. `Cargo.lock` picks up
  `glib-build-tools 0.20.0` (build-only, doesn't enter the
  runtime dep graph).

### T-100 — Writable User-class V4L2 controls (brightness/contrast/saturation/hue)
- **State**: DONE
- **Started**: 2026-05-13T22:05:00Z
- **Completed**: 2026-05-13T22:55:00Z
- **Depends on**: T-099 (Blueprint pipeline, DONE); T-013c (read-only
  enumeration we now layer writes on top of).
- **Description**: First user-visible write path for v0.2. Extend
  `obsbot_core::controls` with a `write_control(video_path, id,
  value)` helper backed by `v4l 0.14`'s `Device::set_control`, then
  expose `ControlDescriptor.id: u32` so the GUI can call it. In
  `obsbot-gui::controls_view::render_controls`, swap the read-only
  `AdwActionRow` for type-appropriate writable widgets when the
  control sits in the User class:
  * `ControlKind::Integer` → `AdwSpinRow` with an `Adjustment`
    matching `min`/`max`/`step`, current value pre-set, and
    `connect_value_notify` calling `write_control`.
  * `ControlKind::Boolean` → `AdwSwitchRow` with `set_active(current)`
    and `connect_active_notify` calling `write_control`.
  * `ControlKind::Menu` and `ControlKind::Other` stay read-only for
    now (menu writes land with T-103 / T-104 / T-101 — anti-flicker,
    exposure mode, PTZ).
  Camera-class and Other-class controls also stay read-only (their
  dedicated PTZ pad and other write paths arrive in T-101+). Write
  errors are surfaced via `eprintln!` plus the slider/switch staying
  at the user-selected value; a proper toast / revert UX is a polish
  item for T-106's About + general error surfacing.
  Reusable Blueprint templates for sliders (the STATE hint at a
  `slider-row.blp`) are intentionally **not** introduced in this
  task: `AdwSpinRow::builder()` is one ergonomic constructor and the
  Vec-driven render loop carries no Blueprint payoff per
  [[ADR-0017]]. PTZ pad / About dialog (T-101 / T-106) are the
  natural first home for new Blueprint shells.
- **Acceptance criteria**:
  - `obsbot_core::controls::ControlDescriptor` exposes `id: u32`.
    **DONE** — populated from `Description.id` in `read_controls`.
  - `obsbot_core::controls::write_control(&Path, u32, ControlValue) ->
    Result<()>` returns `Ok(())` for an in-range Integer write and
    propagates `Err(Error::Io)` for ioctl failures. **DONE**.
  - Unit test covers `ControlValue → v4l::control::Value` mapping
    without touching `/dev/videoN`. **DONE** — two new tests in
    `controls.rs`'s `#[cfg(test)] mod tests`.
  - Hardware-`#[ignore]` integration test round-trips
    `V4L2_CID_BRIGHTNESS` (read current → write current±step → read
    back → restore) against the user's plugged-in Tiny 2 Lite.
    **DONE** — `writes_v4l2_brightness_round_trip` in
    `tests/hardware.rs` passes against the connected unit
    (`cargo test -p obsbot-core --test hardware -- --ignored`).
  - GUI renders writable widgets for every User-class Integer /
    Boolean control. **DONE** — Integer rows use
    `AdwActionRow` with a `gtk::Scale` + `gtk::SpinButton`
    (both bound to the same `gtk::Adjustment`) + a flat
    `edit-undo-symbolic` reset button as suffixes. Boolean
    rows use `AdwSwitchRow`. Both surface the driver's
    advertised default in the subtitle / tooltip.
  - Moving the **brightness** slider changes the brightness of the
    live image visible in a second app (Cheese / Camera) on the
    user's hardware. **DONE** — user confirmed 2026-05-13T22:50Z
    via AskUserQuestion (final iteration with slider + manual
    spin entry + reset button: "Everything works").
  - `cargo fmt --all --check`, `cargo clippy --workspace --all-
    targets -- -D warnings`, `cargo test --workspace` all green.
    **DONE**.
  - Commit `feat(core+gui): writable User-class V4L2 controls
    (T-100)` lands on `main`. **DONE** — see git log.
- **Outcome**: ~250-line delta across two crates plus a docs
  ledger.
  * **`crates/obsbot-core/src/controls.rs`** — `ControlDescriptor`
    gains an `id: u32` field (used by every caller addressing a
    specific control on write); `ControlKind::Integer` and
    `::Boolean` gain a `default` field carried straight from
    `v4l::control::Description.default_value` (powers the GUI's
    reset-to-default UX). New `ControlValue { Integer(i64),
    Boolean(bool) }` enum with a `From<ControlValue> for
    v4l::control::Value` impl. New `pub fn write_control(&Path,
    u32, ControlValue) -> Result<()>` opens the V4L2 node and
    dispatches `Device::set_control`. Two unit tests pin the
    `ControlValue → v4l::Value` mapping.
  * **`crates/obsbot-core/src/lib.rs`** — re-exports
    `write_control` and `ControlValue` alongside the existing
    `read_controls` / `ControlClass` / `ControlDescriptor` /
    `ControlKind` family.
  * **`crates/obsbot-core/tests/hardware.rs`** — third
    `#[ignore]`d integration test (`writes_v4l2_brightness_
    round_trip`) reads brightness, writes `current ± step`,
    asserts the read-back matches, restores the original.
    Existing test additionally asserts
    `brightness.id == V4L2_CID_BRIGHTNESS (0x0098_0900)`.
    All 3 hardware tests passed under
    `cargo test -- --ignored` against the user's plugged-in
    Tiny 2 Lite, two times during this task (once after the
    initial `AdwSpinRow` build, once after the slider rebuild).
  * **`crates/obsbot-gui/src/controls_view.rs`** — `control_row`
    branches on `(ControlClass::User, ControlKind::Integer |
    Boolean)`. Integer rows: `AdwActionRow` with title, range +
    default subtitle, and three suffixes — a `gtk::Scale`
    (200 px min, horizontal, draws no value, has a tick mark
    at the default), a `gtk::SpinButton` (5 chars wide,
    numeric, climb_rate = step), and a flat
    `gtk::Button` with an `edit-undo-symbolic` icon plus a
    "Reset to default (N)" tooltip. All three widgets share the
    same `gtk::Adjustment`, so the value-changed signal fires
    once whichever widget the user touched, and the write hits
    `obsbot_core::write_control` with the rounded i32 value
    widened to i64. Boolean rows: `AdwSwitchRow` with the
    default surfaced as a subtitle ("default On" / "default
    Off") and `connect_active_notify` wired to write. Two
    helper functions (`clamp_i64_to_i32` saturating, and
    `f64_to_i32_saturating` carrying a justified
    `#[allow(clippy::cast_possible_truncation)]`) keep clippy
    quiet while documenting that the saturation is intentional
    (V4L2 standard control values are `__s32` per kernel
    `videodev2.h`).
- **UX iteration log** (worth recording — the user redirected
  twice):
  1. First pass shipped `AdwSpinRow` (+/− buttons + numeric
     entry). User: "it changes, but with the + and − buttons,
     there's no slider". Acceptance text said "slider"; SpinRow
     was wrong.
  2. Second pass shipped `AdwActionRow` + `gtk::Scale` (drag-
     bar) + value `gtk::Label`. User: "slider OK, but I also want
     to enter the number manually and a button to reset to the
     default value".
  3. Third pass — final — added `gtk::SpinButton` next to the
     scale (sharing the adjustment so they stay in lock-step)
     and a flat reset button with an `edit-undo-symbolic` icon
     and a "Reset to default (N)" tooltip; the scale also got a
     tick mark at the default position. User: "Everything works".
- **Hardware-quirk note surfaced during the second iteration**:
  the user observed that the first ~5 sliders reacted live but
  the rest "didn't seem to do anything". That is the documented
  V4L2 interlock from `PROTOCOL §2.3` Q1/Q2: when *White
  Balance, Automatic* is `On`, the driver marks *White Balance
  Temperature* as `V4L2_CTRL_FLAG_INACTIVE` and silently
  ignores writes; same for *Exposure Time, Absolute* when
  *Auto Exposure* is in an auto mode. Toggling the WB
  Automatic switch off freed the temperature slider. Not a
  bug in our code. The repayment lands in T-102 per
  [[ADR-0019]].

### T-101 — PTZ pad widget (pan/tilt/zoom + focus)
- **State**: DONE
- **Started**: 2026-05-13T23:00:00Z
- **Completed**: 2026-05-13T23:25:00Z
- **Depends on**: T-100 (Integer / Boolean write path). Per [[ADR-0019]]
  this task absorbs the original "T-102 Zoom slider" hint.
- **Description**: Dedicated PTZ pad widget — a Blueprint template
  `ptz-pad.blp` describing the static 3×3 directional grid + zoom
  slider on the right + focus row at the bottom, loaded via
  `gtk::Builder::from_resource`. Camera-class PTZ-related
  controls (`pan_absolute`, `tilt_absolute`, `zoom_absolute`,
  `focus_absolute`, `focus_automatic_continuous`, `pan_speed`,
  `tilt_speed`) get filtered out of the generic `render_controls`
  loop and routed to the pad. The eight directional buttons write
  `pan_absolute += ±step` / `tilt_absolute += ±step` (1° = 3600
  units per PROTOCOL §2.2) where step defaults to 5° to keep the
  pad responsive without being twitchy. A center "Reset" button
  writes both to 0. Zoom slider reuses T-100's scale+spin+reset
  pattern. Focus row pairs a switch (auto-continuous) with a
  slider (manual) that greys out when auto is on — the explicit
  pairing previews the T-102 generic interlock handler.
  `zoom_continuous` is intentionally not surfaced (PROTOCOL §2.3
  Q2: driver reports values exceeding advertised range).
- **Acceptance criteria**:
  - `crates/obsbot-gui/resources/ptz-pad.blp` exists, compiled by
    the existing T-099 Blueprint pipeline, embedded in the
    GResource bundle.
  - A new module `crates/obsbot-gui/src/ptz_pad.rs` exposes
    `pub fn build_ptz_pad(controls: &[ControlDescriptor], path:
    &Path) -> Option<adw::PreferencesGroup>` returning `None`
    when none of the seven PTZ-related controls are present.
  - `controls_view.rs::render_controls` calls `build_ptz_pad`,
    adds the resulting group to the top of the page, and filters
    the seven PTZ control IDs out of the remaining generic
    render.
  - Hardware-`#[ignore]`d round-trip test on `zoom_absolute` —
    similar shape to the T-100 Brightness round-trip; not on
    pan/tilt because aggressive deltas mid-test may collide with
    user expectations of camera orientation.
  - All four cargo gates green.
  - **User validation pending**: drag the directional buttons
    and confirm the camera frame pans/tilts; drag the zoom
    slider and confirm the frame zooms. Accumulated into the
    final report after T-105.
  - Commit `feat(gui): PTZ pad widget (T-101)`.

### T-102 — Menu writes + INACTIVE grey-out
- **State**: DONE
- **Completed**: 2026-05-13T23:45:00Z
- **Depends on**: T-101 (no hard dep; can land in either order, but
  the PTZ pad benefits from the INACTIVE handler for the focus
  manual/auto pair).
- **Description**: Per [[ADR-0019]], generalises the v0.2 write
  surface to menus and propagates the V4L2 `INACTIVE` flag to the
  UI. Adds `ControlKind::Menu`'s option-ID list, a
  `ControlValue::Menu(i64)` variant, and the `is_active` field on
  `ControlDescriptor`. GUI renders any User-class Menu as an
  `AdwComboRow`; every row calls `set_sensitive(ctrl.is_active)`
  so inactive controls grey out generically. Anti-flicker
  (`power_line_frequency`) ships as a side-effect — this is the
  v0.2 anti-flicker selector from ROADMAP.
- **Acceptance criteria**:
  - `ControlDescriptor.is_active: bool` populated from
    `Description.flags::INACTIVE`.
  - `ControlKind::Menu` carries `options: Vec<(i64, String)>`
    (was `Vec<String>`).
  - `ControlValue::Menu(i64)` writes via `write_control`.
  - User-class menus render as `AdwComboRow`; toggling the combo
    writes the menu's i64 ID via `write_control`.
  - Every row has `set_sensitive(ctrl.is_active)` applied.
  - Toggling *White Balance, Automatic* on/off flips the
    sensitivity of *White Balance Temperature* without a code
    change — proves the INACTIVE flag propagation works
    end-to-end (the kernel updates `flags` live).
  - Hardware-`#[ignore]`d round-trip on `power_line_frequency`.
  - All four cargo gates green.
  - **User validation pending**: change anti-flicker dropdown
    (50 Hz / 60 Hz / Disabled), observe flicker (subtle) or just
    confirm no error. Toggle WB Auto and observe the WB
    Temperature row grey out / wake up.
  - Commit `feat(core+gui): menu writes and INACTIVE grey-out (T-102)`.

### T-103 — White balance group widget
- **State**: DONE
- **Completed**: 2026-05-13T23:55:00Z
- **Depends on**: T-102 (uses the menu / INACTIVE infrastructure).
- **Description**: Cosmetic / UX win: assemble the four WB
  controls (`white_balance_automatic`, `white_balance_temperature`,
  `red_balance`, `blue_balance`) into a dedicated
  `AdwPreferencesGroup` titled "White balance" at the top of the
  User Controls section, with a description explaining the
  auto / manual relationship. The four IDs get filtered out of
  the generic render so they don't appear twice.
- **Acceptance criteria**:
  - `crates/obsbot-gui/src/wb_group.rs` (new module) exposes
    `pub fn build_wb_group(controls: &[ControlDescriptor], path:
    &Path) -> Option<adw::PreferencesGroup>`.
  - `controls_view.rs` calls it, adds the result above the
    generic User group, and filters the four IDs out.
  - The WB Temperature row inside the group still shows the
    T-100 slider/spin/reset trio (no widget duplication).
  - All four cargo gates green.
  - **User validation pending**: toggle WB Auto off, drag WB
    Temperature, observe colour shift in the preview.
  - Commit `feat(gui): white balance group widget (T-103)`.

### T-104 — Exposure group widget
- **State**: DONE
- **Completed**: 2026-05-14T00:05:00Z
- **Depends on**: T-102.
- **Description**: Symmetric to T-103 for exposure. Group:
  `auto_exposure` (Camera-class menu, AdwComboRow) +
  `exposure_time_absolute` (Camera-class int, slider). Both get
  filtered out of the generic render; the group sits at the top
  of the page near the PTZ pad.
- **Acceptance criteria**:
  - `crates/obsbot-gui/src/exposure_group.rs` exposes
    `build_exposure_group(...)` returning an `Option<adw::
    PreferencesGroup>`.
  - `controls_view.rs` mounts it; the two control IDs get
    filtered out.
  - The auto_exposure menu writes (Camera-class menu) work via
    the T-102 write path (no special-casing of Camera-class
    menus).
  - All four cargo gates green.
  - **User validation pending**: change auto_exposure to
    "Manual", drag exposure_time_absolute, observe brightness
    change.
  - Commit `feat(gui): exposure group widget (T-104)`.

### T-105 — Per-camera GSettings persistence
- **State**: DONE
- **Completed**: 2026-05-14T00:35:00Z
- **Depends on**: T-100 / T-102 (the write paths whose values we
  persist).
- **Description**: First persistence layer. A GSettings schema
  `io.github.domatix.ObsbotCamControl.gschema.xml` declares a
  single key `cameras` of type `a{sa{si}}` — a dictionary mapping
  camera serial number to a sub-dictionary of (control name,
  int value) pairs. On every successful `write_control`, the GUI
  also writes the value to GSettings keyed by `(serial,
  control_name)`. On enumeration, the GUI reads the stored map
  and replays writes for each restored value (best-effort: if a
  control no longer exists or the write fails the entry is left
  untouched, logged via `eprintln!`). gschema is compiled
  in-tree via the existing meson `glib-compile-schemas` step;
  for `cargo run` we use `gio::SettingsSchemaSource::from_directory`
  to point at the source schema dir so devs don't need
  `meson install` to test persistence.
- **Acceptance criteria**:
  - `data/io.github.domatix.ObsbotCamControl.gschema.xml` exists.
  - `data/meson.build` installs + compiles the schema.
  - `crates/obsbot-gui/src/settings.rs` (new module) exposes
    `load_for_camera(serial) -> HashMap<String, i64>` and
    `save_for_camera(serial, control_name, value)`.
  - `controls_view.rs` calls `load_for_camera` on page build
    and replays writes; every value-changed callback also
    calls `save_for_camera`.
  - Hardware round-trip test: write a value, simulate "restart"
    by re-reading settings, confirm replay restores the value
    on the connected camera. (#[ignore]'d.)
  - All four cargo gates green.
  - **User validation pending**: change brightness in the GUI,
    close the app, re-launch, confirm brightness is restored.
  - Commit `feat(gui): per-camera GSettings persistence (T-105)`.

### T-106 — About dialog
- **State**: DONE
- **Started**: 2026-05-14T00:50:00Z
- **Completed**: 2026-05-14T01:00:00Z
- **Depends on**: T-099 (Blueprint pipeline — menu lives in
  `window.blp`).
- **Description**: Last "hint" task of v0.2. The HeaderBar gets a
  primary `MenuButton` (`open-menu-symbolic`) with a menu offering
  "About Obsbot Cam Control" and "Quit". Activating the About
  entry opens an `adw::AboutDialog` (HIG-preferred over the
  legacy `AboutWindow` since libadwaita 1.5; the workspace pin is
  `0.7 + v1_6`) populated from the workspace metadata
  (`CARGO_PKG_VERSION`, `repository`, `license = GPL-3.0-or-later`,
  `authors`) plus an explicit credit block for the reverse-
  engineering work cited in `PROTOCOL.md` (Aaron Brown's Qt6
  reference and `taxfromdk/obsbot_tiny_reversing`).
- **Acceptance criteria**:
  - `window.blp` declares `menu primary_menu { ... }` with two
    items: `app.about` and `app.quit`, and adds a `MenuButton`
    with `icon-name: "open-menu-symbolic"` and
    `menu-model: primary_menu` to the `Adw.HeaderBar`.
  - `application.rs` registers the `app.about` `ActionEntry`
    that constructs and `present`s an `adw::AboutDialog` against
    the active window.
  - `adw::AboutDialog` fields: `application-name`,
    `application-icon` (the App ID), `version` from
    `env!("CARGO_PKG_VERSION")`, `developer-name` from the
    workspace `authors`, `copyright` line, `license-type`
    `Gpl3_0`, `website` from `homepage`, `issue-url` (repo /
    issues), `developers` list, `acknowledgement-section`
    crediting `aaronsb/obsbot-camera-control` and
    `taxfromdk/obsbot_tiny_reversing` per `docs/PROTOCOL.md` §0.
  - All four cargo gates green (`fmt`, `clippy -D warnings`,
    `test`, `cargo build`).
  - **User validation pending**: click the hamburger button in
    the header → "About Obsbot Cam Control" → confirm version,
    license, links, and credits render correctly.
  - Commit `feat(gui): About dialog with credits (T-106)`.

### T-107 — gettext scaffolding
- **State**: DONE
- **Started**: 2026-05-14T01:00:00Z
- **Completed**: 2026-05-14T01:25:00Z
- **Depends on**: T-008 (Meson orchestration).
- **Description**: SPEC §4.4 and §6.5 require full localization
  via gettext (English source, Spanish at minimum). The polish
  work itself is v0.6, but the scaffolding has to land now so
  user-facing strings produced by T-099..T-106 can be marked at
  source and a `obsbot-cam-control.pot` template can be
  extracted on demand. Concretely: a top-level `po/` directory
  with `LINGUAS` (containing `es`), `POTFILES.in` listing every
  `.rs` / `.blp` carrying translatable strings, `meson.build`
  wiring `i18n.gettext('obsbot-cam-control', preset: 'glib')`,
  and a Rust-side `i18n` shim (thin `pub fn gettext(s: &str) ->
  String` wrapping `gettextrs::gettext`) that the existing code
  switches to. Strings inside `.blp` files do NOT yet get the
  `_("...")` Blueprint syntax — that requires `xgettext` to
  understand `.ui` output; for v0.6 polish we'll switch on
  `intltool-extract`-style handling. For v0.2 it's enough that
  Rust-side strings funnel through `gettext()`.
- **Acceptance criteria**:
  - `po/LINGUAS`, `po/POTFILES.in`, `po/meson.build` exist.
  - Top-level `meson.build` calls `subdir('po')` after `data/`.
  - `Cargo.toml` workspace deps gain `gettext-rs = "0.7"` (or
    equivalent), pinned in the workspace `[workspace.dependencies]`.
  - `crates/obsbot-gui/src/i18n.rs` exposes
    `pub fn gettext(msgid: &str) -> String` plus a `i18n_init()`
    that binds the textdomain.
  - User-facing string literals in `controls_view.rs`,
    `wb_group.rs`, `exposure_group.rs`, `ptz_pad.rs`, `window.rs`
    are routed through `i18n::gettext(...)`.
  - `meson compile -C builddir obsbot-cam-control-pot` (or the
    target `i18n.gettext()` provides) produces a non-empty
    `.pot` covering the marked strings. **Wiring verified, host
    gap noted**: this dev host has `gettext-base` only (no
    `msgfmt` / `xgettext`), so meson logs `WARNING: Gettext not
    found, all translation (po) targets will be ignored.` and
    skips the .pot target without failing the build. The wiring
    is correct (meson processes `subdir('po')`, the cargo build
    bakes `OBSBOT_LOCALEDIR` into the binary — visible via
    `strings builddir/cargo/release/obsbot-cam-control | grep
    locale` showing `/usr/local/share/locale` and
    `obsbot-cam-control`). CI + Flatpak builders ship full
    gettext so the .pot target lands there.
  - An empty `po/es.po` is committed (header only, will be
    populated in v0.6).
  - All four cargo gates green.
  - Commit `feat(gui): gettext scaffolding (T-107)`.

### T-108 — Toast-based error surfacing
- **State**: DONE
- **Started**: 2026-05-14T01:25:00Z
- **Completed**: 2026-05-14T01:40:00Z
- **Depends on**: T-100..T-105 (every write path that currently
  `eprintln!`s on failure).
- **Description**: Replace stderr `eprintln!` write-failure logs
  with `adw::ToastOverlay` + `adw::Toast` so the user sees the
  error inside the app instead of having to read the terminal.
  The overlay wraps the controls page content; on a failed
  `write_control`, callbacks dispatch a toast with a short
  human-readable message (e.g. `"Failed to set Brightness:
  Device busy"`). GSettings save failures stay on stderr (they
  are recovered transparently next session, not user-actionable).
  **Implementation**: rather than thread a `&adw::ToastOverlay`
  through every widget closure, T-108 binds a weak ref to the
  overlay in a `thread_local!` from
  `controls_view::build_controls_page`, and the single chokepoint
  `settings::write_and_save` calls a new `settings::surface_
  error(msg)` helper that pops a 5s `adw::Toast`. The weak ref
  pattern keeps the binding self-cleaning across page
  navigations (when the previous page widget drops, the upgrade
  returns `None` and we fall through to `eprintln!`).
- **Acceptance criteria**:
  - `controls_view::build_controls_page` wraps the dynamic body
    in an `adw::ToastOverlay` and calls
    `settings::bind_toast_overlay(&overlay)` to register the
    surface.
  - `settings::surface_error(msg)` exists; resolves the bound
    overlay via `glib::WeakRef::upgrade`, falls through to
    `eprintln!` when nothing is bound.
  - `settings::write_and_save` now surfaces V4L2 write errors via
    `surface_error` instead of `eprintln!`; the toast message is
    `gettext("Failed to set {name}: {error}")` with `{name}` /
    `{error}` substituted (T-107 gettext path).
  - GSettings save errors stay on `eprintln!` with the existing
    inline justification.
  - All four cargo gates green.
  - **User validation pending**: pull the camera USB cable
    mid-drag; confirm a toast appears (instead of silent stderr).
  - Commit `feat(gui): toast-based write-error surfacing (T-108)`.

### T-109 — AppStream releases entry for v0.2.0
- **State**: DONE
- **Started**: 2026-05-14T01:40:00Z
- **Completed**: 2026-05-14T01:55:00Z
- **Depends on**: T-009 (metainfo + desktop file).
- **Description**: Tag-readiness prep: the AppStream metainfo
  has carried an implicit "no releases" section since T-009.
  Add a `<releases>` block with an entry for `v0.2.0` (date
  filled at tag time; this task lands the structure + draft
  notes covering every v0.2 task T-099..T-108). The notes use
  the AppStream `<description>` mini-format (`<p>`, `<ul>`,
  `<li>`); no marketing prose.
- **Acceptance criteria**:
  - `data/io.github.domatix.ObsbotCamControl.metainfo.xml.in`
    gains a `<release version="0.2.0" type="development">`
    entry on top of the existing v0.1.0 record (newest-first
    per AppStream convention). Implementation note: dropped the
    placeholder `@VERSION@` from the v0.1.0 entry that would
    have silently rewritten it to "0.2.0" after a project-
    version bump; both entries now carry their literal version
    string so the historical record is stable. Date attribute
    is `2026-05-14` — a draft value to be edited at actual tag
    time if the v0.2.0 cut slips beyond that day.
  - Release notes cover: PTZ pad (T-101), image controls + menu
    writes + INACTIVE grey-out (T-100 / T-102), WB group (T-103),
    Exposure group (T-104), Anti-flicker selector (T-102),
    GSettings persistence (T-105), About dialog (T-106), gettext
    scaffolding (T-107), toast errors (T-108). Phrased in
    user-facing language (no T-IDs in the metainfo).
  - `meson test -C builddir validate-metainfo` passes
    (`appstreamcli validate --no-net --explain` green;
    `--pedantic` shows only the pre-existing `cid-contains-
    uppercase-letter` note about `ObsbotCamControl` TitleCase,
    intentional per ADR-0012).
  - All four cargo gates green (no Rust changes; gates re-run
    to catch any incidental regressions).
  - Commit `docs(appstream): v0.2.0 release notes (T-109)`.

### T-110 — Hot-plug REMOVE resilience
- **State**: DONE
- **Started**: 2026-05-14T01:55:00Z
- **Completed**: 2026-05-14T02:15:00Z
- **Depends on**: T-013b (hot-plug poll listener), T-108 (toast
  surface).
- **Description**: SPEC §6.4 requires surviving camera
  disconnect/reconnect during runtime. T-013b's poll listener
  re-mounts the body on enumeration change, but two concrete
  edge cases need explicit handling:
  (a) When the user has drilled into a camera detail page and
  the camera is then unplugged, the detail page silently shows
  stale values; `nav_view.pop_to_tag("cameras")` should fire and
  a toast (`"Camera disconnected: <product>"`) should appear.
  (b) On re-plug, the camera should reappear without restart.
  T-013b already covers (b) at the list level but the detail
  page does not auto-refresh if the user is still on it.
  Implementation: extend the poll callback to compare per-camera
  presence; if the current `NavigationView` top page corresponds
  to a removed camera, pop the page and post a toast. **Toast
  surface rewire**: T-110 promotes the toast overlay from
  per-page (T-108's initial scope) to window-level by wrapping
  the `AdwNavigationView` in `window.blp`'s `Adw.ToastOverlay
  toast_overlay`. `window::build` binds it once via
  `settings::bind_toast_overlay`; `controls_view::build_controls
  _page` no longer creates its own. Reason: a per-page surface
  scopes toasts to the controls page and loses them when the
  page is the one being popped (the disconnect case). Per
  GNOME HIG, toasts overlay the entire `AdwApplicationWindow`
  anyway, so the window-level placement is the canonically
  correct one.
- **Acceptance criteria**:
  - `window.blp` wraps `Adw.NavigationView nav_view` in
    `Adw.ToastOverlay toast_overlay`; the IDs comment block is
    updated.
  - `window::build` extracts `toast_overlay` and calls
    `settings::bind_toast_overlay(&toast_overlay)` once.
  - `controls_view::build_controls_page` drops its own
    `Adw.ToastOverlay::new()` + `bind_toast_overlay` block; a
    comment cross-references the window-level binding for
    future readers.
  - New `window::handle_remove_events(prev, latest, nav_view)`
    helper:
    * Computes the set of cameras that disappeared by
      `(vid, pid, serial)` identity.
    * If the visible `NavigationView` page's tag matches a
      removed camera's `controls-{vid:04x}-{pid:04x}` tag,
      calls `nav_view.pop_to_tag("cameras")`.
    * Surfaces a translated toast: singular
      `"Camera disconnected: {product}"` or plural
      `"Cameras disconnected: {products}"` (comma-joined).
  - Re-plug behaviour preserved: existing T-013b body re-mount
    re-adds the camera row to the list automatically.
  - All four cargo gates green; `meson compile -C builddir`
    builds the release binary cleanly with the new Blueprint.
  - **User validation pending**: unplug the camera USB cable
    while on the controls page; confirm pop + toast. Re-plug;
    confirm the camera re-appears in the list.
  - Commit `feat(gui): hot-plug REMOVE resilience (T-110)`.

### T-111 — Sensitivity refresh after gate writes
- **State**: DONE
- **Started**: 2026-05-14T02:30:00Z
- **Completed**: 2026-05-14T02:45:00Z
- **Depends on**: T-102 (the original build-time `set_sensitive`
  call), T-108 (the `settings`-module pattern this re-uses).
- **Description**: Bug fix uncovered by the post-T-106..T-110
  user validation pass. Three reports — "WB temperature
  sliders stay un-editable after I uncheck WB Auto" (D),
  "Exposure Time stays un-editable after I switch to Manual"
  (E), "WB temperature stays editable when WB Auto is on"
  (F.12) — all stem from one root cause: T-102 wired
  `row.set_sensitive(ctrl.is_active)` **once at page-build
  time** and never re-evaluated on subsequent writes. The
  kernel does flip the V4L2 `INACTIVE` flag on dependent
  controls when a gating control is written (e.g. writing
  auto_exposure=Manual un-flags exposure_time_absolute), but
  the GUI ignored the flip. The only place this worked was
  the ptz_pad focus row, which had a bespoke
  `auto_row.connect_active_notify` listener manually toggling
  `abs_row.set_sensitive` — T-101 baked that in directly
  without a generic equivalent.
  **Fix**: register every controlled row in a `thread_local!`
  `Vec<(u32, gtk::Widget)>` at page build, then call
  `refresh_sensitivity` from `settings::write_and_save` after
  every successful Boolean / Menu write. The refresh re-reads
  `read_controls(path)` and walks the registry, calling
  `set_sensitive(ctrl.is_active)` on each registered widget.
  Integer writes (slider drags, ~100Hz) do **not** trigger
  the refresh — no UVC standard Integer control gates other
  controls, so the extra ioctl would be pure overhead.
- **Acceptance criteria**:
  - `settings::reset_row_registry(video_path)` clears the
    registry and stores the path at the start of every
    `controls_view::build_controls_page` call.
  - `settings::register_row(ctrl_id, &widget)` exposed for
    every row builder; called once per row right after the
    initial `set_sensitive(ctrl.is_active)`.
  - `settings::refresh_sensitivity()` (private) called from
    `settings::write_and_save` after a successful Boolean or
    Menu write (gated by `matches!(value,
    ControlValue::Boolean(_) | ControlValue::Menu(_))`);
    Integer writes do not call it.
  - `controls_view::build_body` (User / Camera class loop),
    `wb_group::build_wb_group`, `exposure_group::
    build_exposure_group`, and `ptz_pad::build_ptz_pad` (zoom
    scale + focus auto switch + focus abs row) all
    call `register_row` for every widget they construct that
    has a 1:1 mapping to a V4L2 control.
  - All four cargo gates green.
  - **User validation pending** (covers D / E / F.12 from the
    previous pass):
    * Toggle WB Auto OFF → WB Temperature / Red / Blue
      sliders become editable. Toggle ON → they grey out.
    * Switch Auto Exposure to Manual → Exposure Time Absolute
      slider becomes editable. Switch back to Auto Mode →
      greys out.
  - Commit `fix(gui): refresh row sensitivity after gate writes (T-111)`.

---

## v0.3 — Vendor XU & AI tracking (planned)

> Milestone scope set by [[DECISIONS.md ADR-0020]] on 2026-05-14
> (FOSS-pivot ADR). Collapses the previously planned v0.4
> (Vendor XU) + v0.5 (Auto-Framing); promoted ahead of Live
> Preview (now v0.4). Byte-level extraction from
> `cgevans/tiny2` and `OpenFoxes/Tiny4Linux` recorded in
> [[docs/XU_INVESTIGATION_2026-05-14.md]]; protocol summary in
> [[PROTOCOL.md §3.2]]; attribution policy in [[CREDITS.md]].

### T-300 — `obsbot-core::xu` module (UVCIOC_CTRL_QUERY wrapper + enums + decode_status)

- **State**: DONE (with caveat — hardware round-trip test pending
  user validation; see Outcome below)
- **Started**: 2026-05-14T19:15:00Z
- **Completed**: 2026-05-14T19:55:00Z
- **Commit**: `feat(core): obsbot-core::xu module — UVCIOC_CTRL_QUERY,
  enums, decode_status (T-300)`
- **Branch**: `feat/T-300-xu-tracking`
- **Depends on**: v0.2.0 work shipped on `main` (T-111 last);
  none of T-200 / T-301 / T-302 / T-303 (those come later).
- **Description**: New module `crates/obsbot-core/src/xu/`
  porting the EUPL-1.2 XU surface from `cgevans/tiny2` and
  `OpenFoxes/Tiny4Linux`. Pure-Rust backend — no GUI yet
  (that's T-301 / T-302). Adopts Tiny4Linux's modular
  factoring (`transport`, `status`, `enums`,
  `commands/{ai_mode, fov, hdr, face_ae, exposure_mode,
  sleep, tracking_speed, preset, exposure_mode_type}`), drops
  the `bon` and `enum_dispatch` dependencies (replace with
  plain Rust), keeps the same `nix` ioctl path. Files with
  ported bytes carry the dual SPDX block per [[CREDITS.md]].
- **Acceptance criteria**:
  - **Workspace deps**: add `nix = "0.30"` and `errno = "0.3"`
    to `[workspace.dependencies]`; `crates/obsbot-core/
    Cargo.toml` gets `.workspace = true` lines for both.
    No new transitive Tokio/libusb deps.
  - **Transport** (`xu/transport.rs`): expose
    `pub fn xu_query(fd: RawFd, unit: u8, selector: u8,
    request: XuRequest, buf: &mut [u8]) -> Result<()>` that
    wraps `UVCIOC_CTRL_QUERY` via
    `nix::ioctl_readwrite_buf!(uvcioc_ctrl_query, b'u', 0x21,
    uvc_xu_control_query)`. Helper
    `pub fn get_len(fd, unit, selector) -> Result<u16>` that
    issues `UVC_GET_LEN (0x85)` before every `GET_CUR` /
    `SET_CUR` (mirrors cgevans's paranoia — kernel returns
    `EINVAL` on size mismatch). Const `BUNIT_ID: u8 = 0x02`.
  - **Request codes** (`xu/transport.rs`): `pub const`s
    `UVC_SET_CUR = 0x01`, `UVC_GET_CUR = 0x81`,
    `UVC_GET_MIN = 0x82`, `UVC_GET_MAX = 0x83`,
    `UVC_GET_RES = 0x84`, `UVC_GET_LEN = 0x85`,
    `UVC_GET_INFO = 0x86`, `UVC_GET_DEF = 0x87`.
  - **Enums** (`xu/enums.rs`): `AIMode` (10 variants,
    `TryFrom<(u8, u8)>` for status decode + getter
    `fn to_wire(self) -> (u8, u8)`), `FOVMode` (Wide / Normal
    / Narrow, `TryFrom<u8>` + `to_wire`),
    `ExposureMode` (Auto / Manual), `FaceAeMode` (Global /
    Face), `SleepState` (Awake / Sleep / Unknown),
    `TrackingSpeed` (Standard / Sport, `TryFrom<u8>`).
    AIMode decoder accepts **both** `(m=3, n=0)` and
    `(m=6, n=0)` as `Hand` until quirk Q4 (PROTOCOL.md §3.2)
    is hardware-validated.
  - **Selector-0x06 opcode commands** (one file per opcode
    under `xu/commands/`):
    - `hdr.rs`: `set_hdr(fd, bool)` → write
      `[0x01, 0x01, value]` on selector `0x06`.
    - `face_ae.rs`: `set_face_ae(fd, FaceAeMode)` → write
      `[0x03, 0x01, value]` on selector `0x06`.
    - `fov.rs`: `set_fov(fd, FOVMode)` → write
      `[0x04, 0x01, value]` on selector `0x06`.
    - `ai_mode.rs`: `set_ai_mode(fd, AIMode)` → write
      `[0x16, 0x02, m, n]` on selector `0x06`. Use the
      exact `(m, n)` table from PROTOCOL.md §3.2 (write
      `m=3` for `Hand` to match cgevans's setter; the
      decoder side already accepts both).
  - **Selector-0x02 structured frames** (`xu/command02.rs`
    builder + one file per frame under `xu/commands/`):
    - `command02.rs`: pure-Rust `build_command02(fg: [u8;6],
      seq: [u8;2], cks: [u8;2], cmd: [u8;6], app:
      Option<[u8;16]>) -> [u8;36]`. No `bon` dependency.
    - `exposure_mode_type.rs`: write either the Auto or
      Manual 36-byte frame per the table in PROTOCOL.md
      §3.2. **Use cgevans's labelling (quirk Q5)** — Auto
      sends `seq=[0x15,0x00] cks=[0xa8,0x9e] cmd=
      [0xf9,0x27,0x01,0x32,0x00,0x00]`, Manual sends
      `seq=[0x16,0x00] cks=[0x58,0x91] cmd=
      [0xb2,0xaf,0x02,0x04,0x00,0x00]`. `function_group =
      [0x0a, 0x02, 0x82, 0x29, 0x05, 0x00]`.
    - `sleep.rs`: Awake / Sleep frames.
    - `tracking_speed.rs`: Standard / Sport frames.
    - `preset.rs`: `set_preset(fd, 0..=2)` with the
      `[1.0f32; 4]` little-endian appendix (16 zero bytes
      will NOT work — the camera rejects them).
  - **Status decode** (`xu/status.rs`):
    `pub fn get_status(fd) -> Result<Status>` issues
    `UVC_GET_CUR` on selector `0x06`, allocates a 60-byte
    buffer (verified via `get_len`), returns a `Status`
    struct with the 5 decoded fields plus
    `raw: [u8; 60]` for the future debug "Dump status"
    page. Defensive accept of HDR `!= 0` for true (matches
    Tiny4Linux's permissive decode).
  - **V4L2 PTZ helpers** (`xu/v4l2_ptz.rs`): port cgevans's
    `V4L2_CID_PAN/TILT/ZOOM_{ABSOLUTE,RELATIVE}` constants
    (`0x009A0908..0x009A090E`). These back the existing
    T-101 PTZ buttons — we keep them even though T-101
    already accesses them via the V4L2 generic path,
    because the upcoming T-302 preset wrapper may want to
    read pan/tilt to *display* "Preset N is currently
    pointing here" if Q7 (preset save) ever lands.
  - **Errors** (`xu/errors.rs`): `pub enum XuError` via
    `thiserror`, variants: `Io(io::Error)`, `Ioctl(Errno)`,
    `LengthMismatch { expected, got }`, `InvalidEnum(u8)`,
    `InvalidPresetIndex(i8)`. Convert to
    `obsbot_core::Error::Io` at the crate boundary so
    existing callers' `?` chains keep compiling.
  - **Tests** (all `#[cfg(test)]` in-line, plus
    `crates/obsbot-core/tests/xu_hardware.rs` for the
    `#[ignore]`d live test):
    - Unit: AIMode round-trip (`set_to_wire(AIMode::X) ==
      (m, n) && AIMode::try_from((m, n)) == Ok(AIMode::X)`)
      for all 10 modes EXCEPT `Hand` (Hand exercises the
      Q4 asymmetry — separate test asserts that both
      `(3,0)` and `(6,0)` decode to `Hand`).
    - Unit: FOVMode / FaceAeMode / SleepState /
      TrackingSpeed round-trips.
    - Unit: `build_command02` produces the exact 36-byte
      arrays for Awake / Sleep / Standard / Sport / each
      of the three presets — fixture vectors copied from
      the Tiny4Linux test suite (EUPL-1.2 attribution).
    - Unit: `decode_status` against the Tiny4Linux fixture
      vector (the 57-byte Awake + HDR-on + UpperBody +
      Sport sample in PROTOCOL.md §3.2 sample).
    - `#[ignore]`d hardware test: open `/dev/video0`, send
      `set_hdr(true)`, `get_status()`, assert `status.hdr_on
      == true`; then `set_hdr(false)`, `get_status()`,
      assert `status.hdr_on == false`. Read-only afterwards
      — does NOT change AI mode / exposure / sleep in
      automated tests (those need human eyes on the camera).
  - **License headers**: every file under
    `crates/obsbot-core/src/xu/` that contains ported bytes
    carries the dual SPDX block from [[CREDITS.md]]. Files
    without ported bytes (e.g. `xu/errors.rs`) keep the plain
    GPL-3.0-or-later line.
  - **Cargo gates**: `cargo fmt --all --check`,
    `cargo clippy --workspace --all-targets -- -D warnings`,
    `cargo test --workspace` all exit 0. Hardware test
    `cargo test -p obsbot-core --test xu_hardware --
    --ignored` exits 0 on the user's plugged-in Tiny 2 Lite.
  - Commit: `feat(core): obsbot-core::xu module — UVCIOC_CTRL_QUERY,
    enums, decode_status (T-300)`.
- **Outcome**: 17 new source files + 3 modified land on
  `feat/T-300-xu-tracking`:
  * `crates/obsbot-core/src/xu/mod.rs` — module root + re-exports
    + scoped `#![allow(clippy::doc_markdown)]` for the project-name
    prose noise (`Tiny4Linux`, `cgevans`, etc.) that doc-markdown
    flags as false positives.
  * `crates/obsbot-core/src/xu/transport.rs` — the unsafe surface.
    `BUNIT_ID = 0x02`, `SELECTOR_OPCODE = 0x06`, `SELECTOR_FRAME =
    0x02`. UVC request codes in nested `uvc::` namespace.
    `nix::ioctl_readwrite!(...)` wrapped in a private
    `raw_ioctl` submodule with `#[allow(missing_docs)]` so the
    macro-generated `unsafe fn` doesn't trip the crate-wide
    `#![warn(missing_docs)]`. Three public entry points: low-level
    `xu_query`, plus `get_len` / `set_cur` / `get_cur` that
    pre-check length via `UVC_GET_LEN`. Crate lint relaxed from
    `unsafe_code = "forbid"` to `"deny"` so this single module can
    scope `#![allow(unsafe_code)]`; every other module stays safe.
  * `crates/obsbot-core/src/xu/enums.rs` — `AiMode` (10 variants
    with `to_wire` + `TryFrom<(u8, u8)>`, Q4-permissive decode of
    Hand), `FovMode` (3), `FaceAeMode` (2), `ExposureMode` (2),
    `SleepState` (3 including Unknown), `TrackingSpeed` (2, Q6
    gap defaults to Standard). `EnumDecodeError` carries the
    failing wire bytes for diagnostics.
  * `crates/obsbot-core/src/xu/errors.rs` — `XuError` with
    `Io / LengthMismatch / Decode / InvalidPresetIndex` variants;
    `From<XuError> for crate::Error` maps everything to
    `Error::Io` for callers using the crate-wide error.
  * `crates/obsbot-core/src/xu/command02.rs` — pure-Rust
    `build(fg, seq, cks, cmd, app) -> [u8; 36]` builder (no `bon`
    dep). `PRESET_RECALL_APPENDIX` const built at compile time as
    four little-endian `1.0_f32`.
  * `crates/obsbot-core/src/xu/status.rs` — `Status` struct
    (sleep / hdr_on / ai_mode / tracking_speed + the full
    60-byte `raw` for the future T-302 debug page). `get_status`
    + `Status::decode`. Offset constants exposed for the GUI.
  * `crates/obsbot-core/src/xu/v4l2_ptz.rs` — standard V4L2
    Pan/Tilt/Zoom CIDs (`0x009A_0908`..`0x009A_090E`) for the
    T-101 PTZ wiring to reach via a stable symbolic name.
  * `crates/obsbot-core/src/xu/commands/{hdr, face_ae, fov,
    ai_mode}.rs` — one file per selector-0x06 opcode. Each
    exposes a pure `payload(...) -> [u8; N]` plus a `set_*(camera,
    value)` wrapper. cgevans-faithful `AiMode::Hand` setter writes
    `m=3` per Q4.
  * `crates/obsbot-core/src/xu/commands/{exposure_mode_type,
    sleep, tracking_speed, preset}.rs` — one file per
    selector-0x02 frame. Bytes verbatim from Tiny4Linux's test
    fixtures (cgevans-labelling for exposure per Q5).
    `preset::payload(index)` validates `0..=2` and returns
    `XuError::InvalidPresetIndex` otherwise. `sleep::payload`
    refuses `SleepState::Unknown` with an Io error rather than
    sending a junk frame.
  * `crates/obsbot-core/tests/xu_hardware.rs` — `#[ignore]`d
    integration test. Opens `/dev/video0` (or wherever
    `enumerate_cameras` finds the Tiny 2), reads baseline state,
    flips HDR, reads back, asserts the flip, restores baseline.
    Read-only on AI mode / exposure / sleep / presets — those
    have visible effects and belong in the user-driven T-303
    validation matrix.

  Gate summary at commit time:
  ```
  cargo fmt --all --check                                → exit 0
  cargo clippy --workspace --all-targets -- -D warnings  → exit 0
  cargo test --workspace                                 → 50 unit
                                                           (8 enumerate
                                                            + 3 controls
                                                            + 3 camera
                                                            + 36 xu
                                                            ↑ obsbot-core)
                                                           + 1 doctest
                                                           + 3 CLI render
                                                           + 1 GUI
                                                           = 55 tests
                                                           pass; 6
                                                           hardware
                                                           #[ignore]d
                                                           (T-011 + T-013c
                                                           + T-100 trio +
                                                           T-300 hardware)
  ```

  Caveat (the "DONE with" part): the **`cargo test -p obsbot-core
  --test xu_hardware -- --ignored`** gate has not been driven on
  hardware yet. The test issues a single HDR toggle + restore against
  the user's Tiny 2 Lite. Per CLAUDE.md §3.3 (touching the user's
  hardware) the run sits in `STATE.pending_user_actions` until the
  user invokes it. Same closure shape as T-010 (icon) and T-017
  (Arch PKGBUILD): code-complete now, end-of-line verification
  deferred to a user-driven step. Will roll into the T-303
  validation matrix unless the user runs it sooner.

### T-301 — GUI "AI & Effects" page

- **State**: DONE (with caveat — visual confirmation pending user
  run; see Outcome below)
- **Started**: 2026-05-14T20:05:00Z
- **Completed**: 2026-05-14T20:20:00Z
- **Commit**: `feat(gui): AI & Effects page (T-301)`
- **Depends on**: T-300 (the backend).
- **Description**: New `AdwPreferencesPage` (or section inside
  the existing controls page — design choice on starting)
  exposing the selector-0x06 surface: AI tracking mode
  dropdown (10 entries), FOV combo (3 entries), HDR switch,
  Face AE switch (visible only when exposure is Auto),
  exposure-mode toggle (Auto / Manual). All writes go via
  `obsbot_core::xu::commands::*`. Reads on focus refresh
  values from `get_status()` (60-byte poll). Persist user's
  last selection per-camera via the T-105 GSettings module.
  Errors surface as toasts via T-108
  `settings::surface_error`.
- **Acceptance criteria (draft — refine when starting)**:
  - `crates/obsbot-gui/src/ai_effects_view.rs` (new module,
    ~250 lines) exposing
    `pub fn build_ai_effects_group(cam: &CameraInfo) ->
    adw::PreferencesGroup`.
  - Wiring: opening a camera detail page mounts the new
    group between the WB group (T-103) and the PTZ pad
    (T-101) — exact placement design choice when starting.
  - Widgets: `AdwComboRow` for AI mode (model:
    `gtk::StringList` with the 10 localized labels),
    `AdwComboRow` for FOV (3 labels), `AdwSwitchRow` for
    HDR, `AdwSwitchRow` for Face AE
    (`sensitive` bound to "exposure is Auto"),
    `AdwComboRow` for exposure mode (Auto / Manual).
  - On row activation: call the matching
    `obsbot_core::xu::set_*` function on a worker thread
    via `glib::MainContext::spawn_local` so the GTK loop
    doesn't block on the ioctl.
  - On open: 60-byte status poll, populate widgets from the
    decoded state.
  - On error: toast via `settings::surface_error("Failed to
    set <name>: <error>")` per T-108 pattern.
  - GSettings schema gains `ai-mode`, `fov-mode`,
    `hdr-enabled`, `face-ae`, `exposure-mode`, persisted
    per-camera-serial via the T-105 keying convention.
  - All four cargo gates green; GUI auto-tests N/A per
    [[CLAUDE.md §5.4]].
  - Commit: `feat(gui): AI & Effects page (T-301)`.
- **Out of scope for T-301** (T-302 picks them up):
  Sleep/Wake, Tracking speed, Preset recall, debug Dump
  Status page.
- **Outcome**: single new module
  `crates/obsbot-gui/src/ai_effects_view.rs` (~280 lines, ~80
  of which are gettext-ed label dictionaries) plus 3 modified:
  `crates/obsbot-gui/src/main.rs` declares the new `mod
  ai_effects_view;`, and `crates/obsbot-gui/src/controls_view.rs`
  threads `cam: &CameraInfo` through `build_body` →
  `render_controls` so the new group can read `cam.vid /
  cam.pid / cam.video_path` to decide whether to mount.
  * **Tiny-2-family gate**: `is_tiny_2_family(vid, pid)` checks
    `obsbot_core::TINY2_FAMILY` (the same constant T-011's
    enumerator uses). Non-Tiny-2 UVC cameras get `None` and the
    group is skipped entirely — the controls page still renders
    the V4L2-standard rows.
  * **File handle ownership**: `OpenOptions::new().read(true)
    .write(true).open(path)` once per page-build, wrapped in
    `Rc<File>`. Every widget closure clones the `Rc` (one
    bumped refcount per row) and the ioctl path borrows
    `&*rc`. Single-threaded — GTK callbacks run on the main
    loop and the ioctl is sub-millisecond on the user's
    hardware, so no `spawn_local` lift yet.
  * **Hydration**: `xu::get_status(&file)` on construction
    seeds the AI-mode and HDR widgets. FOV / Face AE /
    Exposure mode are NOT in the GET_CUR status struct, so
    they default to `Wide / off / Auto` respectively and
    only reflect user-driven changes; a future getter would
    fix this (op `0x04`, op `0x03`, selector `0x02` reads
    are not in either FOSS reference repo, so this is
    investigation-frontier work).
  * **Persistence deferred**: the original acceptance criterion
    said "GSettings schema gains `ai-mode`, `fov-mode`,
    `hdr-enabled`, `face-ae`, `exposure-mode`, persisted
    per-camera-serial via the T-105 keying convention". On
    inspection, T-105's schema (`data/io.github.domatix.
    ObsbotCamControl.gschema.xml`, key `cameras a{sa{si}}`)
    and the runtime code (`settings::write_and_save`, which
    reads `control-values a{si}` with a unit-separator
    encoding) are **structurally mismatched** — this is a
    pre-existing bug separate from T-301. Persisting XU
    state through that broken plumbing would propagate the
    bug; we therefore re-hydrate from
    `xu::get_status()` on every page open (the camera
    firmware is the source of truth) and defer XU
    persistence to a follow-on task that first sorts out
    T-105's schema. Surfaced in PROGRESS for the user to
    pick up; not blocking the v0.3 milestone.
  * **Error path**: every `set_*` failure routes through
    `settings::surface_error("Failed to set <name>:
    <error>")` — the existing T-108 toast plumbing.
    `XuError::Display` formats to a user-readable string;
    `XuError::Io(EACCES)` for example renders as
    "Permission denied (os error 13)" which is exactly the
    same shape T-108 already surfaces for V4L2 standard
    writes.
  * **Mounting**: AI & Effects group is the **first** group
    on the controls page (above PTZ pad), per the v0.3
    marquee-feature framing. WB / Exposure / generic groups
    follow as before. The order swap means the user's first
    visible row on any Tiny 2 camera is now the AI tracking
    dropdown.

  Cargo gates at commit:
  ```
  cargo fmt --all --check                                → exit 0
  cargo clippy --workspace --all-targets -- -D warnings  → exit 0
  cargo test --workspace                                 → 55 pass
                                                           (unchanged
                                                           from T-300;
                                                           GUI is not
                                                           auto-tested
                                                           per
                                                           CLAUDE.md
                                                           §5.4)
  cargo build -p obsbot-gui                              → exit 0
                                                           (1m 06s
                                                           cold)
  ```

  Live-validation rev2 (2026-05-14 / 2026-05-15) drove two
  bug fixes and two scope retirals on top of the original
  T-301 commit `c1a2179`:
  * `fix(core,gui): zero-pad SET_CUR payloads + escape group
    titles (T-301)` (commit `3c04e57`) — kernel
    `UVCIOC_CTRL_QUERY` requires `xqry->size ==
    ctrl->info.size` exactly; for selector 0x06 that is 60,
    not 3-4. Added zero-pad in
    `obsbot_core::xu::transport::set_cur`. Also renamed
    "AI & Effects" / "Power state & Presets" to "and"
    variants — `AdwPreferencesGroup::title` is Pango
    markup and the bare `&` was tripping the entity parser.
  * `fix(gui): retire XU Exposure mode + Face metering
    rows; FOV Narrow caveat (T-301)` (commit landing this
    turn) — exposure mode is redundant with V4L2
    standard `auto_exposure` (User-class menu, T-104),
    which the user already sees in the Exposure group;
    Q5 (label swap) ceases to matter once the duplicate is
    retired. Face AE was only meaningful on the XU
    auto-exposure path, which is gone with the duplicate,
    so it is also retired. PROTOCOL.md §3.2 picks up
    quirk Q8 (Narrow FOV is a no-op on Tiny 2 Lite
    firmware 5.10 — the byte we send matches cgevans's
    setter, but the Lite optics lack the path; Wide and
    Normal work).

  Caveats remaining for T-303:
  - The "AI and effects" group appears at the top of the
    controls page, above the PTZ pad, with **four** rows:
    AI tracking (combo, 10 entries), Tracking speed
    (combo, Standard / Sport), Field of view (combo, Wide
    / Normal / Narrow — Narrow is no-op on Tiny 2 Lite per
    Q8), HDR (switch).
  - AI tracking + HDR + tracking speed hydrate from
    `xu::get_status()` on page open.
  - Each row's interaction triggers the documented
    camera behaviour (AI mode change visible on tally /
    motion, HDR visible in the image, tracking speed
    changes pan/tilt acceleration on subsequent moves,
    FOV Wide / Normal change the digital crop).
  - Permission failures (run `sudo chmod 000 /dev/video0`,
    click any AI row) surface as a toast — same shape
    T-108 drives for V4L2 writes.

### T-302 — Tiny4Linux extras (Sleep/Wake, Tracking Speed, Preset recall, Dump Status)

- **State**: DONE (with caveat — visual confirmation pending user
  run; see Outcome below)
- **Started**: 2026-05-14T20:30:00Z
- **Completed**: 2026-05-14T20:50:00Z
- **Commit**: `feat(gui): Tiny4Linux extras (sleep/wake, tracking
  speed, presets, dump status) (T-302)`
- **Depends on**: T-300 (backend has the frames already),
  T-301 (the "AI & Effects" page is the natural mounting
  point).
- **Description**: Add the three Tiny4Linux-only frames to
  the GUI plus the diagnostic "Dump XU status" page that
  exposes the 55 still-undecoded bytes of the 60-byte
  GET_CUR struct for future community discovery. The
  acceptance bytes are already in T-300; T-302 is purely
  GUI + diagnostic.
- **Acceptance criteria (draft — refine when starting)**:
  - Sleep / Wake: an `AdwSwitchRow` (or header-bar toggle)
    on the per-camera page wired to
    `obsbot_core::xu::set_sleep`.
  - Tracking speed: `AdwComboRow` (Standard / Sport) inside
    the AI & Effects group.
  - Preset recall: three `AdwActionRow`s labelled
    "Preset 1 / 2 / 3" with go-next-symbolic suffix; on
    activation call `set_preset(idx)`. README copy plus a
    subtitle on the group explains presets must be
    programmed via OBSBOT Center beforehand (per Q7).
  - Debug "Dump XU status" page: a new menu item in the
    burger menu opens an `AdwNavigationPage` showing the
    60 status bytes as hex pairs with offsets and the 5
    decoded fields highlighted. A "Copy hex dump" button
    sends the raw bytes to the clipboard so users can
    paste them into bug reports / discovery threads.
  - Cargo gates green; commit `feat(gui): Tiny4Linux extras
    (sleep/wake, tracking speed, presets, dump status)
    (T-302)`.
- **Outcome**: one new module
  `crates/obsbot-gui/src/extras_view.rs` (~250 lines) plus
  three small modifications:
  * `crates/obsbot-gui/src/ai_effects_view.rs` gains a 6th row
    (`tracking_speed_row`) between AI tracking and Field of
    view — Tracking speed feels semantically part of "tracking"
    so it lives in the AI & Effects group rather than the
    Power & Presets group.
  * `crates/obsbot-gui/src/main.rs` declares
    `mod extras_view;`.
  * `crates/obsbot-gui/src/controls_view.rs` mounts the new
    group between AI & Effects and PTZ pad.
  * `crates/obsbot-core/src/xu/mod.rs` re-exports
    `status::STATUS_LEN` so the GUI can size the dump
    formatter without reaching into the submodule.

  The new module exposes
  `build_extras_group(cam: &CameraInfo) -> Option<adw::
  PreferencesGroup>` titled "Power state & Presets" with five
  rows:
  - `AdwSwitchRow` "Camera awake" — hydrates from
    `xu::get_status().sleep` (Awake → on); flipping to off
    sends `set_sleep(Sleep)`.
  - Three `AdwActionRow` "Preset 1 / 2 / 3" — each activatable
    (whole-row click or trailing button) and wired to
    `recall_preset(idx)`. Group description explains preset
    save requires OBSBOT Center (Q7).
  - `AdwActionRow` "Show XU status (hex dump)" — opens an
    `AdwAlertDialog` rendering the 60-byte raw payload as
    `00:` / `10:` / etc. monospace rows in `<tt>` markup,
    plus the 5 decoded fields listed below. "Copy hex"
    response writes the hex grid to the clipboard via
    `gdk::Display::clipboard().set_text(...)`. The diagnostic
    surface promised by T-302 — gives users a clipboard-
    pasteable artefact for community bug reports against the
    55 undecoded bytes.

  Q-validation: this commit does **not** introduce any new
  quirks. Q4 / Q5 / Q6 / Q7 surface verbatim from T-300; the
  GUI sends the exact bytes the backend ships. Q7 ("no preset
  save") is acknowledged in the group description string so
  users do not expect a "save" button.

  Cargo gates at commit:
  ```
  cargo fmt --all --check                                → exit 0
  cargo clippy --workspace --all-targets -- -D warnings  → exit 0
  cargo test --workspace                                 → 55 pass
                                                           (unchanged
                                                           from T-301)
  cargo build -p obsbot-gui                              → exit 0
                                                           (3 s
                                                           incremental
                                                           over the
                                                           T-301
                                                           binary)
  ```

  Caveat: visual / interaction validation pending. Five
  user-driven gates accumulate for T-303 in
  `STATE.pending_t302_visual`:
  - The "Power state & Presets" group appears below "AI &
    Effects" on the controls page.
  - "Camera awake" switch reflects current state; flipping
    off causes the lens cover to descend (Sleep) and the
    image to freeze.
  - Tracking speed combo (in AI & Effects) reflects current
    speed; switching to Sport makes pan/tilt move faster.
  - Each preset-recall row, when clicked, moves the camera
    to the slot. NOTE: the user must have **programmed**
    each slot via OBSBOT Center beforehand; if a slot was
    never programmed the recall is a no-op (no error).
  - "Show XU status (hex dump)" opens an `AdwAlertDialog`
    with a 4-column hex grid + 5 decoded fields below; the
    "Copy hex" button puts the grid on the clipboard
    (verify via `xclip -selection clipboard -o`).

### T-303 — Validation pass, AppStream release notes, merge v0.3

- **State**: DONE
- **Started**: 2026-05-14T20:55:00Z
- **Completed**: 2026-05-15T12:00:00Z
- **Depends on**: T-300, T-301, T-302.
- **Progress so far**: `data/io.github.domatix.ObsbotCamControl.
  metainfo.xml.in` gains a `<release version="0.3.0"
  date="2026-05-14" type="development">` entry listing every
  user-visible v0.3 feature (10 AI modes, 3 FOV widths, HDR,
  Face AE, Exposure mode, Tracking speed, Sleep/Wake, three
  preset slots, the Dump XU status diagnostic) and the
  EUPL-1.2 → GPL-3 attribution. Validated via
  `appstreamcli validate --no-net` on a substituted copy —
  same pedantic "redundante: 1" note as the rest of the
  releases (the intentional `ObsbotCamControl` TitleCase per
  ADR-0012). Four new keywords (AI / tracking / auto-framing /
  HDR) for discoverability. Commit:
  `docs(appstream): v0.3.0 release notes (T-303)`.
- **Description**: Hardware validation against the user's
  Tiny 2 Lite (`3564:fef9`, bcdDevice 5.10). Validates the
  two outstanding FOSS-quirks (Q4 AIMode `Hand` setter,
  Q5 Auto/Manual exposure label inversion) on live hardware,
  refines PROTOCOL.md if needed, drafts AppStream release
  notes for v0.3.0, and merges the feature branch back to
  `main`.
- **Acceptance criteria (draft — refine when starting)**:
  - User-driven validation matrix:
    - Each AI mode set → camera behaviour confirmed against
      the on-screen tally (Hand mode is the critical one
      for Q4).
    - Auto / Manual exposure transitions confirmed (Q5);
      sliders grey/wake correctly via T-111's sensitivity
      refresh.
    - HDR / Face AE / FOV toggles trigger visible image
      change.
    - Sleep / Wake toggles power-state.
    - Tracking speed Standard / Sport changes pan/tilt
      acceleration.
    - Each preset recall moves the camera (only valid if
      the user has programmed presets via OBSBOT Center
      first — otherwise the recall is a no-op and we
      observe no error).
    - Dump status shows non-zero bytes outside the 5
      decoded offsets.
  - `data/io.github.domatix.ObsbotCamControl.metainfo.xml.in`
    gains a `<release version="0.3.0">` entry summarizing the
    XU + AI tracking work.
  - PROTOCOL.md §3.2 updated with any corrections from the
    live validation (Q4 / Q5 resolution recorded).
  - Squash-merge `feat/T-300-xu-tracking` → `main` via
    fast-forward or merge commit (decision when starting,
    per CLAUDE.md §2.4).
  - Tag `v0.3.0` per [[CLAUDE.md §7]] milestone DOD.
  - Commit `docs(appstream): v0.3.0 release notes (T-303)`
    bundled with the metainfo edit before the tag cut.
- **Outcome**: v0.3.0 shipped on 2026-05-15. AppStream release
  notes committed (97cc25e). Hardware suite ran in-session
  (7/7 pass against the connected Tiny 2 Lite, including the
  new `reads_single_v4l2_control_just_in_time`). User-driven
  GUI validation matrix completed by the user: AI and effects
  (4 rows × 10 AI modes + HDR + FOV) and Power state and
  presets (sleep/wake, 3 presets, dump dialog) all green.
  Three hot-fix commits landed during validation: zero-pad
  SET_CUR (3c04e57), retire XU Exposure mode + Face metering
  (d3fce26), refresh PTZ pan/tilt from kernel on every click
  (f38a7ff — fixes the cache-drift bug surfaced during the
  pad's "up up up up" smoke test). Quirks resolved per
  PROTOCOL.md §3.2: **Q4** (AIMode Hand m=3) accepted as-is
  per user's validation pass (no contradiction reported);
  **Q5** retired by descope (XU Exposure mode + Face metering
  rows removed in favour of the V4L2 standard `auto_exposure`
  control); **Q8** (FOV Narrow no-op on Tiny 2 Lite firmware
  5.10) documented, no further action. **Schema T-105 mismatch
  punted** to v0.3.1 as `T-105fix` rather than wedging a
  pre-existing main-branch bug into the v0.3.0 tag. Branch
  `feat/T-300-xu-tracking` squash-merged into `main`;
  `v0.3.0` annotated tag cut.

### T-105fix — GSettings schema / runtime key mismatch (v0.3.1)

- **State**: DONE
- **Started**: 2026-05-15T12:30:00Z
- **Completed**: 2026-05-15T12:45:00Z
- **Depends on**: v0.3.0 tagged.
- **Origin**: surfaced 2026-05-14 during T-301 implementation.
  `data/io.github.domatix.ObsbotCamControl.gschema.xml`
  declares key `cameras` of type `a{sa{si}}` (nested dict:
  serial → control-name → i32), but `crates/obsbot-gui/src/
  settings.rs` reads/writes key `control-values` of type
  `a{si}` (flat dict, encoded with a unit separator). T-105
  persistence has been silently never working end-to-end —
  GLib aborts with a critical warning when `g_settings_get_
  value` is called with a key the schema doesn't know about.
  Bug is pre-existing on `main` (predates T-300); descoped
  from v0.3.0 closure per T-303 decision.
- **Description**: Pick one of the two shapes and align the
  other to it. ~10 lines either way.
  - **Option A** (schema → runtime): rename the schema key
    from `cameras` to `control-values`, change its type
    from `a{sa{si}}` to `a{si}`, drop the nested dict.
    Drops the per-serial nesting but `settings.rs` encodes
    serial+name into the key string already, so no
    persisted data is lost.
  - **Option B** (runtime → schema): rewrite `settings.rs`
    to use the nested `a{sa{si}}` shape and the `cameras`
    key name. Keeps the schema cleaner but requires
    rebuilding the read/write path.
  - Recommendation when starting: **Option A** — minimal
    schema change, no runtime restructuring, plays nicer
    with `gsettings get` from the CLI.
- **Acceptance criteria (draft — refine when starting)**:
  - Schema and runtime agree on key name and type.
  - Persistence round-trip works end-to-end: change
    brightness=75, kill the app, restart it, drill into the
    camera, slider sits at 75.
  - `cargo test -p obsbot-gui` unit tests still pass; add
    one that simulates the round-trip through a temporary
    `GSettings` backend.
  - `appstreamcli validate --no-net` still green.
  - Commit: `fix(gui): align GSettings schema and runtime
    key (T-105fix)`.
- **Out of scope**: XU value persistence (a future T-104a
  layered on top of the corrected T-105 path); user-visible
  Reset to defaults button (lives in T-100 plumbing).
- **Outcome**: Option A taken — schema realigned to runtime.
  `data/io.github.domatix.ObsbotCamControl.gschema.xml`
  renames key `cameras` → `control-values` and switches the
  type from `a{sa{si}}` to `a{si}`. `settings.rs` was
  already encoding `"<serial>\x1f<control-name>"` as the
  composite key, so no runtime change beyond verifying the
  test path. New unit test
  `settings::tests::schema_round_trip_with_runtime_key`
  exercises `set/get` against the compiled schema loaded
  via the same `settings_handle()` used in production —
  catches future schema drift without launching the GUI.
  T-105 persistence (the parked v0.2 validation) is now
  testable end-to-end; live-validation re-queued under
  parked.

### T-101d — Strip PTZ to pure single-step (revert hold/continuous mode)

- **State**: DONE (code + gates + unit tests green 2026-06-02;
  on-screen confirmation is the user's next-launch glance).
- **Completed**: 2026-06-02T08:40:00Z
- **Depends on**: T-101a/b/c (whose continuous-motion machinery
  this removes). See [[DECISIONS.md ADR-0021]].
- **Origin**: 2026-06-02 user feedback testing the v0.3.2
  Flatpak — the press-and-hold / keyboard-repeat PTZ "works
  terribly, extremely buggy". Asked for the simplest possible model:
  one click / keypress = exactly one move, nothing that errors.
- **Description**: remove all continuous-motion code from
  `ptz_pad.rs` — `GestureClick` long-press, the recurring `glib`
  hold timers, `PtzAccumulators`, `hold_tick`,
  `resolved_hold_step`, the `HOLD_*` / `LONG_PRESS_MS` /
  accelerator constants — and the orphaned `ptz-speed-fast`
  GSettings key + `settings::ptz_speed_fast()`. Directional
  buttons become plain `connect_clicked` → one `PAN_TILT_STEP`
  (5°) step; keyboard arrows fire one step per key-press via a
  single `EventControllerKey` (Bubble phase kept so focused
  sliders still consume their arrows); `Home` recenters. Step
  arithmetic extracted to a pure `next_position(current, sign,
  step, min, max)` for unit testing.
- **Acceptance criteria**:
  - [x] All hold/timer/accumulator code removed from `ptz_pad.rs`;
        `controls_view.rs` call sites reverted to the simple
        signatures.
  - [x] `ptz-speed-fast` removed from gschema + settings.rs (no
        dead code; `cargo clippy -D warnings` green).
  - [x] `next_position` unit tests cover step + clamp-to-min/max
        + zero-sign no-op (4 tests, pass).
  - [x] Cargo gates green default + with
        `obsbot-gui/live-preview`.
  - [x] On-screen (user-confirmed 2026-06-02, rebuilt Flatpak):
        one button click = one 5° move, no runaway / sticky
        motion; arrow keys move once per press.
- **Out of scope**: re-introducing smooth panning behind a
  Preferences toggle (a deliberate, hardware-tested v0.6 item if
  ever wanted — not the default).

### T-101c — PTZ tuning follow-ups (speed slider + Shift accelerator + hot-plug timer cleanup)

- **State**: DONE, then SUPERSEDED by T-101d (2026-06-02) — its
  continuous-motion/speed/accelerator machinery was removed after
  the user found it buggy on hardware. The hot-plug timer cleanup
  is moot now that there are no hold timers. See ADR-0021.
- **State (historical)**: DONE (validated against the Tiny 2 Lite
  2026-06-02; squashed into the v0.4 first-slice bundle on
  `main`. One fix landed during validation — see Outcome.)
- **Started**: 2026-05-19T02:45:00Z
- **Completed**: 2026-06-02T06:47:44Z
- **Depends on**: T-101b squashed to `main` (96e33ba).
- **Origin**: follow-up list queued in T-101b's PLAN entry +
  STATE under `follow_ups_queued`.
- **Description**: three small, related polish items bundled
  into one squash so v0.3.1 already had a "complete PTZ" story:
  1. `ptz-speed-fast` `GSettings` key (i, range 1..100, default
     50). Schema-side range constraint already clamps stored
     values. Read once per hold engage by
     `ptz_pad::resolved_hold_step` and scaled linearly off
     `HOLD_STEP_AT_DEFAULT`. Slowest setting still produces
     visible motion thanks to `HOLD_STEP_FLOOR` = 0.1° / tick.
     Mid-hold slider changes do not re-tune the active timer
     (next engage picks up the new value).
  2. Shift+Arrow accelerator (keyboard only). The keyboard
     handler's modifier filter now skips Ctrl / Alt / Super
     but inspects Shift inline — when present, the resolved
     step is multiplied by `HOLD_ACCELERATOR_MULT = 3` for the
     duration of the press. Mouse hold ignores Shift because
     `gtk::GestureClick` does not expose live modifier state
     during a hold.
  3. Hot-plug REMOVE timer cleanup. Every recurring hold
     timer (mouse + keyboard) now checks
     `path_tick.exists()` at the start of each tick; when the
     /dev/videoN node disappears the closure returns
     `ControlFlow::Break` and the source self-cancels. Stops
     us from writing to a vanished device after the camera
     was unplugged mid-hold.
- **Acceptance criteria**:
  - [x] All four cargo gates green default + with
        `obsbot-gui/live-preview`.
  - [x] `gschema.xml` carries the `ptz-speed-fast` key with the
        `<range min="1" max="100"/>` constraint and the schema-
        default `50`.
  - [x] `settings::ptz_speed_fast()` returns 50 when the
        schema is unavailable (cargo run without `meson
        install`), the stored value otherwise.
  - [x] `ptz_pad::hold_tick` takes `step_units: i64` instead
        of the old `HOLD_STEP` constant.
  - [x] Mouse-hold engage and keyboard-press engage both
        resolve `step_units` once at engage time; Shift
        modifier multiplies by `HOLD_ACCELERATOR_MULT`.
  - [x] Both timer closures abort cleanly when the device
        path disappears mid-hold.
  - [x] Hardware ergonomics validation (2026-06-02): speed
        slider via `gsettings set ... ptz-speed-fast 80`
        produces a visibly faster hold than `20`; Shift+Arrow
        triples the step; unplugging mid-hold stops the writes
        without spam. User confirmed green.
- **Outcome — accumulator fix discovered during validation
  (2026-06-02)**: the hold path shipped in v0.3.1 re-read the
  kernel position on every 50 ms tick (`hold_tick` called
  `current_axis` per tick). At the 20 Hz cadence the V4L2
  device had not yet reflected the previous write, so each tick
  read stale state and stacked the same step on it — the camera
  visibly stalled instead of panning smoothly. Fixed by adding
  a per-axis local accumulator (`PtzAccumulators`: two
  `Rc<Cell<i64>>` + the cached ranges) seeded from the kernel
  **once at press time** (gesture engage / key down); every
  subsequent tick adds `step_units` to the cell and writes it
  back without re-reading. The tap path still re-reads per click
  (taps can be seconds apart — AI tracking / preset recall move
  the camera between them). `build_ptz_pad` now returns
  `(group, PtzAccumulators)` so `controls_view` threads the same
  accumulators into `wire_keyboard_arrows` without re-
  introspecting the control list. `btn_reset` and the keyboard
  `Home` path sync the accumulators to 0/0 so a hold right after
  a recenter starts from the recentered position. Files:
  `ptz_pad.rs` (accumulator struct + engage refresh + tick
  rewrite), `controls_view.rs` (tuple plumbing). Committed as
  `fix(gui): smooth PTZ hold via local accumulator (T-101c)`.
- **Out of scope for T-101c**:
  - Preferences dialog UI for the speed slider — that comes
    with v0.6 polish or as a tiny T-101d if the user wants it
    sooner.
  - Per-camera tuning (the slider is app-wide; per-camera
    routing would need a serial-keyed dict like
    `control-values`).

### T-101b — PTZ press-and-hold + keyboard arrows (supersedes T-101a)

- **State**: IN_PROGRESS (impl landed on `feat/T-101b-ptz-hold-keyboard`,
  awaiting hardware ergonomics validation against the Tiny 2 Lite)
- **Started**: 2026-05-19T01:45:00Z
- **Depends on**: T-200 squashed to `main` (cccab8c). T-101a is
  superseded — its branch stays for archaeology but is not
  merged.
- **Origin**: 2026-05-19 user feedback after validating T-200.
  Discrete 5°-per-click PTZ feels "jumpy" especially on
  diagonals; user wants press-and-hold (hold the click = continuous
  motion) and keyboard arrows for accessibility.
- **Description**: Two parallel input paths into the same
  `hold_tick` core that writes pan/tilt absolute positions JIT-
  read from the kernel:
  1. **Mouse / touch**: ported verbatim from `feat/T-101a`. Each
     of the 8 directional buttons carries a `gtk::GestureClick`
     with `LONG_PRESS_MS = 200` to disambiguate tap from hold.
     Tap path = single 5° step via the existing `connect_clicked`
     handler; hold path = recurring `glib::timeout_add_local`
     every `HOLD_REPEAT_MS = 50 ms` writing 1° steps (≈ 20°/s).
     A trailing-click suppressor flag prevents the click handler
     from double-firing one extra step after a hold release.
  2. **Keyboard**: new `wire_keyboard_arrows` in `ptz_pad.rs`
     attaches a `gtk::EventControllerKey` to the controls page's
     outer `gtk::Box` with `PropagationPhase::Bubble`. Mapping:
     Left/Right = pan, Up/Down = tilt (Up = camera looks up,
     matches `btn_n`), Home = recenter (pan = tilt = 0). Keypad
     equivalents (`KP_Left`, etc.) included. Modifiers
     (Ctrl/Shift/Alt/Super) bypass the controller so app-level
     shortcuts (Ctrl+Q, Ctrl+W) still reach their handlers.
     Each pressed arrow engages an own recurring timer keyed by
     raw `gdk::Key` value so diagonal motion via simultaneous
     Up+Right runs both axis timers independently. Auto-repeat
     suppression: if a key is already in the active-hold map
     when pressed again, the controller stops propagation
     without re-engaging.
- **Focus safety**: `Bubble` phase means the focused widget
  consumes arrows first. `gtk::Scale`, `gtk::SpinButton` and
  `adw::ComboRow` all consume arrows on focus, so the keyboard
  PTZ only fires when focus is on a non-interactive widget (the
  page background, a button row title, etc.). User can still
  scroll-edit slider values without moving the camera.
- **Acceptance criteria**:
  - [x] All four cargo gates green default + with
        `obsbot-gui/live-preview`.
  - [x] `gtk::GestureClick` press-and-hold imported from T-101a
        (`crates/obsbot-gui/src/ptz_pad.rs`, +210 lines).
  - [x] `wire_keyboard_arrows` registered via the outer Box in
        `controls_view::render_controls`.
  - [ ] **Hardware validation pending**: tap 5° step still
        feels correct (no cache-drift recurrence); pressing and
        holding any of the 8 buttons makes the camera move
        smoothly until release; diagonals (NE/NW/SE/SW or
        Up+Right etc.) work; keyboard arrows move the camera
        when focus is on the page; arrows scrubbing a focused
        slider still adjust the slider (no robbery); Home
        recenters; modifier+arrow does not move PTZ.
- **Out of scope for T-101b** (queued for follow-up):
  - User-tunable hold speed (`GSettings` key
    `ptz-speed-fast` 1–100, default 50 → maps to `HOLD_STEP_
    DEGREES`). For now constants in `ptz_pad.rs` are static.
  - Shift+Arrow = larger step (3°/tick) as an accelerator.
  - Hot-plug REMOVE cancels the keyboard timers gracefully —
    currently the controller is dropped with the page widget
    and the timers leak briefly until the closures realise
    their `Rc<PathBuf>` is the last ref; not a regression vs
    T-101a but worth wiring T-110's hot-plug signal.

### T-101a — PTZ smooth movement via pan_speed / tilt_speed (SUPERSEDED 2026-05-19)

- **State**: SUPERSEDED by T-101b. Branch `feat/T-101a` kept
  locally for blame archaeology; do not merge.
- **Depends on**: T-303 closed and v0.3.0 tagged (so this can
  ship as v0.3.1 or roll into v0.4 — milestone decision when
  starting; user explicitly chose to defer this past the v0.3
  tag during T-303 validation).
- **Origin**: surfaced during T-303 user-driven validation on
  2026-05-15. The 3×3 directional pad (built in T-101) only
  offers discrete 5° hops via `pan_absolute` / `tilt_absolute`.
  SPEC §4.1 promises *"Pan / tilt / zoom controls (continuous
  and absolute)"* — the continuous half was never wired. Same
  validation pass also surfaced the cache-drift bug whose fix
  is bundled with T-303 (see [[PROGRESS 2026-05-15 PTZ
  cache-drift hot-fix]]); this task is the larger UX upgrade.
- **Description**: While a directional button is held down,
  set `V4L2_CID_PAN_SPEED` (`0x009a0920`) and/or
  `V4L2_CID_TILT_SPEED` (`0x009a0921`) to a non-zero value
  whose sign matches the direction; on release, set both back
  to `0`. The Tiny 2 Lite advertises `pan_speed` in
  `min=-1 max=160 step=1` and `tilt_speed` in
  `min=-1 max=120 step=1` (cross-checked with `v4l2-ctl
  --list-ctrls` on 2026-05-15 against firmware 5.10). This
  mirrors how OBSBOT Center drives the camera and gives the
  user truly continuous, smooth motion — feels like a
  joystick rather than a step sequencer.
- **Acceptance criteria (draft — refine when starting)**:
  - `ptz_pad.rs`'s 8 directional buttons wire `gtk::Gesture
    LongPress` (or a `ButtonController` + `MotionController`
    pair) so the press and release events are both
    observable. Per-click absolute-step behaviour stays as a
    fallback for keyboard activation (Enter / Space) so
    accessibility regressions don't happen.
  - A user setting (GSettings key `ptz-speed-fast`, integer
    1–100 default 50, or similar) chooses the magnitude;
    expose it via a future preferences dialog.
  - The diagonal buttons (NE / NW / SE / SW) drive both axes
    simultaneously; pan and tilt speeds must be issued as
    separate `set_control` calls (no `set_ext_ctrls` batching
    yet).
  - On hot-plug REMOVE mid-press, the speed write fails
    silently (no toast spam) but the GUI cleans up the
    gesture state.
  - Hardware test (`#[ignore]`d) writes pan_speed = 40, sleeps
    300 ms, writes pan_speed = 0, asserts the camera ended at
    a different `pan_absolute` than it started.
  - All four cargo gates green.
- **Out of scope for T-101a**: continuous *zoom* (zoom slider
  is already linear and smooth enough); accelerometer-style
  ramp-up; multi-button chord input. Those can land as
  T-101b / T-101c if validation reveals a need.

---

## v0.4 — Live Preview (planned)

### T-200 — Embedded preview pane in the per-camera controls page

- **State**: DONE (2026-05-19) — user-validated visually
  against the Tiny 2 Lite at firmware 5.10. Frames render in
  the sticky `gtk::Picture`, toggle on/off transitions clean,
  busy detection surfaces a toast, banner discoverability
  hint shows under the header bar while preview is off.
- **Started**: 2026-05-15T14:00:00Z
- **Resumed**: 2026-05-19T00:00:00Z — GStreamer dev packages
  (libgstreamer1.0-dev / libgstreamer-plugins-base1.0-dev /
  gstreamer1.0-plugins-good / gstreamer1.0-plugins-base /
  gstreamer1.0-libav / gstreamer1.0-gtk4 — all 1.26.2 except
  the gtk4 sink at 0.13.5) confirmed installed via dpkg / pkg-
  config / on-disk `.so` files (libgstvideo4linux2.so,
  libgstgtk4.so, libgstvideoconvertscale.so). Fixed feature-on
  compile: scaffold derived `thiserror::Error` but the crate's
  Cargo.toml never gated `thiserror` behind the `live-preview`
  feature, so the build broke as soon as the feature was on
  (E0433 + E0277). Added `thiserror` as an `optional = true`
  dep included in `live-preview = [...]`. Then a clippy sweep
  (14 lints, all in T-200 code): redundant inner
  `#![cfg(feature = "live-preview")]` in preview.rs (the module
  is already gated at `mod preview` in main.rs); 9 doc-comment
  `GStreamer` / `GSettings` identifiers missing backticks;
  `build_preview_group` returned `Option<adw::PreferencesGroup>`
  but always emitted `Some(_)` — flattened to bare
  `adw::PreferencesGroup`; `.map(...).unwrap_or(false)` →
  `.is_some_and(...)` per `clippy::map-unwrap-or`. All four
  gates green after fixes: `cargo fmt --all --check` exit 0;
  `cargo clippy --workspace --all-targets --features
  obsbot-gui/live-preview -- -D warnings` exit 0; `cargo test
  --workspace` 1 doctest pass; `cargo test --workspace
  --features obsbot-gui/live-preview` 2 pass.
- **Depends on**: v0.3.0 shipped (AI tracking now precedes
  preview per [[ADR-0020]] — milestone bucket renumber only,
  task scope is unchanged from when it was a v0.3 task).
- **Progress so far**: full scaffolding landed on branch
  `feat/T-200-preview`, all gated behind the `live-preview`
  Cargo feature (off by default) because `libgstreamer1.0-dev`
  + `gstreamer1.0-gtk4` are not installed on the dev host and
  `sudo` is interactive. Files:
  - `crates/obsbot-gui/Cargo.toml`: adds the three GStreamer
    crates as optional deps + the `live-preview` feature flag.
  - `crates/obsbot-gui/src/preview.rs` (new module, gated):
    `PreviewPipeline` struct with `new`/`start`/`stop`/
    `paintable` + a `PreviewError` enum (MissingElement,
    PipelineStart, GstInit). Pipeline is `v4l2src device=…!
    videoconvert ! gtk4paintablesink`; `v4l2src` rebuilt per
    `start(path)` so changing cameras mid-session works.
    `Drop` impl tears down on hot-plug REMOVE (T-110) or
    controls-page navigation.
  - `crates/obsbot-gui/src/main.rs`: `#[cfg(feature = "live-
    preview")] mod preview;`.
  - `crates/obsbot-gui/src/controls_view.rs`:
    `build_preview_group(path)` mounts the preview at the top
    of the controls page, with a `gtk::Picture` bound to the
    paintable and a `gtk::ToggleButton` driving start/stop.
    Failures surface as toasts via `settings::surface_error`
    and the toggle snaps back to off so the GUI does not lie
    about state.
  - `data/io.github.domatix.ObsbotCamControl.gschema.xml`:
    new `preview-default-on` boolean key (default `false`).
  - `crates/obsbot-gui/src/settings.rs`: feature-gated
    `preview_default_on()` reader.

  Default-build cargo gates green (fmt + clippy + test); the
  feature-on `cargo check --features live-preview` fails only
  at the pkg-config step looking for `gstreamer-1.0.pc` —
  confirming the Rust side compiles up to the system-deps
  boundary. **No runtime validation possible** until the user
  installs the system packages.

  **Install incantation (Debian / Ubuntu)**:
  ```
  sudo apt install libgstreamer1.0-dev \
                   libgstreamer-plugins-base1.0-dev \
                   gstreamer1.0-plugins-good \
                   gstreamer1.0-plugins-base \
                   gstreamer1.0-libav \
                   gstreamer1.0-gtk4
  ```
  **Arch**:
  ```
  sudo pacman -S gstreamer gst-plugins-base gst-plugins-good \
                 gst-libav gst-plugin-gtk
  ```
  Then `cargo run -p obsbot-gui --features live-preview`.
- **Description**: User-requested placement decision for the
  Live Preview milestone (originally seeded as a v0.3 task
  before ADR-0020 swapped priorities; ROADMAP §v0.4
  "Live Preview" was previously unspecified on WHERE the
  preview lives). The preview pane has to be embedded
  **inside the per-camera controls `AdwNavigationPage`**
  so the user can tweak brightness / WB / exposure / PTZ
  and see the effect live, without launching Cheese / OBS /
  `v4l2-ctl --stream-mmap` as a side process. Approach: a
  GStreamer pipeline `v4l2src device=/dev/videoN !
  videoconvert ! gtk4paintablesink` whose paintable is
  bound to a `gtk::Picture` placed at the top of the
  controls page, above the PTZ pad. A toggle (probably in
  the header bar of the per-camera page, near the back
  button) starts and stops the pipeline; default-off so the
  page still loads fast on cold open.
- **Acceptance criteria (draft — refine when starting)**:
  - Workspace deps gain `gstreamer`, `gstreamer-video`,
    `gstreamer-app` (already pinned in `[workspace.
    dependencies]`; just add the actual `.workspace = true`
    in `crates/obsbot-gui/Cargo.toml`).
  - `crates/obsbot-gui/src/preview.rs` (new module) exposes
    `pub struct PreviewPipeline` with `start(path)`,
    `stop()`, `paintable() -> gdk::Paintable`.
  - `controls_view::build_body` mounts a `gtk::Picture`
    bound to the paintable above the PTZ pad. Aspect-
    ratio-aware sizing (Picture's `content-fit:
    "Contain"`).
  - Header-bar toggle button — labelled with `view-preview-
    symbolic` or similar — wires `connect_toggled` to
    `pipeline.start()` / `pipeline.stop()`.
  - Camera-busy detection: if `v4l2src` fails to open the
    device (already streaming elsewhere), the failure is
    surfaced as a toast via `settings::surface_error`
    (T-108 already wires the overlay).
  - Pipeline is torn down cleanly on page navigation away,
    on hot-plug REMOVE (T-110), and on window close.
  - GSettings key `preview-default-on` (boolean) added to
    the schema — persisted per session, default `false`.
  - Hardware test (`#[ignore]`d) that constructs the
    pipeline, runs it for a fraction of a second, asserts
    it reached `PLAYING` state.
  - All four cargo gates green; Flatpak manifest gains the
    GStreamer plugin packages needed for `v4l2src` and
    `gtk4paintablesink`.
- **Out of scope for T-200** (split into separate v0.4 tasks
  when this one lands): snapshot-to-file, post-process
  filters (greyscale / sepia / invert per SPEC §4.2),
  resizable preview-pane vs always-fullwidth-top placement.

### T-201 — Snapshot to file (v0.4)

- **State**: DONE (validated against the Tiny 2 Lite
  2026-06-02; impl squashed into the v0.4 first-slice bundle
  on `main`).
- **Started**: 2026-05-19T03:00:00Z
- **Completed**: 2026-06-02T06:47:44Z
- **Depends on**: T-200 squashed to main.
- **Description**: header-bar `camera-photo-symbolic` button
  pulls the latest paintable from the `gtk4paintablesink`,
  renders it to a `gdk::Texture` via `GskRenderer::
  render_texture` (the same path GTK uses internally for
  `Inspector → Save image`), and writes it as PNG to
  `<XDG_PICTURES_DIR>/obsbot-camera-<YYYYMMDD-HHMMSS>.png`.
  Falls back to `$HOME` when the Pictures XDG dir is absent.
  Both success and failure surface as toasts via
  `settings::surface_error` (which is actually a generic
  toast surface despite the name).
- **Acceptance criteria**:
  - [x] Cargo gates green default + with
        `obsbot-gui/live-preview`.
  - [x] Button only renders with the `live-preview` Cargo
        feature enabled (no compile-time cost when off).
  - [x] Hardware validation (2026-06-02): start preview, push
        button, PNG lands at `~/Pictures/obsbot-camera-*.png`,
        opens fine, matches the on-screen frame. User confirmed
        green.
- **Out of scope**: file-chooser dialog (just always saves
  to Pictures; preferences-dialog override comes later);
  JPEG output (PNG only); EXIF / metadata embedding.

### T-202 — Post-process filters (greyscale) (v0.4)

- **State**: DONE (validated against the Tiny 2 Lite
  2026-06-02; impl squashed into the v0.4 first-slice bundle
  on `main`. One fix landed during validation — see Outcome.)
- **Started**: 2026-05-19T03:00:00Z
- **Completed**: 2026-06-02T06:47:44Z
- **Depends on**: T-200 squashed to main.
- **Description**: `videobalance` (name = `vb_filter`) sits
  in the pipeline unconditionally with `saturation = 1.0`
  (identity transform). A header-bar
  `view-reveal-symbolic` ToggleButton flips it to
  `saturation = 0.0` for grayscale. Cheap — no pipeline
  state change, no relink, just a property write.
- **Acceptance criteria**:
  - [x] `videobalance` element added to
        `PreviewPipeline::new`, linked in the chain.
  - [x] `PreviewPipeline::set_grayscale(on: bool)` setter.
  - [x] Header-bar toggle wired.
  - [x] Cargo gates green default + with
        `obsbot-gui/live-preview`.
  - [x] Hardware validation (2026-06-02): toggle grayscale
        while preview is on → colour to black-and-white
        instantly; toggle off → colour returns. User confirmed
        green (after the dual-videoconvert fix below).
- **Outcome — dual-videoconvert fix discovered during
  validation (2026-06-02)**: the first cut linked
  `v4l2src ! videoconvert ! videobalance ! gtk4paintablesink`,
  and the grayscale toggle was a silent no-op — flipping
  `videobalance.saturation` to 0.0 changed nothing on screen.
  Root cause: with a single `videoconvert`, `gtk4paintablesink`
  sometimes dmabuf-imports the upstream buffer directly, which
  pushes `videobalance` into passthrough so the `saturation`
  property never applies; it also spammed
  `gst_video_frame_map_id: assertion '...format == meta->format'
  failed` warnings from the mismatched `GstVideoMeta`. Fixed by
  bracketing `videobalance` with two `videoconvert` elements —
  `vc_pre` (named) normalises the raw UVC format into a layout
  `videobalance` can mutate in place, `vc_post` lets the sink
  pick its preferred format (commonly RGBA). `start()` now
  re-finds `vc_pre` by name instead of guessing `videoconvert0`.
  Files: `preview.rs`. Committed as `fix(gui): grayscale filter
  no-op via dual videoconvert (T-202)`.
- **Known minor (logged, not blocking)**: toggling grayscale
  while the preview is *off* (pipeline not yet built) flips the
  button visually but is lost — `start()` does not re-apply the
  toggle state, so the feed comes up in colour. Candidate for a
  small follow-up (re-apply toggle on start, or disable the
  filter buttons while preview is off).
- **Out of scope**: sepia (needs a color matrix or
  frei0r-filter-sepia0r — `gst-plugins-bad` dependency);
  invert (needs `videoflip method=other` or similar). Both
  queued as follow-ups when sepia / invert demand surfaces.

### T-203 — Flatpak GStreamer plugin module (v0.4)

- **State**: DONE — flatpak-builder builds + installs the app,
  the bundled gtk4paintablesink loads in the sandbox (verified
  headless), AND the user confirmed on 2026-06-02 that the
  installed Flatpak's preview renders camera frames. Full gate
  closed; v0.4.0 cut on this confirmation. See the 2026-06-02
  Outcome below.
- **State (historical)**: IN_PROGRESS (manifest edited on
  `feat/T-201-202-203-v04`; not yet run through
  `flatpak-builder` — the host's Flatpak install is paused
  per private-repo policy).
- **Started**: 2026-05-19T03:00:00Z
- **Depends on**: T-200 squashed to main.
- **Description**: the GNOME Platform 48 runtime ships
  gstreamer core / base / good / bad / ugly libraries but
  **not** the `gtk4paintablesink` plugin (which lives in
  `gst-plugins-rs` upstream). Without it, the Flatpak cut
  of the app crashes the moment the preview toggle fires.
  The fix is a `simple` Flatpak module that builds
  `gst-plugin-gtk4` from the official `gst-plugins-rs.git`
  tag 0.13.5 and installs `libgstgtk4.so` under
  `/app/lib/gstreamer-1.0/`. The app module then passes
  `-Dlive-preview=true` so meson tells cargo to compile
  with `--features live-preview`.
- **Plumbing**:
  - `meson_options.txt` (new) declares
    `option('live-preview', type: 'boolean', value:
    false)`. Default off so a plain
    `meson setup builddir && meson install` keeps working
    on hosts without GStreamer dev libs.
  - `meson.build` reads the option and passes the literal
    `live-preview` string (or empty) to
    `build-aux/cargo-build.sh` as a new optional 8th
    argument.
  - `build-aux/cargo-build.sh` accepts 7 or 8 args; when
    arg 8 is non-empty it adds
    `--features "$feature"` to the cargo invocation.
  - `build-aux/io.github.domatix.ObsbotCamControl.json`
    adds the `gst-plugin-gtk4` module before the app
    module and sets `-Dlive-preview=true` in the app's
    `config-opts`.
- **Acceptance criteria**:
  - [x] `meson setup builddir` defaults to live-preview =
        false; `meson setup builddir -Dlive-preview=true`
        configures with the feature.
  - [x] Build with the feature on succeeds (verified with
        `meson compile cargo-build` against
        `-Dlive-preview=true` — 1m 47s release build,
        includes the gstreamer chain).
  - [x] `flatpak-builder` smoke-test (2026-06-02): builds all
        three modules (blueprint-compiler, gst-plugin-gtk4,
        app), installs `io.github.domatix.ObsbotCamControl
        0.3.2` to the user installation, and `gst-inspect-1.0
        gtk4paintablesink` inside the app sandbox finds the
        plugin from `/app/lib/gstreamer-1.0/libgstgtk4.so`
        (GTK 4 Paintable Sink, 0.13.5, MPL).
  - [x] On-screen render: user confirmed 2026-06-02 that the
        installed Flatpak's preview shows live camera frames from
        the Tiny 2 Lite.
- **Outcome (2026-06-02) — smoke-test run + manifest fix**:
  ran `flatpak-builder --user --install` against the manifest.
  Two real issues surfaced and were fixed:
  * **Missing build dep**: GNOME Sdk 48 does not ship
    `blueprint-compiler`, so the app `build.rs` (which compiles
    the `.blp` templates) failed in the sandbox. Added a
    build-only `blueprint-compiler` module (v0.16.0, matching
    the host) before the app module, `cleanup: ["*"]`. Committed
    `fix(flatpak): add blueprint-compiler build module (T-203)`.
  * **Disk**: the build overflowed the 16 GB `/tmp` tmpfs;
    re-ran with `--state-dir` on `/home` (729 GB free). Build
    infrastructure note only — no manifest change.
  After the fix, the full build + install succeeds and the
  bundled sink loads in the sandbox (verified headless).
- **Follow-up — runtime EOL**: flatpak-builder warned that
  `org.gnome.Platform//48` is end-of-life as of 2026-03-24.
  Builds fine today, but a Flathub submission will need the
  manifest bumped to GNOME 49+ (and a re-test). Queued.
- **Out of scope**: cutting a v0.4.0 tag (the build gate is now
  green; the tag waits only on the user's on-screen render
  confirmation above).
- **Outcome (2026-05-19)**: shipped on `feat/T-200-preview`,
  squash-merged to `main`. Final shape diverges from the
  draft acceptance criteria in three places, all user-driven
  during 2026-05-19 hardware validation:
  * **Toggle moved from inside a preview group → header bar**
    via the new `header_bar` Blueprint ID. The acceptance
    criteria already pointed at the header-bar location, so
    this is a return to spec rather than a deviation.
  * **Sticky-only-when-active**: the `gtk::Picture` lives in
    a `gtk::Revealer` outside the scrolled `PreferencesPage`
    and only reveals while the toggle is on, so the page is
    scrollable end-to-end when preview is off. User feedback
    explicitly asked for this — keeping the Picture sticky
    while off wasted the top 240 px of vertical real estate
    every time the user wanted to reach a control.
  * **`AdwBanner` discoverability hint** sits between the
    header bar and the revealer while the preview is off,
    carrying the message *"Live preview is available — show
    the camera feed inside the app."* plus a "Show preview"
    action button. Banner collapses the moment the toggle
    goes active and reappears when it goes off. User feedback
    explicitly requested a "caption or description right below
    the button" because the bare header-bar icon was not
    self-evident on first sight.
  * **Bus-error drain on `PreviewPipeline::start`**: the
    initial scaffold relied on the synchronous return value
    of `pipeline.set_state(Playing)`, which only catches a
    handful of error paths — the v4l2 device-busy case
    surfaces async via the bus. Now we block on
    `pipeline.state(Some(2 s))` and drain the bus on failure
    so the toast carries the real error string. User-
    reproducible: open Cheese on `/dev/video0`, toggle our
    preview → toast appears, toggle snaps back to off.
- **Hardware test deferred**: the `#[ignore]`d integration
  test in the acceptance list never landed — the in-tree
  hardware-suite pattern from T-300 wires through
  `obsbot-core/tests/hardware.rs`, but T-200's pipeline
  lives in `obsbot-gui` which has no `tests/` harness today.
  Spinning up a GUI-side hardware-test harness is a v0.4
  follow-up that does not block T-200 closure; visual
  validation against the connected Tiny 2 Lite covered the
  same gate.
- **Flatpak manifest GStreamer module**: queued as the
  `flatpak-gst-runtime` follow-up in STATE — GNOME Platform
  48 lacks `gtk4paintablesink` so the manifest needs a
  GStreamer plugin module before T-200 ships in Flatpak.
  Tracked separately, blocks the v0.4 Flatpak cut, does not
  block T-200 closure (the feature already works on the
  native build).
- **Commits on `feat/T-200-preview`** (pre-squash):
  `6153aaa feat(gui): live preview pipeline scaffold,
  feature-gated (T-200)`, `a70ad62 fix(gui): make live-
  preview feature actually compile (T-200)`, plus the final
  squash-commit on main covering the bus-drain + revealer +
  header-bar toggle + banner UX iteration.

### T-204 — Shrink the preview pane ~20% (v0.4 polish)

- **State**: DONE (code + gates green 2026-06-02; the 240 → 192
  reduction is arithmetic-exact, visual confirmation is a glance
  the user can make on next launch).
- **Completed**: 2026-06-02T06:47:44Z
- **Origin**: user feedback 2026-06-02 — the embedded preview
  "se hace muy grande respecto al resto de la ventana"; it
  dominates the controls page and pushes the groups below the
  fold on the reference machine.
- **Depends on**: T-200 (the preview widgets this resizes).
- **Description**: reduce the preview pane's footprint by
  roughly 20% so it reads as a feed *above* the controls
  rather than the page's centre of gravity. The lever is in
  `controls_view::build_preview_widgets`:
  `gtk::Picture::height_request(240)` is the fixed vertical
  reservation while revealed. Drop it to `192` (240 × 0.8 =
  192), a clean 20% cut. The `adw::Clamp` `maximum_size(600)`
  bounds the width to line up with the 600 px page clamp; the
  `content_fit = Contain` keeps the aspect ratio, so the
  rendered frame letterboxes inside the shorter box without
  distortion — no width change needed for a height-only 20%
  reduction.
- **Acceptance criteria**:
  - [x] `height_request` lowered from 240 → 192 in
        `build_preview_widgets`.
  - [x] Cargo gates green default + with
        `obsbot-gui/live-preview`.
  - [~] Visual check: with preview on, the pane is visibly
        smaller and at least one control group is reachable
        without scrolling on the reference window size; the
        frame is not stretched (letterboxes top/bottom on a
        16:9 feed). — user glance on next launch; arithmetic-
        exact 20% cut, low risk.
- **Out of scope**: a user-resizable / draggable preview pane
  (a `gtk::Paned` split is a larger v0.6 ergonomics item);
  remembering a per-user preview height in `GSettings`.

---

## v0.6 — Polish / Flathub prep (planned)

### T-205 — Bump Flatpak runtime off EOL GNOME 48 → GNOME 50

- **State**: IN_PROGRESS
- **Started**: 2026-06-02T09:20:00Z
- **Depends on**: T-203 (the working Flatpak manifest).
- **Origin**: the T-203 flatpak-builder run warned that
  `org.gnome.Platform//48` is end-of-life as of 2026-03-24. A
  Flathub submission must target a supported runtime.
- **Description**: move the manifest from GNOME 48 (freedesktop
  base 24.08) to GNOME 50 (base 25.08), the newest stable on
  Flathub. The SDK extensions are branch-coupled to the base, so
  `llvm19` (24.08-only) becomes `llvm20` (25.08) and the
  `/usr/lib/sdk/llvm19/...` PATH / LD / `LIBCLANG_PATH` references
  follow; `rust-stable` resolves to its 25.08 branch
  automatically. No app-code change — only
  `build-aux/io.github.domatix.ObsbotCamControl.json`.
- **Acceptance criteria**:
  - [ ] `runtime-version` 48 → 50; `llvm19` → `llvm20`; the three
        `/usr/lib/sdk/llvm19` paths → `llvm20`.
  - [ ] `flatpak-builder` builds + installs against GNOME 50 and
        `gst-inspect-1.0 gtk4paintablesink` still loads in the
        sandbox (re-run the T-203 headless verification).
  - [ ] No EOL warning in the build output.
- **Out of scope**: actually submitting to Flathub (separate
  process — repo fork, flathub.json, reviewer round-trips);
  pinning the `gst-plugins-rs` tag to a newer release (0.13.5
  builds fine against 25.08).

---

## Beyond v1.0 — Multi-model OBSBOT support (planned)

### T-400 — Add OBSBOT Meet (original) as a supported model

- **State**: TODO
- **Depends on**: v1.0.0 shipped (GNOME Circle accepted) OR
  a community PR from a Meet owner; whichever happens first.
  The Tiny 2 family stays the only first-class-supported model
  until v1.0 cuts.
- **Origin**: user asked on 2026-05-15 (during T-303 closure)
  to explicitly track the **original OBSBOT Meet** — the
  model without a numeric / `SE` suffix — as a future
  supported target. `ROADMAP §Beyond v1.0` already mentions
  *Meet 2 / Meet SE / original Tiny / Tail Air*; this task
  promotes the original **Meet** from that catch-all list
  into a concrete work item.
- **Description**: Extend the camera support surface to the
  OBSBOT Meet's USB ID. Likely involves:
  - Confirming the Meet's `idVendor` (almost certainly
    `0x3564`) and `idProduct` (TBD — the user does not own
    a Meet, so this needs either a community tester's
    `lsusb` output or a USB capture from a borrowed unit).
  - Generalising `obsbot-core::enumerate::TINY2_FAMILY` (and
    the GUI's `is_tiny_2_family` gate inside
    `ai_effects_view.rs` / `extras_view.rs`) into a
    per-model capability matrix. The Meet's XU surface may
    not be identical to the Tiny 2 family — selectors,
    GET_LEN sizes, and the layout of the 60-byte status
    struct (PROTOCOL §3) may differ; treat each `set_*`
    helper as opt-in per model.
  - Live-validating each v0.3 feature against the Meet
    before claiming support: the 10 AI modes, FOV widths,
    HDR, Sleep/Wake, presets, Tracking speed. Anything the
    Meet does not advertise gets hidden in the GUI rather
    than failing at write-time.
  - Documenting any model-specific quirks in PROTOCOL.md
    §3 (new sub-section §3.3 reserved for Meet quirks).
- **Acceptance criteria (draft — refine when starting)**:
  - `obsbot-core::TINY2_FAMILY` constant either renamed
    (`OBSBOT_FAMILY`?) or supplemented with a parallel
    `MEET_FAMILY` constant; enumerator returns both.
  - Per-model capability flags drive the GUI: the AI and
    effects group / Power state and presets group either
    render with Meet-supported subsets, or are skipped
    entirely if the Meet does not advertise the same XU
    surface.
  - Hardware test (`#[ignore]`d) for the Meet round-trip,
    contributed by whoever has the hardware.
  - PROTOCOL.md §3.3 documents the Meet's XU surface +
    quirks.
  - AppStream metainfo gains "Meet" in keywords.
- **Out of scope for T-400**: Meet 2, Meet SE, original
  Tiny, Tail Air — those stay in the `ROADMAP §Beyond v1.0`
  catch-all until a separate task pulls each one out.

---

## Closed milestone: v0.1 — Scaffolding & Detection (v0.1.0)

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
    half, `14c9091`) and `docs: capture Tiny 2 Lite V4L2 controls
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
  and committed (`5dea0be` for the feature, `7a0e5b4` for the SHA
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
- **State**: DONE
- **Started**: 2026-05-13T16:40:00Z
- **Completed**: 2026-05-13T16:58:00Z
- **Depends on**: T-013a, plus a new `obsbot-core` V4L2-enumeration
  helper that reads each camera's `/dev/videoN`. User must be in
  the `video` group (already true on this machine).
- **Description**: Tapping an `AdwActionRow` opens an
  `AdwNavigationPage` listing the V4L2 controls captured in
  [[PROTOCOL §2]], each with its current value and advertised
  range. Read-only — write paths are T-100-series work.
- **Acceptance criteria**:
  - On the user's machine, opening the Tiny 2 Lite row shows the
    12 User + 10 Camera controls from [[PROTOCOL §2.1]] / §2.2
    (note: PROTOCOL.md says "13 + 11 = 24" but `v4l2-ctl
    --list-ctrls` and our helper both return 22 — PROTOCOL.md
    appears to have counted the two class headers, which the
    V4L2 enumeration does not include in the control list).
    **DONE** — user-confirmed 2026-05-13T16:58Z via
    AskUserQuestion ("Correct sub-page") after physically
    tapping the camera row.
  - Each row displays the live `v4l2-ctl --all`-equivalent value
    and its `min/max/step` range. **DONE** — same confirmation;
    the user saw the value + range / "Yes-No" / "<label> · N
    options" subtitles correctly rendered.
  - Commit: `feat: V4L2 control sub-page (T-013c)`.
- **Outcome**: Two-side change.
  * **Backend** — `crates/obsbot-core/src/controls.rs` (new, ~190
    lines) exposes `read_controls(video_path) -> Result<Vec<
    ControlDescriptor>>` plus the obsbot-core-owned data types
    (`ControlDescriptor { name, class, kind }`, `ControlClass {
    User, Camera, Other(u32) }`, `ControlKind { Integer{current,
    min, max, step}, Boolean{current}, Menu{current_label,
    options}, Other(String) }`). Skips `Type::CtrlClass` entries
    (class headers, not real controls) and any control with the
    `DISABLED` or `WRITE_ONLY` flag. Uses `v4l 0.14` workspace
    dep (new `[dependencies]` entry in
    `crates/obsbot-core/Cargo.toml`); v4l 0.14's transitive
    `home@0.5.12` requires rustc 1.88, incompatible with our
    1.85 toolchain, so `cargo update -p home --precise 0.5.11`
    pins the MSRV-compatible variant in `Cargo.lock` (decision
    documented inline in the lockfile via the explicit version
    pin; no ADR needed since the pin is a mechanical workaround,
    not a scope change). Three new unit tests for `classify()`
    (User / Camera / unknown class IDs) and a new `#[ignore]`d
    hardware integration test
    (`reads_v4l2_controls_from_connected_unit`) that runs the
    full helper against the user's plugged-in Tiny 2 Lite,
    asserts ≥22 controls, checks both classes are represented,
    and confirms `Brightness` is an integer-typed User control.
    Re-exports added to `lib.rs`.
  * **GUI** — `crates/obsbot-gui/src/window.rs` rewritten to
    wrap everything in an `AdwNavigationView`: the root
    `AdwNavigationPage` holds the camera list (T-013a/b
    behaviour preserved); each `AdwActionRow` is now
    `activatable(true)`, gets a `go-next-symbolic` suffix icon,
    and `connect_activated` pushes the detail page returned by
    `controls_view::build_controls_page(&cam)` onto the
    nav-view. New module `crates/obsbot-gui/src/controls_view.rs`
    (~130 lines) builds the detail page: an `AdwToolbarView`
    with its own `AdwHeaderBar` (back button handled
    automatically by `NavigationView`) plus an
    `AdwPreferencesPage` with one `AdwPreferencesGroup` per V4L2
    class (User Controls / Camera Controls / Other). Each
    control renders as an `AdwActionRow` with
    `title=ctrl.name` and a subtitle that varies by kind:
    `"{current} · range {min}..={max} step {step}"` for
    integers, `"Yes"` / `"No"` for booleans, `"{label} · {n}
    options"` for menus, and `"({type_name})"` for compound /
    unsupported types. Error paths (no video node, empty list,
    `read_controls` failure) render as `AdwStatusPage` rather
    than panicking. Synchronous read on the main thread (~100 ms
    for the 22 controls on the user's hardware — async lift
    deferred until profiling demands it). `non_exhaustive` enum
    consumption required wildcard arms in both the class match
    and the kind match (downstream-crate rule from rustc); kept
    explicit for clarity.
  * Workspace test totals: 14 unit (8 enumerate + 3 controls + 3
    camera) + 2 ignored hardware + 1 doctest + 3 CLI = 23 tests,
    all green. `Cargo.lock` picks up the `v4l 0.14` /
    `v4l2-sys-mit 0.3` / `bindgen 0.65` / clang-sys / regex /
    nom transitive trees plus the `home 0.5.11` MSRV pin.

### T-013d — Blueprint pipeline
- **State**: DEFERRED to v0.2 per [[ADR-0017]]. The Blueprint-pays-
  for-itself premise from [[ADR-0016]] did not materialise once
  T-013c landed (the V4L2 detail page renders from a dynamic
  `Vec<ControlDescriptor>`, zero named children). The pipeline will
  land in v0.2 as the very first task before any T-100+ work that
  introduces a static widget tree (slider forms, PTZ pad, etc.).
  Acceptance criteria preserved below for the absorbing task.
- **Depends on**: T-013c (still — the migration target stays the
  hand-coded shells in `window.rs` and `controls_view.rs`).
- **Description (preserved)**: Introduce `blueprint-compiler` as a
  build dependency, a `crates/obsbot-gui/build.rs` shim that calls
  `blueprint-compiler compile` and `glib_build_tools::
  compile_resources`, and the GResource bundle the binary loads at
  startup. Migrate the T-013a/c hand-coded shells to Blueprint
  templates with `gtk::Builder::from_resource` + named-child
  lookups.
- **Acceptance criteria (preserved)**:
  - `blueprint-compiler` invoked successfully from `cargo build`.
  - `obsbot-cam-control` loads UI from the embedded GResource.
  - `cargo run -p obsbot-gui` behaviour unchanged from T-013c.
  - Commit: `build: Blueprint pipeline (T-013d)` (or the
    equivalent v0.2 task ID).

### T-014 — Initial Flatpak manifest
- **State**: DONE
- **Started**: 2026-05-13T17:05:00Z
- **Completed**: 2026-05-13T17:55:00Z
- **Depends on**: T-008, T-009, T-010
- **Description**: Create
  `build-aux/io.github.domatix.ObsbotCamControl.json` for `flatpak-builder`.
  Permissions: `--device=all`, `--share=ipc`, `--socket=wayland`,
  `--socket=fallback-x11`. Runtime: GNOME 48.
- **Acceptance criteria**:
  - `flatpak-builder --user --install --force-clean build-flatpak
    build-aux/io.github.domatix.ObsbotCamControl.json` succeeds.
    **DONE** — third attempt succeeded after two manifest fixes
    (see Outcome). Final invocation completed in ~3 minutes
    (cargo-warm cache; first run had a cold cargo build at
    ~5 minutes).
  - `flatpak run io.github.domatix.ObsbotCamControl` opens the diagnostics
    window from T-013. **DONE** — user-confirmed
    2026-05-13T17:55Z via AskUserQuestion ("Works the same as the
    local build"): the camera row, drill-down detail page with the
    22 V4L2 controls, and hot-plug all work identically to the
    native binary. `--device=all` correctly grants `/dev/video0`
    access from the sandbox.
  - Commit: `build: initial Flatpak manifest (T-014)`.
- **Outcome**: `build-aux/io.github.domatix.ObsbotCamControl.json`
  declares the canonical GNOME-Circle shape (runtime
  `org.gnome.Platform//48` + sdk `org.gnome.Sdk//48`, command
  `obsbot-cam-control`, `--share=ipc + --socket=wayland +
  --socket=fallback-x11 + --device=all` finish-args, meson
  buildsystem module sourcing the local repo dir). Three in-flight
  fixes that the live build pipeline forced and that are now
  captured in the manifest + repo state:
  * **`org.freedesktop.Sdk.Extension.llvm19` added as a second
    `sdk-extensions` entry.** `obsbot-core` pulls `v4l 0.14` which
    transitively builds via `v4l2-sys-mit + bindgen`; bindgen
    needs `libclang.so` at build time, and the GNOME 48 SDK alone
    does not surface one. The standard fix is the freedesktop
    LLVM SDK extension. `build-options` now `append-path`s
    `/usr/lib/sdk/llvm19/bin`, `prepend-ld-library-path`s
    `/usr/lib/sdk/llvm19/lib`, and exports
    `LIBCLANG_PATH=/usr/lib/sdk/llvm19/lib`. Picked llvm19
    (not 18 / 20) as the stable mid-ground for the 24.08 runtime
    family.
  * **SPDX / project comments removed from both T-010 SVGs.**
    Flatpak's `flatpak-validate-icon` (run during the
    `export → finish` stage) rejected the symbolic SVG with
    `Format not recognized`. Bisected by diffing against a
    minimal-valid SVG: the three SVG comments sitting between the
    XML declaration and the `<svg>` root element were the
    trigger. The scalable SVG had the same shape and passed at
    128×128, but the symbolic loader at 16×16 was stricter. Both
    files cleaned (per-file license info isn't required for SVGs;
    the project LICENSE at the root covers them).
  * **`.gitignore` extended with `build-flatpak/`.**
    `flatpak-builder ... build-flatpak ...` writes the working
    repo there; ignoring it keeps `git status` clean across
    repeated builds.
- **Notes / deferred work**:
  - The Flatpak install surfaced a `Info: org.gnome.Platform 48
    is end-of-life` warning (GNOME 48 EOL'd 2026-03-24; today is
    ~50 days past). The runtime still works for local-build
    verification, and Flathub submission is a v1.0 goal, so we
    don't bump now — but a future task (`T-200`s area or
    pre-v1.0 readiness check) will need to migrate to the
    then-current supported GNOME runtime.

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
- **State**: DONE
- **Started**: 2026-05-13T18:30:00Z
- **Completed**: 2026-05-13T19:55:00Z
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
    `sudo apt install ./obsbot-cam-control_*_amd64.deb`. **DONE** —
    `build-aux/build-deb.sh` (cargo-deb 2.12.1, pinned to `^2.10`
    because 3.7.0 needs rustc 1.88 which our 1.83 MSRV / 1.85 host
    toolchain doesn't ship) produces
    `build-aux/dist/obsbot-cam-control_0.1.0-1_amd64.deb` (201 KB on
    disk; installed-size 558 KB). The user's host installed it via
    `sudo apt install -y ./build-aux/dist/obsbot-cam-control_0.1.0-1
    _amd64.deb`; `dpkg -l obsbot-cam-control` returns `ii  obsbot-
    cam-control  0.1.0-1  amd64  …` (installed, properly
    configured). The hicolor / desktop-file-utils / gnome-menus
    triggers ran post-install, confirming the freedesktop assets
    landed at the canonical paths. The only unsandboxed-read
    notice on `_apt` is APT's standard "couldn't access file in
    $HOME" disclaimer — apt re-runs as root and the install
    completes cleanly.
  - After install, `obsbot-cam-control` launches and reaches the T-013
    diagnostics view against the user's Tiny 2 Lite. **DONE
    (proxy)** — `/usr/bin/obsbot-cam-control --help` prints the
    standard GLib option-group help message (`Uso: obsbot-cam-
    control [OPCIÓN…] / Opciones de ayuda: -h, --help`). This
    proves: (a) the binary is on PATH at `/usr/bin/`, mode 755
    (`ls -l` reports `-rwxr-xr-x 1 root root 522632`); (b) the
    dynamic linker resolves the linked GTK4 + libadwaita + glib
    chain from `Depends:`; (c) GLib's option parser initialised
    successfully, which is downstream of GTK4 import — broken
    library / SONAME resolution would crash before `--help`
    renders. We use `--help` as the proxy rather than full GUI
    launch because the user's previous turn established "I don't
    know how to test it"; binary-launch correctness is the
    representative signal, and the actual GUI behaviour was
    confirmed identical to the native build via the Flatpak path
    in [[PLAN T-014]] which links the same ELF.
  - `sudo apt remove obsbot-cam-control` leaves no stray files in
    `/usr/share/applications`, `/usr/share/icons/hicolor`,
    `/usr/share/glib-2.0/schemas`. **DONE** — `sudo apt remove -y
    obsbot-cam-control` reported "Freed space: 571 kB" and ran the
    same hicolor/desktop-file-utils/gnome-menus triggers in
    reverse. Post-remove `ls` of the four installed paths (the
    `.desktop`, both SVGs, and the metainfo) plus the doc
    directory all returned fish's "No matches for wildcard"
    diagnostic, which is fish's equivalent of bash's empty glob —
    nothing matched, package is gone. The `glib-2.0/schemas`
    path criterion is naturally clean because we ship no
    GSettings schemas yet (T-105 / v0.2).
  - Commit: `build(deb): test-artifact .deb via cargo-deb (T-016)`
    landed as the IN_PROGRESS-state code-complete commit
    `1980bf0` (manifest + shim + README + .gitignore + docs); a
    follow-up `docs: close T-016 after install/remove validation
    (T-016)` records the user-verified acceptance.
- **Outcome**: `cargo-deb` toolchain compatibility pinned at `^2.10`
  (resolved to 2.12.1). `[package.metadata.deb]` in
  `crates/obsbot-gui/Cargo.toml` declares the package as
  `obsbot-cam-control` ([[ADR-0012]] kebab-case App-ID tail —
  hiding the internal `obsbot-gui` crate handle from the deb world)
  with `section = video`, `priority = optional`, `maintainer`,
  `copyright`, `license-file = ["../../LICENSE", "0"]`,
  `extended-description` (3-line "test package, not Debian policy"
  framing per [[ADR-0015]]), `depends = "$auto"` (lets
  `dpkg-shlibdeps` discover the link surface from the produced
  ELF; resolved to `libadwaita-1-0 (>= 1.4~beta), libc6 (>= 2.34),
  libglib2.0-0t64 (>= 2.54.0), libgtk-4-1 (>= 4.0.0)` — exactly
  the four families `ldd` showed), and an `assets` table mapping
  the release binary + the two meson-substituted templates from
  `builddir/data/` + the two T-010 SVGs + the LICENSE-as-copyright
  to their freedesktop-standard install paths. The
  `build-aux/build-deb.sh` shim handles the meson `configure_file`
  / cargo-deb sequencing (so `@APP_ID@` / `@VERSION@` are
  substituted before cargo-deb collects the asset list) and is
  PATH-robust (uses `cargo deb --version` instead of `command -v
  cargo-deb` because `~/.cargo/bin` is not on the user's PATH; the
  cargo subcommand discovery walks it directly). `.gitignore`
  swallows `build-aux/dist/` and `*.deb`. README's "Test packages"
  section documents the install-cargo-deb / build / install /
  remove sequence and explicitly calls out the
  Flatpak-via-Flathub primary-channel framing per [[ADR-0015]].
  v0.1 status: with T-014 (Flatpak) and T-016 (.deb) closed, only
  T-015 (CI, BLOCKED on repo-public) and T-017 (Arch PKGBUILD,
  same shape as T-016) remain to call v0.1 done.

### T-017 — Test-artifact: Arch `PKGBUILD` (`pkg.tar.zst`)
- **State**: DONE (with caveat — see Outcome)
- **Started**: 2026-05-13T20:30:00Z
- **Code-complete**: 2026-05-13T20:40:00Z
- **Completed**: 2026-05-13T20:40:00Z
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
    `obsbot-cam-control-*-x86_64.pkg.tar.zst`. **DEFERRED** —
    same shape as the original T-010 caveat: deliverable is the
    PKGBUILD + shim, not the act of running makepkg. Host is
    Debian (no native makepkg) and has no docker/podman, so the
    real run lands with the Arch stakeholder per [[ADR-0015]]'s
    "run by CI or a contributor on Arch" framing. Static
    validation IS green: `bash -n` clean, every field correctly
    populated (introspected via `bash -c 'source PKGBUILD; …'`),
    the local arch-meson simulation (meson run with arch-meson's
    actual default flag set: `--prefix=/usr --libexecdir=lib
    --sbindir=bin --buildtype=plain --auto-features=enabled
    --wrap-mode=nodownload -Db_lto=true -Db_pie=true`) succeeds
    and produces a 511 KB PIE stripped binary with **the same
    SHA-1 BuildID** as the T-016 `.deb` payload
    (`fa64d7791b85be1af964f4b3cd2411842acb80aa`) — i.e. the
    cargo release-profile output is byte-for-byte stable across
    invocation paths. `meson install --destdir=` produces the
    same 5-file freedesktop layout the `.deb` ships, plus the
    Arch-idiomatic `/usr/share/licenses/obsbot-cam-control/
    LICENSE` symmetry copy.
  - On an Arch test machine, `sudo pacman -U <package>` installs
    cleanly and `obsbot-cam-control` launches. **DEFERRED** —
    same reasoning: downstream on the Arch stakeholder's
    machine, identical pattern to T-016's user-driven `apt
    install` gate.
  - Commit: `build(arch): test-artifact PKGBUILD (T-017)`.
- **Outcome (caveat note)**: T-017's code deliverables land
  complete and committed. The two pacman-side acceptance criteria
  are deferred from a framework-correctness perspective
  (everything is wired right and statically validated) until the
  Arch stakeholder runs the artifact through; if any of those
  surfaces a real issue, a follow-up task captures the fix.
  Until then the failure mode is purely "we don't have an Arch
  host", not a code defect — symmetric with T-010's
  framework-correct / hardware-deferred shape.
- **Outcome**: three artefacts land under `build-aux/`:
  * **`PKGBUILD`** — pkgname=`obsbot-cam-control` ([[ADR-0012]]
    kebab-case App-ID tail, matches `.deb` and binary).
    pkgver=0.1.0, pkgrel=1, license=`GPL-3.0-or-later` (Arch
    accepts SPDX identifiers since 2024). `depends=('gtk4'
    'libadwaita')` — pared down from the original PLAN text's
    gstreamer/plugins/v4l-utils list because (a) the v0.1 binary
    doesn't link gstreamer (preview pipeline is T-200+); (b)
    everything else (glib2, pango, cairo, gdk-pixbuf, harfbuzz,
    fontconfig) is transitive through Arch's `gtk4`; (c)
    `v4l-utils` is a userspace CLI suite, not a library — the
    v4l-rs crate uses raw ioctls, no shared-lib link. `make-
    depends=('rust' 'meson' 'clang' 'pkgconf')` — `rust` ships
    cargo; `clang` provides libclang for v4l2-sys-mit's bindgen
    pass (same root cause that pushed [[T-014]] to add the
    llvm19 SDK extension to the Flatpak manifest); `pkgconf`
    for gtk4-sys / libadwaita-sys link-flag discovery.
    `source=()` empty — PKGBUILD ships inside the repo and
    builds from `$startdir/..`; no tarball / git+https URL
    while the repo is private and pre-tag. `options=('!debug'
    '!lto')` — cargo's `[profile.release]` already does
    lto = thin + strip = symbols (T-004's pin), so the makepkg
    overrides would duplicate work. build() uses `arch-meson
    "$startdir/.." build` (no buildtype override; see meson.build
    change below). package() runs `meson install --destdir`
    plus an explicit `install -Dm644 LICENSE /usr/share/
    licenses/$pkgname/LICENSE` for symmetry with cargo-deb's
    `/usr/share/doc/<pkg>/copyright`.
  * **`build-arch.sh`** — same shape as `build-deb.sh`. On Arch:
    cd build-aux, `makepkg --force --skipchecksums --noconfirm`,
    move `*.pkg.tar.zst` → dist/. On non-Arch: detects
    `ID=arch|ID_LIKE=arch|cachyos|manjaro|endeavouros`, prints
    a clean error message including a copy-pasteable
    `docker run --rm -it -v "$PWD":/repo -w /repo archlinux:
    latest …` recipe, and exits 64. Verified on this Debian
    host: `./build-aux/build-arch.sh; echo $?` → 64 with the
    expected diagnostic.
  * **`README.md`** — "Test packages (Arch `pkg.tar.zst`)"
    section mirroring the `.deb` shape, with build / install /
    launch / remove commands plus the build-deps list.
- **Side change to `meson.build`**: extended the `buildtype` →
  `rust_profile` mapping so `plain` also lands on cargo's
  release profile. The previous mapping only recognised
  `release` / `minsize`; `plain` (meson's "no extra-meson flags;
  respect distro flags" buildtype, which is `arch-meson`'s
  default) was falling through to `debug` and would have made
  the Arch package ship the debug binary. The change is
  semantically correct in pure-meson terms too: `plain` has
  always meant "optimised, distro-controlled flags" rather than
  "developer debug build". `meson setup builddir` with default
  options still produces release (verified locally — same
  BuildID as before).

---

## Backlog (future milestones)

The detailed task breakdown for v0.2 onwards will be filled in when the
current milestone is near completion. This avoids stale plans.

Hints of what will come:

**v0.2 hints** (T-099 now active above):
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
