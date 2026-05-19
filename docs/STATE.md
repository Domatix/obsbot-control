# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: T-200  # final commit on the feature branch; about to squash-merge to main
active_task_state: DONE on `feat/T-200-preview` — user-validated visually, awaiting squash-merge to main
active_branch: feat/T-200-preview
last_completed_task: T-105fix  # on main; T-101a still DONE on its branch awaiting hardware ergonomics
last_milestone: v0.3.0  # tag cut 2026-05-15 on 6c954e5
last_commit_on_main: 5066881  # docs: README v0.3 status block + PROTOCOL Q4 resolution
last_step: T-200 visually validated against the Tiny 2 Lite on 2026-05-19. UX iteration this session moved the toggle to the header bar (Blueprint `header_bar` ID), wrapped the Picture in a `gtk::Revealer` outside the scrolled page so the sticky preview only takes vertical space while active, and added an `AdwBanner` discoverability hint under the header bar (collapses on activation). Bus-error drain in `PreviewPipeline::start` now surfaces device-busy as a toast via `settings::surface_error`. Four cargo gates green default + with `obsbot-gui/live-preview`. Final commit pending on `feat/T-200-preview` before squash-merge.
next_step: (1) commit the UX iteration on `feat/T-200-preview` (Blueprint header_bar + revealer + banner + bus drain); (2) squash-merge `feat/T-200-preview` → `main` with a milestone-style commit message; (3) verify cargo gates on `main` default profile; (4) open T-101b on `main` for PTZ press-and-hold ergonomics + keyboard arrow navigation, building on the press-and-hold polling from `feat/T-101a`.
blockers: none
working_tree:
  status: uncommitted changes on feat/T-200-preview (controls-view.blp + controls_view.rs + preview.rs + docs)
follow_ups_queued:
  - validate-t101a: in-person PTZ press-and-hold on `feat/T-101a` — tap/hold disambiguation, diagonals, ergonomics. SUPERSEDED in spirit by T-101b which bundles the same hold logic with keyboard arrows; the branch is still retained for blame archaeology.
  - t101b: PTZ press-and-hold + keyboard arrows on `main` — adopts hold-repeat polling at 50 ms (1° step) from feat/T-101a, adds keyboard handlers (Left/Right/Up/Down + Home reset) on the controls page, validates diagonals.
  - verify-q9-tiny2-regular: pan_speed/tilt_speed inert on Tiny 2 Lite firmware 5.10; re-test on Tiny 2 (regular) when a unit is available to confirm Q9 scope.
  - flatpak-gst-runtime: GNOME Platform 48 likely lacks `gtk4paintablesink`; manifest at `build-aux/io.github.domatix.ObsbotCamControl.json` will need a GStreamer module before T-200 ships in Flatpak (v0.4 cut blocker).
  - v0.4-out-of-scope: snapshot-to-file, post-process filters (greyscale / sepia / invert), resizable preview pane — all listed as v0.4 follow-ups in PLAN T-200 outcome block.
  - branch-hygiene: local `feat/T-300-xu-tracking` + `feat/T-101a` + `feat/T-200-preview` retained post-merge for archaeology; delete only on explicit user ask.
  - T-400 (post-v1.0): Add OBSBOT Meet (original) to the model matrix.
v0_2_pending_validation:
  parked:
    - T-108: kernel-permission denial toast (cable plugged, `sudo chmod 000 /dev/videoN`, drag slider → toast).
    - T-110: USB unplug on detail page pops to list + toast; re-plug reappears in ~2s.
    - T-101: full PTZ matrix (cache-drift fixed, remaining matrix items still to revisit). Will be revisited as part of T-101b.
    - T-102 / T-103 / T-104: power_line_frequency, WB group, Exposure Auto/Manual flow.
    - T-105: brightness persists across restart (now unblocked by T-105fix on main; re-run).
    - T-010: GNOME Shell icon visible.
    - T-017: Arch PKGBUILD smoke (build/install/remove).
known_issues:
  - Q9 (PROTOCOL.md): pan_speed/tilt_speed accept writes but produce no motion on Tiny 2 Lite firmware 5.10. T-101a switched to pan_absolute polling as a workaround; T-101b will adopt the same workaround.
updated_at: 2026-05-19T01:15:00Z
