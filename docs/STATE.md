# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: T-208  # auto-sleep the camera when the preview stops (firmware power-down); code DONE, hardware validation pending
active_task_state: IN_PROGRESS
active_branch: main
last_completed_task: T-207  # close V4L2 fd on navigate-away + window-close — confirmed working via /proc fd monitor 2026-06-10 (fd released on back AND on close; no lingering process). LED-stays-on turned out to be firmware, not a leaked fd → led to T-208.
last_milestone: v0.4.0  # Live Preview milestone cut 2026-06-02 (Flatpak-validated); v0.3.2 same day (native rollup)
last_commit_on_main: 51f251a  # fix(gui): stop preview pipeline on navigate-away and window close (T-207); T-208 commit follows this STATE update
last_step: 2026-06-10 — T-208 (ADR-0024). Colleagues' "camera stays on when unused" was the OBSBOT firmware not powering down on stream-stop (T-207 already proved the fd closes). User confirmed the manual Sleep switch (T-302) powers their Tiny 2 Lite down, then asked to auto-sleep on every preview stop. Implemented in PreviewPipeline::stop (the T-207 chokepoint): record device_path in start, send set_sleep(Sleep) on a fresh fd after NULL, skip if /proc shows another process holds the device. All cargo gates green. Also diagnosed (separate, NOT fixed): the preview pipeline has a format bug — camera offers only MJPG/YUYV, no MJPEG decoder + dmabuf passthrough → grayscale dead, GStreamer-Video-CRITICAL spam, preview stalls (T-202 root cause + missing decoder).
next_step: USER — hardware-validate T-208: preview on (LED on) → toggle off → LED off + lens cover; repeat for back + window close; safeguard check (camera open in another app → stays awake). Then mark T-207 + T-208 DONE. Pending decision: tackle the preview pipeline format bug (proposed T-209) — grayscale/CRITICAL/stall.
blockers: none on main. T-208 pending eyes-on-hardware. T-017b Arch validation still transferred to incoming dev (ADR-0023).
working_tree:
  status: after the T-208 commit, only the untracked build artifact obsbot-cam-control-0.4.0-1-x86_64.pkg.tar.zst remains (leftover .pkg from a PKGBUILD test run — user said leave it untracked, do not add to git).
v0_4_0_gate:
  - T-203 build gate: DONE + verified headless 2026-06-02 (flatpak-builder builds all 3 modules, installs io.github.domatix.ObsbotCamControl 0.3.2, sandbox gst-inspect-1.0 finds gtk4paintablesink in /app/lib/gstreamer-1.0/libgstgtk4.so).
  - T-203 render check: PENDING USER — launch the installed Flatpak, toggle preview, confirm camera frames render on screen. Not machine-verifiable. Last thing before v0.4.0.
  - runtime EOL: CLOSED 2026-06-04 (T-205) — manifest now targets org.gnome.Platform//50 (verified headless). Flathub-unblocked on the runtime front.
still_open_non_hardware:
  - T-017 Arch PKGBUILD build/install/remove on an Arch host (community stakeholder, no rush).
  - T-202 minor: grayscale toggled while preview off is lost on start (re-apply on start, or disable filter buttons while off).
follow_ups_queued:
  - preview-visibility-pause (T-207 follow-up, ADR-0024): pause/resume the preview on window minimise / focus-loss so an explicitly-on preview does not keep the camera lit while the window is hidden. Natural home for the unimplemented SPEC §4.3 XDG Background Portal work.
  - sepia-invert-filters, file-chooser-snapshot, preferences-dialog (preview-default-on), verify-q9-tiny2-regular — unchanged from prior STATE. (ptz-speed-fast dropped in T-101d.)
  - branch-hygiene: feat/* branches retained locally; delete only on explicit user ask.
  - T-400 (post-v1.0): add OBSBOT Meet (original) to the model matrix.
known_issues:
  - Q9 (PROTOCOL.md): pan_speed/tilt_speed accept writes but no motion on Tiny 2 Lite firmware 5.10. PTZ moves via discrete pan_absolute/tilt_absolute single steps (T-101d).
updated_at: 2026-06-05T07:00:00Z
