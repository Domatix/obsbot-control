# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none
active_task_state: not_started
last_completed_task: T-002
last_commit: chore: set namespace and license (T-002)
last_step: T-002 closed — GPL-3.0-or-later + io.github.domatix.ObsbotCamControl wired through docs; LICENSE installed
next_step: propose T-003 (capture and document Tiny 2 USB descriptor) — requires user to run lsusb/v4l2-ctl
blockers: none
session_notes: |
  Identity is now fixed: App ID io.github.domatix.ObsbotCamControl,
  display name "Obsbot Cam Control", repo under github.com/Domatix,
  copyright "© 2026 Domatix and contributors", license GPL-3.0-or-later.
  T-003 needs hardware (Tiny 2 plugged in). T-004 (cargo workspace) is the
  next code-only task if T-003 is deferred for hardware reasons.
updated_at: 2026-05-12T10:55:00Z
