# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: T-010
active_task_state: awaiting_user_visual
last_completed_task: T-009
last_commit: docs: record T-009 SHA in STATE.md (T-009)  # 39c1a51  # refreshes to T-010 SHA after this commit
last_step: T-010 code-complete — two SVGs land under `data/icons/{scalable,symbolic}/apps/`; `data/meson.build` installs them under hicolor and runs `gnome.post_install` for icon-cache + desktop-database refresh on real (non-DESTDIR) install; `crates/obsbot-gui/src/application.rs` calls `gtk::Window::set_default_icon_name(app_id)` at startup. All cargo gates + the two meson tests green; install under `/tmp/install-test` produces the expected five files. Visual confirmation handed to the user, matching the T-007 precedent.
next_step: user-visual confirmation closes T-010; then T-011 (USB enumeration for the Tiny 2 family — first real backend code, depends on T-005 + T-003). After T-011: T-012 (CLI list), T-013 (diagnostics view), T-014 (Flatpak), T-015 (CI), T-016 (.deb), T-017 (Arch) remain to close v0.1.
blockers: none — T-010 is code-complete and committed; only a user-side visual check remains, which does not block T-011.
working_tree:
  pre_commit_modified: []
  pre_commit_untracked: []
  pre_commit_deleted: []
pending_user_actions:
  - T-013 (later, v0.1): log out / log back in to pick up the new
    `video` group membership.
  - T-010 (now): the simplest visual check is
    `meson install -C builddir --destdir=$HOME/.local-icontest`
    followed by `XDG_DATA_DIRS=$HOME/.local-icontest/usr/local/share:$XDG_DATA_DIRS gtk4-update-icon-cache -t -f $HOME/.local-icontest/usr/local/share/icons/hicolor`
    and then `XDG_DATA_DIRS=$HOME/.local-icontest/usr/local/share:$XDG_DATA_DIRS cargo run -p obsbot-gui` — the icon should
    show in GNOME Shell's window list / Alt+Tab. A simpler smoke test
    is `gtk4-icon-browser` after installing under `~/.local/share`.
updated_at: 2026-05-13T13:18:54Z
