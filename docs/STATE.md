# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: T-223  # Lock zoom. Code + docs done, four gates green. Awaits user validation with the camera.
active_task_state: IN_PROGRESS
active_branch: feat/T-223-lock-zoom
last_completed_task: T-015  # CI live + publication: repo flipped public, main pushed, CI green (run 30627008041) after 3 env fixes, Release v0.4.2 with .deb/.pkg.tar.zst, repo description+topics set.
last_milestone: v0.4.2  # unchanged. v0.6 (Polish) is now the active milestone per ROADMAP/PLAN.
last_commit_on_main: 474360b  # chore(release): PKGBUILD pkgver 0.4.2. PUSHED to origin/main. Prior: 5ae089a (clippy fix), b2728df/990f1e0 (CI env), ca4bf0e (T-222 close).
last_step: 2026-08-06 — issue #1 filed, T-223 implemented on feat/T-223-lock-zoom (blp switch row, ZoomLock, restore_target + 4 unit tests, watchdog, persistence). fmt/clippy(default)/clippy(live-preview)/test all green.
next_step: user validation of T-223 with the camera plugged in (engage lock, let auto-framing move the zoom, confirm it returns; slider inert while locked; state survives restart). Then the PROTOCOL.md finding on whether AI tracking drives zoom_absolute.
blockers: native /usr/local install still pending user sudo (from v0.4.2). T-017b Arch validation still pending an Arch host. T-223 user validation needs the camera.
  status: working_tree clean on feat/T-223-lock-zoom after the T-223 commit (ptz-pad.blp, ptz_pad.rs, PLAN, PROGRESS, STATE). build-aux/dist/ keeps 0.4.2 artifacts, git-ignored.
firmware_notes:
  - Tiny 2 Lite fw 5.10: XU Sleep frame IGNORED for ~3s after streaming stops (accepted at t≈3s); cold Sleep works immediately. set_sleep(Awake)/get_status reliable. Rapid open/close/sleep/wake churn can hang capture (0 buffers, no error) until USB replug. (ADR-0025)
still_open_non_hardware:
  - T-017 Arch PKGBUILD build/install/remove on an Arch host (community stakeholder, no rush).
  - T-202 minor: grayscale toggled while preview off is lost on start (re-apply on start, or disable filter buttons while off).
follow_ups_queued:
  - screenshots for AppStream + Flathub (needs camera plugged in; deferred per user 2026-07-31).
  - PLAN T-015 caveat "workflows green on main" RESOLVED 2026-07-31 (run 30627008041 green) — PLAN still says PENDING PUSH; fix on next PLAN touch.
  - announcement post draft (Reddit r/gnome / r/linux) — drafted at end of T-222 session.
  - preview-visibility-pause (T-207 follow-up, ADR-0024): pause/resume the preview on window minimise / focus-loss.
  - sepia-invert-filters, file-chooser-snapshot, preferences-dialog (preview-default-on), verify-q9-tiny2-regular.
  - branch-hygiene: feat/* branches retained locally; delete only on explicit user ask.
  - T-400 (post-v1.0): add OBSBOT Meet (original) to the model matrix.
known_issues:
  - Q9 (PROTOCOL.md): pan_speed/tilt_speed accept writes but no motion on Tiny 2 Lite firmware 5.10. PTZ moves via discrete pan_absolute/tilt_absolute single steps (T-101d).
updated_at: 2026-07-31T00:00:00Z
