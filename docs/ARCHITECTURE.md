# ARCHITECTURE — Technical Design

> **Purpose**: Document **how** the project is built. Changes require a
> `DECISIONS.md` entry.

---

## 1. Stack

| Layer            | Technology                                      | Reason                                                                   |
|------------------|-------------------------------------------------|--------------------------------------------------------------------------|
| Language         | Rust (edition 2021, MSRV 1.83)                  | Memory safety, modern tooling, growing GNOME ecosystem.                  |
| GUI toolkit      | GTK 4 (≥ 4.14)                                  | Native to GNOME, required by Circle.                                     |
| Design system    | libadwaita (≥ 1.6)                              | GNOME HIG components.                                                    |
| UI definition    | Blueprint                                       | Modern declarative format, compiled to GtkBuilder XML.                   |
| Rust bindings    | `gtk4-rs`, `libadwaita-rs`, `gtk-rs-core`       | Official, maintained.                                                    |
| Build system     | Meson + Cargo (via `meson-rust`)                | Convention for GNOME apps.                                               |
| Video pipeline   | GStreamer 1.22+ + `gstreamer-rs`                | Standard on Linux; integrates with V4L2 and PipeWire.                    |
| Video sink       | `gtk4paintablesink` (from `gst-plugins-rs`)     | Native GTK 4 widget, hardware acceleration friendly.                     |
| V4L2 access      | `v4l` crate (pure Rust)                         | No `libv4l` dependency, simpler Flatpak.                                 |
| USB raw access   | `nusb` crate (pure Rust)                        | Used for direct USB control requests if V4L2/UVC isn't sufficient.       |
| UVC XU ioctls    | `nix` crate                                     | For `UVCIOC_CTRL_QUERY` and friends.                                     |
| Async runtime    | `glib::MainContext` + `async-channel`           | Native to GLib; no Tokio (would conflict with GTK main loop).            |
| Errors           | `thiserror` (libraries) + `anyhow` (binaries)   | Idiomatic Rust.                                                          |
| Logging          | `tracing` + `tracing-subscriber`                | Structured logging, sub-spans for async tasks.                           |
| Settings         | GSettings (`gio::Settings`) via schema XML      | GNOME standard; respects user policy.                                    |
| i18n             | `gettext-rs` + `i18n-helpers` from gtk-rs       | GNOME standard.                                                          |
| Background      | XDG Background Portal                           | No tray icons. HIG-compliant.                                            |
| Packaging        | Flatpak (target: Flathub)                       | GNOME Circle convention.                                                 |
| CI               | GitHub Actions (when repo public)               | Free, popular.                                                           |

## 2. Workspace layout

```
obsbot-control/
├── Cargo.toml              # workspace root
├── meson.build             # top-level Meson
├── CLAUDE.md               # AI agent instructions
├── README.md
├── LICENSE
├── .gitignore
│
├── crates/
│   ├── obsbot-core/        # device abstraction, no GUI deps
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── camera.rs       # trait Camera
│   │       ├── v4l2_backend.rs # impl using V4L2 standard controls
│   │       ├── uvc_xu.rs       # impl using UVC Extension Units
│   │       ├── usb_backend.rs  # impl using raw USB if needed
│   │       ├── models/
│   │       │   ├── mod.rs
│   │       │   └── tiny2.rs    # per-model XU selectors, ranges, quirks
│   │       └── error.rs
│   │
│   ├── obsbot-cli/         # CLI binary, uses core directly
│   │   ├── Cargo.toml
│   │   └── src/main.rs
│   │
│   └── obsbot-gui/         # GTK4 + libadwaita binary
│       ├── Cargo.toml
│       ├── build.rs        # compiles Blueprint, gresources
│       └── src/
│           ├── main.rs
│           ├── application.rs
│           ├── window.rs
│           ├── config.rs   # GSettings wrapper
│           └── widgets/
│               ├── mod.rs
│               ├── ptz_pad.rs
│               ├── preview.rs
│               ├── presets_view.rs
│               └── image_controls.rs
│
├── data/                   # GUI resources
│   ├── ui/                 # Blueprint files (.blp) → compiled to .ui
│   ├── icons/
│   │   ├── scalable/apps/
│   │   └── symbolic/apps/
│   ├── io.github.domatix.ObsbotCamControl.desktop.in
│   ├── io.github.domatix.ObsbotCamControl.metainfo.xml.in
│   ├── io.github.domatix.ObsbotCamControl.gschema.xml
│   └── resources.gresource.xml
│
├── po/                     # translations
│   ├── POTFILES
│   ├── LINGUAS
│   └── es.po               # Spanish, maintained by project
│
├── build-aux/
│   └── io.github.domatix.ObsbotCamControl.json  # Flatpak manifest
│
└── docs/                   # project documentation (see CLAUDE.md §0)
```

The App ID `io.github.domatix.ObsbotCamControl` (resolved in ADR-0012)
appears as the basename of every namespaced asset above. The local folder
and Cargo crate names stay as `obsbot-control`, `obsbot-core`,
`obsbot-cli`, `obsbot-gui`; the user-visible product name is "Obsbot Cam
Control".

## 3. Backend architecture

### 3.1 Trait `Camera`

`obsbot-core` exposes a single trait that the GUI and CLI consume. The trait
hides whether a feature is implemented via V4L2 standard control, V4L2 mapped
XU, raw UVC ioctl, or raw USB request.

```rust
pub trait Camera: Send + Sync {
    fn info(&self) -> CameraInfo;
    fn capabilities(&self) -> Capabilities;

    // Standard controls (V4L2)
    fn brightness(&self) -> Result<i32>;
    fn set_brightness(&self, value: i32) -> Result<()>;
    // ... contrast, saturation, etc.

    // PTZ
    fn pan(&self) -> Result<i32>;
    fn set_pan(&self, value: i32) -> Result<()>;
    // ... tilt, zoom, speed.

    // Vendor (may return Unsupported on non-OBSBOT)
    fn hdr_enabled(&self) -> Result<bool>;
    fn set_hdr_enabled(&self, on: bool) -> Result<()>;
    fn fov(&self) -> Result<Fov>;
    fn set_fov(&self, fov: Fov) -> Result<()>;
    // ... auto-framing, face-AE, gestures, etc.
}
```

`Capabilities` is a struct of `bool`s describing which features the connected
device actually supports. The GUI uses it to hide unavailable controls.

### 3.2 Models registry

`crates/obsbot-core/src/models/` contains one file per supported model
(initially just `tiny2.rs`). Each file defines:

- USB VID/PID for detection.
- XU GUID(s), unit IDs, selectors for each vendor feature.
- Per-control ranges, defaults, units.
- Quirks (e.g. minimum-speed sign convention from the kernel patch we found).

Detection: scan `/sys/class/video4linux/*` for matching VID/PID, return
the matching model descriptor.

### 3.3 Async model

GTK runs on the GLib main loop. Long-running operations (USB I/O,
GStreamer pipeline setup, file I/O) must not block.

- Camera operations: spawn on a dedicated worker thread that owns the device
  handle. GUI sends `CameraCommand` enum messages via `async_channel::Sender`;
  worker sends `CameraEvent` back the same way. GUI listens via
  `glib::MainContext::spawn_local`.
- GStreamer: lives on its own thread internally; only the pipeline handle and
  the paintable cross the thread boundary.

### 3.4 Persistence

- All user-facing settings live in GSettings (XML schema in `data/`).
- Schema namespace: `io.github.domatix.ObsbotCamControl.preferences` and
  `io.github.domatix.ObsbotCamControl.state`.
- Per-camera state (last brightness, last preset) keyed by serial number, so
  multiple cameras don't collide.
- Presets stored as a JSON-serialized list inside a single GSettings string key
  (simple, atomic, schema-evolvable).

### 3.5 Preview pipeline

Default pipeline (logical):
```
v4l2src device=/dev/videoN ! image/jpeg,width=W,height=H ! jpegdec !
  videoconvert ! gtk4paintablesink
```

For Y/UYVY native: same minus `jpegdec`. The pipeline is built
programmatically via `gstreamer-rs` (not from a string) so we can swap
elements when applying filters or switching resolutions.

## 4. Hardware access

### 4.1 V4L2 standard controls (no special permissions)
Used for: PTZ absolute/continuous/speed, zoom, brightness/contrast/saturation,
white balance, exposure, focus, gain, anti-flicker.

Access path: `/dev/videoN` via the `v4l` crate. User must be in the `video`
group (default on most distros).

### 4.2 UVC Extension Units
Used for: vendor features not exposed as V4L2 standard CIDs (HDR, FOV mode,
auto-framing variant, face AE/AF, gesture toggle, voice command, LED, mic).

Access path: `UVCIOC_CTRL_QUERY` ioctl on `/dev/videoN`. The user-space
program defines a `uvc_xu_control_query` struct and calls the ioctl.

Discovery procedure:
1. Decompile USB capture from OBSBOT Center on a Windows VM.
2. Identify GUID + unit_id + selector + size + GET/SET sequence.
3. Document in `PROTOCOL.md` with hex dumps and Wireshark filter expressions.
4. Implement in `models/tiny2.rs`.
5. Cross-verify on the user's device under their explicit consent.

### 4.3 Raw USB control requests
Fallback if XU ioctls are insufficient (some commands may use a different USB
endpoint or vendor-specific bRequest). Used via `nusb`.

### 4.4 Permissions
- Flatpak manifest will grant `--device=all` (specifically for `/dev/videoN`).
- USB raw access through Flatpak requires `--device=all` and may require
  `--filesystem=/sys` for hot-plug. Document caveats.
- udev rule: provided in-tree but installation is documented for distro
  packagers; Flatpak builds skip it.

## 5. Build pipeline

Meson is the top-level orchestrator; it invokes `cargo build` via a custom
target. This is the GNOME convention for Rust apps.

Phases:
1. Meson configures paths, prepares `meson-info`.
2. Meson generates substituted files (.desktop, .metainfo.xml, .service).
3. Meson runs `cargo build --release --workspace`.
4. Meson runs `appstreamcli validate`, `desktop-file-validate`.
5. Meson installs binaries, data, icons, schemas to the prefix.

`flatpak-builder` calls `meson` inside the sandbox.

## 6. Testing strategy

- **Unit tests in `obsbot-core`**: extensive. Mock backend implements `Camera`.
- **Integration tests in `obsbot-core/tests/`**: hardware-dependent,
  `#[ignore]`d by default. Run with `cargo test -- --ignored` on the user's
  machine.
- **GUI**: no automated tests. Manual smoke tests checklist in
  `docs/QA_CHECKLIST.md` (to be created during v0.3).
- **CI**: runs cargo fmt, clippy, test (non-ignored), and a Flatpak build.

## 7. Known unknowns / risks

- **XU selectors for Tiny 2 are not documented anywhere we can cite.** We
  must discover them. Risk mitigation: V4L2 standard controls already cover
  ~60-70% of useful functionality; that's a usable v0.2 even if XU work fails.
- **Auto-framing AI features may require multi-step command sequences** rather
  than simple register writes. May land in v0.5 or later, or never.
- **Tiny 2's USB descriptor** (`lsusb -v` output) must be captured early to
  identify XU unit IDs and GUIDs. T-003 in `PLAN.md`.
- **Firmware versions matter.** Document the firmware version this app is
  tested against. If OBSBOT changes XU layout in firmware updates, we'll
  detect and warn.

## 8. Decisions deferred

- Final app namespace: depends on where the repo will live publicly.
- Whether to support presets as separate GSettings rows vs JSON in one row.
- Whether to ship a `gst-plugin-rs` subset or rely on system packages (Flathub
  has both).

These will be added to `DECISIONS.md` when resolved.
