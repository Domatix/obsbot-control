# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none
active_task_state: idle
last_completed_task: T-111
last_milestone: v0.1.0  # tag 5e005fd
last_commit: fix(gui): refresh row sensitivity after gate writes (T-111)  # 9fde97d
last_step: Session ended at user's request after T-111 fix and the v0.3 backlog seed. The T-106..T-110 autonomous batch + the T-111 validation-driven fix all landed; user couldn't re-test T-111 (deferred to next session). Pre-close addition: T-200 ("embedded preview pane in the per-camera controls page") added to PLAN.md as the first v0.3 task — it pins the UX decision that the GStreamer / `gtk4paintablesink` preview from ROADMAP §v0.3 must live INSIDE the controls page rather than as a separate window, so adjustments to brightness / WB / exposure / PTZ are visible without launching Cheese / OBS / `v4l2-ctl --stream-mmap` as a side process. Commits since previous STATE close-out: a688714 (T-106), 39c1206 (T-107), 6df5294 (T-108), ee8bcb5 (T-109), 7ab7b2a (T-110), 9fde97d (T-111) — six commits on `main`, none pushed (private repo, explicit no-push working agreement). `window.blp` now wraps the `AdwNavigationView` in an `Adw.ToastOverlay toast_overlay`; `window::build` binds it once via `settings::bind_toast_overlay`. The per-page overlay binding from T-108's initial scope is removed in `controls_view::build_controls_page` — single window-level surface so toasts dispatched right around a page navigation never get orphaned (per GNOME HIG, toasts overlay the entire ApplicationWindow). New `window::handle_remove_events(prev, latest, nav_view)` helper, invoked from `start_hotplug_poll` before the body re-mount: computes removed cameras by `(vid, pid, serial)` identity; if the visible page's tag is `controls-{vid:04x}-{pid:04x}` for any removed camera, calls `nav_view.pop_to_tag("cameras")`; surfaces a singular `"Camera disconnected: {product}"` or plural `"Cameras disconnected: {products}"` toast via `settings::surface_error`. Re-plug path unchanged (existing T-013b body re-mount). Gates: fmt, clippy -D warnings, 14 unit + 1 doctest + 1 settings unit-test = 16 native pass; 5 hardware ignored; meson compile of the release binary clean. T-106..T-110 autonomous batch closed; 5 commits on `main`, none pushed.
next_step: Next session resumes with the user's GUI re-validation pass: first the T-111 fix re-test (T-103 / T-104 / T-102 toggles for sensitivity), then the rest of the accumulated `pending_user_actions`. When everything is green, bump `workspace.package.version` from `0.1.0` to `0.2.0` and cut the `v0.2.0` tag per CLAUDE.md §7 (gates green, fmt / clippy / test, Flatpak builds, README current, AppStream release-notes date adjusted to the actual tag day if it slipped past 2026-05-14). Only after v0.2.0 ships does T-200 (embedded preview) become the active task — opening v0.3.
blockers: none.
working_tree:
  pre_commit_modified: []
  pre_commit_untracked: []
  pre_commit_deleted: []
pending_user_actions:
  # First autonomous run (T-101..T-105), second autonomous run
  # (T-106..T-110), and the validation-driven T-111 fix accumulate
  # for the second validation pass. Items the user already
  # validated green in the first pass are NOT re-listed.
  - T-111 (re-test after the fix; replaces previous D / E / F.12):
    toggle WB Auto OFF → WB Temperature / Red / Blue sliders
    become editable; toggle ON → grey out. Switch Auto Exposure
    to Manual → Exposure Time Absolute editable; back to Auto
    Mode → greys out.
  - T-106: click the hamburger button in the header bar → "About
    Obsbot Cam Control" → confirm version, license, repo link,
    issue-tracker link, and the "Reverse-engineering references"
    acknowledgement block render correctly.
  - T-108: yanking the USB cable is the WRONG test here —
    that triggers T-110 first and pops the page out. Use the
    permission-revocation workaround instead: keep the cable
    plugged in, then in a terminal run
    `sudo chmod 000 /dev/video0`; back in the GUI (still on
    the controls page; the device stays in the enumeration
    since `enumerate_cameras` reads sysfs, not /dev) drag a
    slider → toast reads "Failed to set <control name>:
    Permission denied". Restore with `sudo chmod 660
    /dev/video0`.
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
updated_at: 2026-05-14T02:55:00Z  # session ended cleanly; T-200 seeded for v0.3
