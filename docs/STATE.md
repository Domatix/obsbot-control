# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: T-219  # DONE in working tree, not yet committed. UX text trim.
active_task_state: IDLE
active_branch: main
last_completed_task: T-219  # UX text trim: removed decorative group descriptions + redundant row subtitles, shortened the 3 essential ones (dropped the PROTOCOL §3.2 Q8 ref from the FOV subtitle). Gates green. Prior: T-218 (UX layout round 2).
last_milestone: v0.4.1  # tagged 2026-06-12 (dd7d6cf). GitHub Release published with the Arch .pkg.tar.zst asset (ADR-0027). Prior: v0.4.0 Live Preview 2026-06-02.
last_commit_on_main: 88d8296  # pushed to origin/main. T-219 code sits uncommitted in the working tree (see working_tree). Prior on main: T-218 (ad589ab code + 88d8296 DONE mark), dd7d6cf (tagged v0.4.1).
last_step: 2026-06-16 — T-219 DONE in working tree. Trimmed verbose UI text across ai_effects_view.rs, wb_group.rs, exposure_group.rs, extras_view.rs, ptz_pad.rs, resources/ptz-pad.blp + doc updates. All four cargo gates green. Awaiting commit (no hardware verification needed — pure text change).
next_step: commit T-219 (`feat(gui): trim verbose descriptions and subtitles (T-219)`); optionally push main.
blockers: none on main. T-017b Arch validation pending an Arch host (transferred to incoming dev, ADR-0023).
  status: T-219 DONE in working tree, gates green, NOT committed. `git status --short` shows the 6 source files + 3 doc files (PLAN/PROGRESS/STATE) for T-219, plus one untracked artifact — obsbot-cam-control-0.4.1-1-x86_64.pkg.tar.zst (v0.4.1 Release asset, stays untracked per ADR-0027). target/release binaries + dist/ gitignored.
firmware_notes:
  - Tiny 2 Lite fw 5.10: XU Sleep frame IGNORED for ~3s after streaming stops (accepted at t≈3s); cold Sleep works immediately. set_sleep(Awake)/get_status reliable. Rapid open/close/sleep/wake churn can hang capture (0 buffers, no error) until USB replug. (ADR-0025)
v0_4_0_gate:
  - T-203 build gate: DONE + verified headless 2026-06-02 (flatpak-builder builds all 3 modules, installs io.github.domatix.ObsbotCamControl 0.3.2, sandbox gst-inspect-1.0 finds gtk4paintablesink in /app/lib/gstreamer-1.0/libgstgtk4.so).
  - T-203 render check: PENDING USER — launch the installed Flatpak, toggle preview, confirm camera frames render on screen. Not machine-verifiable. Last thing before v0.4.0.
  - runtime EOL: CLOSED 2026-06-04 (T-205) — manifest now targets org.gnome.Platform//50 (verified headless). Flathub-unblocked on the runtime front.
still_open_non_hardware:
  - T-017 Arch PKGBUILD build/install/remove on an Arch host (community stakeholder, no rush).
  - T-202 minor: grayscale toggled while preview off is lost on start (re-apply on start, or disable filter buttons while off).
follow_ups_queued:
  - preview-visibility-pause (T-207 follow-up, ADR-0024): pause/resume the preview on window minimise / focus-loss so an explicitly-on preview does not keep the camera lit while the window is hidden. Natural home for the unimplemented SPEC §4.3 XDG Background Portal work.
  - sepia-invert-filters, file-chooser-snapshot, preferences-dialog (preview-default-on), verify-q9-tiny2-regular — unchanged from prior STATE. (ptz-speed-fast dropped in T-101d.)
  - branch-hygiene: feat/* branches retained locally; delete only on explicit user ask.
  - T-400 (post-v1.0): add OBSBOT Meet (original) to the model matrix.
known_issues:
  - Q9 (PROTOCOL.md): pan_speed/tilt_speed accept writes but no motion on Tiny 2 Lite firmware 5.10. PTZ moves via discrete pan_absolute/tilt_absolute single steps (T-101d).
updated_at: 2026-06-16T00:00:00Z
