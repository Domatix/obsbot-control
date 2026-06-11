# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none  # T-210/T-211/T-212 implemented + gated green this session; awaiting user visual/hardware validation
active_task_state: IDLE
active_branch: main
last_completed_task: T-212  # GUI redesign (ViewSwitcher tabs + preview card + custom CSS). T-210 (mirror toggle) + T-211 (drop Camera-awake switch) also DONE this session.
last_milestone: v0.4.0  # Live Preview milestone cut 2026-06-02 (Flatpak-validated); v0.3.2 same day (native rollup)
last_commit_on_main: 6a9357d  # next commits (T-211, T-210, T-212) follow once staged
last_step: 2026-06-11 — Colleague feedback session. T-210: added a `videoflip` (vf_flip) to the preview pipeline + `set_mirror` + a header mirror toggle (object-flip-horizontal). T-211: removed the non-functional "Camera awake" SwitchRow from extras_view (group renamed "Presets"; auto-sleep machinery untouched). T-212: redesigned the controls page into AdwViewStack tabs (Image/Move/AI/Extras) with an AdwViewSwitcher in the header, promoted the preview to a rounded shadowed card (gtk::Stack placeholder↔video), added resources/style.css (loaded via CssProvider in application::run) + a build.rs stage to pack it, and a hero on the camera-list landing. All cargo gates green; startup smoke clean.
next_step: User to launch the app and validate visually: tabs look good, preview card + mirror/grayscale/snapshot work, every control still writes, "Camera awake" gone. Then commit T-211/T-210/T-212 (3 logical commits) — staged but NOT yet committed.
blockers: none on main. T-017b Arch validation pending an Arch host (transferred to incoming dev, ADR-0023).
working_tree:
  status: UNCOMMITTED edits for T-210/T-211/T-212 — crates/obsbot-gui/{build.rs, src/application.rs, src/controls_view.rs, src/extras_view.rs, src/preview.rs, src/window.rs}, crates/obsbot-gui/resources/{obsbot.gresource.xml, style.css(new)}, docs/{PLAN.md, STATE.md, PROGRESS.md}. Plus the long-untracked stale obsbot-cam-control-0.4.0-1-x86_64.pkg.tar.zst (leave untracked). dist/ artifacts gitignored.
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
updated_at: 2026-06-11T12:00:00Z
