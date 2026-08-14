# ARCHITECTURE — Technical Design

> **Purpose**: Document **how** the project is built. Changes require a
> `DECISIONS.md` entry. This document describes the code as shipped in
> v0.4.2; aspirational designs that were never implemented were removed
> in the pre-publication reconciliation ([[ADR-0030]]).

---

## 1. Stack

| Layer            | Technology                                      | Reason                                                                   |
|------------------|-------------------------------------------------|--------------------------------------------------------------------------|
| Language         | Rust (edition 2021, MSRV 1.83)                  | Memory safety, modern tooling, growing GNOME ecosystem.                  |
| GUI toolkit      | GTK 4 (≥ 4.14)                                  | Native to GNOME, required by Circle.                                     |
| Design system    | libadwaita (≥ 1.6)                              | GNOME HIG components.                                                    |
| UI definition    | Blueprint (`window.blp`, `ptz-pad.blp`)         | Static shells; dynamic per-control trees are code-built ([[ADR-0017]]).  |
| Rust bindings    | `gtk4-rs` 0.9, `libadwaita-rs` 0.7, `gtk-rs-core` 0.20 | Official, maintained.                                              |
| Build system     | Meson + Cargo (custom target)                   | Convention for GNOME apps.                                               |
| Video pipeline   | GStreamer 1.x + `gstreamer-rs` 0.23             | Standard on Linux; integrates with V4L2.                                 |
| Video sink       | `gtk4paintablesink` (from `gst-plugins-rs`)     | Native GTK 4 widget. Built in-tree for Flatpak; `gstreamer1.0-gtk4` on native. |
| V4L2 access      | `v4l` crate 0.14 (pure Rust)                    | No `libv4l` dependency, simpler Flatpak.                                 |
| UVC XU ioctls    | `nix` crate 0.29 (`ioctl` feature)              | For `UVCIOC_CTRL_QUERY`.                                                 |
| Errors           | `thiserror` 2                                   | Typed errors in `obsbot-core`; the GUI surfaces toasts.                  |
| Logging          | `tracing` 0.1 (library spans only)              | No subscriber is installed; GTK apps log to stderr/journal.              |
| Settings         | GSettings (`gio::Settings`) via schema XML      | GNOME standard; respects user policy.                                    |
| i18n             | `gettext-rs` 0.7 (system gettext)               | GNOME standard. Translations land in v0.6 ([[ADR-0029]]).                |
| Packaging        | Flatpak (target: Flathub)                       | GNOME Circle convention.                                                 |
| CI               | GitHub Actions                                  | fmt + clippy + test + Flatpak build (T-015).                             |

Not used (removed from the workspace manifest in the v0.4.2 cleanup):
`nusb`, `anyhow`, `tracing-subscriber`, `async-channel`. Raw USB
control requests were a fallback design that proved unnecessary — the
XU surface works entirely through `UVCIOC_CTRL_QUERY`.

## 2. Workspace layout

```
obsbot-control/
├── Cargo.toml              # workspace root (shared metadata + dep pins)
├── meson.build             # top-level Meson orchestrator
├── meson_options.txt       # -Dlive-preview=true|false
├── CLAUDE.md               # AI agent instructions
├── README.md  CREDITS.md  LICENSE  .gitignore
│
├── crates/
│   ├── obsbot-core/        # device abstraction, no GUI deps
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── camera.rs       # CameraInfo, CameraModel, types
│   │       ├── controls.rs     # V4L2 standard control read/write
│   │       ├── enumerate.rs    # sysfs/V4L2 camera discovery
│   │       ├── error.rs        # thiserror crate error
│   │       └── xu/             # vendor UVC Extension Unit surface
│   │           ├── transport.rs    # UVCIOC_CTRL_QUERY wrapper (only unsafe)
│   │           ├── enums.rs        # AiMode, FovMode, SleepState, ...
│   │           ├── status.rs       # 60-byte GET_CUR decode
│   │           ├── command02.rs    # 36-byte selector-0x02 frame builder
│   │           ├── v4l2_ptz.rs     # symbolic V4L2 PTZ CIDs
│   │           ├── errors.rs
│   │           └── commands/       # one file per opcode/frame
│   │               ├── ai_mode.rs  hdr.rs  fov.rs  face_ae.rs
│   │               ├── exposure_mode_type.rs  sleep.rs
│   │               ├── tracking_speed.rs  preset.rs
│   │       └── tests/            # #[ignore]d hardware tests
│   │
│   ├── obsbot-cli/         # `obsbot-cli list` — headless camera info
│   │   └── src/main.rs
│   │
│   └── obsbot-gui/         # GTK4 + libadwaita binary `obsbot-control`
│       ├── build.rs        # blueprint-compiler → .ui → GResource
│       ├── resources/
│       │   ├── window.blp      # app window shell
│       │   ├── ptz-pad.blp     # 3×3 pad + zoom slider
│       │   ├── style.css       # preview card styling
│       │   └── obsbot.gresource.xml
│       └── src/
│           ├── main.rs  application.rs  window.rs
│           ├── controls_view.rs    # tab container + generic control rows
│           ├── ai_effects_view.rs  # "AI and effects" + "Image enhancements"
│           ├── extras_view.rs      # preset recall + misc groups
│           ├── wb_group.rs  exposure_group.rs  ptz_pad.rs
│           ├── preview.rs          # GStreamer pipeline (feature-gated)
│           ├── settings.rs         # GSettings read/write + toast surface
│           └── i18n.rs             # gettext shim
│
├── data/                   # desktop/metainfo/gschema templates + icons
├── po/                     # gettext: LINGUAS, POTFILES.in, meson.build
├── build-aux/              # Flatpak manifest, PKGBUILD, packaging shims
├── .github/workflows/      # CI (T-015)
└── docs/                   # project documentation (see CLAUDE.md §0)
```

The App ID `io.github.domatix.obsbot-control` (resolved in ADR-0012)
appears as the basename of every namespaced asset above. The local folder
and Cargo crate names stay as `obsbot-control`, `obsbot-core`,
`obsbot-cli`, `obsbot-gui`; the user-visible product name is "Obsbot Cam
Control".

## 3. Backend architecture

### 3.1 Synchronous calls on the GLib main loop

Camera I/O is **synchronous and runs on the main loop**. V4L2 control
ioctls and XU queries complete in microseconds, so no worker thread or
async channel was ever needed — the originally sketched
`CameraCommand`/`CameraEvent` worker design was dropped as overkill.
The only threaded component is GStreamer, which runs its own streaming
threads internally; only the `gtk4paintablesink` paintable crosses into
GTK.

Hot-plug is a 2 s GLib timeout that re-runs enumeration and diffs the
result (`window.rs`); removals pop the controls page and post a toast.

### 3.2 V4L2 standard controls (`obsbot-core::controls`)

`read_controls(path)` enumerates every User/Camera-class control the
kernel advertises, returning `ControlDescriptor`s (id, kind, range,
default, current value, `is_active` from `V4L2_CTRL_FLAG_INACTIVE`).
`write_control(path, id, value)` dispatches `Device::set_control`.
The GUI renders rows generically from these descriptors and greys out
inactive rows; after a Boolean/Menu write it re-reads the descriptors
so kernel-driven INACTIVE flips (e.g. auto-WB gating temperature)
propagate to sensitivity immediately.

### 3.3 Vendor XU surface (`obsbot-core::xu`)

All OBSBOT-specific features go through one ioctl:

- `xu::transport` wraps `UVCIOC_CTRL_QUERY` on `/dev/videoN` (unit
  `0x02`), issuing `UVC_GET_LEN` before every GET/SET. This is the only
  `unsafe` code in the workspace, scoped to a single module.
- Selector `0x06` carries opcode-multiplexed 3–4 byte payloads (HDR,
  face AE, FOV, AI mode) plus a 60-byte `GET_CUR` status struct.
- Selector `0x02` carries 36-byte structured frames (sleep/wake,
  tracking speed, exposure mode, preset recall) built by
  `xu::command02`.

Every byte sequence is ported from EUPL-1.2 free-software projects —
see `CREDITS.md` and `docs/PROTOCOL.md` §3.2. There is no raw-USB
(libusb/nusb) path.

### 3.4 Persistence

One GSettings schema, `io.github.domatix.obsbot-control`:

- `control-values` (`a{si}`): per-camera control values, keyed
  `"<serial>\x1f<control-name>"`. Cameras without a USB serial (the
  Tiny 2 Lite reports `iSerial=0`) are simply not persisted.
- `color-scheme`: Follow system / Light / Dark.
- `preview-default-on`: whether the preview starts automatically.

Writes replay on camera enumeration (best-effort; failures are logged,
not surfaced).

### 3.5 Preview pipeline

Feature-gated behind the `live-preview` Cargo feature (on in the meson
option and the Flatpak; off for bare `cargo build`):

```
v4l2src device=/dev/videoN ! videoconvert ! capsfilter !
  videobalance ! videoflip ! videoconvert ! gtk4paintablesink
```

`videobalance` backs the (currently hidden) grayscale/saturation
control; `videoflip` backs the (currently hidden) mirror toggle. The
snapshot button grabs the paintable's current frame and writes a PNG
to `~/Pictures`. Camera power is managed around the pipeline: the XU
sleep frame is sent ~3 s after streaming stops (firmware ignores it
earlier — see `STATE.md` firmware notes and [[ADR-0025]]).

## 4. Hardware access and permissions

- V4L2 controls and XU ioctls both operate on `/dev/videoN`; the user
  must be in the `video` group (default on most distros). No udev rule
  is shipped — none is needed.
- Flatpak manifest grants `--device=all` (for `/dev/videoN`),
  `--socket=wayland` + `--socket=fallback-x11`, `--share=ipc`, and
  `--filesystem=xdg-pictures` (snapshot saves).

## 5. Build pipeline

Meson is the top-level orchestrator; it invokes `cargo build -p
obsbot-gui` via a custom target (profile follows `buildtype`). The CLI
is cargo-only and not installed by meson.

Phases:
1. Meson configures paths and substitutes `@APP_ID@` / `@VERSION@` in
   the desktop file and AppStream metainfo.
2. `build.rs` runs `blueprint-compiler` on the `.blp` sources and packs
   the GResource; `data/meson.build` compiles the GSettings schema.
3. `meson test` runs `appstreamcli validate` and
   `desktop-file-validate`.
4. Meson installs the binary, data files, icons, and schema.

`flatpak-builder` calls meson inside the sandbox; the manifest also
builds `blueprint-compiler` and `gst-plugin-gtk4` from upstream tags.

## 6. Testing strategy

- **Unit tests in `obsbot-core`**: enumeration (mock sysfs via
  `tempfile`), control mapping, XU payload/status fixtures from the
  EUPL-1.2 upstream test suites.
- **Integration tests in `crates/obsbot-core/tests/`**: 7
  hardware-dependent tests, `#[ignore]`d by default; run with
  `cargo test --workspace -- --ignored` on a machine with the camera.
- **GUI**: a few logic tests; GTK widgets are not auto-tested
  (industry standard). Manual validation with the real camera gates
  every release.
- **CI**: cargo fmt, clippy, test, plus a Flatpak build on every push
  (`.github/workflows/`).

## 7. Known constraints

- **Firmware matters.** All validation is against the Tiny 2 Lite,
  `bcdDevice` 5.10. If OBSBOT changes the XU layout in a firmware
  update, we detect and document it in `PROTOCOL.md`.
- **No serial on the Lite** means per-camera persistence silently does
  nothing on that unit (documented in §3.4 and `PROTOCOL.md` §5).
- **`pan_speed`/`tilt_speed` are dead on firmware 5.10** (writes
  accepted, no motion); PTZ moves via discrete `pan_absolute` /
  `tilt_absolute` steps. See `PROTOCOL.md` Q9.
- **Regular Tiny 2 unvalidated on this project** — it is a declared
  first-class target ([[ADR-0014]]) but needs community testers.
