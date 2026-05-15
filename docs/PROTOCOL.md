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
- **V4L2 standard controls**: pending — needs `v4l2-ctl --all`
  + `--list-ctrls-menus` on `/dev/video0` and `/dev/video1`. Requires
  the `video` group membership / sudo (see §1.1 below).
- **Vendor XU**: descriptor metadata captured (Unit ID, GUID, bmControls
  mask). Per-selector semantics: **the majority of the surface is now
  known from free-software sources** (`cgevans/tiny2` and
  `OpenFoxes/Tiny4Linux`, both EUPL-1.2). See §3.2 below for the
  decoded command table and [[DECISIONS.md ADR-0020]] for the FOSS
  pivot. The previously documented "Wireshark + Windows VM" capture is
  no longer a v0.3 prerequisite (it was the old v0.4's prereq before
  the milestone collapse); it remains a *future, optional* avenue for
  probing the still-unmapped opcodes (selector `0x06` opcodes
  `0x02`, `0x05`, `0x06`-`0x15`, `0x17`+) and the 55 still-undecoded
  bytes of the 60-byte GET_CUR status struct.

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
`white_balance_automatic` switch. The GUI (T-103 in v0.2) should group
them under a single "White balance" section with a sub-toggle.

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
  conversion bug for this specific selector. The GUI clamps display
  to `0..100` and writes via `zoom_absolute` for static targets;
  whether to surface `zoom_continuous` at all is a T-102 decision.
- **Q3 — gamma is not advertised** despite SPEC.md §4.1 listing it.
  The PROCESSING_UNIT bmControls in §1.1 has no gamma bit. Treat
  gamma as XU-only on this family until disproven; if a vendor XU
  selector carries it, document under §3.

### 2.4 Streaming formats and frame sizes

Default at capture time: `MJPG 1920×1080 @ 30 fps`. A separate
`v4l2-ctl --list-formats-ext` capture is recommended before
implementing the preview pipeline (v0.3 / T-200+), so the full
{MJPG, YUYV} × {sizes} × {framerates} matrix is locked down. Pending.

---

## 3. UVC Extension Units (XU)

The Linux uvcvideo driver enumerates XUs at probe time; they appear in
`lsusb -v` output and can be queried/written via `UVCIOC_CTRL_QUERY`
ioctl (see [[ARCHITECTURE §3.3]] for the wrapper).

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

The 19 advertised selectors on this XU turn out to be **largely
unused**. The OBSBOT firmware multiplexes the entire vendor surface
across just **two** selectors:

- **Selector `0x06`** — opcode-multiplexed. Payload format is
  `[opcode, payload_len, ...payload_bytes]`. Four opcodes known.
- **Selector `0x02`** — structured 18/36-byte frames. Five frame
  shapes known (Manual/Auto exposure, Sleep/Wake, Tracking Speed,
  three Preset recalls).

The remaining 17 selector slots in `bmControls` are advertised by the
descriptor but not addressed by either of the two FOSS reference
projects (see §3.2 for the full citation table).

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

#### Selector `0x02` — structured 36-byte frames

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
| Tracking → Standard  | `0a 04 c4 0c 01 00`                  | `20 00`     | `ab cb`     | `e6 3f 00 00 00 00`                  | zero                  | `0x21`   | Tiny4Linux |
| Tracking → Sport     | `0a 04 c4 0c 01 00`                  | `21 00`     | `fa 0e`     | `67 fe 02 00 00 00`                  | zero                  | `0x21`   | Tiny4Linux |
| Recall Preset 1 (idx 0) | `0a 04 c4 39 14 00`               | `20 00`     | `6b dc`     | `d6 fb 00 00 00 00`                  | `(1.0f32)x4` (16 B)   | none     | Tiny4Linux |
| Recall Preset 2 (idx 1) | `0a 04 c4 39 14 00`               | `1a 00`     | `4b 03`     | `eb 2a 01 00 00 00`                  | `(1.0f32)x4`          | none     | Tiny4Linux |
| Recall Preset 3 (idx 2) | `0a 04 c4 39 14 00`               | `26 00`     | `8b c3`     | `af 19 02 00 00 00`                  | `(1.0f32)x4`          | none     | Tiny4Linux |

The four `1.0_f32` little-endian floats in the Preset-recall appendix
are `[0x00, 0x00, 0x80, 0x3f]` repeated four times; their semantic
meaning is unknown but the camera rejects the recall without them.
Copy verbatim.

#### Selector `0x06` — GET_CUR returns 60-byte status struct

| Offset | Field          | Encoding                                                               | Source |
|--------|----------------|------------------------------------------------------------------------|--------|
| `0x02` | Sleep state    | `0x00` Awake, `0x01` Sleep, anything else Unknown                      | Tiny4Linux |
| `0x06` | HDR flag       | `0x00` off, non-zero on                                                | cgevans + T4L |
| `0x18` | AI mode `m`    | first byte of `(m, n)` per the table above                             | cgevans + T4L |
| `0x1c` | AI mode `n`    | second byte of `(m, n)` per the table above                            | cgevans + T4L |
| `0x21` | Tracking speed | `0x00` Standard, `0x02` Sport, anything else defaults to Standard      | Tiny4Linux |

Bytes `0x00`, `0x01`, `0x03`-`0x05`, `0x07`-`0x17`, `0x19`-`0x1b`,
`0x1d`-`0x20`, `0x22`-`0x3b` are returned by the camera but
**undecoded** by either FOSS project. They are the discovery
frontier. The v0.3 GUI (T-302) ships a "Dump status" debug page so
the user can capture a hex dump of any non-default state for future
contributions.

#### Known quirks (Q-series, this XU)

- **Q4 — `AIMode::Hand` setter / decoder mismatch.** cgevans's
  setter writes `[0x16, 0x02, 0x03, 0x00]` (m=3) for `Hand`;
  the decoder maps `(m=6, n=0)` to `Hand`. Tiny4Linux mirrors the
  same mismatch. Almost certainly a typo in cgevans's setter that
  Tiny4Linux inherited; flagged for live validation in T-303.
  Until validated, treat both `(m=3, n=0)` and `(m=6, n=0)` as
  `Hand` in our decoder.
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
- **Q7 — Preset save is not implemented in either FOSS project.**
  Only recall (3 slots) is supported. Presets must be programmed
  via the OBSBOT Center app or the camera's on-device gesture
  mechanism beforehand. Adding preset save is deferred to a
  follow-on milestone pending USB capture against the proprietary
  app.

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
  the USB serial-number descriptor alone. Per-device settings
  persistence ([[ROADMAP v0.2]], T-105) must fall back to:
  1. USB bus/port path (stable as long as the cable stays in the same
     port — fragile across re-plugs to a different port).
  2. A user-assigned name kept in GSettings, applied to the first
     detected device matching the family PIDs.
  This is a deviation from the SPEC.md §4.1 expectation that settings
  key off serial; flagged here, formal decision deferred to T-105.

---

## 6. References

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
