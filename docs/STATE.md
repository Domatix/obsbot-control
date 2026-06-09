# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: T-017b  # PKGBUILD half DONE; Arch validation transferred to incoming dev (ADR-0023 handoff)
active_task_state: IN_PROGRESS
active_branch: main
last_completed_task: T-206  # .deb now ships live-preview (ADR-0022); 0.4.0 hand-out artifacts (.flatpak bundle + .deb) produced for colleague testing
last_milestone: v0.4.0  # Live Preview milestone cut 2026-06-02 (Flatpak-validated); v0.3.2 same day (native rollup)
last_commit_on_main: 065fe14  # build(deb): ship live-preview in the .deb (T-206, pushed); handoff commit follows this STATE update
last_step: 2026-06-05 — PROJECT HANDOFF (ADR-0023). T-017b PKGBUILD refresh committed (pkgver 0.4.0, makedepends += blueprint-compiler, -Dlive-preview=true, gstreamer deps). Arch validation could not run here (Debian host, no container runtime); transferred to incoming developer. Added docs/HANDOFF.md as the human "start here". Audit confirmed all 5 local-only feat/* branches are residue already in main / v0.3.x tags — nothing orphaned on a fresh clone.
next_step: INCOMING DEV — read CLAUDE.md → this file → docs/HANDOFF.md. First actionable task is T-017b: run the Arch validation (./build-aux/build-arch.sh on Arch, or the docker archlinux:latest recipe it prints) — makepkg → pacman -U → binary exec → pacman -R, drop .pkg.tar.zst in build-aux/dist/. Then T-202, Flathub prep, T-400.
blockers: none on main. T-017b Arch validation pending an Arch host (transferred to incoming dev, not a blocker on this machine).
working_tree:
  status: clean after the handoff commit (build-aux/PKGBUILD + docs/PLAN.md + docs/PROGRESS.md + docs/DECISIONS.md + docs/STATE.md + new docs/HANDOFF.md committed together).
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
