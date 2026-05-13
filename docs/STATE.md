# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none
active_task_state: idle
last_completed_task: T-107
last_milestone: v0.1.0  # tag 5e005fd
last_commit: feat(gui): About dialog with credits (T-106)  # a688714 (T-107 commit pending in this turn)
last_step: T-107 DONE — top-level `po/` populated (`LINGUAS = es`, `POTFILES.in` listing the six GUI .rs files, `meson.build` invoking `i18n.gettext('obsbot-cam-control', preset: 'glib')`, header-only `es.po`); root `meson.build` now calls `subdir('po')` and forwards `localedir` to `build-aux/cargo-build.sh` as a 7th arg; the wrapper exports it as `OBSBOT_LOCALEDIR` so `build.rs` stage 4 re-emits it via `cargo:rustc-env` for `option_env!`. `crates/obsbot-gui/src/i18n.rs` is a thin wrapper over `gettextrs::{setlocale, bindtextdomain, bind_textdomain_codeset, textdomain, gettext}`; `main.rs` calls `i18n::init()` before `application::run`. User-facing strings in `window.rs` / `controls_view.rs` / `wb_group.rs` / `exposure_group.rs` / `ptz_pad.rs` / the About-dialog copyright + section title now flow through `gettext()`; `ptz-pad.blp` literals were marked with `_("...")` for v0.6's blueprint extraction follow-up. Gates: fmt, clippy -D warnings, 14 unit + 1 doctest + 1 settings unit-test = 16 native pass; 5 hardware tests still `#[ignore]`d. Caveat: this Debian 13 host has `gettext-base` only (no `msgfmt`); meson logs the standard "Gettext not found" warning and skips the .pot target — wiring is correct (`OBSBOT_LOCALEDIR` is baked into the release binary, verified via `strings`), CI / Flatpak builders ship full gettext.
next_step: Advance to T-108 (toast-based error surfacing) — wrap the controls page in an `adw::ToastOverlay`, pass the overlay handle into `integer_scale_row` / `boolean_switch_row` / `menu_combo_row` / `wb_group::*` / `exposure_group::*` / `ptz_pad::*` write callbacks, and dispatch a `Failed to set {control}: {error}` toast on every `write_control` failure (replacing the current `eprintln!`). Keep GSettings-save eprintln in place (justified inline). Commit `feat(gui): toast-based write-error surfacing (T-108)`.
blockers: none.
working_tree:
  pre_commit_modified: []
  pre_commit_untracked: []
  pre_commit_deleted: []
pending_user_actions:
  # First autonomous run (T-101..T-105) + second autonomous run
  # (T-106..T-110) accumulate here for one single validation pass.
  - T-106: click the hamburger button in the header bar → "About
    Obsbot Cam Control" → confirm version, license, repo link,
    issue-tracker link, and the "Reverse-engineering references"
    acknowledgement block render correctly.
  - T-101: drag the 8 PTZ buttons + center-reset, confirm pan/tilt;
    drag the vertical zoom slider, confirm the frame zooms; toggle
    "Auto-focus" off and drag "Manual focus" — focus distance changes.
  - T-102: find "Power Line Frequency" in the User Controls section,
    change between Disabled / 50 Hz / 60 Hz (visible effect is subtle
    — usually just no error is enough). Toggle "White Balance,
    Automatic" off, then on; confirm the "White Balance Temperature"
    row greys out / wakes up automatically (generic INACTIVE handler).
  - T-103: confirm the four WB controls now live inside a dedicated
    "White balance" group with a description text, near the top of
    the page, instead of scattered in the User Controls section.
  - T-104: in the "Exposure" group, change "Exposure, Auto" to
    "Manual"; drag "Exposure Time, Absolute" — preview gets darker
    or brighter. Switch back to "Auto"; confirm the exposure time
    slider greys out.
  - T-105: pick any non-default value (e.g. brightness = 75), close
    the GUI, re-launch, drill into the camera — the slider should
    come up at 75 and the camera image should reflect it. Cleanup
    afterwards (optional): `gsettings reset-recursively io.github.
    domatix.ObsbotCamControl`.
  - T-010 (still): observe whether GNOME Shell paints our webcam icon
    when you launch the app.
  - T-017 (Arch stakeholder, whenever): build/install/remove the
    PKGBUILD on Arch.
updated_at: 2026-05-14T01:25:00Z  # T-107 closed, T-108 next in the autonomous batch
