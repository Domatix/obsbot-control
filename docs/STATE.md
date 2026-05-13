# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none
active_task_state: idle
last_completed_task: T-099
last_milestone: v0.1.0  # tag 5e005fd
last_commit: build(gui): Blueprint pipeline (T-099)  # 8248d07
last_step: T-099 DONE. Blueprint pipeline live: `crates/obsbot-gui/resources/{window,controls-view}.blp` describe the static shells, `build.rs` compiles them via `blueprint-compiler 0.16.0` and packs into `obsbot.gresource` through `glib-build-tools 0.20`, `application::run` registers the embedded GResource at startup. `window.rs` + `controls_view.rs` load the shells via `gtk::Builder::from_resource`. User-confirmed identical behaviour to T-013c (camera row, drill-down, V4L2 controls page, back nav, Ctrl+Q). All four cargo gates green; meson compile produces a fresh 511 KB stripped PIE (new BuildID — GResource bytes linked in). The pipeline is now the foundation for every T-100+ static widget tree (sliders, PTZ pad, About dialog).
next_step: T-100 — first writable V4L2 controls. Per ROADMAP v0.2 the deliverable is brightness / contrast / saturation / hue (User-class controls per PROTOCOL §2.1). Approach: extend `obsbot_core::controls` with a `write_control(path, id, value)` helper backed by `v4l 0.14`'s control-write ioctls, and in `obsbot-gui` swap the read-only AdwActionRow for an AdwSpinRow / AdwSwitchRow / AdwComboRow per ControlKind. Probably worth introducing a small `slider.blp` template to ship a consistent slider layout reusable across T-101+ widgets.
blockers: none.
working_tree:
  pre_commit_modified: [docs/PROGRESS.md, docs/STATE.md]
  pre_commit_untracked: []
  pre_commit_deleted: []
pending_user_actions:
  - T-013 (later, v0.2 area): log out / log back in to pick up
    the new `video` group membership (needed before T-100 can
    write controls via v4l on `/dev/videoN`).
  - T-010 (next time you log in): observe whether GNOME Shell
    now paints our webcam icon when you launch the app.
  - T-017 (Arch stakeholder, whenever): build/install/remove
    the PKGBUILD on Arch.
updated_at: 2026-05-13T21:30:00Z  # session-end, T-099 closed, T-100 next
