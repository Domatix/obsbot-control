# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none
active_task_state: IDLE
active_branch: main
last_completed_task: T-205  # Flathub prep: Flatpak runtime bumped off EOL GNOME 48 → GNOME 50, verified headless 2026-06-04
last_milestone: v0.4.0  # Live Preview milestone cut 2026-06-02 (Flatpak-validated); v0.3.2 same day (native rollup)
last_commit_on_main: 5ab4ebe  # build(flatpak): bump runtime GNOME 48 → 50 (T-205) — docs commit follows
last_step: 2026-06-04 — closed T-205. flatpak-builder against the GNOME 50 manifest exits 0; installed app links org.gnome.Platform/x86_64/50; sandbox gst-inspect resolves gtk4paintablesink from /app/lib/gstreamer-1.0/libgstgtk4.so (gst-plugins-rs 0.13.5); zero EOL warnings. Manifest committed (5ab4ebe); PLAN→DONE, PROGRESS/STATE updated.
next_step: no active task. Runtime-EOL gate is closed → Flathub submission is unblocked on the runtime front (separate process: repo fork, flathub.json, reviewer round-trips — out of T-205 scope). Other open items: T-202 (grayscale-while-off lost on start), T-017 (Arch PKGBUILD validation on an Arch host), T-400 (post-v1.0 OBSBOT Meet). Propose next on user confirmation.
blockers: none
working_tree:
  status: docs/PLAN.md, docs/PROGRESS.md, docs/STATE.md modified (T-205 closure: DONE + journal entry + this pointer); about to be committed together. Manifest already committed at 5ab4ebe.
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
updated_at: 2026-06-04T00:00:00Z
