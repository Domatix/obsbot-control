# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none
active_task_state: idle
last_completed_task: T-003
last_commit: docs: capture Tiny 2 Lite USB descriptor (T-003)  # 19d8026 (lsusb half); v4l2-ctl half pending in this turn's upcoming commit
last_step: T-003 DONE — PROTOCOL.md §1 (USB descriptor), §2 (V4L2 standard controls, 24 entries + 3 quirks), §3 (XU 2 with media-graph cross-check) all complete for the Tiny 2 Lite. Regular Tiny 2 entries flagged speculative pending community capture. Quirks Q1/Q2/Q3 raised for v0.2 GUI design.
next_step: propose T-005 (stub `obsbot-core` crate) — it depends on T-004 (DONE) only, T-003 is now also DONE so T-011 (USB enumeration) is no longer blocked downstream.
blockers: none for the next batch of tasks (T-005, T-006, T-007, T-008, T-009, T-010 all clear). T-013 will need the `video` group fix to actually open /dev/videoN from the GUI (already prescribed via `sudo usermod -aG video alvaro`).
working_tree:
  pre_commit_modified: [docs/PROTOCOL.md, docs/PLAN.md, docs/STATE.md, docs/PROGRESS.md]
resume_protocol: |
  Same or new session, from /home/alvaro/Documentos/proyectos/obsbot-control:
    1. `claude` (auto-loads CLAUDE.md, reads STATE.md).
    2. Say "continúa" / "siguiente tarea" — Claude proposes T-005.
pending_user_actions:
  - T-013 (later, v0.1): if you have not yet logged out / logged back in
    after `sudo usermod -aG video alvaro`, do so before T-013 so the
    GUI can open /dev/videoN without sudo.
updated_at: 2026-05-13T10:45:37Z
