# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none
active_task_state: idle
last_completed_task: T-017
last_milestone: v0.1.0  # tag 5e005fd, pushed to origin
last_commit: docs: cut v0.1.0 milestone (ADR-0018)  # 5e005fd, tagged v0.1.0
last_step: v0.1.0 shipped. Annotated tag pushed to `github.com/Domatix/obsbot-control` (PRIVATE). T-017 (Arch PKGBUILD) closed DONE-with-caveat; downstream `makepkg`+`pacman` run is the Arch stakeholder's job. ADR-0018 amends CLAUDE.md §7 to accept "explicitly-deferred via ADR" alongside strict DONE, justifying the tag while T-015 stays BLOCKED on public-repo move. v0.1 work delivers `obsbot-cli list` + `obsbot-cam-control` GUI (hot-plug list + read-only V4L2 drill-down) across three distribution channels (Flatpak, .deb, Arch). Session-end clean.
next_step: v0.2 entry point is T-099 (Blueprint pipeline, absorbed from T-013d per [[ADR-0017]]). Must land before any T-100+ task introducing a static widget tree (slider forms, PTZ pad, zoom). After T-099, the T-100 series picks up control-write features.
blockers: none active. T-015 deferred per ADR-0018; lands in v0.1.1 or v0.2 when the public release repo enables it.
working_tree:
  pre_commit_modified: [docs/STATE.md]
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
    entry; that unlocks T-015 (CI + badge) for v0.1.1 / v0.2.
updated_at: 2026-05-13T20:55:00Z  # session-end, v0.1.0 shipped
