# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none
active_task_state: idle
last_completed_task: T-016
last_commit: build(deb): scaffold .deb test-artifact pipeline (T-016)  # 1980bf0 (code-complete); follow-up docs commit for closure pending in this turn
last_step: T-016 DONE. `.deb` test artifact pipeline lives: `[package.metadata.deb]` in `crates/obsbot-gui/Cargo.toml`, `build-aux/build-deb.sh` shim (meson configure_file → cargo deb), README "Test packages" section, `.gitignore` swallows `build-aux/dist/`. `cargo-deb` pinned at `^2.10` (3.7.0 needs rustc 1.88). User-verified on Debian trixie: `sudo apt install` succeeds, `dpkg -l` reports `ii`, `/usr/bin/obsbot-cam-control` is the expected 522632-byte mode-755 ELF, `--help` prints GLib's option-group output (proxy for launch — proves linker + GTK4 prereqs all wired), and `sudo apt remove` is clean (post-remove globs all return fish "No matches"). v0.1 is now at 87% — only T-015 (CI, BLOCKED on public repo) and T-017 (Arch PKGBUILD) remain.
next_step: T-017 — `build-aux/PKGBUILD` for Arch via `makepkg` (same shape as T-016: convenience artifact per [[ADR-0015]], not AUR-grade). Needs a container or fakeroot since the host is Debian, not Arch. With T-017 closed, v0.1 ships modulo the BLOCKED T-015.
blockers: none for T-017. T-015 remains BLOCKED on public-repo move.
working_tree:
  pre_commit_modified: [docs/PLAN.md, docs/STATE.md]
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
  - The installed `.deb` from T-016 is currently on the system
    (the latest `apt install` left it there). Remove with `sudo
    apt remove obsbot-cam-control` if you want a clean state, or
    keep it for daily use — it's identical to the Flatpak
    behaviour-wise.
updated_at: 2026-05-13T19:55:00Z  # T-016 closed
