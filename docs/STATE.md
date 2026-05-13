# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none
active_task_state: idle
last_completed_task: T-011
last_commit: feat(core): USB enumeration for Tiny 2 (T-011)  # 2f97569
last_step: T-011 DONE — `crates/obsbot-core/src/enumerate.rs` lands with `VID_OBSBOT`, `TINY2_FAMILY`, `enumerate_cameras()`, and `enumerate_cameras_in()`. Five new unit tests pass (8 unit + 1 doc total for obsbot-core), and the `#[ignore]`d hardware test detects the user's plugged-in Tiny 2 Lite (VID 0x3564 / PID 0xfef9, video_path /dev/video0, product starts with "OBSBOT"). All cargo gates + meson tests green. `tempfile` added as workspace + dev dependency.
next_step: T-012 (CLI `list` subcommand — depends on T-006 and T-011, both DONE). Then T-013 (diagnostics view, needs hot-plug listener on top of `enumerate_cameras`), T-014 (Flatpak), T-015 (CI), T-016 (.deb), T-017 (Arch) close v0.1.
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
updated_at: 2026-05-13T15:55:02Z
