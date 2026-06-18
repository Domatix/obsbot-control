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
last_commit_on_main: 852a379  # docs: close T-220, cut v0.4.2. PUSHED to origin/main 2026-06-18 (b9df04b..852a379) + tag v0.4.2 pushed. Prior: d2465ba (version bump).
last_step: 2026-06-18 — released v0.4.2: bumped+merged+tagged+pushed (main+tag). Flatpak rebuilt & installed (now 0.4.2). Native binary recompiled in builddir (0.4.2, release+live-preview) but NOT yet installed — `sudo meson install -C builddir` needs the user's password.
next_step: user runs `sudo meson install -C builddir` to refresh /usr/local native binary 0.4.0→0.4.2. Incoming dev builds the Arch .pkg from tag v0.4.2 (ADR-0023/0027); optionally `gh release create v0.4.2` once that .pkg exists.
blockers: native /usr/local install pending user sudo. T-017b Arch validation pending an Arch host (incoming dev, ADR-0023).
  status: v0.4.2 released — main + tag pushed to origin. Flatpak io.github.domatix.ObsbotCamControl installed at 0.4.2. Native binary built (builddir/obsbot-cam-control) awaiting sudo install. working_tree clean except untracked obsbot-cam-control-0.4.1-1-x86_64.pkg.tar.zst (stale 0.4.1 Release asset, untracked per ADR-0027).
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
updated_at: 2026-06-18T00:30:00Z
