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
last_commit_on_main: <release-commit>  # build: release v0.3.2; SHA recorded in the follow-up fixup
last_step: 2026-06-02 — user validated the parked list, then authorised "haz todo hasta el final". Committed the two orphaned validation fixes (07bc379 T-202 grayscale, 2d827aa T-101c hold), hardened §4.4 discipline (55f0cd2), closed T-101c/T-201/T-202 (c4f5290), shipped T-204 preview shrink (1551657), then cut point release v0.3.2: version bump 0.3.1→0.3.2 (Cargo+meson), AppStream v0.3.2 <release> block (validate green), README + .deb description refreshed, annotated tag v0.3.2, pushed main + tags to origin.
next_step: v0.4.0 is the next milestone — its only open gate is the T-203 `flatpak-builder` smoke-test (needs host Flatpak re-enabled + camera-side render check). When ready, run flatpak-builder against build-aux/io.github.domatix.ObsbotCamControl.json, confirm the installed app's preview finds gtk4paintablesink and renders, then cut v0.4.0. Otherwise pick up a v0.6 polish item.
blockers: none
working_tree:
  status: clean  # release committed + tagged + pushed
v0_4_0_gate:
  - T-203 flatpak-builder smoke-test: build the manifest, install, confirm the preview pipeline finds gtk4paintablesink and renders frames. Lone gate before a v0.4.0 milestone cut. Needs host Flatpak re-enabled (paused per private-repo policy) + hardware render validation.
still_open_non_hardware:
  - T-017 Arch PKGBUILD build/install/remove on an Arch host (community stakeholder, no rush).
  - T-202 minor: grayscale toggled while preview off is lost on start (re-apply on start, or disable filter buttons while off).
follow_ups_queued:
  - sepia-invert-filters, file-chooser-snapshot, preferences-dialog (ptz-speed-fast + preview-default-on), verify-q9-tiny2-regular — unchanged from prior STATE.
  - branch-hygiene: feat/* branches retained locally; delete only on explicit user ask.
  - T-400 (post-v1.0): add OBSBOT Meet (original) to the model matrix.
known_issues:
  - Q9 (PROTOCOL.md): pan_speed/tilt_speed accept writes but no motion on Tiny 2 Lite firmware 5.10. PTZ uses pan_absolute accumulator workaround (T-101c).
updated_at: 2026-06-02T07:25:00Z
