# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: T-218  # UX feedback round 2: HDR out of AI, tracking-tab first, relocate preview buttons, single-cam auto-enter.
active_task_state: IN_PROGRESS
active_branch: main
last_completed_task: T-217  # Presets UX (clarify recall-only + toast). T-216 PTZ fix confirmed on hardware. T-210..T-215 DONE.
last_milestone: v0.4.1  # tagged 2026-06-12 (dd7d6cf). GitHub Release published with the Arch .pkg.tar.zst asset (ADR-0027). Prior: v0.4.0 Live Preview 2026-06-02.
last_commit_on_main: ad589ab  # T-218 GUI layout feedback (HDR→Image, AI tab first, preview buttons under card, single-cam auto-enter). NOT yet pushed. Prior: dd7d6cf (tagged v0.4.1), 8a6ac60 (T-217), 715a7f8 (T-216 final), 46bf193, 30ca209, be12854.
last_step: 2026-06-15 — T-218 code complete + all cargo gates green (fmt, clippy with & without live-preview, test). Four GUI edits: HDR→Image tab (build_hdr_group), AI tab now first/default, preview buttons moved to a bar under the preview card, single-camera auto-enter in window.rs. Committed to main.
next_step: user visual validation on hardware of the four points (HDR in Image tab, AI/tracking opens first, preview buttons under the card, single camera lands on config directly). Then mark T-218 DONE.
blockers: none on main. T-017b Arch validation pending an Arch host (transferred to incoming dev, ADR-0023).
  status: T-218 code committed (ad589ab), gates green, pending user visual validation. `git status --short` shows one untracked artifact — obsbot-cam-control-0.4.1-1-x86_64.pkg.tar.zst (published as the v0.4.1 Release asset); stays untracked per ADR-0027 (cleanup is the user's call). target/release binaries + dist/ gitignored.
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
updated_at: 2026-06-15T00:00:00Z
