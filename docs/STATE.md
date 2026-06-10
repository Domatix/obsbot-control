# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none  # session closed 2026-06-11 after the preview lifecycle/sleep/grayscale trio shipped + pushed
active_task_state: IDLE
active_branch: main
last_completed_task: T-209  # capsfilter I420 fix — grayscale works + CRITICAL spam gone (verified headless + user-accepted). T-207 + T-208 also DONE this session.
last_milestone: v0.4.0  # Live Preview milestone cut 2026-06-02 (Flatpak-validated); v0.3.2 same day (native rollup)
last_commit_on_main: 1b115f2  # fix(gui): force system-memory I420 (T-209); session-close docs commit follows + pushed to origin/main
last_step: 2026-06-11 — Closed a 3-fix session sparked by colleagues' "camera stays on when unused": T-207 (release V4L2 fd on navigate-back + window-close, machine-verified via /proc monitor), T-208 (deferred XU Sleep — firmware ignores Sleep for ~3s post-stream, ADR-0025; user-confirmed power-down), T-209 (capsfilter I420 → grayscale works + kills gst_video_frame_map_id CRITICAL spam, verified headless). All four feature commits + docs pushed to origin/main. Claude drove /dev/video0 headlessly for diagnosis (Claude is in group video).
next_step: NONE active. Candidate next tasks: T-017b Arch validation (still transferred to incoming dev, ADR-0023); optional preview HD via decodebin/jpegdec (camera maxes at 640×480 raw YUYV); follow-ups preview-visibility-pause, sleep-switch-sync, auto-sleep-optin (see follow_ups_queued).
blockers: none on main. T-017b Arch validation pending an Arch host (transferred to incoming dev, ADR-0023).
working_tree:
  status: after the T-208-redesign commit, only the untracked build artifact obsbot-cam-control-0.4.0-1-x86_64.pkg.tar.zst remains (user said leave it untracked, do not add to git).
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
updated_at: 2026-06-05T07:00:00Z
