# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none  # T-216 DONE + confirmed on hardware. Releasing 0.4.1 binaries for the Arch packager.
active_task_state: IDLE
active_branch: main
last_completed_task: T-217  # Presets UX (clarify recall-only + toast). T-216 PTZ fix confirmed on hardware. T-210..T-215 DONE.
last_milestone: v0.4.0  # Live Preview milestone cut 2026-06-02 (Flatpak-validated); v0.3.2 same day (native rollup)
last_commit_on_main: 715a7f8  # T-216 final (pushed). T-217 commit follows this STATE update. Prior pushed: 715a7f8, 46bf193, 30ca209, be12854, 445f16c, 1aa10ef, c6823c8, b447901, 0b028ff.
last_step: 2026-06-12 — Built 0.4.1 release binaries (live-preview) + pushed main for the Arch packager. T-217: reworded Presets copy (recall-only; empty slots won't move) + added a "Recalling preset N…" toast so the click is acknowledged (colleague feedback). fmt+clippy+test green.
next_step: none queued. Optional: cut/tag 0.4.1 once the Arch package is validated; user visual check of the Presets copy + toast. Q7 preset-save still deferred.
blockers: none on main. T-017b Arch validation pending an Arch host (transferred to incoming dev, ADR-0023).
working_tree:
  status: extras_view.rs + docs (STATE/PLAN/PROGRESS) modified for T-217, about to commit. Long-untracked stale obsbot-cam-control-0.4.0-1-x86_64.pkg.tar.zst stays untracked (superseded). target/release binaries + dist/ gitignored.
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
updated_at: 2026-06-12T09:00:00Z
