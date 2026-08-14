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
last_step: 2026-08-14 — app id renamed (ADR-0033); v0.5.0 re-cut on 20d9b08; bundle pin + AUR PKGBUILD synced; flatpak-builder-lint manifest+appstream zero errors; tag CI repaired (first tag run exposed never-run packaging jobs) and now green except the release job, whose work was done manually: GitHub Release v0.5.0 published with .deb + Arch + CachyOS artifacts + SHA256SUMS; old-ID Flatpak uninstalled from the dev machine.
next_step: USER opens the submission PR (fork flathub/flathub with "copy master only" unchecked, branch new-pr, add the 3 JSON files of build-aux/flathub/, PR against new-pr titled "Add io.github.domatix.obsbot-control", answer review, `bot, build` when asked). Steps live in build-aux/flathub/README.md. After acceptance: push the bundle to flathub/io.github.domatix.obsbot-control and future releases are plain commits there.
blockers:
  - Submission PR must be opened by the user by hand (Flathub Generative-AI policy: AI must not open or automate submission PRs).
  - native /usr/local install pending user sudo (from v0.4.2); T-017b Arch validation pending an Arch host.
working_tree: clean  # matches `git status --short`
firmware_notes: Tiny 2 Lite fw 5.10 — XU Sleep IGNORED ~3s after streaming stops (cold Sleep immediate); rapid open/close/sleep churn can hang capture until USB replug (ADR-0025). Q9: pan/tilt_speed write but no motion; PTZ via discrete steps (T-101d).
flatpak_build_note: rofiles-fuse cannot mount on /tmp (nodev tmpfs) — run flatpak-builder with --state-dir and build dir under $HOME, never /tmp.
updated_at: 2026-08-14T14:05:00Z
