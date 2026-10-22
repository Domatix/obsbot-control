# PROTOCOL — OBSBOT Communication Protocol

> **Purpose**: Document every fact discovered about how OBSBOT cameras
> respond to USB control requests. Grows during T-003, T-011, and the
> v0.4 milestone. Must remain reproducible: every claim cites a source
> (USB capture, kernel patch, open-source project) or is explicitly
> marked as untested speculation.

---

## Status

- **Hardware identifiers**: filled for Tiny 2 Lite from a direct capture
  (T-003, 2026-05-13). Tiny 2 (regular) entry stays speculative until
  a community lsusb capture lands.
- **V4L2 standard controls**: captured 2026-05-13 (`v4l2-ctl --all`
  + `--list-ctrls-menus` on `/dev/video0` and `/dev/video1`); full
  tables in §2 below.
- **Vendor XU**: descriptor metadata captured (Unit ID, GUID, bmControls
  mask). Per-selector semantics: **the majority of the surface is now
  known from free-software sources** (`cgevans/tiny2` and
  `OpenFoxes/Tiny4Linux`, both EUPL-1.2). See §3.2 below for the
  decoded command table and [[DECISIONS.md ADR-0020]] for the FOSS
  pivot. The previously documented "Wireshark + Windows VM" capture is
  no longer a v0.3 prerequisite (it was the old v0.4's prereq before
  the milestone collapse); it remains a *future, optional* avenue for
  probing the still-unmapped opcodes (selector `0x06` opcodes
  `0x02`, `0x05`, `0x06`-`0x15`, `0x17`+) and the status bytes past
  `0x24`.
- **Presets**: read **and** write verified on hardware (2026-10-20).
  Create/update/delete/rename are accepted and reflected in the store;
  occupancy comes from the flat selector `0x0C` and names from the
  entry cursor `0x0D` (base64). The read path was the last functional
  gap for a preset UI, and it is now the source of truth the GUI
  refreshes from.
- **Status block**: field map per `cgevans/tiny2`, three offsets
  re-verified on the Lite (`0x06` HDR, `0x11` FOV, `0x24` speed).

---

## 1. Hardware identifiers

The project commits to the **Tiny 2 family** as a single primary target
per [[ADR-0014]]; both PIDs below are first-class.

### 1.1 OBSBOT Tiny 2 Lite — `3564:fef9` (development hardware)

Captured 2026-05-13 against the user's plugged-in unit on Debian 13
trixie, kernel 6.12+, GNOME 48. Command used (non-root, no special
privileges):

```
lsusb -v -d 3564:fef9
```

`lsusb` emits `Couldn't open device, some information will be missing`
without root — that only suppresses string-descriptor follow-ups
(class-specific descriptors are fully captured anyway). Re-running with
sudo did **not** yield additional data for this device because
`iSerial = 0` (the device exposes no serial-number string descriptor —
[[see §5]] for the implication on per-device settings persistence).

#### Device descriptor

| Field            | Value                                |
|------------------|--------------------------------------|
| `idVendor`       | `0x3564` (Remo Tech Co., Ltd.)       |
| `idProduct`      | `0xfef9`                             |
| `bcdDevice`      | `5.10` (device-release; provisional firmware version, to confirm) |
| `bcdUSB`         | `2.00`, High-Speed (480 Mbps)        |
| `bDeviceClass`   | `239` (Miscellaneous Device)         |
| `bDeviceProtocol`| `1` (Interface Association)          |
| `iManufacturer`  | `Remo Tech Co., Ltd.`                |
| `iProduct`       | `OBSBOT Tiny 2 Lite`                 |
| `iSerial`        | `0` (none — string descriptor absent)|
| Power            | `Self-Powered`                       |
| Interfaces       | 4 (2× Video Control/Streaming pair, 2× Audio Control/Streaming pair) |

#### VideoControl interface (bInterfaceNumber=0)

UVC 1.00, `dwClockFrequency = 48 MHz`, one collection of one streaming
interface.

**INPUT_TERMINAL** `bTerminalID=1`, `wTerminalType=0x0201` (Camera
Sensor). `bmControls = 0x00023e3a` — declares the following standard
UVC selectors:

- Auto-Exposure Mode
- Exposure Time, Absolute *and* Relative
- Focus, Absolute *and* Auto
- Zoom, Absolute *and* Relative
- Pan/Tilt, Absolute *and* Relative
- Roll, Absolute

**PROCESSING_UNIT** `bUnitID=3` (source=1), `wMaxMultiplier=400`,
`bmControls = 0x0000f7df` — declares:

- Brightness
- Contrast
- Hue
- Saturation
- Sharpness
- White Balance Temperature (+ auto)
- White Balance Component (+ auto)
- Backlight Compensation
- Gain
- Power-Line Frequency
- Digital Multiplier (+ Limit)

`bmVideoStandards = 0x1d` → None, PAL 625/50, SECAM 625/50, NTSC 625/50.

**EXTENSION_UNIT** `bUnitID=2` (source=PU3), `bNumControls=19`,
`bControlSize=4`, `bmControls = ff ff 3f 00`. See [[§3.1]] for the
vendor-XU table.

**OUTPUT_TERMINAL** `bTerminalID=7`.

**Status interrupt endpoint**: `EP_INTERRUPT` (subtype 3) on the
VideoControl interface, used by uvcvideo for asynchronous status
events (e.g., button press, focus-lock change — usage to be
confirmed).

#### VideoStreaming interface (bInterfaceNumber=1)

Two payload formats:

| Format             | Frame sizes (3 each, exact dimensions pending §2) |
|--------------------|----------------------------------------------------|
| MJPEG (subtype 6)  | 3 `FRAME_MJPEG` descriptors                        |
| Uncompressed       | 3 `FRAME_UNCOMPRESSED` descriptors (likely YUYV — to confirm via `v4l2-ctl --list-formats-ext`) |

A `COLORFORMAT` descriptor follows each format block.

#### Audio interfaces (bInterfaceNumber=2,3)

USB Audio Class 1 (UAC1). One INPUT_TERMINAL, one OUTPUT_TERMINAL, one
FEATURE_UNIT (`bUnitID=4`, mic gain/mute — to confirm). Sample rates
in the `AS_GENERAL`/`FORMAT_TYPE` block pending §2.

### 1.2 OBSBOT Tiny 2 (regular) — `3564:fef8` (community-supported)

- **USB VID**: `0x3564` (Remo Tech Co., Ltd.)
- **USB PID**: `0xfef8`
- Descriptor data: **not yet captured** by this project. Inferred to
  share the same broad layout as the Lite (camera-sensor INPUT_TERMINAL
  + PROCESSING_UNIT + one vendor EXTENSION_UNIT + UAC1 audio) based on
  the linuxtv-commits kernel patch and OBSBOT Center behavior, but
  **bmControls masks, XU `bNumControls`, and the XU GUID are NOT
  guaranteed identical** until cross-validated.

Source for the PID claim: kernel patch tested on Tiny 2 (linuxtv-commits
2025-12, message-id ≈ `linuxtv-commits/msg48291`), which references
`Bus 008 Device 002: ID 3564:fef8 Remo Tech Co., Ltd. OBSBOT Tiny 2`.

### 1.3 Other OBSBOT models (reference, best-effort)

| Model               | VID      | PID    | Status     |
|---------------------|----------|--------|------------|
| OBSBOT Tiny (orig.) | `0x6e30` | `0xfef0`| best-effort (kernel patch source) |
| OBSBOT Meet 2, Meet SE, Tail Air | `0x3564`? | TBD | TBD; community reports welcome |

---

## 2. V4L2 standard controls

Captured 2026-05-13 from the user's Tiny 2 Lite on Debian trixie,
kernel 6.12.73, driver `uvcvideo`. The device exposes two V4L2 nodes
backed by the same physical camera (USB path `1-7`):

| Node          | Role                          | Default format / size       |
|---------------|-------------------------------|------------------------------|
| `/dev/video0` | Video capture (frames)        | `MJPG` 1920×1080 @ 30 fps    |
| `/dev/video1` | Metadata capture (UVC headers)| `UVCH` payload, 10240 B/buf  |

The metadata node is created automatically by `uvcvideo` to expose UVC
Payload Header Metadata frames; it carries no user-facing controls
(consistent with `/tmp/obsbot-v4l2-ctrls-1.txt` being empty in the
T-003 capture). The capture sub-graph reported by `v4l2-ctl --all` on
`/dev/video0` links to media entity `Extension 2 (Video Pixel
Formatter)` — that entity is the kernel's mount of the vendor XU
documented in §3.1 (`bUnitID = 2`, GUID `9a1e7291-…`), confirming the
XU is reachable through `UVCIOC_CTRL_QUERY`.

Capture procedure (re-runnable; outputs in `/tmp/`):

```
sudo v4l2-ctl -d /dev/video0 --all              > /tmp/obsbot-v4l2-all-0.txt
sudo v4l2-ctl -d /dev/video0 --list-ctrls-menus > /tmp/obsbot-v4l2-ctrls-0.txt
sudo v4l2-ctl -d /dev/video1 --all              > /tmp/obsbot-v4l2-all-1.txt
sudo v4l2-ctl -d /dev/video1 --list-ctrls-menus > /tmp/obsbot-v4l2-ctrls-1.txt
```

Once `sudo usermod -aG video alvaro` takes effect (next login), the
`sudo` prefix becomes unnecessary.

The `Media Driver Info` block reports `Hardware revision: 0x00000510
(1296)` which decimal-matches `bcdDevice = 5.10` in §1.1, corroborating
the firmware-version hypothesis (still not formally confirmed against
OBSBOT Center's own version readout).

### 2.1 User Controls (`V4L2_CID_USER_CLASS_BASE`)

13 controls, all UVC-standard, served by `uvcvideo` from the
PROCESSING_UNIT bmControls advertised in §1.1.

| V4L2 ID      | Name                          | Type | Range / values                  | Default | Notes                                                                  |
|--------------|-------------------------------|------|----------------------------------|---------|------------------------------------------------------------------------|
| `0x00980900` | `brightness`                  | int  | 0..100, step 1                   | 50      |                                                                        |
| `0x00980901` | `contrast`                    | int  | 0..100, step 1                   | 50      |                                                                        |
| `0x00980902` | `saturation`                  | int  | 0..100, step 1                   | 50      |                                                                        |
| `0x00980903` | `hue`                         | int  | 0..100, step 1                   | 50      |                                                                        |
| `0x0098090c` | `white_balance_automatic`     | bool | {0,1}                            | 1       | When 1, freezes red_balance + blue_balance + white_balance_temperature |
| `0x0098090e` | `red_balance`                 | int  | 0..2048, step 1                  | 1024    | `flags=inactive` while auto WB is on                                   |
| `0x0098090f` | `blue_balance`                | int  | 0..2048, step 1                  | 1024    | `flags=inactive` while auto WB is on                                   |
| `0x00980913` | `gain`                        | int  | 1..64, step 1                    | 1       |                                                                        |
| `0x00980918` | `power_line_frequency`        | menu | 0=Disabled, 1=50 Hz, 2=60 Hz     | **3** ⚠ | Kernel reports `default=3` even though the menu max is 2 — see §2.3 (quirk Q1) |
| `0x0098091a` | `white_balance_temperature`   | int  | 2000..10000 K, step 100          | 5000    | `flags=inactive` while auto WB is on                                   |
| `0x0098091b` | `sharpness`                   | int  | 0..100, step 1                   | 50      |                                                                        |
| `0x0098091c` | `backlight_compensation`      | int  | 0..18, step 1                    | 9       |                                                                        |

Note: the PROCESSING_UNIT bmControls (§1.1) advertises a "White Balance
Component" pair (red_balance / blue_balance) **and** a "White Balance
Temperature" control simultaneously. Most consumer UVC devices only
expose one or the other; OBSBOT exposes both, gated by the same
`white_balance_automatic` switch. The GUI groups them under a single
"White balance" section with an auto toggle (shipped in v0.2, T-103).

### 2.2 Camera Controls (`V4L2_CID_CAMERA_CLASS_BASE`)

11 controls covering AE / focus / PTZ, served from the INPUT_TERMINAL
(Camera Sensor) bmControls in §1.1.

| V4L2 ID      | Name                           | Type | Range / values                                   | Default | Notes                                                                                                                       |
|--------------|--------------------------------|------|--------------------------------------------------|---------|-----------------------------------------------------------------------------------------------------------------------------|
| `0x009a0901` | `auto_exposure`                | menu | 0=Auto, 1=Manual, 3=Aperture Priority            | 0       | Menu value 2 is absent — UVC reserves it for "Shutter Priority", not implemented by this firmware                             |
| `0x009a0902` | `exposure_time_absolute`       | int  | 1..2500, step 1 (× 100 μs)                       | 330     | `flags=inactive` while `auto_exposure ∈ {0, 3}` (Auto / Aperture Priority)                                                  |
| `0x009a0908` | `pan_absolute`                 | int  | −468000..468000, step 3600 (UVC: degrees × 3600) | 0       | ±130° in 1° increments                                                                                                       |
| `0x009a0909` | `tilt_absolute`                | int  | −324000..324000, step 3600                       | 0       | ±90° in 1° increments                                                                                                        |
| `0x009a090a` | `focus_absolute`               | int  | 0..100, step 1                                   | 0       | `flags=inactive` while `focus_automatic_continuous = 1`                                                                      |
| `0x009a090c` | `focus_automatic_continuous`   | bool | {0,1}                                            | 1       |                                                                                                                              |
| `0x009a090d` | `zoom_absolute`                | int  | 0..100, step 1                                   | 0       |                                                                                                                              |
| `0x009a090f` | `zoom_continuous`              | int  | 0..100, step 1                                   | 100     | Captured `value=245` exceeds the advertised max — see §2.3 (quirk Q2)                                                          |
| `0x009a0920` | `pan_speed`                    | int  | −1..160, step 1                                  | 20      | Signed: −1 likely "no speed / idle", positive = magnitude. Matches the linuxtv-commits 2025-12 patch semantics (see §6).     |
| `0x009a0921` | `tilt_speed`                   | int  | −1..120, step 1                                  | 20      | Same signed convention as pan_speed.                                                                                          |

### 2.3 Observed quirks

- **Q1 — `power_line_frequency` default outside menu range.** Kernel
  reports `default=3` for a menu that only declares values
  `{0, 1, 2}`. `v4l2-ctl --get-ctrl power_line_frequency` returns
  `value=0 (Disabled)`. Source: most likely the device's
  `wDefault` byte in the `GET_DEF` UVC request returns 3, while
  `bControlSize` only declares three menu items. The GUI should not
  treat the kernel-reported default as canonical for this control;
  use `0 (Disabled)` as the fallback default and let the user pick
  50/60 Hz explicitly.
- **Q2 — `zoom_continuous` value can exceed the advertised range.**
  Snapshot at capture time read `value=245` against `min=0, max=100`.
  Two hypotheses: (a) `zoom_continuous` semantically encodes a
  *speed* in the device's native units, with the V4L2 mapping rolling
  through saturation rather than clamping (the OBSBOT firmware
  accepts the wider range and reports it back); (b) a uvcvideo
  conversion bug for this specific selector. Decision (T-102, shipped):
  `zoom_continuous` is **not surfaced** in the GUI; zoom writes go
  through `zoom_absolute` only.
- **Q3 — gamma is not advertised** despite SPEC.md §4.1 listing it.
  The PROCESSING_UNIT bmControls in §1.1 has no gamma bit. Treat
  gamma as XU-only on this family until disproven; if a vendor XU
  selector carries it, document under §3.

### 2.4 Streaming formats and frame sizes

Default at capture time: `MJPG 1920×1080 @ 30 fps`. The preview
pipeline (shipped in v0.3.1/v0.4.0, T-200) negotiates through
GStreamer's `v4l2src`, so the full {MJPG, YUYV} × {sizes} ×
{framerates} matrix was never locked down manually. A
`v4l2-ctl --list-formats-ext` capture remains useful reference data
for future resolution-switching work. Pending.

---

## 3. UVC Extension Units (XU)

The Linux uvcvideo driver enumerates XUs at probe time; they appear in
`lsusb -v` output and can be queried/written via `UVCIOC_CTRL_QUERY`
ioctl (see [[ARCHITECTURE §3.3]] for the shipped wrapper).

### 3.1 Tiny 2 Lite XU table

Source: `lsusb -v -d 3564:fef9` on the user's machine, 2026-05-13.
Cross-check: `v4l2-ctl -d /dev/video0 --all` reports a media-graph
entity `Extension 2 (Video Pixel Formatter)` linked to the capture
node via pad `0x100000a` (see §2). That entity is the kernel's mount
of this XU, which means `UVCIOC_CTRL_QUERY` against `bUnitID=2` will
function once the per-selector semantics in the next table are
populated.

| Property            | Value                                              |
|---------------------|----------------------------------------------------|
| `bUnitID`           | `2`                                                |
| Source unit         | `3` (PROCESSING_UNIT — the XU chains after PU)     |
| `guidExtensionCode` | `{9a1e7291-6843-4683-6d92-39bc7906ee49}`           |
| `bNumControls`      | `19`                                               |
| `bNrInPins`         | `1`                                                |
| `baSourceID(0)`     | `3`                                                |
| `bControlSize`      | `4` (32-bit `bmControls` mask)                     |
| `bmControls`        | `ff ff 3f 00` (LSB-first: 0xff, 0xff, 0x3f, 0x00)  |
| `iExtension`        | `0` (no string descriptor for the unit)            |

The 32-bit `bmControls` decodes to selectors 1..22 being declared as
gettable/settable (`0xff = 1..8`, `0xff = 9..16`, `0x3f = 17..22`).
`bNumControls=19` is the number of *meaningful* selectors; the extra
3 bits set in the mask are likely reserved or padding. Selector
numbering in UVC XU is 1-based and vendor-defined.

#### Per-selector decode

The descriptor advertises 19 selectors; a mix of framed channels, flat
record blocks and cursors is actually live. Status as of 2026-10-20
(hardware reads on the Lite plus static RE of the vendor library):

| Selector | Role | Evidence |
|---|---|---|
| `0x02` | framed command channel: 12-byte header plus an optional payload block | hardware + RE |
| `0x03`-`0x05` | further framed sub-channels | xrefs in the vendor library; unused by this app |
| `0x06` | tagged command channel **and** the 60-byte status block | hardware + RE |
| `0x07` | legacy gimbal-preset pose records (6 x 10 bytes) | hardware |
| `0x08` | model / stream name string (`"Tiny 2 Lite StreamCamera"`) | hardware |
| `0x09` | 4-state enum (observed 00 idle / 01 with AI tracking); setter exists in the camera app | hardware + RE |
| `0x0A` | 3-state enum, proxied to the AI module (index `0x25`, wire cmd `0x0944`) | hardware + RE |
| `0x0B` | 2-state enum, proxied to the AI module (index `0x33`, wire cmd `0x0CC4`) | hardware + RE |
| `0x0C` | flat preset slot list `[count, slotIdx...]` | hardware + RE |
| `0x0D` | preset entry cursor, **consuming** (each read advances) | hardware + RE |
| `0x0E` | constant read-only block (`fe 20` + flags); SET is rejected | hardware + RE |
| `0x0F`+ | `ENOENT` from the kernel, not zeros | hardware |

Reading `0x0D` has a side effect, which is how it was mistaken for a
status byte for a while: it walks the preset list one entry per read
and stays exhausted (`status 0x02`) until the `0x0C` block is echoed
back. Never poll it casually.

The flat selectors this app consumes are documented where they are
used: presets in "Reading presets back" below, status in `§3.2`
("Selector `0x06` GET_CUR returns 60-byte status struct").

### 3.2 Tiny 2 XU command table — known surface (FOSS-extracted)

Source: end-to-end read of `cgevans/tiny2` `src/lib.rs` and
`OpenFoxes/Tiny4Linux` `src/libs/camera/commands/*.rs`, recorded
verbatim in [[docs/XU_INVESTIGATION_2026-05-14.md]] (2026-05-14).
Both projects are EUPL-1.2; bytes can be re-used under our
GPL-3.0-or-later licence per the EUPL Appendix (see
[[CREDITS.md]] and [[DECISIONS.md ADR-0020]]).

Conventions used in the tables below:

- *Bytes* are the literal payload sent via `UVCIOC_CTRL_QUERY` with
  `bRequest = UVC_SET_CUR (0x01)`, `bUnit = 0x02`, `bSelector` as
  noted, and the appropriate `wLength` from `UVC_GET_LEN (0x85)`.
- *Status @* is the byte offset within the 60-byte GET_CUR struct
  on selector `0x06` where the camera reflects the resulting state.

#### Selector `0x06` — opcode-multiplexed (SET_CUR)

| Op   | Meaning                | Payload (bytes after op+len) | Values                                         | Status @ | Source        |
|------|------------------------|------------------------------|------------------------------------------------|----------|---------------|
| 0x01 | HDR on/off             | `[len=1, v]`                 | `0x00` off, `0x01` on                          | `0x06`   | cgevans + T4L |
| 0x03 | Face Auto-Exposure     | `[len=1, v]`                 | `0x00` Global, `0x01` Face — only valid in auto-exposure mode | n/a | cgevans + T4L |
| 0x04 | Field of View          | `[len=1, v]`                 | `0x01` Wide (86°), `0x02` Normal (78°), `0x03` Narrow (65°)   | n/a | cgevans       |
| 0x16 | AI Tracking Mode       | `[len=2, m, n]`              | see (m, n) table below — 10 modes              | `0x18` (m) + `0x1c` (n) | cgevans + T4L |

##### AI tracking mode `(m, n)` tuple (op 0x16)

| AIMode           | m    | n    | Notes                                                |
|------------------|------|------|------------------------------------------------------|
| `NoTracking`     | 0x00 | 0x00 |                                                      |
| `NormalTracking` | 0x02 | 0x00 |                                                      |
| `UpperBody`      | 0x02 | 0x01 |                                                      |
| `CloseUp`        | 0x02 | 0x02 |                                                      |
| `Headless`       | 0x02 | 0x03 |                                                      |
| `LowerBody`      | 0x02 | 0x04 |                                                      |
| `DeskMode`       | 0x05 | 0x00 |                                                      |
| `Whiteboard`     | 0x04 | 0x00 |                                                      |
| `Hand`           | 0x06 | 0x00 | ⚠ See quirk Q4 — cgevans + Tiny4Linux setters write `m=3`, decoders read `m=6`. Validate live. |
| `Group`          | 0x01 | 0x00 |                                                      |

#### Selector `0x02` — structured frames

All frames share the layout
`[FRAME_ID=0xaa,0x25, seq_nr(2), SEGMENT_SIZE=0x0c,0x00, checksum(2),
function_group(6), command(6), appendix(16)]`. Checksum values are
opaque to us (per-command constants captured by the FOSS extraction);
do **not** attempt to recompute, copy verbatim. Appendix is zero
unless noted.

| Frame                | function_group                       | seq_nr      | checksum    | command                              | appendix              | Status @ | Source |
|----------------------|--------------------------------------|-------------|-------------|--------------------------------------|-----------------------|----------|--------|
| Exposure → Auto      | `0a 02 82 29 05 00`                  | `15 00`     | `a8 9e`     | `f9 27 01 32 00 00`                  | zero                  | n/a (paired with op `0x03` on sel `0x06` for Face AE selection) | cgevans |
| Exposure → Manual    | `0a 02 82 29 05 00`                  | `16 00`     | `58 91`     | `b2 af 02 04 00 00`                  | zero                  | n/a      | cgevans |
| Sleep → Awake        | `0a 02 c2 a0 04 00`                  | `a5 00`     | `5f ef`     | `be 07 00 00 00 00`                  | zero                  | `0x02`   | Tiny4Linux |
| Sleep → Sleep        | `0a 02 c2 a0 04 00`                  | `42 00`     | `ea 63`     | `bf fb 01 00 00 00`                  | zero                  | `0x02`   | Tiny4Linux |
| Tracking → Standard  | `0a 04 c4 0c 01 00`                  | `20 00`     | `ab cb`     | `e6 3f 00 00 00 00`                  | zero                  | `0x24`   | measured 6.2.5.x |
| Tracking → Sport     | `0a 04 c4 0c 01 00`                  | `21 00`     | `fa 0e`     | `67 fe 02 00 00 00`                  | zero                  | `0x24`   | measured 6.2.5.x |
| Recall Preset 1 (idx 0) | `0a 04 c4 39 14 00`               | `20 00`     | `6b dc`     | `d6 fb 00 00 00 00`                  | `(1.0f32)x4` (16 B)   | none     | Tiny4Linux |
| Recall Preset 2 (idx 1) | `0a 04 c4 39 14 00`               | `1a 00`     | `4b 03`     | `eb 2a 01 00 00 00`                  | `(1.0f32)x4`          | none     | Tiny4Linux |
| Recall Preset 3 (idx 2) | `0a 04 c4 39 14 00`               | `26 00`     | `8b c3`     | `af 19 02 00 00 00`                  | `(1.0f32)x4`          | none     | Tiny4Linux |

The four `1.0_f32` little-endian floats in the Preset-recall appendix
are `[0x00, 0x00, 0x80, 0x3f]` repeated four times; their semantic
meaning is unknown but the camera rejects the recall without them.
Copy verbatim.

#### Preset commands (decoded)

The function group of a frame is `sender, receiver, command`:

| Action | Command | Receiver | Payload |
|--------|---------|----------|---------|
| Recall | `0x39C4` | `0x04` (AI) | `index u32` + four `1.0f32` |
| Create | `0x3944` | `0x04` (AI) | `index u32` + pan/tilt/roll/zoom `f32` + `-1000.0f32` |
| Update | `0x3E04` | `0x04` (AI) | same as create |
| Delete | `0x3984` | `0x04` (AI) | `index u32` |
| Rename | `0x3A84` | `0x04` (AI) | `index u32` + ASCII name |

`index` is zero-based in the frame (slot 1 is index 0). All five were
exercised on hardware on 2026-10-20 (Tiny 2 Lite, firmware 6.2.5.3):
create, update, delete and rename are accepted and reflected in the
store, and the pose floats take the raw V4L2 integer values (see
"Preset pose units" below).

#### Replies

A reply arrives on the same selector and reuses the frame layout, with
these differences:

- byte 1 is `0x29` when the reply carries a payload and `0x89` when it
  does not.
- the length field (bytes 4-5) is the length of the header itself, 12
  for data replies and 16 for error replies.
- the payload block starts at `header_length`.

Error replies carry no payload and add four header bytes: `0x0200`
followed by `0xFFFD`. This signature was confirmed by sending an
unknown command (`0x1234`): it produces the same 16-byte reply as
commands the firmware does not implement on this model (see below).

Commands confirmed working on a Tiny 2 Lite (firmware 6.2.5.3):
`CAM_GET_DEV_STATUS` (`0xA042`, 12-byte payload) and `CAM_GET_EXPOSURE`
(`0x2942`, 5-byte payload).

Commands that answer with the error signature, i.e. not implemented on
this model: `CAM_GET_USB_MODE` (`0x6042`), `CAM_GET_UVC_MODE`
(`0xC442`) and `AI_GET_GIM_STATE` (`0x6604`).

#### Preset pose units (verified)

The `f32` pose in create/update is the raw V4L2 integer value of
`pan_absolute` / `tilt_absolute` / `zoom_absolute`, converted straight
to `f32` with no scaling (1/3600, 100 and 3600 all fail; edge 1.0 is
the shipped `POSE_SCALE`). Verified by saving at a known pose, panning
6 degrees away, recalling, and comparing captured frames by phase
correlation: the view returns to within 2 px of the saved framing.

Reading a pose back through the entry cursor (`0x0D`) uses
centidegrees and inverts the pitch sign: a save at pan 10.00° /
tilt 2.00° reported yaw 984 / pitch -196, i.e. centidegrees within
gimbal positioning error (about 1.6% in that sample).

#### Wire command construction (from the vendor library)

Static RE of the vendor host library (`libdev.dll`, PDB symbols) shows
how the 16-bit wire command is assembled from a category and an index:

```
wire_cmd = (index_table[category][index] << 6) | (category_table[category] & 0x3f)
```

For the gimbal-preset family the low six bits are always `0x04` (the
AI receiver), and the index table supplies the high bits. This
reproduces every captured opcode: index `0x35` gives `0x3B44` (list),
`0x3b` gives `0x3B04` (name get), `0x3d` gives `0x3A84` (set name),
`0x38` gives `0x3984` (delete), `0x39` and `0x41` both give `0x3E04`
(update, two aliases), add gives `0x3944` and trigger gives `0x39C4`.

The camera side confirms the split: its `rpcHandlerCenter::execute`
(libremoproto, used by the AI module) takes the u16 command, derives
`receiver = cmd & 0x3F` and `index = cmd >> 6`, and looks the pair up
in a two-level map of registered handlers.

The preset server is **not in the camera application** but in the AI
module (`aisdk.cross`, which keeps C++ symbols):
`register_gimbal_options_handler` registers receiver 4 with these
indices:

| Index | Wire cmd | Action |
|---|---|---|
| `0xE5` | `0x3944` | create |
| `0xE6` | `0x3984` | delete |
| `0xE7` | `0x39C4` | recall (trigger) |
| `0xE9` | `0x3A44` | read pose |
| `0xEA` | `0x3A84` | rename |
| `0xEC` | `0x3B04` | read name |
| `0xED` | `0x3B44` | list |
| `0xF8` | `0x3E04` | update |

The store behind them is `USettings::Preset` (`Settings/Preset.cpp`).
The camera application is a client of some of these itself (it reads
the flat selectors through index `0xED`/`0xEC`/`0xE9`), which is why
both the framed commands and the flat selectors work.

Worth recording because it means unknown commands in the same category
can be derived from their index instead of guessed, and because it
explains why some opcodes look unrelated at first glance.

#### Preset list

`AI_GET_GIMBAL_PRESET_LIST` (`0x3B44`, receiver `0x04`) is a
header-only GET. **The reply payload is the plain list of stored slot
ids**, one byte per preset, zero-based, in store order, with **no count
byte**: the RPC packet carries the count in its payload length. So
`[]` means nothing stored, `[01]` is one preset in slot 2, `[02, 01]`
is presets in slots 3 and 2, and `[01, 02, 00]` is slots 2, 3 and 1.

Resolved from the camera firmware, where the preset server lives in
the AI module (`aisdk.cross`): `register_gimbal_options_handler`
registers receiver 4 with index `0xED` for the list, whose handler
fills a buffer of exactly `count` bytes with one id per preset and
sends it with `payload.len = count`. The camera app is only a client
of that command (its own `FUN_0007c900` reads `count = payload_len`).

An earlier revision of this document called the first byte an
unreliable count; that was an artefact of treating the id list as
count-prefixed. The flat selector `0x0C` is the one that does carry a
count (`[count, ids...]`, count not including itself).

Reply flags: bit `0x20` announces a payload block, and replies without
it carry a 12-byte header whose length field is the header length, not
a payload length.

#### Reading presets back (flat list and entry cursor)

Hardware-verified on the Tiny 2 Lite (2026-10-20), implemented in
`obsbot-core::xu::commands::preset_read`:

- **Selector `0x0C`** (flat, reliable): `[count:u8, slotIdx:u8 x
  count]`, zeros when empty. This is the occupancy source the GUI
  renders.
- **Selector `0x0D`** (entry cursor): each `GET_CUR` returns one entry
  and **advances the cursor**:
  `[status:u8, slotIdx:u8, _pad:u16, pitch:i16 centideg @4,
  yaw:i16 centideg @6, zoom_x100:u16 @8, base64 name @10
  NUL-terminated]`. `status == 0x02` means exhausted. Entries created
  through the framed channel appear here. **Echo-writing the `0x0C`
  block back resets the cursor** — without that reset a cursor left
  exhausted answers `0x02` immediately. Pitch sign is inverted vs
  `tilt_absolute`; a save at pan 10.00° / tilt 2.00° reported
  yaw 984 / pitch -196, i.e. centidegrees within gimbal positioning
  error (~1.6% in that sample).
- **Names are base64** (standard alphabet, `=` padding) in the cursor
  entry: renaming a preset "TestA" stored `VGVzdEE=`. One rename in
  the first test session ("Diez") was later read back as "Dieg"; seven
  subsequent round-trips (3-5 letters, lowercase and mixed case) came
  back byte-exact, so treat that instance as a transient, not as a
  length-dependent rule.
- **`AI_GET_GIMBAL_PRESET_ID_NAME` (`0x3B04`) is not usable on this
  firmware** despite being in the vendor opcode table: the Lite
  answers a structured error (`00 02 0a 00` in a 16-byte header,
  error code `0x000a`) to the documented 4-byte index request. The
  cursor is the working read path.

#### Preset factory records (selector 7)

Selector `0x07` is the legacy gimbal-preset channel holding six 10-byte
pose records: record 0 is empty; records 1-5 are factory defaults
(pitch 10.02°, yaw 8.04°, zoom 1.0x — centidegree `i16` values
`0x03EA 0x0324`, zoom `0x0064`). They survive preset create/delete and
are not listed by `0x0C`, so treat them as read-only factory residue,
not user data.

#### Frame layout and checksum (decoded)

The frames above are not opaque byte strings: the layout and both
checksums were recovered from the shipping firmware (`PacketParser` in
the `route` daemon, `getcrc16` in `libremocpp_core`) and validated
against every captured frame in this section (header and payload, 18
checksums, all matching). `obsbot_core::xu::frame::encode` builds them
from fields.

Header, 12 bytes:

| Offset | Field |
|--------|-------|
| 0      | `0xAA` |
| 1      | flags; captured frames use `0x25`, and bit `0x60` means a payload block follows |
| 2-3    | sequence number, little endian |
| 4-5    | header length, little endian (12) |
| 6-7    | header CRC16, little endian, computed with this field zeroed |
| 8-11   | function group |

Optional payload block at offset `header_length`:

| Offset | Field |
|--------|-------|
| +0-1   | payload length, little endian |
| +2-3   | payload CRC16, little endian, computed over `length + 4` bytes with this field zeroed |
| +4..   | payload bytes |

CRC16 is the standard CRC-16/USB variant: polynomial `0xA001`
(reflected `0x8005`), initial value `0xFFFF`, final XOR `0xFFFF`. The
firmware implements it with two 256-entry tables, which produce the
same values as the bitwise form.

With this layout the "command" bytes of the tables above decompose as
`CRC16 + payload`; for example the sleep frame payload is four zero
bytes and the preset recall payload is a one-byte slot index plus four
`1.0f32` values.

#### Selector map (measured on a Tiny 2 Lite, firmware 6.2.5.x)

All fourteen selectors answer `GET_LEN = 60`. Idle `GET_CUR` values and
what the experiments pinned down:

| Selector | Value | Meaning |
|----------|-------|---------|
| `0x01` | `00` | unknown |
| `0x02` | `00` | framed command channel and its replies |
| `0x03` | `00` / `01` | tracking active (follows AI tracking on/off) |
| `0x04` | `01` | unknown constant |
| `0x05` | `00` | unknown |
| `0x06` | 60-byte status struct | see below |
| `0x07` | `06` | unknown; the vendor app sends gimbal preset commands through property id 7 |
| `0x08` | `"Tiny 2 Lite StreamCamera"` | stream name |
| `0x09` | `00` / `01` | tracking active (same toggle as 0x03) |
| `0x0A` | `FF` | unknown constant |
| `0x0B` | `00` / `01` | tracking speed (0 Standard, 1 Sport) |
| `0x0C` | `00` | unknown; static across all stimuli tried |
| `0x0D` | `03` | unknown constant |
| `0x0E` | `FE 20` | unknown constant |

Status bytes measured by diffing the struct across known state changes:

| Offset | Meaning |
|--------|---------|
| `0x02` | sleep state |
| `0x04` | crop level (`0x05` wide, `0x0F` normal) |
| `0x06` | HDR flag |
| `0x07` | zoom gesture flag (1 by default, 0 after the gesture is enabled) |
| `0x11` | field of view (1 wide, 2 normal; narrow is a no-op) |
| `0x18`, `0x1C` | AI mode tuple |
| `0x24` | tracking speed (0 Standard, 2 Sport) |

`CAM_GET_DEV_STATUS` (`0xA042`) returns a 12-byte payload: byte 0 is a
read counter, bytes 1-3 are constant, and bytes 4-7 hold a little-endian
value that decreases by roughly 96,000 per second, which looks like an
audio-rate counter.

Still unknown: selectors 1, 4, 5, 7, 10, 12, 13 and 14 semantics, and
whether preset read-back lives behind a different request shape.

#### Selector `0x06` — GET_CUR returns 60-byte status struct

| Offset | Field          | Encoding                                                               | Source |
|--------|----------------|------------------------------------------------------------------------|--------|
| `0x02` | Sleep state    | `0x00` Awake, `0x01` Sleep, anything else Unknown                      | Tiny4Linux |
| `0x06` | HDR flag       | `0x00` off, non-zero on                                                | cgevans + T4L |
| `0x18` | AI mode `m`    | first byte of `(m, n)` per the table above                             | cgevans + T4L |
| `0x1c` | AI mode `n`    | second byte of `(m, n)` per the table above                            | cgevans + T4L |
| `0x24` | Tracking speed | `0x00` Standard, `0x02` Sport (0x21 on firmware older than 6.2.5) | measured 6.2.5.x |

Bytes `0x00`, `0x01`, `0x03`-`0x05`, `0x07`-`0x17`, `0x19`-`0x1b`,
`0x1d`-`0x20`, `0x22`-`0x3b` are returned by the camera but
**undecoded** by either FOSS project. They are the discovery
frontier. The v0.3 GUI (T-302) shipped a "Dump status" debug dialog
for capturing hex dumps of non-default states; it was removed in the
v0.4.2 UI streamlining (T-220). The 60-byte `raw` payload remains
available programmatically via `obsbot_core::xu::Status::raw` for
anyone who wants to resume the discovery work.

#### Known quirks (Q-series, this XU)

- **Q4 — `AIMode::Hand` setter / decoder mismatch.** cgevans's
  setter writes `[0x16, 0x02, 0x03, 0x00]` (m=3) for `Hand`;
  the decoder maps `(m=6, n=0)` to `Hand`. Tiny4Linux mirrors the
  same mismatch. Almost certainly a typo in cgevans's setter that
  Tiny4Linux inherited; flagged for live validation in T-303.
  Until validated, treat both `(m=3, n=0)` and `(m=6, n=0)` as
  `Hand` in our decoder.
  - **Status 2026-05-15 (T-303 closure)**: accepted as-is. User
    validation pass exercised the full AI tracking row (all 10
    modes) without reporting any contradiction on Hand; treat
    `(m=3, n=0)` as the live-correct encoding for Hand on Tiny 2
    Lite firmware 5.10. If a future user reports Hand engaging
    something else on different hardware,
    `obsbot-core::xu::enums::AiMode::to_wire` is the one-line
    flip.
- **Q5 — Auto / Manual exposure label inversion.** cgevans's
  `AUTO_EXP_CMD` bytes equal Tiny4Linux's `MANUAL` literal, and
  vice versa. cgevans's labelling is the more likely-correct one
  (the `[0x03, 0x01, x]` Face-AE follow-up only makes sense after
  putting the camera in auto). Our port adopts cgevans's
  labelling; T-303 validates live.
- **Q6 — Tracking speed value `0x21 = 0x01` is unmapped.** Gap
  between Standard (`0x00`) and Sport (`0x02`) suggests a third
  mode (possibly the "Headroom" mode cgevans declared in an
  unused enum). Decoder defaults the gap value to Standard;
  re-investigate if a user reports a third speed slider in the
  proprietary app.
- **Q7 — Preset save is implemented in this app since the
  `feat/xu-preset-admin` work.** The create (`0x3944`), update
  (`0x3E04`), delete (`0x3984`) and rename (`0x3A84`) commands are
  accepted by the Tiny 2 Lite and reflected in
  `AI_GET_GIMBAL_PRESET_LIST`. Two caveats: the delete ACKs even
  for slots that do not exist (so only the list proves a removal),
  and the pose unit scale baked into the GUI (`POSE_SCALE` in
  `extras_view.rs`) was verified on one Tiny 2 Lite with
  `scripts/verify-recall.sh` — re-run that script before changing
  it.

- **Q8 — FOV Narrow (65°) is a no-op on Tiny 2 Lite firmware 5.10.**
  Sending `[0x04, 0x01, 0x03]` (the byte sequence cgevans declares
  for `FOVMode::Narrow`, byte-identical between our port and the
  upstream) produces no visible crop change on the user's Tiny 2
  Lite (3564:fef9, bcdDevice 5.10). Wide and Normal both work.
  Probable root cause: the Lite's optics lack the narrowest
  digital-crop path the regular Tiny 2 ships with, so the
  firmware silently ignores the byte. Narrow stays in the
  dropdown so regular-Tiny-2 owners can use it; the GUI's FOV
  subtitle calls out the Lite case. Observed during T-301 live
  validation, 2026-05-14.

- **Q10 — The gesture zoom never touches `zoom_absolute`.** Measured on
  a Tiny 2 Lite (3564:fef9, bcdDevice 5.10) on 2026-08-06 during T-228.
  With the live preview running, the on-device L-gesture was triggered
  and the camera visibly zoomed in and back out. Throughout,
  `zoom_absolute` (`0x009a090d`) stayed at `0`: 60 consecutive reads two
  seconds apart from inside the app, plus an independent
  `v4l2-ctl --get-ctrl=zoom_absolute` afterwards, all returned `0`. The
  firmware applies the gesture zoom internally without reflecting it in
  the UVC Camera Terminal control the kernel exposes.
  **Consequence.** Anything that tries to constrain the gesture zoom by
  watching or rewriting `zoom_absolute` cannot work, because the value
  never moves. That is what T-223's zoom lock attempted and why T-228
  reworked it. Constraining it would mean turning the gesture feature
  off in the firmware, and no FOSS project maps that command:
  `cgevans/tiny2` and `taxfromdk/obsbot_tiny_reversing` have nothing on
  gestures, and `OpenFoxes/Tiny4Linux`'s README documents configuring
  gesture settings from OBSBOT Center inside a Windows VM as the only
  known route. The opcode would be among the unmapped selector-`0x06`
  ones (`0x02`, `0x05`, `0x06`-`0x15`, `0x17`+).
  Worth recording alongside: what `taxfromdk` labelled "suspected zoom"
  in `research/rosetta.txt` is
  `controlWrite(0x21, 0x01, 0x0B00, 0x0100, …)`, i.e. UVC selector
  `0x0B` (`CT_ZOOM_ABSOLUTE_CONTROL`) on unit 1, the Camera Terminal.
  The proprietary app's *manual* zoom does travel the same path the
  kernel exposes; only the gesture bypasses it.

- **Q11 — On-device gestures are controllable over the XU, and the
  L-pose is "zoom", not "dynamic zoom".** Captured and confirmed on a
  Tiny 2 Lite (3564:fef9, firmware 6.2.5.3) on 2026-08-06 for T-229.
  No FOSS project maps these; the frames were obtained by observing the
  `UVCIOC_CTRL_QUERY` traffic the vendor SDK emits under an `LD_PRELOAD`
  shim over `ioctl`, while calling its gesture API. Only the observed
  bytes were kept: the SDK is neither linked nor redistributed, so
  ADR-0002 stands.
  Selector `0x02`, standard frame as described above: 12-byte header
  plus a payload block. The `function_group` picks the gesture and the
  `command` carries the value in its third byte:

  | Gesture | `function_group` |
  |---|---|
  | Target selection | `0a 04 c4 30 01 00` |
  | Zoom | `0a 04 44 31 01 00` |
  | Dynamic zoom | `0a 04 44 33 01 00` |
  | Dynamic zoom direction | `0a 04 c4 33 01 00` |

  `command` is `e6 3f 00 00 00 00` for off and `27 ff 01 00 00 00` for
  on, identical across all four.
  **The naming is a trap.** On hardware, switching off *Zoom* stops the
  L-shaped hand pose; switching off *Dynamic zoom* changes nothing
  observable. The obvious reading of the vendor's names is backwards,
  and it was implemented the wrong way round first.
  This is the answer to Q10: the gesture zoom cannot be constrained from
  the V4L2 side, but the gesture itself can be turned off here.
  **Checksum: decoded.** Bytes 2-3 are a sequence number and 6-7 a
  header CRC16; the payload block carries its own CRC16. The format was
  recovered from the firmware and every captured pair validates against
  it, so frames no longer need to be replayed verbatim. See "Frame
  layout and checksum (decoded)" above.

- **Q12 — Preset pose units are the raw V4L2 integers, and the
  firmware is fragile around streaming restarts.** Both facts
  verified live on a Tiny 2 Lite (3564:fef9, bcdDevice 5.10) on
  2026-10-20 with `scripts/verify-recall.sh`.
  **Pose units.** `AI_ADD_GIMBAL_PRESET` takes the
  `pan_absolute` / `tilt_absolute` / `zoom_absolute` integers
  converted straight to `f32` (scale 1.0; alternatives 1/3600,
  100 and 3600 all failed). Phase correlation between captured
  frames shows the recalled view returns to within 2 px of the
  saved view after a 6-degree pan away. This is the `POSE_SCALE`
  constant in `extras_view.rs`.
  **Streaming fragility.** Two failure patterns make the camera
  drop off the USB bus (`-71 EPROTO`, device re-enumerates after
  ~20 s or stays gone until a replug):
  1. Starting a streaming client in the first ~minute after USB
     enumeration. One drop happened 20 s after a replug with no
     host traffic at all, then a self-reset; runs that waited
     several idle minutes survived.
  2. More than ~2 cycles of (streaming negotiation open/close
     interleaved with XU writes and absolute pan sets) in quick
     succession. A single save / pan / recall / capture round
     trips is safe; looping it killed the device twice at the
     start of the third pass.
  **Consequence.** The GUI's own usage (one long-lived streaming
  session, XU traffic in between) matches the pattern the vendor
  app uses and is safe. Test harnesses must keep to one cycle and
  give the camera idle time after replug before streaming.
  `scripts/verify-recall.sh` encodes exactly that discipline.

- **Q13 — The framed `0xA042` is `CAM_GET_SYS_TIME`, not a status
  query, and the 60-byte block is XU selector 6.** Static RE of
  `libdev.dll` (PDB symbols) places `0xA042 = CAM_GET_SYS_TIME`
  (receiver `0x02`) in the vendor opcode table; `CAM_SET_DEV_STATUS`
  is `0xA0C2`. The 60-byte block a framed GET returns is the flat
  status block the camera pushes on the reply channel — the same
  bytes selector `0x0C`-style flat reads expose as selector `6`
  `GET_CUR` (60 bytes). The vendor host code copies the block raw
  with no per-offset struct; the only field decode that exists is
  `cgevans/tiny2`'s hardware-verified one (on the regular Tiny 2):
  zoom u16 @0x04 (% of the 1.0-2.0x range), HDR flag @0x06,
  face-AE/gesture-zoom @0x07, device status @0x09 (1 run / 3 sleep /
  4 privacy), auto-sleep seconds @0x0a, FOV @0x11, mirror @0x13,
  AI mode+submode @0x18/0x1c, fps @0x1f, LED brightness @0x21,
  tracker speed @0x24. On the Lite (bcdDevice 5.10) the observed
  baseline matches several of these (zoom 0 = 1.0x, status 01 =
  run, HDR 01, mirror 01 with the T-210 mirror on, auto-sleep 120 s)
  and a diff run on the Lite (2026-10-20) confirms the offset map by
  observation: toggling HDR moves `0x06` 01->00, setting Sport moves
  `0x24` 00->02, and the FOV byte `0x11` reads 01 for Wide, 02 for
  Normal (Narrow leaves it at 02, consistent with Q8).
  **No counter exists.** The camera builds the block on demand
  (`FUN_0004a348` case 6) with `payload[0] = 0x27` and writes only
  these fields: zoom u16 @0x04, HDR @0x06, face-AE @0x07, device
  status @0x09 (raw `{0,1,2,6}` mapped to `{1,2,3,4}`, anything else
  to 8), auto-sleep seconds @0x0a-0x0b, FOV @0x11, mirror @0x13,
  AI mode/submode @0x18/0x1c, fps @0x1f, LED @0x21, tracker speed
  @0x24; bytes 0x25-0x3b are never written. Repeated reads one
  second apart return identical bytes, so the earlier "u32
  decrementing ~96000/s" reading was an artefact of decoding three
  separate fields as one integer.
  Our `Status` decoder keeps the Lite-observed offsets (sleep @0x02,
  HDR @0x06, AI @0x18/0x1c, speed @0x24) until a fuller Lite map is
  diffed.

- **Q14 — Selector `0x0D` is a consuming cursor, not a status byte.**
  The "selector 13" mystery (read `03`, later `02`, never recovering)
  was self-inflicted: every `GET_CUR` on `0x0D` returns one preset
  entry and advances; once exhausted it stays at status `0x02`.
  Echo-writing the `0x0C` block back resets it. See "Reading presets
  back" above.

- **Q5 resolution (Auto / Manual exposure label swap).** Live
  validation on 2026-05-14 showed cgevans's labelling produces
  the opposite of the V4L2 standard `auto_exposure` control —
  i.e. our XU "Auto" frame puts the camera in Manual and vice
  versa. **Rather than flipping the labels and inheriting the
  ambiguity, T-301 removed the XU exposure-mode widget
  entirely.** The V4L2 standard `auto_exposure` menu (Camera
  class, exposed by T-104) is now the sole exposure-mode entry
  point in the GUI; it uses kernel labels (no swap risk),
  supports three values (Auto / Manual / Aperture Priority),
  and greys the exposure-time slider via the kernel INACTIVE
  flag. The XU encoder
  `obsbot_core::xu::commands::exposure_mode_type` stays
  available for any future caller, but the GUI does not call
  it. The same retire-rather-than-fix decision retired the
  Face-AE row, which only meters correctly when the camera
  is in auto-exposure via the XU frame path (and so silently
  no-ops when the user takes the V4L2 standard route).

### 3.3 Tiny 2 (regular) XU

Not yet captured. Working hypothesis (to verify):

- Same `guidExtensionCode` as the Lite — vendor XUs typically share a
  GUID across a model family.
- Same or larger `bNumControls` and `bmControls` (Lite is feature-reduced
  vs. regular).

If a regular Tiny 2 capture reveals a different GUID, the
`obsbot-core` XU dispatch becomes per-PID rather than family-wide.

---

## 4. Raw USB control requests (fallback)

If `UVCIOC_CTRL_QUERY` proves insufficient (some vendor traffic may bypass
the UVC class), document the raw control transfers here:

- `bmRequestType`, `bRequest`, `wValue`, `wIndex`, `wLength`.
- Direction.
- Payload format.

To be filled if needed.

---

## 5. Firmware quirks

- **Tiny 2 Lite firmware version**: `bcdDevice = 5.10` in the device
  descriptor (provisional). Whether this corresponds to the firmware
  revision reported by OBSBOT Center is unconfirmed — may instead be a
  hardware-revision number unrelated to user-visible firmware.
- **No iSerial string**: the Lite exposes `iSerial=0`. This means the
  device cannot be uniquely identified across reboots / re-plugs using
  the USB serial-number descriptor alone. Decision (T-105 + T-105fix,
  shipped): per-camera settings are keyed by serial in the
  `control-values` GSettings map; on the Lite (no serial) the GUI
  simply does not persist values. The two heavier fallbacks considered
  (USB bus/port path, user-assigned names) were rejected as fragile /
  not worth the UX cost for a single-camera user. This is a deviation
  from the SPEC.md §4.1 expectation that settings key off serial.

---

## 6. Cross-project knowledge map

What each project knows, so a future reader does not re-derive it or
trust the wrong source. "Verified" below means verified on physical
hardware by that project; "static" means read out of the firmware or
the vendor binary without a hardware check.

| Claim | Source | Status |
|---|---|---|
| CRC-16/USB over header and payload block, field zeroed | `cgevans/tiny2` (`crc16_usb`), this repo, vendor firmware `getcrc16` | verified, three independent derivations |
| Status offsets: zoom `0x04`, HDR `0x06`, face-AE `0x07`, dev status `0x09`, auto-sleep `0x0a`, FOV `0x11`, mirror `0x13`, AI `0x18/0x1c`, fps `0x1f`, LED `0x21`, speed `0x24` | `cgevans/tiny2` `CameraStatus::decode` (regular Tiny 2) | verified there; partly re-verified on the Lite here (HDR `0x06`, speed `0x24`, FOV `0x11`) |
| Flat preset list on selector `0x0C`, entry cursor on `0x0D`, cursor reset by echo-writing `0x0C` | `cgevans/tiny2` (`XU_SEL_PRESET_LIST` / `XU_SEL_PRESET_ENTRY`), `lxman/obsbot-mcp` (`decodePresetList`, `decodePresetEntry`) | verified here on the Lite |
| The framed preset list payload is ids only, no count byte | this repo (camera firmware: AI module RPC handler for index `0xED`) | resolved here; explains every capture taken before the fix |
| Preset names are base64, `=` padded | `lxman/obsbot-mcp` | verified here (`TestA` -> `VGVzdEE=`) |
| Opcode `0xA042` is `CAM_GET_SYS_TIME`, `CAM_SET_DEV_STATUS` is `0xA0C2` | `lxman/obsbot-mcp` opcode table (from `libdev.dll` PDB symbols) | static, accepted (matches our captures showing a status block, not a data frame) |
| Wire command = `(index << 6) | 0x04` for the preset family | this repo, from `libdev.dll` | static, reproduces every capture |
| Preset create/update/delete/rename opcodes and payloads | vendor application table, `OpenFoxes/Tiny4Linux` (`command02` call sites) | verified here on hardware |
| Preset recall needs the four `1.0f32` floats in the payload | `OpenFoxes/Tiny4Linux` captures, this repo | verified (camera rejects the recall without them) |
| Selector `0x06` tag commands (suspend time, vertical, face AE, FOV, AI mode, LED, mirror, boot mode) | `cgevans/tiny2` (`TAG_*`), `libdev.dll` xrefs | verified there; this app uses its own framed commands instead |
| Sleep/Wake and Tracking Speed commands | `OpenFoxes/Tiny4Linux` | verified there and here |
| Gesture on/off frames and the backwards naming of Zoom vs Dynamic zoom | this repo (captures under an `LD_PRELOAD` shim) | verified here, see Q11 |
| Manual zoom travels selector `0x0B` on the Camera Terminal (`controlWrite(0x21, 0x01, 0x0B00, 0x0100, ...)`) | `taxfromdk/obsbot_tiny_reversing` (`research/rosetta.txt`) | historical model, consistent with V4L2 `zoom_absolute` |

Open questions, unanswered anywhere:

- Whether `AI_GET_GIMBAL_PRESET_ID_NAME` (`0x3B04`) works on the
  regular Tiny 2 (it returns error `0x000a` on the Lite).
- What the four states of selector `0x09` and the three states of
  `0x0A` mean, beyond "tracking is one of the `0x09` states".
- What the 10-bit index range `0xE1`-`0xE4`, `0xEE`, `0xF6`, `0xF7`
  and `0xF9`-`0xFB` in the gimbal options table do; they are
  registered but not exercised by this app.
- Whether the vendor UI ever writes a preset name longer than 63
  bytes (the host parser clamps to 63).

Upstream contributions from this repo are listed in §7.

## 7. References

- Linux UVC driver documentation:
  https://kernel.org/doc/html/latest/userspace-api/media/drivers/uvcvideo.html
- USB Video Class 1.5 specification (publicly available from USB-IF):
  https://www.usb.org/document-library/video-class-v15-document-set
- Wireshark USB capture tutorial:
  https://wiki.wireshark.org/CaptureSetup/USB
- `cgevans/tiny2` (Rust, EUPL-1.2, the primary XU reference our v0.3
  port is based on):
  https://github.com/cgevans/tiny2
- `OpenFoxes/Tiny4Linux` (Rust, EUPL-1.2, AUR-packaged active fork
  adding Sleep/Wake + Tracking Speed + Preset recall):
  https://github.com/OpenFoxes/Tiny4Linux
- `samliddicott/meet4k` (EUPL-1.2, the upstream pattern cgevans/tiny2
  is "substantially based on"):
  https://github.com/samliddicott/meet4k
- `taxfromdk/obsbot_tiny_reversing` (earlier OBSBOT Tiny — different
  model, reference only):
  https://github.com/taxfromdk/obsbot_tiny_reversing
- `aaronsb/obsbot-camera-control` (reference Qt6 app, uses proprietary
  SDK — NOT a citation source for our port):
  https://github.com/aaronsb/obsbot-camera-control
- Linux kernel patch confirming Tiny 2 PTZ speed via standard UVC:
  http://www.mail-archive.com/linuxtv-commits@linuxtv.org/msg48291.html
- EUPL-1.2 → GPL-3 compatibility (EUPL Appendix):
  https://eupl.eu/1.2/en/
