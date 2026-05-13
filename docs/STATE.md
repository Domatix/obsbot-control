# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none
active_task_state: idle
last_completed_task: T-013c
last_commit: feat: V4L2 control sub-page (T-013c)  # 6480adb
last_step: T-013c DONE. Backend: new `crates/obsbot-core/src/controls.rs` exposes `read_controls(path) -> Result<Vec<ControlDescriptor>>` against the `v4l 0.14` workspace dep; reshapes the v4l Description/Value types into the obsbot-core-owned `ControlDescriptor / ControlClass / ControlKind` so consumers never see the v4l crate types. Skips CtrlClass headers + DISABLED/WRITE_ONLY flags. 3 new unit tests on the `classify()` ID→class mapping; 1 new `#[ignore]`d hardware test asserts ≥22 controls + Brightness present on the user's Tiny 2 Lite. `home@0.5.11` pinned in Cargo.lock to keep the MSRV 1.85 compatible (transitive bindgen pulled in home 0.5.12 which needs rustc 1.88). GUI: `crates/obsbot-gui/src/window.rs` wraps everything in an `AdwNavigationView`; each camera row is now activatable with a `go-next-symbolic` suffix and `connect_activated` pushes the detail page returned by the new module `controls_view::build_controls_page(&cam)`. Detail page = AdwToolbarView + AdwPreferencesPage with one PreferencesGroup per V4L2 class; each control surfaces as `{name}: {current} · range {min}..={max} step {step}` / Yes-No / `{label} · N options`. Error paths render as AdwStatusPage. User-confirmed drill-down 2026-05-13T16:58Z: 22 controls show with values + ranges, back button works. Commit `6480adb` on `main`.
next_step: T-013d (Blueprint pipeline — `blueprint-compiler` apt install, meson custom_target, GResource bundle, migrate T-013a/c .ui from hand-coded to .blp templates). Then T-014 Flatpak, T-015 CI, T-016 .deb, T-017 Arch to close v0.1.
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
updated_at: 2026-05-13T17:00:00Z  # T-013c SHA recorded
