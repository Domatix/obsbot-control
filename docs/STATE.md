# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: T-232  # Everything automatable is done (rename, bundle, linter clean, CI green, release published). Only the user-opened Flathub PR remains.
active_task_state: IN_PROGRESS
active_branch: main
last_completed_task: T-232 (automation half)  # v0.5.0 released on GitHub with artifacts; bundle verified; linter clean. User PR pending.
last_milestone: v0.5.0  # Flathub submission release (tag at 20d9b08). v0.6 (Polish) next.
last_commit_on_main: 0206bcd  # fix(ci): GH_REPO for the release job. Tree clean.
last_step: 2026-08-14 — submission branch pushed to the user's fork (alvaro-domatix/flathub@obsbot-control-submission, from new-pr, exactly the 3 bundle JSONs). PROGRESS recorded. Only the user's PR click remains.
next_step: USER opens the PR in the github.com web interface (base must be new-pr, title "Add io.github.domatix.obsbot-control"), writes their own description, answers review and comments `bot, build` when asked. After acceptance: push the bundle to flathub/io.github.domatix.obsbot-control; future releases are plain commits there.
blockers:
  - Submission PR must be opened by the user by hand (Flathub Generative-AI policy: AI must not open or automate submission PRs).
  - native /usr/local install pending user sudo (from v0.4.2); T-017b Arch validation pending an Arch host.
working_tree: clean  # matches `git status --short`
firmware_notes: Tiny 2 Lite fw 5.10 — XU Sleep IGNORED ~3s after streaming stops (cold Sleep immediate); rapid open/close/sleep churn can hang capture until USB replug (ADR-0025). Q9: pan/tilt_speed write but no motion; PTZ via discrete steps (T-101d).
flatpak_build_note: rofiles-fuse cannot mount on /tmp (nodev tmpfs) — run flatpak-builder with --state-dir and build dir under $HOME, never /tmp.
updated_at: 2026-08-14T14:05:00Z
