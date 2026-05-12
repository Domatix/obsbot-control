# ROADMAP

> **Purpose**: Milestone-level view of the project. Detailed tasks for the
> current milestone live in `PLAN.md`.

---

## v0.1 — Scaffolding & Detection

**Goal**: Project compiles and runs an empty window; the app detects an
OBSBOT Tiny 2 and shows its V4L2 capabilities.

**Includes**:
- Cargo workspace + Meson build system + Flatpak manifest.
- Empty `AdwApplicationWindow` opens.
- USB device enumeration finds Tiny 2 by VID/PID.
- A diagnostics view shows the device's V4L2 controls (read-only).
- CI runs fmt, clippy, test, Flatpak build.

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

## v0.3 — Live Preview

**Goal**: In-app live preview with filters.

**Includes**:
- GStreamer pipeline with `gtk4paintablesink`.
- Toggle preview on/off.
- Snapshot to file (PNG/JPEG).
- Detect "camera busy" via PipeWire or `/proc/self/fd` checks; show clear
  error and suggested actions.
- Aspect-ratio-aware resizing of the preview pane.

**Does NOT include**: GLSL filters, recording, virtual camera.

---

## v0.4 — Vendor Features (XU)

**Goal**: OBSBOT-specific controls working via reverse-engineered protocol.

**Prerequisites**: User performs USB capture against OBSBOT Center on a
Windows VM with their Tiny 2. Capture procedure documented in `PROTOCOL.md`.

**Includes**:
- HDR toggle.
- FOV: wide / medium / narrow.
- Face AE and face AF.
- LED brightness.
- Microphone pickup pattern (if applicable to Tiny 2).
- Gesture control toggle.
- Voice command toggle (if applicable).

**Does NOT include**: auto-framing (deferred to v0.5).

---

## v0.5 — Auto-Framing & AI Features

**Goal**: The hardest vendor features that may require multi-step protocols.

**Includes**:
- Auto-framing: off / single / group / upper-body modes.
- Face zone selection (if exposed).

**Open risk**: this milestone may not be achievable without OBSBOT's
cooperation or extensive reverse engineering. If blocked after reasonable
effort, ship v1.0 without it and document why.

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

- Support for more OBSBOT models (Meet 2, Tiny 2 Lite, Meet SE) as community
  reports identify quirks.
- GLSL filters in preview (reuse approach from `aaronsb/obsbot-camera-control`).
- Custom preset hotkeys (global, via GNOME extensions or keyboard shortcuts).
- Migration to GTK 5 if/when released.
- Translation infrastructure on Weblate / Damned Lies.

---

## Version mapping

| Tag        | Milestone                | Status   |
|------------|--------------------------|----------|
| `v0.1.0`   | Scaffolding & Detection  | active   |
| `v0.2.0`   | V4L2 Standard Controls   | planned  |
| `v0.3.0`   | Live Preview             | planned  |
| `v0.4.0`   | Vendor XU                | planned  |
| `v0.5.0`   | Auto-Framing             | risky    |
| `v0.6.0`   | Polish                   | planned  |
| `v1.0.0`   | GNOME Circle             | goal     |
