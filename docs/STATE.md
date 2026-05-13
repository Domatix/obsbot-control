# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none
active_task_state: idle
last_completed_task: T-017
last_milestone: v0.1.0
last_commit: build(arch): test-artifact PKGBUILD (T-017)  # ea58595 (milestone close commit pending in this turn)
last_step: v0.1.0 milestone closed per CLAUDE.md §7. 17 tasks DONE (2 with caveat: T-010, T-017), T-013 SUPERSEDED, T-013d DEFERRED to v0.2, T-015 BLOCKED but explicitly deferred to v0.1.1 / v0.2 per [[ADR-0018]] (this session). Gates green: cargo fmt/clippy/test pass, Flatpak builds, .deb installs/launches/removes cleanly (user-verified T-016), PKGBUILD statically validates against arch-meson simulation. README current with all three distribution channels documented (Flatpak + .deb + Arch). About to commit ADR-0018 + this STATE + the milestone PROGRESS entry, then `git tag v0.1.0` annotated and `git push --tags`.
next_step: after tag lands, session-end checkpoint. v0.2 entry point will be T-099 (Blueprint pipeline) per [[ADR-0017]] — that's the first task to pick up next session when ready to start v0.2.
blockers: T-015 deferred (ADR-0018); will re-evaluate when public release repo lands.
working_tree:
  pre_commit_modified: [docs/DECISIONS.md, docs/PROGRESS.md, docs/STATE.md]
  pre_commit_untracked: []
  pre_commit_deleted: []
pending_user_actions:
  - T-013 (later, v0.2 area): log out / log back in to pick up
    the new `video` group membership.
  - T-010 (next time you log in): observe whether GNOME Shell now
    paints our webcam icon when you launch the app via `cargo run
    -p obsbot-gui`. If it still shows the generic placeholder
    after a fresh session, file a follow-up task (the install
    path via Flatpak/distro should resolve it; we revisit only
    if the same failure persists there).
  - T-017 (Arch stakeholder, whenever): clone the repo on an Arch
    box, run `./build-aux/build-arch.sh`, `sudo pacman -U` the
    artifact, launch obsbot-cam-control, and `sudo pacman -R` to
    verify clean removal. Any issues land in v0.1.1.
  - Public release repo / Flathub prep (v1.0 area): split out per
    the strategy noted in the GitHub-remote-online PROGRESS
    entry; that unlocks T-015 (CI + badge) for v0.1.1 or v0.2.
updated_at: 2026-05-13T20:50:00Z  # v0.1.0 milestone close
