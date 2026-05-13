# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none
active_task_state: idle
last_completed_task: T-108
last_milestone: v0.1.0  # tag 5e005fd
last_commit: feat(gui): gettext scaffolding (T-107)  # 39c1206 (T-108 commit pending in this turn)
last_step: T-108 DONE — toast-based error surfacing. `controls_view::build_controls_page` wraps the dynamic body in an `adw::ToastOverlay` and calls `settings::bind_toast_overlay` to register it. `settings.rs` gains a `thread_local!` `Option<glib::WeakRef<adw::ToastOverlay>>` plus a `surface_error(msg)` helper that upgrades the weak ref and pops a 5s `adw::Toast`; falls through to `eprintln!` when no overlay is bound (cargo run before navigating into a camera) or the previously-bound overlay has been dropped (page navigation race). `settings::write_and_save` now routes V4L2 write failures through `surface_error(gettext("Failed to set {name}: {error}").replace(...))`; GSettings save failures stay on `eprintln!` (inline-justified — best-effort, transparent recovery next session). No widget-builder signature changes — the thread_local sidesteps the alternative of threading `Rc<adw::ToastOverlay>` through every closure. Gates: fmt, clippy -D warnings, 14 unit + 1 doctest + 1 settings unit-test = 16 native pass; 5 hardware ignored.
next_step: Advance to T-109 (AppStream `<releases>` for v0.2.0) — add a `<releases>` block in `data/io.github.domatix.ObsbotCamControl.metainfo.xml.in` with a `<release version="0.2.0" date="@RELEASE_DATE@">` entry; release notes cover T-099..T-108 (Blueprint pipeline, image controls, menu writes + INACTIVE grey-out, WB / Exposure groups, PTZ pad, GSettings persistence, About dialog, gettext scaffolding, toast errors). Substitute `@RELEASE_DATE@` via `data/meson.build`'s `configuration_data()` with a default `unreleased`; tag-time bumps it to the ISO date. Validate via `meson test -C builddir validate-metainfo`. Commit `docs(appstream): v0.2.0 release notes (T-109)`.
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
  - T-108: while on a camera detail page, yank the USB cable
    (or temporarily revoke `/dev/videoN` permissions via `chmod
    000`) and drag a slider; confirm a toast appears reading
    "Failed to set <control name>: <error>" instead of a silent
    stderr line. Re-plug to restore.
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
updated_at: 2026-05-14T01:40:00Z  # T-108 closed, T-109 next in the autonomous batch
