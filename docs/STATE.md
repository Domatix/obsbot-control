# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none
active_task_state: idle
last_completed_task: T-105
last_milestone: v0.1.0  # tag 5e005fd
last_commit: feat(gui): per-camera GSettings persistence (T-105)  # d7a13a8
last_step: Autonomous run T-101..T-105 closed. v0.2 milestone status — T-099 + T-100 (last session) + T-101 (PTZ pad) + T-102 (Menu writes + INACTIVE grey-out) + T-103 (White balance group widget) + T-104 (Exposure group widget) + T-105 (GSettings persistence) all DONE. v0.2 backlog remaining: T-106 About dialog, plus the symbolic-icon + Adwaita-styling polish bullet (mostly delivered already), plus any UX cleanups the user's validation pass surfaces. ADR-0019 documents the re-scope of T-102 from "Zoom slider" to "Menu writes + INACTIVE grey-out" (Zoom slider absorbed into T-101's pad). Gates: fmt / clippy / 14 unit + 1 doctest + 1 settings unit-test = 16 native tests passing; 5 / 5 hardware tests pass under `cargo test -- --ignored`. Commits since last STATE update: 0bb49b4 (T-101), c204ffd (T-102), b3e6040 (T-103), 2d67ba8 (T-104), d7a13a8 (T-105).
next_step: Stop. The user requested 5 tasks (T-101..T-105) and then a pause. Next time we resume, T-106 (About dialog with credits + license info, the last "v0.2 hint" task) and any UX adjustments surfaced by the user's validation pass are the natural follow-ons. After T-106 + validation we can evaluate v0.2.0 tag-readiness per CLAUDE.md §7.
blockers: none.
working_tree:
  pre_commit_modified: []
  pre_commit_untracked: []
  pre_commit_deleted: []
pending_user_actions:
  # All five tasks of this run share one validation pass — list collected
  # for the user to walk through in a single GUI session.
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
updated_at: 2026-05-14T00:40:00Z  # five-task autonomous run closed, awaiting user validation
