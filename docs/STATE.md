# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none
active_task_state: idle
last_completed_task: T-012
last_commit: feat(core): USB enumeration for Tiny 2 (T-011)  # 2f97569
last_step: T-012 DONE — `crates/obsbot-cli/Cargo.toml` picks up the `obsbot-core` path dep that T-006 deferred; `crates/obsbot-cli/src/main.rs` becomes a `clap` subcommand router with `Commands::List` calling `obsbot_core::enumerate_cameras()` through a pure `render(&[CameraInfo]) -> String` helper. Three new unit tests (`render_zero_cameras`, `render_one_camera_missing_serial`, `render_two_cameras_indexed_and_pluralised`) pin the documented stanza format. Live smoke test against the user's plugged-in Tiny 2 Lite prints the expected single-camera stanza (3564:fef9 / /dev/video0); `obsbot-cli list --help` surfaces the six-field schema verbatim. All four cargo gates green; commit pending.
next_step: commit `feat(cli): list command (T-012)`, then T-013 (diagnostics view in the GUI — hot-plug listener on top of `enumerate_cameras`), followed by T-014 (Flatpak), T-015 (CI), T-016 (.deb), T-017 (Arch) to close v0.1.
blockers: none.
working_tree:
  pre_commit_modified: [crates/obsbot-cli/Cargo.toml, crates/obsbot-cli/src/main.rs, Cargo.lock, docs/PLAN.md, docs/STATE.md, docs/PROGRESS.md]
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
updated_at: 2026-05-13T16:12:00Z  # T-012 DONE, commit pending
