# PROTOCOL — OBSBOT Communication Protocol

> **Purpose**: Document every fact discovered about how OBSBOT cameras
> respond to USB control requests. Grows during T-003, T-011, and the
> v0.4 milestone. Must remain reproducible: every claim cites a source
> (USB capture, kernel patch, open-source project) or is explicitly
> marked as untested speculation.

---

## Status

Empty as of project bootstrap. Will be populated during T-003 (capture
Tiny 2 USB descriptor) and the v0.4 milestone (vendor XU discovery).

---

## 1. Hardware identifiers

### OBSBOT Tiny 2
- **USB VID**: `0x3564` (Remo Tech Co., Ltd.)
- **USB PID**: `0xfef8`

Source: confirmed in kernel patch tested on Tiny 2
(`Bus 008 Device 002: ID 3564:fef8 Remo Tech Co., Ltd. OBSBOT Tiny 2`).

### Other models (for reference, not primary target)
- **OBSBOT Tiny (original)**: VID `0x6e30`, PID `0xfef0`
- **OBSBOT Tiny 2 Lite**, **Meet 2**, **Meet SE**, etc.: TBD.

---

## 2. V4L2 standard controls

To be filled in T-003. Plan: run `v4l2-ctl --device=/dev/videoN --list-ctrls-menus`
and document the table.

Expected categories (per kernel uvcvideo support):
- Brightness, Contrast, Saturation, Hue, Sharpness, Gamma.
- White balance temperature (auto + manual).
- Power line frequency (50/60 Hz / disabled).
- Exposure absolute, exposure auto.
- Focus absolute, focus auto.
- Pan/Tilt absolute, Pan/Tilt speed.
- Zoom absolute, Zoom continuous.

The kernel patch we found (linuxtv-commits, December 2025) specifically
fixes Tiny 2 pan/tilt/zoom *speed* signed-value handling. Worth verifying
the kernel version on the user's machine is recent enough (Debian 13 has
kernel 6.12+, which should include the fix).

---

## 3. UVC Extension Units (XU)

To be discovered. The Linux uvcvideo driver enumerates XUs at probe time;
they appear in `lsusb -v` output and can be queried via
`UVCIOC_CTRL_QUERY` ioctl.

### Discovery workflow (T-003 and v0.4)

1. **Identify XU descriptors** from `lsusb -v`:
   - Look for blocks starting with `VideoControl Interface Descriptor:` and
     `bDescriptorSubtype 6 (EXTENSION_UNIT)`.
   - Record `bUnitID`, `guidExtensionCode`, `bNumControls`, `bmControls`.

2. **Capture OBSBOT Center traffic** against the Tiny 2:
   - Run OBSBOT Center in a Windows VM (KVM or VirtualBox).
   - Pass the camera through to the VM.
   - On the Linux host: `sudo modprobe usbmon` then capture in Wireshark
     filtering by USB device address.
   - Toggle each setting one at a time. Stop the capture, save as `.pcapng`.
   - Decode SET_CUR / GET_CUR requests addressed to XU unit IDs. The
     payload bytes are the protocol.

3. **Cross-reference** with existing open-source work:
   - `taxfromdk/obsbot_tiny_reversing` — earlier model, similar bus design.
   - `samliddicott/meet4k` — Meet 4K Rust implementation, may share selectors.
   - `aaronsb/obsbot-camera-control` — uses libdev.so but the C++ caller
     code reveals which features map to which logical commands.

4. **Document** each XU control in this file under "Tiny 2 XU table" below.

### Tiny 2 XU table (to be filled)

| Unit ID | GUID | Selector | Size | Direction | Logical meaning | Source |
|---------|------|----------|------|-----------|-----------------|--------|
| TBD     | TBD  | TBD      | TBD  | GET/SET   | TBD             | TBD    |

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

Document any behavior that depends on firmware version of the connected
device:

- Tiny 2 firmware version readable via TBD.
- Known firmware-related quirks: TBD.

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
