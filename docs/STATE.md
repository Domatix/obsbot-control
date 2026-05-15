# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none  # v0.3.0 shipped; awaiting user-direction for the next milestone
active_task_state: —
active_branch: main  # post-merge; feat/T-300-xu-tracking kept for reference, do not delete without explicit ask
last_completed_task: T-303  # v0.3.0 milestone closure
last_milestone: v0.3.0  # tag cut 2026-05-15; commit SHA recorded in PROGRESS milestone entry
last_commit_on_main: feat: v0.3.0 Vendor XU & AI tracking (T-300 / T-301 / T-302 / T-303)  # squash-merge of feat/T-300-xu-tracking
last_step: T-303 closed. User validated every GUI gate green (AI and effects 4 rows + 10 AI modes + HDR + FOV; Power state and presets 5 rows; PTZ pad post-cache-drift hot-fix; dump dialog with clipboard). Hardware suite ran in-session (7/7 pass). Three hot-fix commits during validation: 3c04e57 (zero-pad SET_CUR), d3fce26 (descope XU Exposure mode + Face metering), f38a7ff (refresh PTZ from kernel). Quirk resolutions per PROTOCOL.md §3.2: Q4 accepted as-is (Hand setter m=3), Q5 retired by descope, Q8 documented (FOV Narrow no-op on Tiny 2 Lite firmware 5.10). Branch `feat/T-300-xu-tracking` squash-merged into main; annotated `v0.3.0` tag cut. PUSH HELD per private-repo policy.
next_step: User picks the next direction. Three candidates queued in PLAN.md: (a) **T-105fix** — GSettings schema vs runtime alignment, v0.3.1 hot-fix train, ~10 lines; (b) **T-101a** — PTZ smooth movement via pan_speed/tilt_speed press-and-hold, milestone TBD (v0.3.1 or v0.4); (c) **T-200** — start the v0.4 Live Preview pipeline. Plus the parked v0.2 validation list (T-108 / T-110 / T-101 / T-102 / T-103 / T-104 / T-105). Plus the post-v1.0 **T-400** Add OBSBOT Meet to the model matrix.
blockers: none
working_tree:
  # Clean after the squash-merge commit + tag. The feature branch
  # still exists locally for reference / blame archaeology.
  status: clean
follow_ups_queued_in_plan:
  - T-105fix (v0.3.1): GSettings schema/runtime key mismatch — pre-existing bug, descoped from v0.3.0 per T-303 decision.
  - T-101a (v0.3.1 or v0.4): PTZ smooth movement via pan_speed/tilt_speed press-and-hold. User chose press-and-hold approach in the T-303 AskUser prompt.
  - T-400 (post-v1.0): Add OBSBOT Meet (original, no suffix) as a supported model. Filed per user request 2026-05-15.
v0_2_pending_validation:
  # Still parked from 2026-05-14. Mostly bug-flushed during v0.3 work
  # (PTZ pad got a hot-fix, settings persistence will be reborn via
  # T-105fix). Worth a re-pass before v0.4 cuts.
  parked:
    - T-108: keep cable plugged in, `sudo chmod 000 /dev/videoN`, drag a slider → toast "Failed to set <name>: Permission denied". Restore `sudo chmod 660 /dev/videoN`.
    - T-110: unplug USB while on detail page → page pops to list + toast "Camera disconnected: <name>". Re-plug → reappears in ~2 s.
    - T-101: 8 PTZ buttons + center-reset + zoom slider + manual-focus toggle. PARTIALLY validated during T-303 (hot-fix f38a7ff resolved the cache-drift bug); the rest of the matrix is still the original T-101 list.
    - T-102: power_line_frequency (Disabled / 50 / 60); WB Auto on/off → temp slider grey/wake (generic INACTIVE).
    - T-103: WB controls live inside the dedicated "White balance" group.
    - T-104: Exposure Auto → Manual → drag Exposure Time → image change; back to Auto → grey.
    - T-105: change brightness=75, restart app, drill in → slider at 75. BLOCKED on T-105fix (schema mismatch).
    - T-010 (still): GNOME Shell icon visible.
    - T-017 (Arch stakeholder, whenever): build/install/remove the PKGBUILD on Arch.
known_issues:
  - **T-105 schema / runtime mismatch (pre-existing, surfaced 2026-05-14, descoped from v0.3.0)**: queued as T-105fix for v0.3.1. Not a regression; persistence already silently broken on main before v0.3 work began.
updated_at: 2026-05-15T12:00:00Z  # v0.3.0 milestone closure
