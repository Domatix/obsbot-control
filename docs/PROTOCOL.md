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
  mask). Per-selector semantics pending Wireshark capture against
  OBSBOT Center (v0.4 milestone, T-300+).

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

**Status**: pending — to be captured with `v4l2-ctl --all
--list-ctrls-menus` on each of `/dev/video0` and `/dev/video1` (both
nodes belong to the same physical Tiny 2 Lite on the user's machine —
verified via `/sys/class/video4linux/videoN` → `1-7:1.0` USB path).

Capture procedure (re-runnable; outputs land in `/tmp/`):

```
sudo v4l2-ctl -d /dev/video0 --all              > /tmp/obsbot-v4l2-all-0.txt
sudo v4l2-ctl -d /dev/video0 --list-ctrls-menus > /tmp/obsbot-v4l2-ctrls-0.txt
sudo v4l2-ctl -d /dev/video1 --all              > /tmp/obsbot-v4l2-all-1.txt
sudo v4l2-ctl -d /dev/video1 --list-ctrls-menus > /tmp/obsbot-v4l2-ctrls-1.txt
```

(Once the user is added to the `video` group via
`sudo usermod -aG video alvaro` and the next login picks it up, the
`sudo` prefix becomes unnecessary.)

Expected categories, inferred from §1.1's INPUT_TERMINAL and
PROCESSING_UNIT bmControls and from the linuxtv-commits 2025-12 kernel
patch:

- `V4L2_CID_BRIGHTNESS`, `_CONTRAST`, `_HUE`, `_SATURATION`, `_SHARPNESS`,
  `_GAMMA` (Gamma TBD — not advertised in PU bmControls, may be
  XU-only).
- `V4L2_CID_WHITE_BALANCE_TEMPERATURE` + `_AUTO_WHITE_BALANCE`.
- `V4L2_CID_BACKLIGHT_COMPENSATION`, `_GAIN`, `_POWER_LINE_FREQUENCY`.
- `V4L2_CID_EXPOSURE_AUTO`, `_EXPOSURE_ABSOLUTE`, `_EXPOSURE_AUTO_PRIORITY`.
- `V4L2_CID_FOCUS_ABSOLUTE` + `_FOCUS_AUTO`.
- `V4L2_CID_ZOOM_ABSOLUTE`, `_ZOOM_CONTINUOUS`.
- `V4L2_CID_PAN_ABSOLUTE`, `_TILT_ABSOLUTE`, `_PAN_SPEED`, `_TILT_SPEED`
  (PTZ speed handling fixed by the linuxtv-commits 2025-12 patch on the
  regular Tiny 2; the Lite shares the descriptor shape so the same
  semantics should hold).

Concrete `min`/`max`/`step`/`default` ranges live in the captured tables
above once filled.

---

## 3. UVC Extension Units (XU)

The Linux uvcvideo driver enumerates XUs at probe time; they appear in
`lsusb -v` output and can be queried/written via `UVCIOC_CTRL_QUERY`
ioctl (see [[ARCHITECTURE §3.3]] for the wrapper).

### 3.1 Tiny 2 Lite XU table

Source: `lsusb -v -d 3564:fef9` on the user's machine, 2026-05-13.

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

#### Per-selector decode (pending v0.4 / T-300+)

| Selector | Logical meaning              | wLength | GET/SET | Source                       |
|----------|------------------------------|---------|---------|------------------------------|
| 1        | TBD                          | TBD     | TBD     | TBD                          |
| …        | …                            | …       | …       | …                            |
| 19       | TBD                          | TBD     | TBD     | TBD                          |

To populate, cross-reference with:

- `taxfromdk/obsbot_tiny_reversing` — earlier model, may share the
  selector numbering convention.
- `samliddicott/meet4k` — Rust XU library for Meet 4K, same vendor.
- A fresh Wireshark + usbmon capture of OBSBOT Center toggling each
  feature one at a time against a Tiny 2 in a Windows VM.

### 3.2 Tiny 2 (regular) XU

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
- `taxfromdk/obsbot_tiny_reversing`:
  https://github.com/taxfromdk/obsbot_tiny_reversing
- `samliddicott/meet4k`:
  https://github.com/samliddicott/meet4k
- `aaronsb/obsbot-camera-control` (reference Qt6 app, uses proprietary SDK):
  https://github.com/aaronsb/obsbot-camera-control
- Linux kernel patch confirming Tiny 2 PTZ speed via standard UVC:
  http://www.mail-archive.com/linuxtv-commits@linuxtv.org/msg48291.html
