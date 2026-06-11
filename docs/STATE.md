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
last_commit_on_main: 6a9357d  # build: bump version to 0.4.1 (carries T-207/208/209); docs commit follows + pushed to origin/main
last_step: 2026-06-11 — Cut 0.4.1 (T-207/208/209 over the 0.4.0 Live Preview milestone) and regenerated the hand-out artifacts. Bumped Cargo.toml + PKGBUILD pkgver + AppStream release entry; rebuilt build-aux/dist/obsbot-cam-control_0.4.1-1_amd64.deb (cargo-deb, features=live-preview) and obsbot-cam-control-0.4.1-x86_64.flatpak (flatpak-builder, -Dlive-preview=true). Deleted the stale 0.4.0 .deb/.flatpak from dist (dist/ is gitignored — binaries not committed).
next_step: NONE active. Hand out the 0.4.1 .deb + .flatpak to colleagues; ask the Arch colleague to rebuild the .pkg from the 0.4.1 PKGBUILD (ADR-0023). Candidate work: optional HD preview via decodebin/jpegdec (camera maxes at 640×480 raw YUYV); follow-ups preview-visibility-pause, sleep-switch-sync, auto-sleep-optin.
blockers: none on main. T-017b Arch validation pending an Arch host (transferred to incoming dev, ADR-0023).
working_tree:
  status: after the docs commit, only the untracked stale obsbot-cam-control-0.4.0-1-x86_64.pkg.tar.zst remains (user said leave it untracked; superseded by 0.4.1 — the colleague regenerates the Arch pkg). dist/ artifacts are gitignored.
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
