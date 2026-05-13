# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none
active_task_state: idle
last_completed_task: T-109
last_milestone: v0.1.0  # tag 5e005fd
last_commit: feat(gui): toast-based write-error surfacing (T-108)  # 6df5294 (T-109 commit pending in this turn)
last_step: T-109 DONE — AppStream `<releases>` draft for v0.2.0. New `<release version="0.2.0" date="2026-05-14" type="development">` entry on top of the existing v0.1.0 record (which lost its `@VERSION@` placeholder and gained literal `"0.1.0"` so the historical row stays stable through future project-version bumps). User-facing release notes cover PTZ pad, image controls + INACTIVE grey-out, WB / Exposure groups, anti-flicker, GSettings persistence, About dialog, toast errors, and the gettext scaffolding (positioned as an "internal" bullet so non-developers know what changed but the entry stays user-relevant). Vendor features (HDR / FOV / auto-framing) explicitly punted to v0.4. Date `2026-05-14` is a draft — editable at actual tag time if the cut slips. Validation: `meson test -C builddir validate-metainfo` exit 0; `appstreamcli validate --pedantic` only flags the pre-existing `cid-contains-uppercase-letter` note (ADR-0012, intentional). Gates: fmt, clippy -D warnings, 14 unit + 1 doctest + 1 settings unit-test = 16 native pass; 5 hardware ignored. No Rust code changes.
next_step: Advance to T-110 (hot-plug REMOVE resilience) — extend `window::start_hotplug_poll` so that when the currently-active controls page corresponds to a camera that disappears from the enumeration, the poll callback pops the `NavigationView` back to the cameras list and posts a "Camera disconnected" toast. Re-plug still works (existing T-013b body re-mount). Commit `feat(gui): hot-plug REMOVE resilience (T-110)`. After T-110, write the run-closing `session-checkpoint` entry covering T-106..T-110, then stop and hand back.
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
updated_at: 2026-05-14T01:55:00Z  # T-109 closed, T-110 last in the autonomous batch
