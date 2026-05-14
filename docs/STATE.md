# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none
active_task_state: pivot_pending  # decision deferred to next session
last_completed_task: T-111
last_milestone: v0.1.0  # tag 5e005fd
last_commit: fix(gui): refresh row sensitivity after gate writes (T-111)  # 9fde97d
last_step: Mid-session pivot. v0.2.0 validation pass STOPPED AT 100% PER USER REQUEST after T-111 and T-106 were validated live (the rest of the pending validation — T-108 / T-110 / T-101..T-105 / T-010 / T-017 — is now parked; the user was explicit: "aparca ahora mismo lo que haya para validar"). User asked for camera AI tracking ("tracking de la cámara… cara, upper body, mano, etc.") with two hard constraints: must be 100% free software (no `libdev.so`, per SPEC §6.1 / §7) and must match the feature richness of `aaronsb/obsbot-camera-control` (the Qt6 reference, which depends on the proprietary SDK). Investigation done THIS session (see PROGRESS entry [2026-05-14T11:33Z]) discovered five FOSS Tiny 2 projects that did NOT appear in PROTOCOL.md §3 before — most importantly `cgevans/tiny2` (Rust, EUPL-1.2 → GPL-3 compatible, 51 stars, `src/lib.rs` contains the **fully-decoded XU table for the Tiny 2 family** — AI modes ×10, FOV ×3, HDR, Face AE, exposure modes; all against `unit=0x2 selector=0x6`) and `OpenFoxes/Tiny4Linux` (active, AUR-packaged fork of cgevans adding sleep/wake, tracking speed, presets). **This invalidates PROTOCOL.md §6's "Wireshark + Windows VM" prerequisite** — the per-selector decode that PROTOCOL.md §3.1 currently lists as `TBD × 19` is already cracked open-source and license-compatible. Decision left UNRESOLVED at session end: how to arrange the pivot (4 options were on the table, user chose to close the session instead of picking; see next_step). Working tree DIRTY (T-111 + T-106 validation notes in STATE.md + PROGRESS.md, no commit yet) — Claude did NOT touch PLAN.md, PROTOCOL.md, ROADMAP.md, or any code this session per the "investigación primero, sin tocar plan" working-mode the user selected mid-conversation.
next_step: Next session opens with a 4-way pivot decision the user deferred: (A) start T-300 immediately on a `feat/T-300-xu-tracking` branch, leaving the dirty docs in `main` untouched until later; (B) commit the T-111 + T-106 validation docs to `main` first (single `docs:` commit) to clean the working tree, then branch to T-300; (C) before deciding to start, expand the investigation by reading Tiny4Linux's `src/libs/` end-to-end to extract the tracking-speed / preset / sleep-wake commands and produce a refined T-300 / T-301 / T-302 acceptance-criteria draft; (D) full pause for user-side thinking — let the user read `cgevans/tiny2` / `OpenFoxes/Tiny4Linux` themselves before committing. T-300 scope (provisional): a new module `obsbot-core::xu` exposing `UVCIOC_CTRL_QUERY` wrappers + `AIMode` / `FOVMode` / `ExposureMode` enums + `decode_status(bytes)`, all ported from `cgevans/tiny2/src/lib.rs` under EUPL-1.2 attribution in a new `CREDITS.md` (EUPL-1.2 is OSI-approved and explicitly compatible with GPL-3 per its appendix). The v0.2.0 tag is NOT cancelled — it's postponed until either after T-300 lands or after the user resumes the validation pass; either way, the docs commits accumulating in `main` (`9fde97d` plus the pending one for T-111+T-106) keep the milestone within reach.
blockers: user-side decision on pivot path (A/B/C/D).
working_tree:
  pre_commit_modified:
    - docs/STATE.md       # this file (current edit)
    - docs/PROGRESS.md    # validation notes T-111 + T-106 + pivot investigation
  pre_commit_untracked: []
  pre_commit_deleted: []
pending_user_actions:
  # First autonomous run (T-101..T-105), second autonomous run
  # (T-106..T-110), and the validation-driven T-111 fix accumulate
  # for the second validation pass. Items the user already
  # validated green in the first pass are NOT re-listed.
  # T-111 VALIDATED 2026-05-14T06:31Z — D / E / F.12 closed.
  # T-106 VALIDATED 2026-05-14T06:53Z — license lives behind
  #   "Legal" sub-page per AdwAboutDialog HIG default; user
  #   confirmed all sections (version, license, repo, issues,
  #   acknowledgements) render correctly.
  # ⏸ PARKED 2026-05-14T11:33Z — user pivoted to AI-tracking
  #   investigation. Remaining items below are NOT being worked
  #   on; they re-enter scope only when the user resumes the
  #   v0.2.0 validation pass (either before or after T-300).
  - T-108: yanking the USB cable is the WRONG test here —
    that triggers T-110 first and pops the page out. Use the
    permission-revocation workaround instead: keep the cable
    plugged in, then in a terminal run
    `sudo chmod 000 /dev/video0`; back in the GUI (still on
    the controls page; the device stays in the enumeration
    since `enumerate_cameras` reads sysfs, not /dev) drag a
    slider → toast reads "Failed to set <control name>:
    Permission denied". Restore with `sudo chmod 660
    /dev/video0`.
  - T-110: while on a camera detail page, unplug the USB
    cable; confirm the page pops back to the cameras list
    automatically AND a toast appears reading "Camera
    disconnected: <product name>". Re-plug; confirm the
    camera reappears in the cameras list within ~2 s (poll
    interval).
  - T-101: drag the 8 PTZ buttons + center-reset, confirm pan/tilt;
    drag the vertical zoom slider, confirm the frame zooms; toggle
    "Auto-focus" off and drag "Manual focus" — focus distance changes.
  - T-102: find "Power Line Frequency" in the User Controls section,
    change between Disabled / 50 Hz / 60 Hz (visible effect is subtle
    — usually just no error is enough). Toggle "White Balance,
    Automatic" off, then on; confirm the "White Balance Temperature"
    row greys out / wakes up automatically (generic INACTIVE handler).
  - T-103: confirm the four WB controls now live inside a dedicated
    "White balance" group with a description text, near the top of
    the page, instead of scattered in the User Controls section.
  - T-104: in the "Exposure" group, change "Exposure, Auto" to
    "Manual"; drag "Exposure Time, Absolute" — preview gets darker
    or brighter. Switch back to "Auto"; confirm the exposure time
    slider greys out.
  - T-105: pick any non-default value (e.g. brightness = 75), close
    the GUI, re-launch, drill into the camera — the slider should
    come up at 75 and the camera image should reflect it. Cleanup
    afterwards (optional): `gsettings reset-recursively io.github.
    domatix.ObsbotCamControl`.
  - T-010 (still): observe whether GNOME Shell paints our webcam icon
    when you launch the app.
  - T-017 (Arch stakeholder, whenever): build/install/remove the
    PKGBUILD on Arch.
updated_at: 2026-05-14T11:33:43Z  # mid-session pivot to AI tracking; validation parked; T-300 decision deferred
