# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: T-014
active_task_state: in_progress
last_completed_task: T-013c
last_commit: docs: defer T-013d Blueprint pipeline to v0.2 (ADR-0017)  # ce6206e
last_step: T-013d deferred to v0.2 via [[ADR-0017]] (ADR-0016's "T-013c will have many named children" premise didn't materialise; the V4L2 detail page renders from a dynamic Vec<ControlDescriptor> with zero named children, so the Blueprint pipeline overhead is premature now and will land in v0.2 as new T-099). T-014 (Flatpak manifest) started. flatpak and flatpak-builder are not installed on the host, so the manifest will be written + cargo gates checked locally, with runtime validation flagged PENDING_USER (needs ~1-2 GB GNOME 48 runtime/SDK download via `flatpak install`).
next_step: write `build-aux/io.github.domatix.ObsbotCamControl.json` (GNOME 48 runtime, --device=all + --socket=wayland/x11 + --share=ipc, rust-stable SDK extension, source from local dir, meson buildsystem), update README to point at the manifest, run cargo gates (no functional code change so should stay green), commit `build: initial Flatpak manifest (T-014)`. Acceptance criteria for `flatpak-builder` succeeding + `flatpak run` opening the diagnostics window remain PENDING_USER until they install the Flatpak environment.
blockers: T-014 runtime validation requires user-side `sudo apt install flatpak flatpak-builder` and `flatpak install --user flathub org.gnome.Platform//48 org.gnome.Sdk//48 org.freedesktop.Sdk.Extension.rust-stable//24.08` (the rust-stable extension is hosted under freedesktop). Not blocking the manifest commit.
working_tree:
  pre_commit_modified: [docs/PLAN.md, docs/STATE.md, docs/PROGRESS.md]
  pre_commit_untracked: [build-aux/io.github.domatix.ObsbotCamControl.json]
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
updated_at: 2026-05-13T17:05:00Z  # T-014 started, T-013d deferred
