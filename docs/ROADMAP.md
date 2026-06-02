# ROADMAP

> **Purpose**: Milestone-level view of the project. Detailed tasks for the
> current milestone live in `PLAN.md`.

---

## v0.1 — Scaffolding & Detection

**Goal**: Project compiles and runs an empty window; the app detects any
unit of the OBSBOT **Tiny 2 family** (regular `3564:fef8` or Lite
`3564:fef9` — see [[ADR-0014]]) and shows its V4L2 capabilities.

**Includes**:
- Cargo workspace + Meson build system + Flatpak manifest.
- Empty `AdwApplicationWindow` opens.
- USB device enumeration finds Tiny 2 family units by VID/PID.
- A diagnostics view shows the device's V4L2 controls (read-only).
- CI runs fmt, clippy, test, Flatpak build, plus a `.deb` and an Arch
  `pkg.tar.zst` test-artifact build (per [[ADR-0015]]).

**Does NOT include**: any camera control, preview, or settings persistence.

**Definition of done**: see `CLAUDE.md` §7. Tag: `v0.1.0`.

---

## v0.2 — V4L2 Standard Controls

**Goal**: Useful subset of controls actually working from the GUI.

**Includes**:
- PTZ pad with absolute and continuous pan/tilt/zoom (uses V4L2 standard CIDs).
- Zoom slider.
- Image controls: brightness, contrast, saturation, hue, sharpness, gamma.
- White balance: auto + manual temperature.
- Exposure: auto + manual.
- Anti-flicker selector.
- Settings persistence per camera (keyed by serial).
- Symbolic icon and full Adwaita styling.
- About dialog with credits and license info.

**Does NOT include**: vendor XU features (HDR, FOV mode, auto-framing).

**Note**: At this point the app is already useful as a polished UVC controller
for any compliant camera, with OBSBOT as primary target.

---

## v0.3 — Vendor XU & AI tracking

**Goal**: OBSBOT-specific controls working via reverse-engineered
protocol — collapses the previously planned v0.4 (Vendor XU) and v0.5
(Auto-Framing) into a single milestone, and is promoted ahead of Live
Preview per [[ADR-0020]] after the FOSS investigation removed the
Windows-VM prerequisite.

**Prerequisites**: none beyond the user's Tiny 2 Lite plugged in. The
XU command surface is already reverse-engineered in FOSS sources
([[CREDITS.md]]; [[PROTOCOL.md §3.2]]); we port it under EUPL-1.2 →
GPL-3 attribution.

**Includes**:
- AI auto-framing — 10 modes: No tracking, Normal, Upper body,
  Close-up, Headless, Lower body, Desk mode, Whiteboard, Hand,
  Group. (Old v0.5 "auto-framing" + face/group/upper-body modes.)
- HDR toggle.
- Field of View: Wide (86°) / Normal (78°) / Narrow (65°).
- Face Auto-Exposure on/off (paired with auto-exposure mode).
- Manual / Auto exposure mode toggle.
- Sleep / Wake camera power state.
- Tracking speed: Standard / Sport.
- Preset position recall: 3 slots (program them via OBSBOT Center
  beforehand — preset *save* is out of scope, see [[PROTOCOL.md §3.2
  Q7]]).
- Debug "Dump XU status" page exposing the 55 still-undecoded bytes
  of the 60-byte GET_CUR struct for future community contributions.

**Does NOT include**:
- Live preview (moved to v0.4).
- LED brightness, microphone pickup pattern, Gesture control, Voice
  command — these stay in SPEC scope but await either a USB capture
  session against the proprietary app or a community contribution.
- Preset save (deferred; see Q7).

---

## v0.4 — Live Preview

**Goal**: In-app live preview with filters. Was previously slated for
v0.3; demoted to v0.4 per [[ADR-0020]] to honour the user's pivot to
AI tracking.

**Includes**:
- GStreamer pipeline with `gtk4paintablesink` (T-200).
- Toggle preview on/off.
- Snapshot to file (PNG/JPEG).
- Detect "camera busy" via PipeWire or `/proc/self/fd` checks; show clear
  error and suggested actions.
- Aspect-ratio-aware resizing of the preview pane.

**Does NOT include**: GLSL filters, recording, virtual camera.

---

## v0.6 — Polish for GNOME Circle

**Goal**: Quality bar suitable for submission to GNOME Circle.

**Includes**:
- AppStream metainfo with full release notes, screenshots (HiDPI), categories.
- Spanish translation complete; framework ready for additional languages.
- Keyboard shortcuts (`Ctrl+,` settings, `?` help, etc.) following GNOME
  conventions.
- Help / Onboarding for first-time use.
- Bug fixes from internal testing.
- Performance audit: startup time, idle CPU, memory.
- Accessibility audit: keyboard navigation, screen-reader labels, contrast.
- Flathub submission of the package.

---

## v1.0 — GNOME Circle submission

**Goal**: Apply to GNOME Circle.

**Includes**:
- Submission of an issue in `gitlab.gnome.org/Teams/Circle`.
- Address all review feedback in iterative point releases.
- Tag `v1.0.0` only when accepted.

---

## Beyond v1.0 (ideas, unprioritized)

- Support for more OBSBOT models (**Meet**, Meet 2, Meet SE, original
  Tiny, Tail Air, …) as community reports identify quirks. The Tiny 2
  Lite is **already** a first-class target — see [[ADR-0014]]. The
  original **Meet** is tracked explicitly as `T-400` in `PLAN.md` per
  user request 2026-05-15.
- GLSL filters in preview (reuse approach from `aaronsb/obsbot-camera-control`).
- Custom preset hotkeys (global, via GNOME extensions or keyboard shortcuts).
- Migration to GTK 5 if/when released.
- Translation infrastructure on Weblate / Damned Lies.

---

## Version mapping

| Tag        | Milestone                  | Status   |
|------------|----------------------------|----------|
| `v0.1.0`   | Scaffolding & Detection    | shipped (2026-05-13) |
| `v0.2.0`   | V4L2 Standard Controls     | active   |
| `v0.3.0`   | Vendor XU & AI tracking    | shipped (2026-05-15) |
| `v0.4.0`   | Live Preview               | shipped (2026-06-02) |
| `v0.6.0`   | Polish                     | planned  |
| `v1.0.0`   | GNOME Circle               | goal     |
