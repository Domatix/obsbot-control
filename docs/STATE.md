# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: T-209  # preview pipeline format fix (capsfilter I420) — grayscale now works + kills CRITICAL spam; verified headless, user visual confirmation pending
active_task_state: IN_PROGRESS
active_branch: main
last_completed_task: T-208  # deferred auto-sleep (ADR-0025) — user confirmed "ya parece que se apaga". T-207 (fd close) also DONE.
last_milestone: v0.4.0  # Live Preview milestone cut 2026-06-02 (Flatpak-validated); v0.3.2 same day (native rollup)
last_commit_on_main: b199267  # fix(gui): defer the auto-sleep so the firmware accepts it (T-208); T-209 commit follows this STATE update
last_step: 2026-06-11 — T-209. User confirmed T-208 sleep works ("ya parece que se apaga"); same report: grayscale toggle still dead. Root cause: gtk4paintablesink dmabuf-imports the camera's YUY2 → videobalance passthrough + per-frame gst_video_frame_map_id CRITICAL. Fix: capsfilter vb_caps pinning video/x-raw,format=I420 in system memory between vc_pre and videobalance. Verified headless on the real camera (read I420 U plane: |U-128| = 19.10 colour vs 0.00 grayscale). Marked T-207 + T-208 DONE. Gates green.
next_step: USER — visual check in the app: grayscale toggle visibly desaturates the preview, and the GStreamer-Video-CRITICAL spam is gone from the terminal. Then mark T-209 DONE. Optional follow-up: decodebin/jpegdec for HD (MJPG) preview — camera maxes at 640×480 raw YUYV. T-017b Arch validation still transferred to incoming dev (ADR-0023).
blockers: none on main. T-209 pending eyes-on-hardware. T-017b Arch validation pending an Arch host (transferred to incoming dev).
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
