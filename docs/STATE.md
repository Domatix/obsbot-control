# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none
active_task_state: idle
last_completed_task: T-017
last_commit: docs: record GitHub remote online (PRIVATE)  # 4636662 (T-017 work pending commit)
last_step: T-017 DONE-with-caveat. `build-aux/PKGBUILD` + `build-aux/build-arch.sh` + README Arch section land. Side-fix in `meson.build`: buildtype=plain now maps to cargo release (was debug; arch-meson uses --buildtype=plain by default so this matters). Static validation green: arch-meson simulation produces same BuildID as the .deb binary, install layout matches the .deb's freedesktop paths plus the Arch-idiomatic /usr/share/licenses/$pkgname/. cargo fmt/clippy/test all green. The literal `makepkg` + `pacman -U/-R` validation is deferred to the Arch stakeholder per [[ADR-0015]] (host is Debian, no docker/podman) — same shape as T-016's apt-install gate. v0.1 milestone evaluation pending in this session.
next_step: write ADR-0018 deciding whether to tag v0.1.0 with T-015 BLOCKED, append PROGRESS milestone entry, tag + push --tags. Then session-end checkpoint.
blockers: T-015 still BLOCKED on public-repo move (handled separately in milestone decision).
working_tree:
  pre_commit_modified: [README.md, docs/PLAN.md, docs/PROGRESS.md, docs/STATE.md, meson.build]
  pre_commit_untracked: [build-aux/PKGBUILD, build-aux/build-arch.sh]
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
  - T-017 (Arch stakeholder, whenever): clone the repo on an Arch
    box, run `./build-aux/build-arch.sh`, `sudo pacman -U` the
    artifact, launch obsbot-cam-control, and `sudo pacman -R` to
    verify clean removal.
updated_at: 2026-05-13T20:40:00Z  # T-017 closed
