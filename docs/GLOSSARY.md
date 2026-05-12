# GLOSSARY

> **Purpose**: Disambiguate terms used in this project's docs and code.
> Read this when a sentence in another doc has unfamiliar terms.

---

## Project-specific terms

- **App ID / namespace**: the reverse-DNS identifier that names the app
  across the system. For this project:
  **`io.github.domatix.ObsbotCamControl`** (resolved in ADR-0012). Used in
  `.desktop`, `.metainfo.xml`, Flatpak manifest, GSettings schema, D-Bus
  name. Must match everywhere.

- **ADR — Architecture Decision Record**: a single entry in `DECISIONS.md`
  documenting one decision: context, decision, consequence. Append-only.

- **Atomic task**: a `PLAN.md` task small enough to complete in one AI
  session (~30 min — 2 h), with explicit acceptance criteria, that produces
  one or more commits when done.

- **GNOME Circle**: an initiative by the GNOME Foundation that recognizes
  high-quality apps and libraries built on the GNOME platform but not part
  of the GNOME core suite. Acceptance criteria include free license, HIG
  compliance, Flatpak distribution. https://circle.gnome.org/

- **HIG — Human Interface Guidelines**: GNOME's UX rulebook. Defines
  navigation patterns, controls, spacing, typography for GNOME apps.
  https://developer.gnome.org/hig/

- **Milestone**: a versioned, release-worthy collection of tasks. Listed in
  `ROADMAP.md`. Becomes a git tag `vX.Y.0` when complete.

- **Sub-step**: a meaningful intermediate point inside a task — a function
  implemented, a test passing, a refactor done. `PROGRESS.md` is updated
  at every sub-step.

---

## USB / video terms

- **UVC — USB Video Class**: a standard USB class for cameras. Compliant
  cameras work without vendor drivers via the kernel's `uvcvideo` driver.
  https://www.usb.org/document-library/video-class-v15-document-set

- **V4L2 — Video4Linux 2**: the Linux kernel API for video capture devices.
  Exposes UVC cameras as `/dev/videoN` with a standard ioctl interface.

- **V4L2 CID — Control ID**: a numeric identifier for a camera control in
  V4L2, like `V4L2_CID_BRIGHTNESS`, `V4L2_CID_PAN_ABSOLUTE`. Standard CIDs
  are defined by the kernel; private CIDs can be vendor-specific.

- **XU — Extension Unit**: a UVC concept allowing vendors to expose custom
  controls beyond the standard ones (HDR, AI features, etc.). Each XU has a
  GUID, a unit ID, and a set of selectors (one per control). Accessed from
  user space via `UVCIOC_CTRL_QUERY` ioctl.

- **Selector**: an integer identifying a specific control within a UVC unit
  (e.g. XU). Documented by the device manufacturer (or reverse-engineered).

- **GUID — Globally Unique Identifier**: a 128-bit value identifying a UVC
  XU type. Same XU on multiple cameras shares the GUID.

- **PTZ — Pan/Tilt/Zoom**: standard camera movement vocabulary. "Pan" is
  horizontal rotation, "Tilt" is vertical, "Zoom" is focal-length change.

- **udev**: the Linux device manager. Controls `/dev/*` permissions via
  rules. We may ship a udev rule for OBSBOT cameras (out of Flatpak; for
  distro packaging only).

- **PipeWire**: modern multimedia framework for Linux, replacing
  PulseAudio and parts of V4L2. Handles camera streams on modern GNOME.

- **`usbmon`**: Linux kernel module that exposes USB traffic for capture.
  Used with Wireshark to reverse-engineer protocols.

---

## Rust / GNOME / build terms

- **Crate**: the unit of compilation in Rust. A crate can be a library
  (`lib.rs`) or a binary (`main.rs`). A workspace groups multiple crates.

- **Workspace**: a top-level `Cargo.toml` listing member crates that share
  a `target/` directory and dependency resolution.

- **MSRV — Minimum Supported Rust Version**: the oldest Rust toolchain on
  which the project must compile. We declare 1.83.

- **`gtk-rs`**: the umbrella for Rust bindings to GTK and friends. Includes
  `gtk4`, `libadwaita`, `gstreamer` (separate crates).

- **libadwaita / Adwaita**: GNOME's design system, providing widgets like
  `AdwApplicationWindow`, `AdwPreferencesPage`, `AdwActionRow`. Inherits
  from GTK 4.

- **Blueprint**: a modern, ergonomic syntax for defining GTK UIs,
  compiled to GtkBuilder XML at build time. Replaces hand-written `.ui`
  files. https://jwestman.pages.gitlab.gnome.org/blueprint-compiler/

- **GSettings**: GNOME's settings API. Schemas in XML, runtime via
  `gio::Settings`. Backed by dconf.

- **GResource**: GNOME's resource bundling mechanism. Embeds UI files,
  icons, CSS into the binary at build time.

- **GLib MainContext**: GLib's main event loop. GTK runs on it. All
  async/await in this project uses GLib-compatible futures, not Tokio.

- **GStreamer**: the multimedia framework for video pipelines. `v4l2src
  ! ... ! gtk4paintablesink` is a typical Linux camera pipeline.

- **gtk4paintablesink**: GStreamer element that outputs to a GTK 4
  `gdk::Paintable`, displayable in a `gtk::Picture`. From `gst-plugins-rs`.

- **Meson**: the GNOME-standard build system. Orchestrates Cargo and
  data-file processing for this project.

- **Flatpak**: the universal Linux packaging format used by Flathub.
  Sandboxed; requires explicit permissions.

- **Flathub**: the main public Flatpak repository.
  https://flathub.org/

- **AppStream metainfo**: an XML file describing the app to software
  centers (GNOME Software, KDE Discover). Contains description,
  screenshots, releases, categories.

- **`.desktop` file**: standard XDG menu entry; tells GNOME Shell how to
  launch the app.

---

## AI workflow terms

(Defined in `docs/AI_WORKFLOW.md` §12, repeated here for cross-reference.)

- **Session**: one run of `claude` from launch to exit.
- **Context**: everything Claude remembers in this session.
- **Token**: the billing/measurement unit for AI input/output.
- **`/clear`**: wipe conversation context, keep files.
- **`/compact`**: summarize conversation to reduce token usage.
- **Sub-agent**: ephemeral helper Claude session.

---

## Symbols you'll see in docs

- **T-XYZ**: task ID (e.g. T-014) referencing `PLAN.md`.
- **ADR-NNNN**: ADR id in `DECISIONS.md`.
- **vX.Y.Z**: semver version tag.
- **`<app-id>`**: legacy placeholder for the reverse-DNS app namespace,
  resolved by ADR-0012 to `io.github.domatix.ObsbotCamControl`. Kept here
  for readers of older revisions of the docs.
