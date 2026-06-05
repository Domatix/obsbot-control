# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none
active_task_state: IDLE
active_branch: main
last_completed_task: T-206  # .deb now ships live-preview (ADR-0022); 0.4.0 hand-out artifacts (.flatpak bundle + .deb) produced for colleague testing
last_milestone: v0.4.0  # Live Preview milestone cut 2026-06-02 (Flatpak-validated); v0.3.2 same day (native rollup)
last_commit_on_main: 13f61a4  # docs: mark T-205 done (pushed); T-206 commit follows this STATE update
last_step: 2026-06-05 — T-206 done. Hand-out artifacts in build-aux/dist/ (git-ignored): obsbot-cam-control-0.4.0-x86_64.flatpak (install-verified, runtime GNOME 50) + obsbot-cam-control_0.4.0-1_amd64.deb (now with live-preview compiled in, GStreamer plugins as Recommends per ADR-0022). T-205 commits pushed with user OK.
next_step: no active task. Flathub submission unblocked on the runtime front (separate process: repo fork, flathub.json, reviewer round-trips). Other open items: T-202 (grayscale-while-off lost on start), T-017 (Arch PKGBUILD validation on an Arch host), T-400 (post-v1.0 OBSBOT Meet). Propose next on user confirmation.
blockers: none
working_tree:
  status: crates/obsbot-gui/Cargo.toml, docs/PLAN.md, docs/PROGRESS.md, docs/DECISIONS.md, docs/STATE.md modified (T-206: deb features+recommends, task record, ADR-0022, journal, this pointer); about to be committed together as the T-206 commit.
v0_4_0_gate:
  - T-203 build gate: DONE + verified headless 2026-06-02 (flatpak-builder builds all 3 modules, installs io.github.domatix.ObsbotCamControl 0.3.2, sandbox gst-inspect-1.0 finds gtk4paintablesink in /app/lib/gstreamer-1.0/libgstgtk4.so).
  - T-203 render check: PENDING USER — launch the installed Flatpak, toggle preview, confirm camera frames render on screen. Not machine-verifiable. Last thing before v0.4.0.
  - runtime EOL: CLOSED 2026-06-04 (T-205) — manifest now targets org.gnome.Platform//50 (verified headless). Flathub-unblocked on the runtime front.
still_open_non_hardware:
  - T-017 Arch PKGBUILD build/install/remove on an Arch host (community stakeholder, no rush).
  - T-202 minor: grayscale toggled while preview off is lost on start (re-apply on start, or disable filter buttons while off).
follow_ups_queued:
  - sepia-invert-filters, file-chooser-snapshot, preferences-dialog (preview-default-on), verify-q9-tiny2-regular — unchanged from prior STATE. (ptz-speed-fast dropped in T-101d.)
  - branch-hygiene: feat/* branches retained locally; delete only on explicit user ask.
  - T-400 (post-v1.0): add OBSBOT Meet (original) to the model matrix.
known_issues:
  - Q9 (PROTOCOL.md): pan_speed/tilt_speed accept writes but no motion on Tiny 2 Lite firmware 5.10. PTZ moves via discrete pan_absolute/tilt_absolute single steps (T-101d).
updated_at: 2026-06-05T07:00:00Z
