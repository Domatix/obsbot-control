# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: T-232  # App id renamed (ADR-0033), v0.5.0 re-cut on 0e35052. Bundle rebuild + linter re-run in progress; then PR by the user.
active_task_state: IN_PROGRESS
active_branch: main
last_completed_task: T-232 (rename half)  # app id io.github.domatix.obsbot-control; T-230/T-231 DONE; submission PR pending user.
last_milestone: v0.5.0  # Flathub submission release, tag re-cut 2026-08-14 on 0e35052. v0.6 (Polish) next.
last_commit_on_main: 0e35052  # refactor: rename app id (T-232). Bundle pin bump + docs commits pending below.
last_step: 2026-08-14 — official linter run: only error was appid-url-not-reachable; user chose the rename. ID now io.github.domatix.obsbot-control (schema, resources, textdomain, icons, binary obsbot-control, manifests, CI, PKGBUILDs, docs). v0.5.0 tag deleted + re-cut; main pushed. appstream lint OK. Bundle rebuild in progress (verify2).
next_step: finish bundle rebuild (offline, exit 0) + re-run flatpak-builder-lint (expect appid error gone) → commit bundle pin + PROGRESS/STATE → push → USER opens the submission PR against flathub/flathub@new-pr by hand (Flathub AI policy).
blockers:
  - Submission PR must be opened by the user (Flathub Generative-AI policy: no AI-opened/automated submission PRs).
  - native /usr/local install pending user sudo (from v0.4.2); T-017b Arch validation pending an Arch host.
working_tree:  # matches `git status --short`
  - M build-aux/flathub/io.github.domatix.obsbot-control.json  # git commit pin bumped to 0e350527 (T-232)
  - M docs/{DECISIONS,PLAN,PROGRESS,STATE}.md  # ADR-0033, T-232 updates, journal
firmware_notes: Tiny 2 Lite fw 5.10 — XU Sleep IGNORED ~3s after streaming stops (cold Sleep immediate); rapid open/close/sleep churn can hang capture until USB replug (ADR-0025). Q9: pan/tilt_speed write but no motion; PTZ via discrete steps (T-101d).
flatpak_build_note: rofiles-fuse cannot mount on /tmp (nodev tmpfs) — run flatpak-builder with --state-dir and build dir under $HOME, never /tmp.
updated_at: 2026-08-14T13:35:00Z
