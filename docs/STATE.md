# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none  # T-200 squashed to main; about to open T-101b
active_task_state: —
active_branch: main
last_completed_task: T-200  # Live Preview, squashed to main as cccab8c
last_milestone: v0.3.0  # tag cut 2026-05-15 on 6c954e5
last_commit_on_main: cccab8c  # feat(gui): Live Preview pipeline behind --features live-preview (T-200)
last_step: T-200 closed end-to-end on 2026-05-19. User-validated visually: frames render in the sticky `gtk::Picture` revealer above the scrolled page, header-bar toggle starts/stops cleanly, AdwBanner discoverability hint shows under the header while preview is off, device-busy surfaces a real toast via the new `drain_bus_error` helper. Squash-merged `feat/T-200-preview` → `main` (3 pre-squash commits: 6153aaa / a70ad62 / 8579db7). Four cargo gates green on `main` default and with `obsbot-gui/live-preview`. Branch retained locally for blame archaeology. No tag cut — v0.4.0 waits for snapshot-to-file + filters + Flatpak GStreamer module.
next_step: Open T-101b on a fresh `feat/T-101b-ptz-hold-keyboard` branch covering (a) PTZ press-and-hold (hold-repeat at 50 ms, 1° step via JIT pan_absolute polling — same approach as feat/T-101a, which is now superseded), (b) two-button-at-once diagonals or a single chord-style controller for diagonals, (c) keyboard arrow navigation (Left/Right/Up/Down + Home reset) when the controls page has focus and no input widget is consuming the keys.
blockers: none
working_tree:
  status: uncommitted change in docs/STATE.md (this update)
follow_ups_queued:
  - t101b: PTZ press-and-hold + keyboard arrows on `feat/T-101b-ptz-hold-keyboard`. Supersedes `feat/T-101a` (kept locally for archaeology). User feedback 2026-05-19: discrete 5° steps feel "rayados", diagonals worse; wants joystick-like feel + keyboard accessibility.
  - verify-q9-tiny2-regular: pan_speed/tilt_speed inert on Tiny 2 Lite firmware 5.10; re-test on Tiny 2 (regular) when a unit is available to confirm Q9 scope.
  - flatpak-gst-runtime: GNOME Platform 48 likely lacks `gtk4paintablesink`; manifest at `build-aux/io.github.domatix.ObsbotCamControl.json` will need a GStreamer module before T-200 ships in Flatpak (v0.4 cut blocker).
  - v0.4-out-of-scope: snapshot-to-file, post-process filters (greyscale / sepia / invert), resizable preview pane — all listed as v0.4 follow-ups in PLAN T-200 outcome block.
  - branch-hygiene: local `feat/T-300-xu-tracking` + `feat/T-101a` + `feat/T-200-preview` retained post-merge for archaeology; delete only on explicit user ask.
  - T-400 (post-v1.0): Add OBSBOT Meet (original) to the model matrix.
v0_2_pending_validation:
  parked:
    - T-108: kernel-permission denial toast (cable plugged, `sudo chmod 000 /dev/videoN`, drag slider → toast).
    - T-110: USB unplug on detail page pops to list + toast; re-plug reappears in ~2s.
    - T-101: full PTZ matrix — will be revisited as part of T-101b.
    - T-102 / T-103 / T-104: power_line_frequency, WB group, Exposure Auto/Manual flow.
    - T-105: brightness persists across restart (now unblocked by T-105fix on main; re-run).
    - T-010: GNOME Shell icon visible.
    - T-017: Arch PKGBUILD smoke (build/install/remove).
known_issues:
  - Q9 (PROTOCOL.md): pan_speed/tilt_speed accept writes but produce no motion on Tiny 2 Lite firmware 5.10. T-200 unaffected; T-101b will adopt the pan_absolute-polling workaround from T-101a.
updated_at: 2026-05-19T01:30:00Z
