# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none  # T-222 + T-015 DONE. Repo publication-ready; awaiting user's public flip.
active_task_state: IDLE
active_branch: main
last_completed_task: T-222  # Pre-publication docs reconciliation + CI. Full audit vs v0.4.2 code; ARCHITECTURE rewritten, README/ROADMAP/PLAN/SPEC/PROTOCOL/SKILLS/HANDOFF/AI_WORKFLOW/CREDITS fixed, AppStream description corrected, unused deps dropped, flatpak xdg-pictures, CI workflow (T-015) committed.
last_milestone: v0.4.2  # unchanged. v0.6 (Polish) is now the active milestone per ROADMAP/PLAN.
last_commit_on_main: bd2338c  # fix(flatpak) xdg-pictures. Series: 9c22184(ci) 894339d(arch) b91d750(docs) 18492a5(appstream) e93fb5e(chore) bd2338c(flatpak). NOT yet pushed to origin (origin still at 1a1190b).
last_step: 2026-07-31 — T-222 committed in 6 commits; gates green (fmt/clippy/61 tests/meson 2:2); ADR-0029 (Spanish deferral) + ADR-0030 (docs=reality rule) recorded; PLAN T-222+T-015 DONE.
next_step: user pushes main + flips repo public, then watches first CI run (T-015 caveat). Afterwards: screenshots for AppStream/Flathub, announcement post (draft offered in chat), optionally `gh release create v0.4.2` with build-aux/dist/ artifacts (or push a fresh tag to let CI build them).
blockers: none new. Native /usr/local install still pending user sudo (from v0.4.2). T-017b Arch validation still pending an Arch host.
  status: working_tree holds only this docs ledger update (PLAN/PROGRESS/DECISIONS/STATE) — final T-222 commit follows. build-aux/dist/ keeps the .deb/.flatpak/.pkg.tar.zst artifacts, git-ignored.
firmware_notes:
  - Tiny 2 Lite fw 5.10: XU Sleep frame IGNORED for ~3s after streaming stops (accepted at t≈3s); cold Sleep works immediately. set_sleep(Awake)/get_status reliable. Rapid open/close/sleep/wake churn can hang capture (0 buffers, no error) until USB replug. (ADR-0025)
still_open_non_hardware:
  - T-017 Arch PKGBUILD build/install/remove on an Arch host (community stakeholder, no rush).
  - T-202 minor: grayscale toggled while preview off is lost on start (re-apply on start, or disable filter buttons while off).
follow_ups_queued:
  - screenshots for AppStream + Flathub (needs camera plugged in; deferred per user 2026-07-31).
  - announcement post draft (Reddit r/gnome / r/linux) — drafted at end of T-222 session.
  - preview-visibility-pause (T-207 follow-up, ADR-0024): pause/resume the preview on window minimise / focus-loss.
  - sepia-invert-filters, file-chooser-snapshot, preferences-dialog (preview-default-on), verify-q9-tiny2-regular.
  - branch-hygiene: feat/* branches retained locally; delete only on explicit user ask.
  - T-400 (post-v1.0): add OBSBOT Meet (original) to the model matrix.
known_issues:
  - Q9 (PROTOCOL.md): pan_speed/tilt_speed accept writes but no motion on Tiny 2 Lite firmware 5.10. PTZ moves via discrete pan_absolute/tilt_absolute single steps (T-101d).
updated_at: 2026-07-31T00:00:00Z
