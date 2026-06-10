# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: T-207  # stop preview on navigate-away + window close (camera-stays-on bug from colleague testing); code DONE, hardware LED validation pending
active_task_state: IN_PROGRESS
active_branch: main
last_completed_task: T-206  # .deb now ships live-preview (ADR-0022); 0.4.0 hand-out artifacts (.flatpak bundle + .deb) produced for colleague testing
last_milestone: v0.4.0  # Live Preview milestone cut 2026-06-02 (Flatpak-validated); v0.3.2 same day (native rollup)
last_commit_on_main: 5e23c7c  # build(arch): refresh PKGBUILD to 0.4.0 and hand off (T-017b); T-207 fix commit follows this STATE update
last_step: 2026-06-10 — T-207 (ADR-0024). Colleagues reported the camera LED staying on when unused. Root cause: preview pipeline only stopped on explicit toggle-off or non-deterministic Drop; nothing released the V4L2 node on navigate-back or window close. Fix (3 edits, behind live-preview feature): preview.rs ACTIVE_PREVIEW thread_local + register_active/stop_active; controls_view wires AdwNavigationPage::connect_hidden → stop; window wires connect_close_request → stop_active. All cargo gates green (default + --features obsbot-gui/live-preview). User chose "robust stop" scope (no minimise/focus visibility-pause yet).
next_step: USER/COLLEAGUE — hardware-validate T-207: preview on → press back → confirm camera LED goes off while on the list; repeat for closing the window. Then mark T-207 DONE. Separately, T-017b Arch validation still transferred to incoming dev (ADR-0023).
blockers: none on main. T-207 LED behaviour pending eyes-on-hardware. T-017b Arch validation pending an Arch host (transferred to incoming dev).
working_tree:
  status: after the T-207 fix commit, only the untracked build artifact obsbot-cam-control-0.4.0-1-x86_64.pkg.tar.zst remains (a leftover .pkg from a PKGBUILD test run — not committed; candidate for .gitignore or build-aux/dist/, awaiting user call).
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
