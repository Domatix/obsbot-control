# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none  # v0.3.2 cut + pushed; T-101d PTZ simplification + flatpak ready for user re-test
active_task_state: —
active_branch: main
last_completed_task: T-101d  # stripped PTZ to pure single-step (reverted hold) after user found it buggy
last_milestone: v0.3.2  # point release cut 2026-06-02 (native channel); v0.3.1 was 2026-05-19 on e3ad521
last_commit_on_main: b0018c9  # fix(gui): strip PTZ to single-step (T-101d) + docs; this SHA fixup follows
last_step: 2026-06-02 — testing the v0.3.2 Flatpak the user reported the PTZ press-and-hold / keyboard-repeat "va fatal, se buguea muchísimo". Rewrote ptz_pad.rs to pure single-step (one click/keypress = one 5° move), removing ALL hold timers + PtzAccumulators + ptz-speed-fast (gschema + settings). Added next_position unit tests (4, pass). ADR-0021 records the reversal; T-101d DONE, T-101c SUPERSEDED. Gates green default + live-preview. Rebuilt + reinstalled the Flatpak so the user can re-test.
next_step: TWO user-glance confirmations now pending on the freshly-rebuilt Flatpak (flatpak run io.github.domatix.ObsbotCamControl): (1) PTZ — one button click / one arrow press = exactly one move, no runaway; (2) preview renders camera frames. On both green → cut v0.4.0 (bump 0.3.2→0.4.0, AppStream <release>, README, tag, push).
blockers: none  # awaiting two user visual confirmations (PTZ single-step + preview render) for v0.4.0
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
updated_at: 2026-06-02T08:40:00Z
