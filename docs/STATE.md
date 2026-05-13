# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none
active_task_state: idle
last_completed_task: T-013a
last_commit: feat(gui): initial camera list (T-013a)  # 6e8b861
last_step: T-013a DONE. T-013 was split via [[ADR-0016]] into T-013a (initial list, now done), T-013b (hot-plug, TODO), T-013c (V4L2 controls, TODO), T-013d (Blueprint pipeline, TODO). `crates/obsbot-gui/src/window.rs` now mounts an `AdwPreferencesPage` with one `AdwActionRow` per camera returned by `obsbot_core::enumerate_cameras()` at startup; empty-state remains an `AdwStatusPage`. User-confirmed visual: the Tiny 2 Lite shows as a row with subtitle `3564:fef9 · /dev/video0` and a camera prefix icon, inside a "Connected cameras" group. Four cargo gates green; xwininfo confirmed the same 842x662 window shape T-007 verified. Commit `6e8b861` on `main`.
next_step: T-013b (hot-plug listener — polling first-pass per [[ADR-0016]]: a `glib::timeout_add_local` on ~1 s diffing the latest `enumerate_cameras()` against the previous snapshot, adding/removing `AdwActionRow`s in place). After: T-013c V4L2 controls, T-013d Blueprint, T-014 Flatpak, T-015 CI, T-016 .deb, T-017 Arch.
blockers: none.
working_tree:
  pre_commit_modified: []
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
updated_at: 2026-05-13T16:28:00Z  # T-013a SHA recorded
