# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none
active_task_state: idle
last_completed_task: T-106
last_milestone: v0.1.0  # tag 5e005fd
last_commit: feat(gui): About dialog with credits (T-106)  # pending in this turn
last_step: T-106 DONE — `window.blp` carries a `menu primary_menu` (About + Quit items) and a `Gtk.MenuButton` in the `Adw.HeaderBar`; `application::register_actions` now takes the App ID and registers `app.about`, whose callback presents an `adw::AboutDialog` populated from `CARGO_PKG_*` + a credits acknowledgement section for `aaronsb/obsbot-camera-control` and `taxfromdk/obsbot_tiny_reversing` (PROTOCOL.md §0). Gates: fmt, clippy -D warnings, 14 unit + 1 doctest + 1 settings unit-test = 16 native pass; 5 hardware tests still ignored (no hardware-touching code changed this task).
next_step: Advance to T-107 (gettext scaffolding) — add top-level `po/` (LINGUAS, POTFILES.in, meson.build, empty es.po), wire `subdir('po')` in root meson.build, add `gettext-rs` to workspace deps, add `crates/obsbot-gui/src/i18n.rs` with `gettext()` + textdomain init, route user-facing string literals in window.rs / controls_view.rs / wb_group.rs / exposure_group.rs / ptz_pad.rs through it, run gates, commit `feat(gui): gettext scaffolding (T-107)`.
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
updated_at: 2026-05-14T01:00:00Z  # T-106 closed, T-107 next in the autonomous batch
