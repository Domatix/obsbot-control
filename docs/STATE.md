# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none
active_task_state: idle
last_completed_task: T-005
last_commit: docs: capture Tiny 2 Lite V4L2 controls (T-003)  # b5990ba (T-005 commit pending in this turn)
last_step: T-005 DONE — `crates/obsbot-core/` scaffolded with Camera trait, CameraInfo, Capabilities (26 bool flags), supporting enums, and Error type. All four T-005 gates green (fmt-check, check --workspace, clippy with -D warnings, test = 3 unit + 1 doc). Cargo.lock now tracked; `crates/.gitkeep` removed.
next_step: propose T-006 (stub `obsbot-cli` crate — clap-based binary with `--version`). T-006 depends only on T-005, which is now DONE.
blockers: none.
working_tree:
  pre_commit_modified: [docs/PLAN.md, docs/STATE.md, docs/PROGRESS.md]
  pre_commit_deleted: [crates/.gitkeep]
  pre_commit_untracked: [Cargo.lock, crates/obsbot-core/]
pending_user_actions:
  - T-013 (later, v0.1): log out / log back in to pick up the new
    `video` group membership before T-013.
updated_at: 2026-05-13T10:52:32Z
