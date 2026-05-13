# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none
active_task_state: idle
last_completed_task: T-014
last_commit: fix(flatpak): runtime fixes + close T-014 (T-014)  # 68e875d
last_step: T-014 DONE. flatpak + flatpak-builder + GNOME 48 runtime/SDK + rust-stable SDK extension installed by the user. First flatpak-builder run failed (bindgen couldn't find libclang inside the sandbox); fix = `org.freedesktop.Sdk.Extension.llvm19//24.08` added as a second sdk-extension + `LIBCLANG_PATH=/usr/lib/sdk/llvm19/lib` in build-options.env. Second run failed at the export stage (`flatpak-validate-icon: Format not recognized` on the symbolic SVG); fix = remove the SPDX/copyright SVG comments between the XML declaration and the `<svg>` root from both T-010 icons (bisected to those comments; the symbolic loader is stricter than the regular SVG loader at 16x16). Third run succeeded; the app installs as `io.github.domatix.ObsbotCamControl 0.1.0 master`. User-confirmed via `flatpak run`: camera row, drill-down with 22 controls, and hot-plug all work identically to the native binary; `--device=all` grants /dev/video0 access from the sandbox. Two commits on `main`: `39d5d6f` (initial manifest) + `68e875d` (three fixes + closure).
next_step: T-015 (CI workflows — BLOCKED until repo is public per its PLAN note), T-016 (.deb test artifact via cargo-deb), T-017 (Arch PKGBUILD) to close v0.1. Bumping GNOME 48 to a supported runtime (EOL'd 2026-03-24) becomes a pre-v1.0 readiness task — note recorded in T-014 outcome.
blockers: none for T-016/T-017. T-015 is BLOCKED on repo being public on GitHub.
working_tree:
  pre_commit_modified: [.gitignore, build-aux/io.github.domatix.ObsbotCamControl.json, data/icons/scalable/apps/io.github.domatix.ObsbotCamControl.svg, data/icons/symbolic/apps/io.github.domatix.ObsbotCamControl-symbolic.svg, docs/PLAN.md, docs/STATE.md, docs/PROGRESS.md]
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
updated_at: 2026-05-13T17:58:00Z  # T-014 SHA recorded
