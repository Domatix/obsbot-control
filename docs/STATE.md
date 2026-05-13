# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none
active_task_state: idle
last_completed_task: T-006
last_commit: docs: add .deb + Arch test-package scope (ADR-0015)  # a4c1ef8 (T-006 commit pending in this turn)
last_step: T-006 DONE — `crates/obsbot-cli/` scaffolded; `cargo run -p obsbot-cli -- --version` prints `obsbot-cli 0.1.0` (clap), bare run prints `obsbot-cli v0.1.0` (println). Four workspace gates green.
next_step: propose T-007 (stub `obsbot-gui` crate — `adw::Application` opening an empty `AdwApplicationWindow` with header bar "OBSBOT Control"). Depends only on T-005 which is DONE.
blockers: none.
working_tree:
  pre_commit_modified: [Cargo.lock, docs/PLAN.md, docs/STATE.md, docs/PROGRESS.md]
  pre_commit_untracked: [crates/obsbot-cli/]
pending_user_actions:
  - T-013 (later, v0.1): log out / log back in to pick up the new
    `video` group membership before T-013.
updated_at: 2026-05-13T11:02:07Z
