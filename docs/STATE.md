# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none
active_task_state: idle
last_completed_task: T-100
last_milestone: v0.1.0  # tag 5e005fd
last_commit: docs: session-end checkpoint after T-099 (v0.2 at 12%)  # f550064 (pending T-100 commit at end of this turn)
last_step: T-100 DONE. obsbot_core::controls now has `id: u32` on `ControlDescriptor`, `default` on `ControlKind::Integer/Boolean`, a `ControlValue` enum, and `write_control(&Path, u32, ControlValue) -> Result<()>` backed by `v4l::Device::set_control`. obsbot-gui's controls_view replaces the read-only AdwActionRow for User-class controls: Integer → AdwActionRow with `gtk::Scale` (drag bar with a tick mark at the default) + `gtk::SpinButton` (manual entry, shares the same Adjustment so it stays in sync) + flat `edit-undo-symbolic` reset button (tooltip "Reset to default (N)"); Boolean → `AdwSwitchRow` with the default in the subtitle. Camera-class and menu controls remain read-only (T-101 PTZ pad, T-103 WB, T-104 exposure cover those). All four gates green (fmt/clippy/test/hardware-3). User confirmed live brightness/contrast/saturation/hue + WB Temperature (after toggling WB Auto off — documented V4L2 interlock per PROTOCOL §2.3) on the Tiny 2 Lite.
next_step: T-101 — PTZ pad widget. Camera-class controls (`V4L2_CID_PAN_ABSOLUTE`, `_TILT_ABSOLUTE`, `_ZOOM_ABSOLUTE` plus their continuous variants if the driver advertises them) are already enumerated by `read_controls` and currently rendered read-only. T-101 introduces a dedicated PTZ pad — likely a Blueprint template (`ptz-pad.blp`) since the layout is genuinely static (3×3 directional grid + zoom slider on the side + speed adjustment) — and a `pan_tilt(pan_delta, tilt_delta, duration)` plus `set_zoom(level)` helper in obsbot_core. Also worth keeping the generic User-class scale/spin/reset pattern available for any future User Integer that wasn't named explicitly (Gamma, Sharpness, Backlight Compensation — already work today via the generic path).
blockers: none.
working_tree:
  pre_commit_modified: [docs/PLAN.md, docs/STATE.md, docs/PROGRESS.md, crates/obsbot-core/src/{controls.rs,lib.rs}, crates/obsbot-core/tests/hardware.rs, crates/obsbot-gui/src/controls_view.rs]
  pre_commit_untracked: []
  pre_commit_deleted: []
pending_user_actions:
  - T-010 (next time you log in): observe whether GNOME Shell now
    paints our webcam icon when you launch the app.
  - T-017 (Arch stakeholder, whenever): build/install/remove the
    PKGBUILD on Arch.
updated_at: 2026-05-13T22:55:00Z  # T-100 DONE, gates green, ready for commit
