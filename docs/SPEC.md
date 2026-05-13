# SPEC — Functional Specification

> **Purpose**: Define **what** this project is and is not. Stable document.
> Changes require an entry in `DECISIONS.md`.

---

## 1. One-line description

A native GNOME application that lets users configure and control OBSBOT
cameras connected via USB, using only free and open-source code.

## 2. Problem statement

OBSBOT cameras (Tiny 2, Meet 2, etc.) work as standard UVC webcams on Linux,
but their advanced features (PTZ presets, auto-framing, HDR, gesture control,
microphone settings) are inaccessible without the OBSBOT Center application,
which only exists for Windows and macOS. A Qt6 Linux port
([aaronsb/obsbot-camera-control](https://github.com/aaronsb/obsbot-camera-control))
exists but relies on OBSBOT's proprietary closed-source SDK (`libdev.so`),
which prevents adoption by GNOME Circle and inclusion in major distributions.

This project fills the gap with a **fully free** alternative, integrated with
the GNOME desktop, distributable through Flathub.

## 3. Target users

- Linux desktop users who own an **OBSBOT Tiny 2 family** unit — either
  the regular **Tiny 2** (`3564:fef8`) or the **Tiny 2 Lite**
  (`3564:fef9`). Both are first-class targets per [[ADR-0014]].
- Streamers, video-conferencers, and educators who want camera presets
  controllable from a native GUI.
- Linux distribution maintainers who want to ship OBSBOT camera support
  without bundling proprietary blobs.

## 4. In scope

### 4.1 Core functionality
- Auto-detect connected OBSBOT cameras (by USB VID/PID).
- Display per-device info (model, firmware version if readable, serial).
- Pan / tilt / zoom controls (continuous and absolute).
- Zoom level slider.
- Image controls: brightness, contrast, saturation, hue, sharpness, gamma.
- White balance: auto + manual temperature.
- Exposure: auto + manual.
- Anti-flicker: 50 Hz, 60 Hz, disabled.
- Field of view: wide / medium / narrow (via vendor XU).
- HDR toggle (via vendor XU).
- Auto-framing modes (via vendor XU): off, single, group.
- Face auto-exposure and face auto-focus (via vendor XU).
- Gesture control toggle (via vendor XU).
- Save and restore camera state across reboots (persistent settings).
- Presets: user-defined named snapshots of camera state.

### 4.2 Preview
- Live video preview inside the app, using GStreamer + `gtk4paintablesink`.
- Optional grayscale/sepia/invert post-process filters.
- Detect when the camera is busy (used by another app) and show clear feedback.
- Snapshot to file.

### 4.3 Background operation
- Run in the background via the XDG Background Portal, surfacing in GNOME
  Shell's "Background Apps" menu.
- Restore window from background launcher.

### 4.4 Integration
- AppStream metainfo for software-center listing.
- Desktop file with proper categories.
- GSettings schema for all persistent settings.
- Symbolic and regular icons.
- Full localization via gettext (English source, Spanish at minimum).
- Keyboard shortcuts following GNOME conventions.
- Adwaita styling, light/dark mode automatic via system.

### 4.5 Packaging
- **Flatpak** as primary distribution (target: Flathub). This is the
  channel the user docs recommend and the only one with a long-term
  maintenance commitment from the project.
- **Test artifacts** ([[ADR-0015]]): from v0.1 onwards the release
  tooling also produces a non-policy `.deb` (via `cargo-deb`) and a
  non-policy Arch package (via in-tree `PKGBUILD`) for stakeholder
  sideload testing. These are convenience builds, not Debian-policy
  or AUR-grade packages; we do not host an apt/pacman repository.
- Policy-grade Debian / RPM / AUR packaging is a non-goal for v1.0;
  community packagers welcome and can use our `cargo-deb` / `PKGBUILD`
  as starting points.

## 5. Out of scope

- Cameras outside the **Tiny 2 family** (regular + Lite, see §3 and
  [[ADR-0014]]) are **best-effort, no commitment**. Code must not
  actively reject other OBSBOT models (Meet 2, Meet SE, original Tiny,
  Tail Air, …), but their features are not guaranteed and the GUI will
  only surface controls a given unit actually advertises.
- Non-OBSBOT cameras: ignored. The app appears empty if none detected.
- Recording: out of scope. Users record with OBS, Cheese, etc.
- Streaming: out of scope. Users stream with OBS, Zoom, etc.
- Virtual camera (`v4l2loopback` output): out of scope for v1.0; revisit
  later. The PipeWire route is the long-term answer and depends on platform
  maturity.
- Windows or macOS support: out of scope, ever.
- Reverse engineering of network/Bluetooth protocols: USB only.
- Firmware updates: out of scope. Users update with the official tools on
  Windows/macOS.

## 6. Non-functional requirements

### 6.1 Free software
- All first-party code under an OSI-approved free license (decision in `DECISIONS.md`).
- **No** dependency on `libdev.so` or any OBSBOT SDK.
- All runtime dependencies must be free software and packageable in Flatpak.
- No telemetry, no analytics, no network calls.

### 6.2 GNOME Circle eligibility
The project must satisfy the GNOME Circle criteria
(https://gitlab.gnome.org/Teams/Releng/AppOrganization/-/blob/main/AppCriteria.md):
- Uses GNOME platform (GTK4 + libadwaita).
- Follows GNOME HIG.
- Available as Flatpak on Flathub.
- Quality first-class.
- OSI-approved license, no CLA.

### 6.3 Performance
- App startup ≤ 2 s on the user's reference machine (Intel i9-10900, 32 GB
  RAM, integrated graphics).
- Preview latency ≤ 200 ms.
- Idle CPU ≤ 1% (no preview), ≤ 10% (with 1080p preview).

### 6.4 Reliability
- Surviving camera disconnect/reconnect during runtime.
- Recovering from a busy V4L2 device with a clear message.
- Persisted state survives crash.

### 6.5 Internationalization
- All user-facing strings externalized.
- English as source language.
- Spanish translation maintained by the project; others by community.

### 6.6 Accessibility
- Keyboard navigability of all functions.
- Respect system font scale and high-contrast modes (via GTK defaults).

## 7. Constraints

- Hardware available for development: one OBSBOT **Tiny 2 Lite**
  (`3564:fef9`, bcdDevice 5.10), on Debian 13 trixie, GNOME 48 on
  Mutter/X11, by the user. The regular **Tiny 2** (`3564:fef8`) is a
  declared primary target ([[ADR-0014]]) but is not physically present
  on this development machine — regular-Tiny-2-specific behavior must
  be validated by community testers or by cross-referencing the
  linuxtv-commits kernel patches cited in [[PROTOCOL.md §6]].
- USB-level reverse engineering of OBSBOT Center's protocol is required for
  vendor-specific features. The user is the only one who can capture this
  traffic.
- The OBSBOT SDK is explicitly forbidden, even as an optional feature.

## 8. Success criteria

The project is successful when:
1. A first-time GNOME user can install via Flathub, plug in a Tiny 2, and
   adjust PTZ, brightness, and HDR within 30 seconds.
2. The application is accepted into GNOME Circle.
3. At least one Linux distribution ships it (Debian, Fedora, or Arch).
4. The protocol documentation in `PROTOCOL.md` is detailed enough that someone
   else could reimplement the backend.

## 9. Anti-goals (things we explicitly do NOT want)

- Becoming a generic UVC camera tool: scope is OBSBOT.
- Becoming a video editor or recorder.
- Becoming a streaming app.
- Adding system-tray-icon support outside what XDG Background Portal provides:
  GNOME deprecated tray icons; emulating them would be anti-HIG.
- Shipping a `v4l2loopback` modprobe configuration: that's a system-level
  concern users opt into.

## 10. References

- Aaron Brown's Qt6 reference: https://github.com/aaronsb/obsbot-camera-control
- OBSBOT Tiny 2 product page: https://www.obsbot.com/store/products/tiny-2
- OBSBOT Tiny 2 Lite product page: https://www.obsbot.com/store/products/tiny-2-lite
- GNOME HIG: https://developer.gnome.org/hig/
- GNOME Circle criteria: https://gitlab.gnome.org/Teams/Releng/AppOrganization/-/blob/main/AppCriteria.md
- Linux UVC driver docs: https://kernel.org/doc/html/latest/userspace-api/media/drivers/uvcvideo.html
- Existing reverse-engineering work: https://github.com/taxfromdk/obsbot_tiny_reversing
- Related Meet 4K work in Rust: https://github.com/samliddicott/meet4k
