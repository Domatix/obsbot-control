# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none  # v0.3.2 cut + pushed; v0.4.0 deferred behind the Flatpak gate
active_task_state: —
active_branch: main
last_completed_task: T-204  # preview pane 20% shrink — last of the v0.3.2 bundle
last_milestone: v0.3.2  # point release cut 2026-06-02 (native channel); v0.3.1 was 2026-05-19 on e3ad521
last_commit_on_main: 34665aa  # fix(flatpak): blueprint-compiler module (T-203); docs commit follows
last_step: 2026-06-02 — after cutting + pushing v0.3.2 (tag on 50a6143), ran the T-203 flatpak-builder smoke-test at the user's request. It surfaced a real manifest gap (GNOME Sdk 48 lacks blueprint-compiler) — fixed with a build-only blueprint-compiler module (34665aa) — and a /tmp tmpfs disk overflow (re-ran on /home). Build now succeeds: blueprint-compiler + gst-plugin-gtk4 (libgstgtk4.so) + app all build, app installs, and sandbox gst-inspect finds gtk4paintablesink. v0.4.0 now waits only on the user's on-screen render confirmation.
next_step: v0.4.0 is one user-glance away. The T-203 flatpak-builder smoke-test PASSED 2026-06-02 (build + install + sandbox gst-inspect finds gtk4paintablesink); manifest fixed with a blueprint-compiler module (34665aa). The ONLY remaining gate is the user launching the installed Flatpak (`flatpak run io.github.domatix.ObsbotCamControl`), toggling preview with the Tiny 2 connected, and confirming frames render. On that confirmation → cut v0.4.0 (bump 0.3.2→0.4.0, AppStream <release>, README, tag, push).
blockers: none  # awaiting one user visual confirmation for v0.4.0
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
  - sepia-invert-filters, file-chooser-snapshot, preferences-dialog (ptz-speed-fast + preview-default-on), verify-q9-tiny2-regular — unchanged from prior STATE.
  - branch-hygiene: feat/* branches retained locally; delete only on explicit user ask.
  - T-400 (post-v1.0): add OBSBOT Meet (original) to the model matrix.
known_issues:
  - Q9 (PROTOCOL.md): pan_speed/tilt_speed accept writes but no motion on Tiny 2 Lite firmware 5.10. PTZ uses pan_absolute accumulator workaround (T-101c).
updated_at: 2026-06-02T08:10:00Z
