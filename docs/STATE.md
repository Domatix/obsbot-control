# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: T-232  # Flathub submission — release cut + bundle built and verified. Only user steps remain.
active_task_state: IN_PROGRESS
active_branch: main
last_completed_task: T-232 (prep half)  # v0.5.0 tagged (afb9609a), build-aux/flathub/ bundle verified offline (exit 0). T-230/T-231 DONE earlier.
last_milestone: v0.5.0  # Flathub submission release, tagged 2026-08-14. v0.6 (Polish) next.
last_commit_on_main: afb9609  # chore(release): bump version to 0.5.0 (T-232). Bundle + docs commits pending below.
last_step: 2026-08-14 — version bump 0.5.0 + tag pushed; build-aux/flathub/ bundle (submission manifest pinned to tag afb9609a + cargo-sources copies + README) built offline end-to-end exit 0; built metainfo shows 0.5.0 + screenshots. ADR-0032 recorded.
next_step: commit bundle + docs (ADR-0032, PLAN T-232, PROGRESS, STATE) and push; then USER: flathub.org login → submit → push the 3 bundle files to flathub/io.github.domatix.obsbot-control. Future release = bump tag+commit in bundle, regen cargo-sources, add metainfo <release>.
blockers:
  - T-232 submit needs the user's flathub.org login (interactive OAuth). Everything up to that point is done and verified.
  - native /usr/local install pending user sudo (from v0.4.2); T-017b Arch validation pending an Arch host.
working_tree:  # matches `git status --short`
  - M docs/{DECISIONS,PLAN,PROGRESS,STATE}.md  # ADR-0032, T-232 updates, journal
  - ?? build-aux/flathub/{io.github.domatix.obsbot-control.json,README.md,cargo-sources.json,cargo-sources-gst.json}  # submission bundle (T-232)
firmware_notes: Tiny 2 Lite fw 5.10 — XU Sleep IGNORED ~3s after streaming stops (cold Sleep immediate); rapid open/close/sleep churn can hang capture until USB replug (ADR-0025). Q9: pan/tilt_speed write but no motion; PTZ via discrete steps (T-101d).
flatpak_build_note: rofiles-fuse cannot mount on /tmp (nodev tmpfs) — run flatpak-builder with --state-dir and build dir under $HOME, never /tmp.
updated_at: 2026-08-14T13:05:00Z
