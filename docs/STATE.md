# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none  # T-216 DONE + confirmed on hardware. Releasing 0.4.1 binaries for the Arch packager.
active_task_state: IDLE
active_branch: main
last_completed_task: T-216  # PTZ snapshot-slam fix, confirmed on hardware 2026-06-12. T-210..T-215 DONE.
last_milestone: v0.4.0  # Live Preview milestone cut 2026-06-02 (Flatpak-validated); v0.3.2 same day (native rollup)
last_commit_on_main: pending  # T-216 final commit (strip diagnostics) follows this STATE update. Prior: 46bf193 (T-216 interim), 30ca209 (T-215), be12854 (T-214), 445f16c (T-213), 1aa10ef docs, c6823c8 (T-212), b447901 (T-210), 0b028ff (T-211). PUSH PENDING.
last_step: 2026-06-12 — T-216 fix confirmed by user ("va perfecto"): PTZ moves one smooth step under preview, no slam/hang. Stripped the `ptz(T-216):` eprintln diagnostics, kept a `warning: ptz:` on the read-failure skip path. fmt+clippy+test green.
next_step: build release binaries + push main so the Arch packager (incoming dev) can build the PKGBUILD. Then address colleague feedback on Presets (recall-only per Q7: clicking an empty slot does nothing because the slot has no programmed position — clarify in UI/subtitle so users understand).
blockers: none on main. T-017b Arch validation pending an Arch host (transferred to incoming dev, ADR-0023).
working_tree:
  status: ptz_pad.rs + docs (STATE/PLAN/PROGRESS) modified for the T-216 finalization, about to commit. Long-untracked stale obsbot-cam-control-0.4.0-1-x86_64.pkg.tar.zst stays untracked (superseded). dist/ artifacts gitignored.
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
