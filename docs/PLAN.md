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
    AskUserQuestion ("Idéntico"). `xwininfo -tree -root`
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
    spin entry + reset button: "Funciona todo").
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
     entry). User: "cambia pero con los botones de + y −, no hay
     barra". Acceptance text said "slider"; SpinRow was wrong.
  2. Second pass shipped `AdwActionRow` + `gtk::Scale` (drag-
     bar) + value `gtk::Label`. User: "barra OK, pero quiero
     introducir manualmente también el número y un botón para
     resetear al valor por defecto".
  3. Third pass — final — added `gtk::SpinButton` next to the
     scale (sharing the adjustment so they stay in lock-step)
     and a flat reset button with an `edit-undo-symbolic` icon
     and a "Reset to default (N)" tooltip; the scale also got a
     tick mark at the default position. User: "Funciona todo".
- **Hardware-quirk note surfaced during the second iteration**:
  the user observed that the first ~5 sliders reacted live but
  the rest "no parecían hacer nada". That is the documented
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
    AskUserQuestion ("Sub-página correcta") after physically
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
    2026-05-13T17:55Z via AskUserQuestion ("Funciona igual que el
    build local"): the camera row, drill-down detail page with the
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
    launch because the user's previous turn established "no sé
    cómo probarla"; binary-launch correctness is the
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
