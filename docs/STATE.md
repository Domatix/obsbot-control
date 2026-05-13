# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: T-016
active_task_state: IN_PROGRESS
last_completed_task: T-014
last_commit: docs: forbid co-author trailers; resync SHA references  # 291245a
last_step: T-016 code-complete + static validation green. `build-aux/build-deb.sh` (cargo deb 2.12.1 pinned at `^2.10` because 3.7.0 needs rustc 1.88) produces `build-aux/dist/obsbot-cam-control_0.1.0-1_amd64.deb` (201 KB; installed-size 558 KB). `dpkg-deb -I/-c` confirms control fields (`Section: video`, `Priority: optional`, auto-detected `Depends: libadwaita-1-0 (>= 1.4~beta), libc6 (>= 2.34), libglib2.0-0t64 (>= 2.54.0), libgtk-4-1 (>= 4.0.0)`) and the seven freedesktop-standard install paths (binary, desktop, metainfo, scalable+symbolic icons, copyright). `desktop-file-validate` + `appstreamcli validate --no-net` pass on the substituted templates. Remaining acceptance criteria are install / launch / remove on the user's machine.
next_step: ask the user to `sudo apt install ./build-aux/dist/obsbot-cam-control_0.1.0-1_amd64.deb`, launch `obsbot-cam-control` to confirm the camera row + drill-down still work installed-as-deb, then `sudo apt remove obsbot-cam-control` and verify no stray files under `/usr/share/{applications,icons/hicolor,metainfo,doc/obsbot-cam-control}`. After confirmation: PLAN T-016 → DONE, single commit `build(deb): test-artifact .deb via cargo-deb (T-016)`, then T-017 (Arch PKGBUILD) closes v0.1.
blockers: none. T-015 remains BLOCKED on public repo.
working_tree:
  pre_commit_modified: [Cargo.lock, README.md, crates/obsbot-gui/Cargo.toml, docs/PLAN.md, docs/PROGRESS.md, docs/STATE.md]
  pre_commit_untracked: [build-aux/build-deb.sh, build-aux/dist/]
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
  - T-016 (this turn): once the `.deb` builds and `dpkg-deb -I/-c`
    look clean, `sudo apt install ./build-aux/dist/obsbot-cam-control_*_amd64.deb`,
    launch `obsbot-cam-control`, then `sudo apt remove obsbot-cam-control`
    to verify no stray files.
updated_at: 2026-05-13T19:32:00Z  # T-016 code-complete; awaiting install/remove validation
