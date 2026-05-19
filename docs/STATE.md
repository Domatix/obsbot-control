# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none  # T-200 + T-101b both squashed to main; no in-flight task
active_task_state: —
active_branch: main
last_completed_task: T-101b  # PTZ press-and-hold + keyboard arrows, squashed to main as 96e33ba
last_milestone: v0.3.0  # tag cut 2026-05-15 on 6c954e5
last_commit_on_main: 96e33ba  # feat(gui): PTZ press-and-hold + keyboard arrow navigation (T-101b)
last_step: T-101b merged on 2026-05-19. User signalled "todo verde, mergea a main" via the validation AskUserQuestion after running the binary against the Tiny 2 Lite; per-gate breakdown not collected, only the overall pass. Code-side: ported press-and-hold from feat/T-101a verbatim (gtk::GestureClick, 200ms long-press, 50ms repeat at 1°/tick, JIT pan_absolute polling), added wire_keyboard_arrows (gtk::EventControllerKey on the controls page outer Box, Bubble phase so focused sliders consume their own arrows, modifier bypass, per-key timer map keyed by gdk::Key for diagonals). Four cargo gates green default + with obsbot-gui/live-preview on main post-squash.
next_step: User picks the next direction. Candidates: (a) cut v0.3.1 tag bundling T-105fix + T-200 + T-101b — Live Preview is already shippable on the native build, the Flatpak side stays on v0.3.0 until the gst-plugin-gtk module lands; (b) continue v0.4 — snapshot-to-file / post-process filters / Flatpak GStreamer module; (c) re-pass the parked v0.2 validation list (T-105 persistence, T-108 permission toast, T-110 unplug, T-102/T-103/T-104 menu writes); (d) tune-hold-speed (GSettings ptz-speed-fast) + shift-arrow-accelerator follow-ups from T-101b.
blockers: none
working_tree:
  status: uncommitted change in docs/STATE.md (this update)
follow_ups_queued:
  - cut-v0.3.1: optional release tag rolling T-105fix + T-200 + T-101b — decide whether to ship a native-build-only v0.3.1 or wait for v0.4 Flatpak readiness.
  - tune-hold-speed: GSettings `ptz-speed-fast` (1–100, default 50) mapped to HOLD_STEP_DEGREES so users can tune the joystick feel.
  - shift-arrow-accelerator: Shift+Arrow = larger step (3°/tick) for faster keyboard nav.
  - t110-keyboard-cleanup: hot-plug REMOVE currently leaks the keyboard hold timers briefly until the page widget drops; wire T-110's hot-plug signal to cancel them eagerly.
  - verify-q9-tiny2-regular: pan_speed/tilt_speed inert on Tiny 2 Lite firmware 5.10; re-test on Tiny 2 (regular) when a unit is available to confirm Q9 scope.
  - flatpak-gst-runtime: GNOME Platform 48 likely lacks `gtk4paintablesink`; manifest at `build-aux/io.github.domatix.ObsbotCamControl.json` will need a GStreamer module before T-200 ships in Flatpak (v0.4 cut blocker).
  - v0.4-out-of-scope: snapshot-to-file, post-process filters (greyscale / sepia / invert), resizable preview pane — all listed as v0.4 follow-ups in PLAN T-200 outcome block.
  - branch-hygiene: local `feat/T-300-xu-tracking` + `feat/T-101a` (SUPERSEDED) + `feat/T-200-preview` + `feat/T-101b-ptz-hold-keyboard` retained post-merge for archaeology; delete only on explicit user ask.
  - T-400 (post-v1.0): Add OBSBOT Meet (original) to the model matrix.
v0_2_pending_validation:
  parked:
    - T-108: kernel-permission denial toast (cable plugged, `sudo chmod 000 /dev/videoN`, drag slider → toast).
    - T-110: USB unplug on detail page pops to list + toast; re-plug reappears in ~2s.
    - T-101: full PTZ matrix — partially exercised during T-101b validation; rest of the original matrix items still to revisit.
    - T-102 / T-103 / T-104: power_line_frequency, WB group, Exposure Auto/Manual flow.
    - T-105: brightness persists across restart (now unblocked by T-105fix on main; re-run).
    - T-010: GNOME Shell icon visible.
    - T-017: Arch PKGBUILD smoke (build/install/remove).
known_issues:
  - Q9 (PROTOCOL.md): pan_speed/tilt_speed accept writes but produce no motion on Tiny 2 Lite firmware 5.10. T-101b uses the pan_absolute polling workaround.
updated_at: 2026-05-19T02:15:00Z
