# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none
active_task_state: idle
last_completed_task: T-110
last_milestone: v0.1.0  # tag 5e005fd
last_commit: docs(appstream): v0.2.0 release notes (T-109)  # ee8bcb5 (T-110 commit pending in this turn)
last_step: T-110 DONE — hot-plug REMOVE resilience. `window.blp` now wraps the `AdwNavigationView` in an `Adw.ToastOverlay toast_overlay`; `window::build` binds it once via `settings::bind_toast_overlay`. The per-page overlay binding from T-108's initial scope is removed in `controls_view::build_controls_page` — single window-level surface so toasts dispatched right around a page navigation never get orphaned (per GNOME HIG, toasts overlay the entire ApplicationWindow). New `window::handle_remove_events(prev, latest, nav_view)` helper, invoked from `start_hotplug_poll` before the body re-mount: computes removed cameras by `(vid, pid, serial)` identity; if the visible page's tag is `controls-{vid:04x}-{pid:04x}` for any removed camera, calls `nav_view.pop_to_tag("cameras")`; surfaces a singular `"Camera disconnected: {product}"` or plural `"Cameras disconnected: {products}"` toast via `settings::surface_error`. Re-plug path unchanged (existing T-013b body re-mount). Gates: fmt, clippy -D warnings, 14 unit + 1 doctest + 1 settings unit-test = 16 native pass; 5 hardware ignored; meson compile of the release binary clean. T-106..T-110 autonomous batch closed; 5 commits on `main`, none pushed.
next_step: Stop. The user asked for 5 more tasks (T-106..T-110) and they are all DONE. When the user resumes, the natural items are: (a) the accumulated GUI validation pass (10 items in `pending_user_actions`: T-101 PTZ, T-102 PLF + INACTIVE, T-103 WB group, T-104 Exposure group, T-105 GSettings round-trip, T-106 About dialog, T-108 toast on write failure, T-110 unplug/replug cycle, plus the still-pending T-010 GNOME-Shell icon paint and T-017 Arch PKGBUILD smoke); (b) iterate any UX fixes the validation surfaces; (c) optionally bump `workspace.package.version` to `0.2.0` and cut the `v0.2.0` tag per CLAUDE.md §7 once validation is green.
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
  - T-110: while on a camera detail page, unplug the USB
    cable; confirm the page pops back to the cameras list
    automatically AND a toast appears reading "Camera
    disconnected: <product name>". Re-plug; confirm the
    camera reappears in the cameras list within ~2 s (poll
    interval).
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
updated_at: 2026-05-14T02:15:00Z  # T-110 closed; second autonomous run (T-106..T-110) finished, awaiting user validation
