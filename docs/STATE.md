# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: T-101b  # PTZ press-and-hold + keyboard arrows, supersedes T-101a
active_task_state: IN_PROGRESS  # code complete, four cargo gates green; awaiting hardware ergonomics validation
active_branch: feat/T-101b-ptz-hold-keyboard
last_completed_task: T-200  # Live Preview, squashed to main as cccab8c
last_milestone: v0.3.0  # tag cut 2026-05-15 on 6c954e5
last_commit_on_main: 09f2d70  # docs: STATE → T-200 squashed to main, queue T-101b (T-200)
last_step: T-101b code complete on `feat/T-101b-ptz-hold-keyboard`. Brought `crates/obsbot-gui/src/ptz_pad.rs` from `feat/T-101a` verbatim (press-and-hold via gtk::GestureClick + 200ms long-press + 50ms repeat + 1°/tick during hold), then added `wire_keyboard_arrows` that attaches a gtk::EventControllerKey to the controls page's outer Box. Mapping: Left/Right=pan, Up/Down=tilt (Up=camera up matches btn_n), Home=recenter. Bubble propagation so focused sliders / spins / combos keep consuming their own arrows. Modifiers (Ctrl/Shift/Alt/Super) bypass the controller. Per-key timers indexed by gdk::Key value so Up+Right diagonal runs both axes independently. Four cargo gates green default + with obsbot-gui/live-preview. Not yet committed.
next_step: (1) commit T-101b on the branch; (2) user-driven hardware ergonomics validation (mouse hold, button taps, keyboard arrows, Home recenter, focus-on-slider isolation); (3) if green, squash-merge to main and either cut v0.3.1 or roll T-101b into v0.4 alongside T-200.
blockers: none
working_tree:
  status: uncommitted changes on feat/T-101b-ptz-hold-keyboard (ptz_pad.rs + controls_view.rs + docs)
follow_ups_queued:
  - tune-hold-speed: user-tunable hold speed via `GSettings ptz-speed-fast` (1–100, default 50) mapped to HOLD_STEP_DEGREES; for now constants are static in ptz_pad.rs.
  - shift-arrow-accelerator: Shift+Arrow = larger step (3°/tick) for faster motion when keyboard-driven.
  - t110-keyboard-cleanup: hot-plug REMOVE currently leaks the keyboard hold timers briefly until the page widget drops; wire T-110's hot-plug signal to cancel them eagerly.
  - verify-q9-tiny2-regular: pan_speed/tilt_speed inert on Tiny 2 Lite firmware 5.10; re-test on Tiny 2 (regular) when a unit is available to confirm Q9 scope.
  - flatpak-gst-runtime: GNOME Platform 48 likely lacks `gtk4paintablesink`; manifest at `build-aux/io.github.domatix.ObsbotCamControl.json` will need a GStreamer module before T-200 ships in Flatpak (v0.4 cut blocker).
  - v0.4-out-of-scope: snapshot-to-file, post-process filters (greyscale / sepia / invert), resizable preview pane — all listed as v0.4 follow-ups in PLAN T-200 outcome block.
  - branch-hygiene: local `feat/T-300-xu-tracking` + `feat/T-101a` (SUPERSEDED) + `feat/T-200-preview` retained post-merge for archaeology; delete only on explicit user ask.
  - T-400 (post-v1.0): Add OBSBOT Meet (original) to the model matrix.
v0_2_pending_validation:
  parked:
    - T-108: kernel-permission denial toast (cable plugged, `sudo chmod 000 /dev/videoN`, drag slider → toast).
    - T-110: USB unplug on detail page pops to list + toast; re-plug reappears in ~2s.
    - T-101: full PTZ matrix — being revisited as part of T-101b validation.
    - T-102 / T-103 / T-104: power_line_frequency, WB group, Exposure Auto/Manual flow.
    - T-105: brightness persists across restart (now unblocked by T-105fix on main; re-run).
    - T-010: GNOME Shell icon visible.
    - T-017: Arch PKGBUILD smoke (build/install/remove).
known_issues:
  - Q9 (PROTOCOL.md): pan_speed/tilt_speed accept writes but produce no motion on Tiny 2 Lite firmware 5.10. T-101b uses the same pan_absolute polling workaround as T-101a.
updated_at: 2026-05-19T02:00:00Z
