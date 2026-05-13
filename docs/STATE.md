# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: T-003
active_task_state: in_progress_partial
last_completed_task: T-004
last_commit: build: create cargo workspace (T-004)  # 921fb57
last_step: T-003 lsusb half DONE — captured `3564:fef9` Tiny 2 Lite descriptor (XU bUnitID=2, GUID 9a1e7291-…); ADR-0014 expands primary target to Tiny 2 family (regular + Lite); SPEC.md, ROADMAP.md, README.md, PROTOCOL.md §1/§3, PLAN.md T-003 updated. v4l2-ctl half pending — needs user to run `usermod -aG video alvaro` (persistent fix) plus the four `sudo v4l2-ctl` captures in /tmp.
next_step: user runs the v4l2-ctl block; Claude reads /tmp/obsbot-v4l2-*.txt, fills PROTOCOL.md §2, closes T-003 with a follow-up commit, then proposes T-005.
blockers: T-003 v4l2-ctl half blocked on user-side `sudo usermod -aG video alvaro` + four `sudo v4l2-ctl` redirects (PROTOCOL.md §2 has the exact block).
working_tree:
  clean_expected_after_two_commits: true
  pre_commit_modified: [docs/DECISIONS.md, docs/SPEC.md, docs/ROADMAP.md, README.md, docs/PROTOCOL.md, docs/PLAN.md, docs/STATE.md, docs/PROGRESS.md]
resume_protocol: |
  Same or new session, from /home/alvaro/Documentos/proyectos/obsbot-control:
    1. `claude` (Claude Code auto-loads CLAUDE.md and reads this STATE.md).
    2. Say "continúa" — Claude resumes T-003 by reading the v4l2-ctl
       outputs from /tmp/ if present, else asks the user to run the block
       in PROTOCOL.md §2.
pending_user_actions:
  - T-003: run `sudo usermod -aG video alvaro` (then log out / log back
    in) so Claude can read /dev/videoN without sudo from here on.
  - T-003: run the four `sudo v4l2-ctl` redirects from PROTOCOL.md §2
    once, so /tmp/obsbot-v4l2-*.txt exist for Claude to parse into the
    V4L2 controls table.
updated_at: 2026-05-13T10:22:12Z
