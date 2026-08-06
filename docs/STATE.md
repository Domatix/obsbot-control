# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none  # T-223/224/225/226 merged to main via PRs #2/#8/#9/#10. Issues #1/#3/#4/#5 closed; #6 (partial) and #7 open.
active_task_state: IDLE
active_branch: main
last_completed_task: T-226  # Flatpak git sources pinned by commit. Prior in the same batch: T-225 (schema lookup + replay validation), T-224 (CI hardening), T-223 (zoom lock).
last_milestone: v0.4.2  # unchanged. v0.6 (Polish) is now the active milestone per ROADMAP/PLAN.
last_commit_on_main: e36da92  # feat(gui): zoom lock (T-223) (#2). Prior: e6eb797 (T-226), b531ff6 (T-225), c270b23 (T-224).
last_step: 2026-08-06 — security review of v0.4.2 filed as issues #3-#7; four PRs merged. Repo now has a permissions-locked CI with SHA-pinned actions, a cargo-audit job, release checksums, an installed-schema lookup, validated replay, commit-pinned Flatpak sources, and the zoom lock.
next_step: hardware validation of T-223 (engage lock, let the camera move the zoom, confirm it returns; slider inert while locked; state survives restart). Two issues stay open: #6 offline Flatpak build (blocks Flathub), #7 low-severity findings.
blockers: native /usr/local install still pending user sudo (from v0.4.2). T-017b Arch validation still pending an Arch host. T-223 hardware validation needs an OBSBOT the app recognises — the unit currently plugged in is an OBSBOT Meet SE (3564:fefe), which enumerate.rs does not accept (TINY2_FAMILY is fef8/fef9 only).
  status: working_tree clean on main. build-aux/dist/ keeps 0.4.2 artifacts, git-ignored.
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
