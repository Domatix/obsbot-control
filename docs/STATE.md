# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: T-205  # Flathub prep: bump Flatpak runtime off EOL GNOME 48 → GNOME 50
active_task_state: IN_PROGRESS
active_branch: main
last_completed_task: T-101d  # PTZ single-step; closed alongside the v0.4 task set in the v0.4.0 cut
last_milestone: v0.4.0  # Live Preview milestone cut 2026-06-02 (Flatpak-validated); v0.3.2 same day (native rollup)
last_commit_on_main: 8daec14  # docs: record v0.4.0 release SHA (tag v0.4.0 on c542e0f)
last_step: 2026-06-02 — started T-205 (Flathub runtime bump). Edited the Flatpak manifest GNOME 48→50 (freedesktop base 24.08→25.08): runtime-version 50, llvm19→llvm20, the three /usr/lib/sdk/llvm19 paths→llvm20. Installing GNOME 50 Platform/Sdk + rust-stable//25.08 + llvm20//25.08, then re-running flatpak-builder to confirm it builds + gst-inspect still finds gtk4paintablesink with no EOL warning.
next_step: finish T-205 — once GNOME 50 SDK is installed, run flatpak-builder against the updated manifest, verify build + sandbox gst-inspect + no EOL warning, then commit the manifest change.
blockers: none
working_tree:
  status: build-aux/io.github.domatix.ObsbotCamControl.json (runtime 48→50, llvm19→llvm20) held uncommitted pending the GNOME-50 flatpak-builder verification; everything else (docs cleanup + T-205 scaffolding) committed.
v0_4_0_gate:
  - T-203 build gate: DONE + verified headless 2026-06-02 (flatpak-builder builds all 3 modules, installs io.github.domatix.ObsbotCamControl 0.3.2, sandbox gst-inspect-1.0 finds gtk4paintablesink in /app/lib/gstreamer-1.0/libgstgtk4.so).
  - T-203 render check: PENDING USER — launch the installed Flatpak, toggle preview, confirm camera frames render on screen. Not machine-verifiable. Last thing before v0.4.0.
  - runtime EOL: org.gnome.Platform//48 is EOL (2026-03-24). Builds today; bump to GNOME 49+ before any Flathub submission.
still_open_non_hardware:
  - T-017 Arch PKGBUILD build/install/remove on an Arch host (community stakeholder, no rush).
  - T-202 minor: grayscale toggled while preview off is lost on start (re-apply on start, or disable filter buttons while off).
follow_ups_queued:
  - sepia-invert-filters, file-chooser-snapshot, preferences-dialog (preview-default-on), verify-q9-tiny2-regular — unchanged from prior STATE. (ptz-speed-fast dropped in T-101d.)
  - branch-hygiene: feat/* branches retained locally; delete only on explicit user ask.
  - T-400 (post-v1.0): add OBSBOT Meet (original) to the model matrix.
known_issues:
  - Q9 (PROTOCOL.md): pan_speed/tilt_speed accept writes but no motion on Tiny 2 Lite firmware 5.10. PTZ moves via discrete pan_absolute/tilt_absolute single steps (T-101d).
updated_at: 2026-06-02T09:00:00Z
