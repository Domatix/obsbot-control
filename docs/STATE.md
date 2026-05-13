# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none
active_task_state: idle
last_completed_task: T-013b
last_commit: feat(gui): hot-plug listener (T-013b)  # 27384cf
last_step: T-013b DONE. `crates/obsbot-gui/src/window.rs` mounts the body inside an `adw::Bin` slot and installs a `glib::timeout_add_local(POLL_INTERVAL=2s, …)` source that re-enumerates and replaces the slot's child only when `Vec<CameraInfo>` differs. The closure captures `body_slot` weakly (`glib::clone!(#[weak], #[upgrade_or] ControlFlow::Break)`) so the source auto-cleans when the window dies. User-confirmed hot-plug: unplugging the Tiny 2 Lite swaps in the empty-state `AdwStatusPage` within ~2-3 s; re-plugging brings the row back. All four cargo gates green; commit `27384cf` on `main`.
next_step: T-013c (V4L2 control sub-page — needs a new `obsbot-core` helper that reads the device's V4L2 controls via the `v4l` workspace dep and surfaces them as a list of `(name, current, min, max, step)` tuples per `CameraInfo`). After: T-013d Blueprint, T-014 Flatpak, T-015 CI, T-016 .deb, T-017 Arch.
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
updated_at: 2026-05-13T16:38:00Z  # T-013b SHA recorded
