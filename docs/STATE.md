# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none
active_task_state: idle
last_completed_task: T-111
last_milestone: v0.1.0  # tag 5e005fd
last_commit: feat(gui): hot-plug REMOVE resilience (T-110)  # 7ab7b2a (T-111 commit pending in this turn)
last_step: T-111 DONE — validation-driven bug fix. The user's GUI pass on T-103 / T-104 / T-102 (items D, E, F.12) surfaced one bug with three faces: row sensitivity was set ONLY at page-build time by T-102 and never refreshed on subsequent writes, so toggling WB Auto or Auto Exposure didn't update the editable state of the dependent sliders. Fixed by registering every controlled row in a `thread_local!` `Vec<(u32, gtk::Widget)>` (`settings::REGISTERED_ROWS` + `ACTIVE_VIDEO_PATH`), exposing `reset_row_registry(video_path)` + `register_row(ctrl_id, &widget)`, and calling a new `refresh_sensitivity()` from `settings::write_and_save` after every successful Boolean / Menu write (Integer writes intentionally skip — sliders don't gate other controls and we don't want a ~100Hz `read_controls` ioctl during drag). `controls_view::build_controls_page` calls `reset_row_registry`; the four row-emitting paths (generic User/Camera loop, `wb_group`, `exposure_group`, and `ptz_pad` for zoom + focus auto + focus abs) all call `register_row`. PTZ pan/tilt buttons stay one-shot — they don't have a 1:1 control mapping. Gates: fmt, clippy -D warnings, 14 unit + 1 doctest + 1 settings unit = 16 native pass; 5 hardware ignored.
prior_step: T-110 DONE — hot-plug REMOVE resilience. `window.blp` now wraps the `AdwNavigationView` in an `Adw.ToastOverlay toast_overlay`; `window::build` binds it once via `settings::bind_toast_overlay`. The per-page overlay binding from T-108's initial scope is removed in `controls_view::build_controls_page` — single window-level surface so toasts dispatched right around a page navigation never get orphaned (per GNOME HIG, toasts overlay the entire ApplicationWindow). New `window::handle_remove_events(prev, latest, nav_view)` helper, invoked from `start_hotplug_poll` before the body re-mount: computes removed cameras by `(vid, pid, serial)` identity; if the visible page's tag is `controls-{vid:04x}-{pid:04x}` for any removed camera, calls `nav_view.pop_to_tag("cameras")`; surfaces a singular `"Camera disconnected: {product}"` or plural `"Cameras disconnected: {products}"` toast via `settings::surface_error`. Re-plug path unchanged (existing T-013b body re-mount). Gates: fmt, clippy -D warnings, 14 unit + 1 doctest + 1 settings unit-test = 16 native pass; 5 hardware ignored; meson compile of the release binary clean. T-106..T-110 autonomous batch closed; 5 commits on `main`, none pushed.
next_step: User re-runs the GUI validation pass focusing on T-103 / T-104 / T-102 items (D, E, F.12) to confirm the T-111 fix. After that, the remaining accumulated validations (T-105 GSettings round-trip, T-106 About, T-108 toast — via `sudo chmod 000 /dev/video0` workaround per the conversation —, T-110 unplug/replug, T-010 GNOME-Shell icon, T-017 Arch PKGBUILD if available). Once everything is green, bump `workspace.package.version` to `0.2.0` and cut `v0.2.0` per CLAUDE.md §7.
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
updated_at: 2026-05-14T02:45:00Z  # T-111 fix landed in response to user-validation findings D/E/F.12
