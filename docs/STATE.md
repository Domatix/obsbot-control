# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none
active_task_state: idle
last_completed_task: T-013a
last_commit: feat(cli): list command (T-012)  # 1e516b5
last_step: T-013a DONE. T-013 was split via [[ADR-0016]] into T-013a (initial list, now done), T-013b (hot-plug, TODO), T-013c (V4L2 controls, TODO), T-013d (Blueprint pipeline, TODO). `crates/obsbot-gui/src/window.rs` now mounts an `AdwPreferencesPage` with one `AdwActionRow` per camera returned by `obsbot_core::enumerate_cameras()` at startup; empty-state remains an `AdwStatusPage`. User-confirmed visual: the Tiny 2 Lite shows as a row with subtitle `3564:fef9 · /dev/video0` and a camera prefix icon, inside a "Connected cameras" group. Four cargo gates green; xwininfo confirmed the same 842x662 window shape T-007 verified. Commit pending.
next_step: commit `feat(gui): initial camera list (T-013a)`, then T-013b (hot-plug listener — polling first-pass per ADR-0016), T-013c (V4L2 control sub-page), T-013d (Blueprint pipeline), T-014..T-017 to close v0.1.
blockers: none.
working_tree:
  pre_commit_modified: [crates/obsbot-gui/src/window.rs, docs/DECISIONS.md, docs/PLAN.md, docs/STATE.md, docs/PROGRESS.md]
  pre_commit_untracked: []
  pre_commit_deleted: []
pending_user_actions:
  - T-013 (later, v0.1): log out / log back in to pick up the new
    `video` group membership.
  - T-010 (next time you log in): observe whether GNOME Shell now
    paints our webcam icon when you launch the app via `cargo run
    -p obsbot-gui`. If it still shows the generic placeholder
    after a fresh session, file a follow-up task (the install path
    via Flatpak/distro should resolve it; we revisit only if the
    same failure persists there).
updated_at: 2026-05-13T16:25:00Z  # T-013a DONE, commit pending
