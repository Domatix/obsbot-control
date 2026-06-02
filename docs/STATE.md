# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none  # v0.4 first slice validated; release-tag decision + T-204 queued
active_task_state: —
active_branch: main
last_completed_task: T-202  # grayscale (dual-videoconvert fix) — with T-101c + T-201, validated 2026-06-02
last_milestone: v0.3.1  # tag cut 2026-05-19 on e3ad521
last_commit_on_main: <docs-commit>  # this STATE update; preceded by the two validation-fix commits below
last_step: 2026-06-02 — user validated the parked list against the connected Tiny 2 Lite ("he validado todo"). Code review + cargo gates (default + obsbot-gui/live-preview) re-run green by Claude. Two fixes that were sitting UNCOMMITTED in the working tree (discovered during the prior validation session, never recorded — see DECISIONS/§4.4 discipline note) are now committed: `fix(gui): grayscale filter no-op via dual videoconvert (T-202)` and `fix(gui): smooth PTZ hold via local accumulator (T-101c)`. T-101c / T-201 / T-202 marked DONE in PLAN with their validation-discovered fixes recorded. T-204 (shrink preview pane ~20%) queued TODO per user request.
next_step: Decide the next release tag (see open_decision). Either path needs an AppStream <releases> block update. T-204 is a one-line height_request change ready whenever.
blockers: none
working_tree:
  status: clean  # all validation fixes + docs committed this session
open_decision:
  cut-v0.4.0-or-v0.3.2:
    - v0.4.0: declares the "Live Preview" milestone DONE. Per CLAUDE.md §7 this REQUIRES the Flatpak to build — i.e. run flatpak-builder against build-aux/io.github.domatix.ObsbotCamControl.json (the T-203 `flatpak-builder` smoke-test, still the lone open gate) before tagging.
    - v0.3.2: native-only patch rollup (cargo/.deb), like v0.3.1. No Flatpak gate because it does not claim the milestone. Ships the validated snapshot/grayscale/PTZ-tuning work now; defers v0.4.0 until Flatpak is proven.
still_open_non_hardware:
  - T-203 flatpak-builder smoke-test (needs host Flatpak re-enabled; gates a v0.4.0 cut, not a v0.3.2 cut).
  - T-017 Arch PKGBUILD build/install/remove on an Arch host (community stakeholder, no rush).
  - T-202 minor: grayscale toggled while preview off is lost on start (re-apply on start, or disable filter buttons while off).
follow_ups_queued:
  - T-204: shrink preview pane ~20% (height_request 240 → 192 in build_preview_widgets). User feedback 2026-06-02.
  - sepia-invert-filters, file-chooser-snapshot, preferences-dialog (ptz-speed-fast + preview-default-on), verify-q9-tiny2-regular — unchanged from prior STATE.
  - branch-hygiene: feat/* branches retained locally; delete only on explicit user ask.
  - T-400 (post-v1.0): add OBSBOT Meet (original) to the model matrix.
known_issues:
  - Q9 (PROTOCOL.md): pan_speed/tilt_speed accept writes but no motion on Tiny 2 Lite firmware 5.10. PTZ uses pan_absolute accumulator workaround (T-101c).
updated_at: 2026-06-02T06:47:44Z
