# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none  # v0.4-first-slice + T-101c on main; v0.2 parked re-pass pending user-driven validation
active_task_state: —
active_branch: main
last_completed_task: T-203  # Flatpak GStreamer plugin module (squashed in v0.4-first-slice bundle 5ea6d8b)
last_milestone: v0.3.1  # tag cut 2026-05-19 on e3ad521
last_commit_on_main: 5ea6d8b  # feat: v0.4 first slice — snapshot, grayscale filter, Flatpak module (T-201/T-202/T-203)
last_step: Session 2026-05-19 closed a marathon: T-200 (Live Preview pipeline) and T-101b (PTZ hold + keyboard arrows) squashed and tagged as v0.3.1 (e3ad521), then T-101c (speed slider + Shift accelerator + hot-plug timer cleanup) squashed, then T-201/T-202/T-203 bundle (snapshot button + grayscale filter + Flatpak gst-plugin-gtk4 module) squashed. Cargo gates green on every step default + with `obsbot-gui/live-preview`. Last hardware-side validations the user signed off on: T-200 preview, T-101b hold + arrows ("todo verde"). Hardware-side validations still pending (queued under `parked_validation_for_user`): T-201 snapshot, T-202 grayscale, T-101c speed slider + Shift, plus the original v0.2 parked list.
next_step: User-driven validation pass against the connected Tiny 2 Lite. Walk the `parked_validation_for_user` list below; for each green gate, ping back and I update PLAN to DONE + STATE. After the parked list closes, decide whether to cut v0.4.0 (needs Flatpak smoke-test from T-203) or v0.3.2 (native-only rollup of the four follow-ups).
blockers: none  # only awaiting user validation
working_tree:
  status: uncommitted STATE update only
parked_validation_for_user:
  newly_landed_2026_05_19:
    - T-101c (PTZ speed slider + Shift accelerator + hot-plug cleanup) — set `gsettings set io.github.domatix.ObsbotCamControl ptz-speed-fast 80` then hold an arrow → fast pan (~32°/s); reset with `gsettings set ... ptz-speed-fast 20` → slow pan (~8°/s). With Shift+Arrow on keyboard → 3× the resolved step. Hot-plug: unplug USB mid-hold → app stops writing without spam.
    - T-201 (Snapshot) — header-bar `camera-photo-symbolic` button. Start preview, push snapshot → toast says "Snapshot saved: /home/alvaro/Pictures/obsbot-camera-….png"; open the file → matches what was on screen.
    - T-202 (Grayscale) — header-bar `view-reveal-symbolic` ToggleButton. Toggle on while preview active → instant color → grayscale; toggle off → color returns.
    - T-203 (Flatpak gst module) — `flatpak-builder` against build-aux/io.github.domatix.ObsbotCamControl.json builds the new gst-plugin-gtk4 step + the app module; installed app's preview pipeline finds gtk4paintablesink and renders frames. Hardware-side gate.
  v0_2_originally_parked:
    - T-105 (persistence): change brightness=75 in our app, close the app, reopen, drill into the camera → brightness slider at 75 (was BLOCKED by T-105fix which is now on main).
    - T-108 (permission toast): cable plugged, `sudo chmod 000 /dev/videoN`, drag any slider → toast "Failed to set <name>: Permission denied". Restore with `sudo chmod 660 /dev/videoN`.
    - T-110 (USB unplug): unplug the cable while on detail page → page pops to the list + toast "Camera disconnected: <name>". Re-plug → reappears in ~2 s.
    - T-101 PTZ matrix (original): all 8 PTZ buttons (cardinal + diagonals) + reset + zoom slider + manual focus toggle. T-101b hold/keyboard already validated; this is the rest of the matrix.
    - T-102 (power_line_frequency): Disabled / 50 / 60 Hz selector writes.
    - T-103 (white balance): WB Auto on/off → temp slider greys out / wakes up cleanly.
    - T-104 (exposure): Auto → Manual → drag Exposure Time → image changes; back to Auto → grey.
    - T-010 (icon): GNOME Shell icon visible after `meson install`.
    - T-017 (Arch PKGBUILD): build/install/remove on an Arch host (community stakeholder, no rush).
follow_ups_queued:
  - cut-v0.3.2-or-v0.4.0: decision pending the parked-list close. Either way needs an AppStream releases block update.
  - sepia-invert-filters: T-202 follow-up. Needs `gst-plugins-bad` (frei0r-filter-sepia0r etc.) — out of scope for this slice.
  - file-chooser-snapshot: T-201 follow-up. Preferences-dialog override of the auto-Pictures destination + JPEG output.
  - flatpak-builder-smoke: actually run flatpak-builder against the updated manifest to validate T-203 end-to-end.
  - preferences-dialog: surface `ptz-speed-fast` and `preview-default-on` in a real AdwPreferencesDialog (v0.6 polish).
  - verify-q9-tiny2-regular: pan_speed/tilt_speed inert on Tiny 2 Lite firmware 5.10; re-test on Tiny 2 (regular) when a unit is available.
  - branch-hygiene: feat/T-300-xu-tracking + feat/T-101a (SUPERSEDED) + feat/T-200-preview + feat/T-101b-ptz-hold-keyboard + feat/T-101c-ptz-tuning + feat/T-201-202-203-v04 all retained locally; delete only on explicit user ask.
  - T-400 (post-v1.0): Add OBSBOT Meet (original) to the model matrix.
known_issues:
  - Q9 (PROTOCOL.md): pan_speed/tilt_speed accept writes but produce no motion on Tiny 2 Lite firmware 5.10. T-101b/T-101c use pan_absolute polling workaround.
updated_at: 2026-05-19T03:30:00Z
