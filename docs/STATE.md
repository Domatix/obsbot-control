# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none
active_task_state: idle
last_completed_task: T-004
last_commit: build: create cargo workspace (T-004)  # pending — to be created in this turn
last_step: T-004 DONE via [[ADR-0013]]-amended criteria — `cargo metadata` exit 0, `cargo verify-project` success. Rust toolchain 1.85.0 from Debian trixie active.
next_step: propose T-005 (stub `obsbot-core` crate) to the user; T-005 inherits the original `cargo check --workspace` + `cargo fmt --all --check` gates per [[ADR-0013]].
blockers: T-003 BLOCKED on user hardware capture (Tiny 2 lsusb/v4l2-ctl).
working_tree:
  clean_expected_after_commit: true
  pre_commit_modified: [.gitignore, docs/PLAN.md, docs/PROGRESS.md, docs/STATE.md, docs/DECISIONS.md]
  pre_commit_untracked: [Cargo.toml]
resume_protocol: |
  Same or new session, from /home/alvaro/Documentos/proyectos/obsbot-control:
    1. `claude` (Claude Code auto-loads CLAUDE.md and reads this STATE.md).
    2. Say "continúa" or "siguiente tarea" — Claude proposes T-005.
    3. T-003 can be unblocked at any time by pasting `lsusb -v -d <vid>:<pid>`,
       `v4l2-ctl --all`, `v4l2-ctl --list-ctrls-menus` outputs.
pending_user_actions:
  - T-003: paste outputs of `lsusb -v -d VID:PID`, `v4l2-ctl --all`,
      `v4l2-ctl --list-ctrls-menus` (commands given 2026-05-12T11:00Z).
updated_at: 2026-05-13T10:10:18Z
