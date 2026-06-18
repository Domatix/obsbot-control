# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none  # T-220 DONE + shipped in v0.4.2. Awaiting next TODO from PLAN.
active_task_state: IDLE
active_branch: main
last_completed_task: T-220  # Single-page UX (drop camera list, reorg tabs, Main/Presets, focus+HDR moved). User-validated on hardware 2026-06-18. Bundled with T-221 (hid preview grayscale/mirror toggles), T-219, T-218 into v0.4.2.
last_milestone: v0.4.2  # bumped 2026-06-18 (ADR-0028): Cargo+meson reconciled to 0.4.2 (meson was stale at 0.4.0), AppStream 0.4.2 notes. Prior: v0.4.1 tagged 2026-06-12.
last_commit_on_main: d2465ba  # chore(release): bump to 0.4.2. ff-merged feat/T-220-single-page-ux into main, then docs-close commit. Prior pushed: b9df04b (T-219).
last_step: 2026-06-18 — bumped 0.4.1→0.4.2 (Cargo.toml/lock + meson.build + metainfo), gates green, ff-merged feat/T-220 into main, closed PLAN/PROGRESS/DECISIONS(ADR-0028).
next_step: tag v0.4.2; build native (meson release) + Flatpak; install both to refresh user's 0.4.0 installs; push main + tag. Incoming dev then builds the Arch .pkg from the tag (ADR-0023/0027).
blockers: T-017b Arch validation pending an Arch host (incoming dev, ADR-0023).
  status: v0.4.2 docs being closed on main. working_tree: docs/PLAN.md, docs/PROGRESS.md, docs/DECISIONS.md, docs/STATE.md pending the docs-close commit; untracked obsbot-cam-control-0.4.1-1-x86_64.pkg.tar.zst (stale Release asset, untracked per ADR-0027).
firmware_notes:
  - Tiny 2 Lite fw 5.10: XU Sleep frame IGNORED for ~3s after streaming stops (accepted at t≈3s); cold Sleep works immediately. set_sleep(Awake)/get_status reliable. Rapid open/close/sleep/wake churn can hang capture (0 buffers, no error) until USB replug. (ADR-0025)
still_open_non_hardware:
  - T-017 Arch PKGBUILD build/install/remove on an Arch host (community stakeholder, no rush).
  - T-202 minor: grayscale toggled while preview off is lost on start (re-apply on start, or disable filter buttons while off).
follow_ups_queued:
  - preview-visibility-pause (T-207 follow-up, ADR-0024): pause/resume the preview on window minimise / focus-loss. Natural home for the unimplemented SPEC §4.3 XDG Background Portal work.
  - sepia-invert-filters, file-chooser-snapshot, preferences-dialog (preview-default-on), verify-q9-tiny2-regular — unchanged from prior STATE.
  - branch-hygiene: feat/* branches retained locally; delete only on explicit user ask.
  - T-400 (post-v1.0): add OBSBOT Meet (original) to the model matrix.
known_issues:
  - Q9 (PROTOCOL.md): pan_speed/tilt_speed accept writes but no motion on Tiny 2 Lite firmware 5.10. PTZ moves via discrete pan_absolute/tilt_absolute single steps (T-101d).
updated_at: 2026-06-18T00:00:00Z
