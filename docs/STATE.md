# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none
active_task_state: idle
last_completed_task: T-010
last_commit: docs: record T-010 SHA in STATE.md (T-010)  # ec0da31
last_step: T-010 DONE with caveat — code complete (icons + meson + GUI default-icon all green, all gates pass); the two visual acceptance criteria are deferred. End-to-end Alt+Tab test on the user's running session showed the generic-app placeholder because GNOME Shell builds its `.desktop` → window-icon cache at session startup and does not pick up mid-session drops into `~/.local/share/applications/`. The proper visual test path is T-014 Flatpak, T-016 / T-017 distro packages, or the user's next GNOME login.
next_step: T-011 (USB enumeration for the Tiny 2 family — first real backend code; depends on T-005 + T-003, both DONE). After T-011: T-012 (CLI list), T-013 (diagnostics view), T-014 (Flatpak), T-015 (CI), T-016 (.deb), T-017 (Arch) remain to close v0.1.
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
updated_at: 2026-05-13T15:44:58Z
