# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none  # v0.4.0 milestone (Live Preview) cut 2026-06-02; next is v0.6 polish or Flathub prep
active_task_state: —
active_branch: main
last_completed_task: T-101d  # PTZ single-step; closed alongside the v0.4 task set in the v0.4.0 cut
last_milestone: v0.4.0  # Live Preview milestone cut 2026-06-02 (Flatpak-validated); v0.3.2 same day (native rollup)
last_commit_on_main: c542e0f  # chore: bump 0.3.2 → 0.4.0 (tag v0.4.0); this SHA fixup follows
last_step: 2026-06-02 — user confirmed on the rebuilt Flatpak that PTZ is reliable single-step AND the preview renders frames. Both v0.4.0 gates closed → cut the Live Preview milestone v0.4.0: version bump 0.3.2→0.4.0 (Cargo+meson), AppStream v0.4.0 <release> (validate green), README + .deb + ROADMAP map updated, annotated tag v0.4.0, pushed main + tags. CLAUDE.md §7 DoD fully met (Flatpak builds + renders).
next_step: No active task. Candidates: (a) v0.6 polish (Spanish translation, keyboard shortcuts, onboarding, perf/a11y audits — see ROADMAP v0.6); (b) Flathub prep — bump the manifest off the EOL GNOME 48 runtime to 49+ and re-test (prerequisite for any Flathub submission); (c) small queued follow-ups (T-202 grayscale-while-off, sepia/invert, file-chooser snapshot, T-017 Arch PKGBUILD). Propose one to the user.
blockers: none
working_tree:
  status: clean  # release v0.3.2 + the T-203 manifest fix committed (push pending for the manifest fix + docs)
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
