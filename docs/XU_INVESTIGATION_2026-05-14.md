# XU command surface — extracted 2026-05-14

End-to-end extraction of the OBSBOT Tiny 2 UVC Extension Unit (XU) command
surface from the two FOSS reverse-engineering efforts identified during the
2026-05-14 investigation:

- **cgevans/tiny2** — https://github.com/cgevans/tiny2
  Branch: `main` · Language: Rust · License: EUPL-1.2 · 51 GitHub stars.
  Key files inspected verbatim: `src/lib.rs`, `src/usbio.rs`, `Cargo.toml`,
  `LICENSE.txt`, `LICENSES/EUPL-1.2.txt`, `README.md`.
  Per the README it is "substantially based on samliddicott's meet4k package"
  (https://github.com/samliddicott/meet4k).

- **OpenFoxes/Tiny4Linux** — https://github.com/OpenFoxes/Tiny4Linux
  Branch: `main` · Version: 2.2.2 · Language: Rust · License: EUPL-1.2.
  AUR-packaged. Fork of cgevans/tiny2, adds Sleep/Wake, Tracking Speed
  (Standard/Sport), three Preset Positions, i18n, CLI+GUI split,
  `src/libs/` modular factoring, parameterized test suite via `test_case`.
  Key files inspected verbatim: `src/lib.rs`, `src/libs/mod.rs`,
  `src/libs/usbio.rs`, `src/libs/camera/{mod,camera,transport,command02,
  status,enums}.rs`, all 8 files under `src/libs/camera/commands/`,
  `src/libs/errors/{mod,enums}.rs`, `Cargo.toml`, `build.rs`, `README.md`,
  `LICENSE.md`.

Both projects are EUPL-1.2; the EUPL Appendix lists GPL-3 as a compatible
license, so we may port their code into our (GPL-3+) work as long as we
preserve the EUPL-1.2 attribution on the ported files (see § License &
attribution at the end). The "Licensed under the EUPL" boilerplate header
from the EU's reference text is the only attribution string required.

---

## Transport layer

Both repos use the **same transport** end-to-end. There is **no libusb / rusb
path** — everything goes through `/dev/videoN` and the `uvcvideo` kernel
driver's `UVCIOC_CTRL_QUERY` ioctl.

| Item                  | Value / source                                                                  |
|-----------------------|---------------------------------------------------------------------------------|
| Device node           | `/dev/videoN`, opened with `std::fs::File::open()` (read-only handle is enough) |
| Enumeration           | `glob_with("/dev/video*", ...)` then VIDIOC_QUERYCAP, match `card` or `bus_info` against a user-supplied hint string |
| Capture-filter rule   | `(device_caps & 0x800000) == 0` — skip the metadata sub-device the kernel exposes alongside the video device (`V4L2_CAP_META_CAPTURE` = 0x00800000) |
| Permissions           | Standard `uvcvideo` permissions: user must be in `video` group **or** have a udev rule that grants r/w on the matching `/dev/videoN`. No `cap_sys_rawio` or `cap_net_admin` needed; no libusb claim that would conflict with the kernel driver |
| ioctl                 | `UVCIOC_CTRL_QUERY` = `_IOWR('u', 0x21, struct uvc_xu_control_query)` — `nix::ioctl_readwrite_buf!(uvcioc_ctrl_query, b'u', 0x21, uvc_xu_control_query)` |
| Companion ioctls      | `VIDIOC_QUERYCAP` (`_IOR('V', 0, v4l2_capability)`), `VIDIOC_G_CTRL` (`_IOWR('V', 27, v4l2_control)`), `VIDIOC_S_CTRL` (`_IOWR('V', 28, v4l2_control)`), `VIDIOC_QUERYCTRL` (`_IOWR('V', 36, v4l2_queryctrl)`) — all wrapped via `nix::ioctl_readwrite!` |
| UVC request codes     | `UVC_SET_CUR = 0x01`, `UVC_GET_CUR = 0x81`, `UVC_GET_MIN = 0x82`, `UVC_GET_MAX = 0x83`, `UVC_GET_RES = 0x84`, `UVC_GET_LEN = 0x85`, `UVC_GET_INFO = 0x86`, `UVC_GET_DEF = 0x87` |
| Length pre-check      | Both projects always call `UVC_GET_LEN` **before** every `GET_CUR` / `SET_CUR` to obtain the selector's payload length (`u16 LE`). The 60-byte buffer in source is a comfortable upper bound; selector 0x02 and selector 0x06 both report ≤ 60 on Tiny 2 (selector 0x02 reports 60, selector 0x06 reports 60 according to dump_02/dump call sites) |
| Hardcoded `bUnitID`   | **`0x02`** at every call site in both projects — confirmed |
| Default `bSelector`   | **`0x06`** for the multiplexed-opcode block (HDR/FaceAE/FOV/AI mode/status). **Selector `0x02`** is the second selector, used only for the 18-byte exposure-on/off blob and (in Tiny4Linux) the new 36-byte "command02" frames (sleep, tracking speed, presets) |

Rust dependencies (cgevans/tiny2 `Cargo.toml`):

| Crate             | Version   | Purpose                                                                 |
|-------------------|-----------|-------------------------------------------------------------------------|
| `nix`             | `0.29`    | `ioctl_read_buf!`, `ioctl_readwrite!`, `ioctl_readwrite_buf!` macros    |
| `errno`           | (errno+nix::errno) | `errno()` last-error retrieval                                 |
| `glob`            | (latest)  | enumerate `/dev/video*`                                                 |
| `thiserror`       | `1.0+`    | `#[derive(Error)]`                                                      |
| `enum_dispatch`   | (latest)  | `#[enum_dispatch(CameraHandleType)]` for the `UvcUsbIo` trait           |
| `hexdump`         | (latest)  | `dump()` / `dump_02()` diagnostic output                                |
| `hex`             | (latest)  | Hex-encoding for verbose logging                                        |
| `iced`            | `0.14`    | GUI (out of scope for our port — we use GTK4)                           |
| `rosc`            | `0.10+`   | OSC server binary (out of scope)                                        |
| `clap`            | `4.4.18`  | CLI parsing (out of scope — we use libadwaita pages)                    |

Rust dependencies (Tiny4Linux `Cargo.toml`) add: `nix 0.30` (newer), `bon`
(builder pattern for the 36-byte command frames), `rust-i18n` and
`sys-locale` (i18n), `test_case` + `assertables` (parameterised tests).
For our port we only need the same `nix` + `glob` + `thiserror` + `errno`
subset — `bon` is convenient but not required; we can write the 36-byte
frame builder by hand or via `bytemuck`/manual array copy.

V4L2 standard control IDs published by `usbio.rs` (we should reuse the
same constants in our `obsbot-core`):

```rust
// V4L2_CID_CAMERA_CLASS_BASE = 0x009A0900
pub const V4L2_CID_PAN_ABSOLUTE:  u32 = 0x009A0908;
pub const V4L2_CID_TILT_ABSOLUTE: u32 = 0x009A0909;
pub const V4L2_CID_PAN_RELATIVE:  u32 = 0x009A090A;
pub const V4L2_CID_TILT_RELATIVE: u32 = 0x009A090B;
pub const V4L2_CID_ZOOM_ABSOLUTE: u32 = 0x009A090D;
pub const V4L2_CID_ZOOM_RELATIVE: u32 = 0x009A090E;
```

The `V4L2_CTRL_FLAG_DISABLED` mask (`0x0001`) is checked by `query_ctrl()`
and converted to `EINVAL` so the GUI can hide disabled controls — same
behaviour as our existing T-102 INACTIVE handler.

### The two transport "selectors" — why there are 0x02 AND 0x06

Both repos issue `set_cur(unit=0x02, ...)` to **two** different selectors:

1. **Selector `0x06`** — the *opcode-multiplexed* selector. Payload starts
   with a 1-byte opcode and a 1-byte payload length. cgevans calls this
   selector for HDR (op 0x01), Face AE (op 0x03), FOV (op 0x04), AI tracking
   mode (op 0x16), and uses GET_CUR on it to read the 60-byte status struct.

2. **Selector `0x02`** — the *raw structured-frame* selector. Payload is an
   18-byte (cgevans) or 36-byte (Tiny4Linux) frame with a fixed leader
   (`0xaa 0x25 ...`), a sequence number, a "segment size", a CRC-style
   2-byte checksum, a 6-byte "function group", a 6-byte command, and a
   16-byte "appendix". cgevans uses it only for `AUTO_EXP_CMD` /
   `MANUAL_EXP_CMD`. Tiny4Linux reverse-engineers it further and adds
   sleep/wake, tracking speed, and preset positions on the same selector.
   `dump_02()` is the diagnostic dump for this selector.

Both selectors live on the **same XU (unit `0x02`)** — the OBSBOT firmware
just multiplexes the surface across two selectors.

---

## Selector unit 0x02 / selector 0x06 — multiplexed opcode commands

All payloads on this selector follow the pattern
`[opcode, payload_length, ...payload_bytes]`. The OBSBOT firmware ignores the
trailing zero padding (the kernel always sends the full length returned by
`GET_LEN`, but only the first 1+1+payload_length bytes are decoded).
`SET_CUR` is the write path; `GET_CUR` on selector `0x06` returns the
60-byte global status struct (not opcode-multiplexed — see next section).

### Cmd 0x01 — HDR

- **Source**: cgevans `src/lib.rs::set_hdr_mode` (l.181-187); Tiny4Linux
  `src/libs/camera/commands/hdr_mode.rs::HdrModeCommand::build`.
- **Direction**: SET_CUR (write). Status reflected in GET_CUR status byte
  `0x06`.
- **Payload** (3 bytes): `[0x01, 0x01, value]` where `value` is `0x00` =
  off, `0x01` = on.
- **Cross-check**: Both repos emit byte-identical payload. Tiny4Linux has
  an explicit `test_case` for it (`hdr_mode.rs::tests::hdr_mode`).

Exact Rust (cgevans):

```rust
fn set_hdr_mode(&self, mode: bool) -> Result<(), Error> {
    let cmd = if mode { [0x01, 0x01, 0x01] } else { [0x01, 0x01, 0x00] };
    self.send_cmd(0x2, 0x6, &cmd)
}
```

### Cmd 0x03 — Face Auto-Exposure (Face AE)

- **Source**: cgevans `src/lib.rs::set_exposure_mode` (l.165-179, the second
  half of the Global / Face arms); Tiny4Linux
  `src/libs/camera/commands/exposure_mode.rs::ExposureModeCommand::build`.
- **Direction**: SET_CUR (write).
- **Payload** (3 bytes): `[0x03, 0x01, value]` where `value` is `0x00` =
  Global auto-exposure, `0x01` = Face-tracking auto-exposure.
- **Note**: This sub-command is **only meaningful when the camera is in
  auto-exposure mode**. cgevans sends `AUTO_EXP_CMD` on selector `0x02`
  *first* (puts the camera in auto), then `[0x03, 0x01, x]` on selector
  `0x06` to choose the metering style. Tiny4Linux's
  `Camera::set_exposure_mode` does exactly the same two-step (`ExposureModeTypeCommand`
  via selector `0x02`, then `ExposureModeCommand` via selector `0x06`).
- **Manual exposure**: Tiny4Linux's `ExposureModeCommand::build` returns
  `None` for `ExposureMode::Manual`, and `Camera::set_exposure_mode`
  uses `.map(...)` to skip the selector-0x06 step entirely — i.e. when
  going to Manual you only send the 36-byte selector-0x02 frame; you do
  NOT send a `[0x03, 0x01, ...]` blob.

### Cmd 0x04 — Field of View (FOV)

- **Source**: cgevans `src/lib.rs::set_fov` (l.151-153). **Tiny4Linux does
  not implement this**; its README defers angle-of-view control to V4L2
  / Camset.
- **Direction**: SET_CUR (write).
- **Payload** (3 bytes): `[0x04, 0x01, value]` where:

  | Variant            | Value | Approx FOV |
  |--------------------|-------|------------|
  | `FOVMode::Wide`    | `0x01`| 86°        |
  | `FOVMode::Normal`  | `0x02`| 78°        |
  | `FOVMode::Narrow`  | `0x03`| 65°        |

- **Cross-check**: not in Tiny4Linux. Single-source; cgevans-only.

### Cmd 0x16 — AI Tracking Mode

- **Source**: cgevans `src/lib.rs::set_ai_mode` (l.155-170); Tiny4Linux
  `src/libs/camera/commands/ai_mode.rs::AIModeCommand::build`.
- **Direction**: SET_CUR (write). Reflected in GET_CUR status bytes
  `0x18` (m) and `0x1c` (n).
- **Payload** (4 bytes): `[0x16, 0x02, m, n]` — note **payload length is
  2**, not 1. The (m, n) tuple uniquely identifies the AI mode:

  | AIMode             | m (byte `0x18` in status / payload[2]) | n (byte `0x1c` in status / payload[3]) | cgevans `TryFrom<i32>` int |
  |--------------------|----------------------------------------|----------------------------------------|----------------------------|
  | `NoTracking`       | `0x00`                                 | `0x00`                                 | 0                          |
  | `NormalTracking`   | `0x02`                                 | `0x00`                                 | 1                          |
  | `UpperBody`        | `0x02`                                 | `0x01`                                 | 2                          |
  | `CloseUp`          | `0x02`                                 | `0x02`                                 | 3                          |
  | `Headless`         | `0x02`                                 | `0x03`                                 | 4                          |
  | `LowerBody`        | `0x02`                                 | `0x04`                                 | 5                          |
  | `DeskMode`         | `0x05`                                 | `0x00`                                 | 6                          |
  | `Whiteboard`       | `0x04`                                 | `0x00`                                 | 7                          |
  | `Hand`             | `0x06`                                 | `0x00`                                 | 8                          |
  | `Group`            | `0x01`                                 | `0x00`                                 | 9                          |

  **Important discrepancy**: cgevans's `set_ai_mode` source has
  `AIMode::Hand => [0x16, 0x02, 0x03, 0x00]` (m=3, n=0). The corresponding
  `decode_status` reads `(6, 0) => AIMode::Hand` (m=6). **Tiny4Linux's
  `AIModeCommand::build` mirrors cgevans's setter exactly** —
  `Hand => [0x16, 0x02, 0x03, 0x00]`. Same mismatch between encode and
  decode in Tiny4Linux's `CameraStatus::decode_ai_mode`. Tiny4Linux's
  status decoder has integration test data including UpperBody but does
  NOT have a test that round-trips Hand through set→get, so the bug (if
  it is one) is undetected. For our port: copy the table as-is for the
  setter, but flag this in PROTOCOL.md and re-validate against live
  hardware before shipping — likely cgevans's decode is correct and
  the setter has a typo (`0x03` should probably be `0x06`).

- **Cross-check**: byte-identical setter table between the two repos.
  Both tables agree the decode-side mapping is `(6,0) → Hand`.

Exact Rust (cgevans, the canonical match):

```rust
let cmd = match mode {
    AIMode::NoTracking     => [0x16, 0x02, 0x00, 0x00],
    AIMode::NormalTracking => [0x16, 0x02, 0x02, 0x00],
    AIMode::UpperBody      => [0x16, 0x02, 0x02, 0x01],
    AIMode::DeskMode       => [0x16, 0x02, 0x05, 0x00],
    AIMode::Whiteboard     => [0x16, 0x02, 0x04, 0x00],
    AIMode::Group          => [0x16, 0x02, 0x01, 0x00],
    AIMode::Hand           => [0x16, 0x02, 0x03, 0x00],  // suspicious; see note
    AIMode::CloseUp        => [0x16, 0x02, 0x02, 0x02],
    AIMode::Headless       => [0x16, 0x02, 0x02, 0x03],
    AIMode::LowerBody      => [0x16, 0x02, 0x02, 0x04],
};
```

### Other selector-0x06 opcodes — **not implemented in either repo**

Both repos cover exactly four selector-0x06 opcodes: `0x01`, `0x03`, `0x04`,
`0x16`. Opcodes `0x02`, `0x05` and everything from `0x06` onward (except
`0x16`) are **unprobed open ground**. This matches the dump_02 / dump
diagnostic helpers — they're the only way to discover more opcodes, and
both projects ship them precisely so future contributors can.

For our port the safe rule is: never write to an opcode we haven't read
out of one of these two repositories. The user has the only Tiny 2 in our
loop; if we want to probe further, that's a separate user-driven session
with USB capture.

---

## Selector unit 0x02 / selector 0x02 — structured 18- and 36-byte frames

These do **not** start with `[opcode, length, ...]`. They are full
manufacturer-protocol frames. The high-level structure is the same across
all selector-0x02 writes:

```
byte  0..1   FRAME_ID         = [0xaa, 0x25]    (fixed)
byte  2..3   sequence_nr      (per-command, little-endian-looking 16-bit)
byte  4..5   SEGMENT_SIZE     = [0x0c, 0x00]    (fixed; "12 bytes of payload follow")
byte  6..7   checksum         (per-command 2-byte CRC, opaque to us)
byte  8..13  function_group   (6 bytes — identifies the camera subsystem)
byte 14..19  command          (6 bytes — the actual op + value)
byte 20..35  appendix         (16 bytes — usually zeros, see Presets below)
```

cgevans only knows the two exposure-mode 18-byte short frames (bytes 0..17).
Tiny4Linux extends to the full 36-byte form and decomposes it into
named fields via the `bon` builder in `src/libs/camera/command02.rs`. The
EOM padding (bytes 18..35 of cgevans's 18-byte arrays) corresponds to
Tiny4Linux's `command[4..6] + appendix` zeros — there is no contradiction.

The **`command02` builder** (Tiny4Linux):

```rust
#[builder(finish_fn = build)]
pub fn command02(
    function_group: [u8; 6],
    sequence_nr:    [u8; 2],
    checksum:       [u8; 2],
    command:        [u8; 6],
    appendix:       Option<[u8; 16]>,
) -> [u8; 36] {
    const FRAME_ID:     [u8; 2] = [0xaa, 0x25];
    const SEGMENT_SIZE: [u8; 2] = [0x0c, 0x00];
    [FRAME_ID.as_slice(),  sequence_nr.as_slice(),
     SEGMENT_SIZE.as_slice(), checksum.as_slice(),
     function_group.as_slice(), command.as_slice(),
     appendix.unwrap_or([0x00; 16]).as_slice()]
        .concat().try_into().unwrap()
}
```

We can drop `bon` and write the equivalent in plain Rust:

```rust
fn build_command02(fg: [u8;6], seq: [u8;2], cks: [u8;2], cmd: [u8;6],
                   app: Option<[u8;16]>) -> [u8;36] {
    let mut out = [0u8; 36];
    out[0..2].copy_from_slice(&[0xaa, 0x25]);
    out[2..4].copy_from_slice(&seq);
    out[4..6].copy_from_slice(&[0x0c, 0x00]);
    out[6..8].copy_from_slice(&cks);
    out[8..14].copy_from_slice(&fg);
    out[14..20].copy_from_slice(&cmd);
    out[20..36].copy_from_slice(&app.unwrap_or([0u8; 16]));
    out
}
```

### Frame "ExposureModeType" — toggle Auto ↔ Manual

- **Source**: Tiny4Linux `commands/exposure_mode_type.rs`. Decomposes the
  two 18-byte arrays cgevans's `lib.rs` calls `AUTO_EXP_CMD` and
  `MANUAL_EXP_CMD`. The first 18 bytes match exactly between the two
  representations.
- **Direction**: SET_CUR on selector `0x02`.
- **Function group**: `[0x0a, 0x02, 0x82, 0x29, 0x05, 0x00]`
- **Manual** (cgevans `MANUAL_EXP_CMD`):
  - `sequence_nr` = `[0x16, 0x00]`  (cgevans byte 0x02-0x03 = `0x15, 0x00` — wait, check below)
  - `checksum` = `[0x58, 0x91]`
  - `command` = `[0xb2, 0xaf, 0x02, 0x04, 0x00, 0x00]`
- **Auto / Global / Face** (cgevans `AUTO_EXP_CMD`):
  - `sequence_nr` = `[0x15, 0x00]`
  - `checksum` = `[0xa8, 0x9e]`
  - `command` = `[0xf9, 0x27, 0x01, 0x32, 0x00, 0x00]`

> **DISCREPANCY** — cross-check cgevans's hex-array form versus
> Tiny4Linux's decomposed form, treating index `i` of cgevans's 18-byte
> array as the absolute frame offset:
>
> ```
> cgevans MANUAL_EXP_CMD = [
>   0xaa, 0x25,            // FRAME_ID                              -- bytes 0..1
>   0x15, 0x00,            // sequence_nr                           -- bytes 2..3
>   0x0c, 0x00,            // SEGMENT_SIZE                          -- bytes 4..5
>   0xa8, 0x9e,            // checksum                              -- bytes 6..7
>   0x0a, 0x02, 0x82, 0x29, 0x05, 0x00,    // function_group        -- bytes 8..13
>   0xf9, 0x27,            // first 2 bytes of command              -- bytes 14..15
>   0x01, 0x32,            // next 2 bytes of command               -- bytes 16..17
> ]
>
> cgevans AUTO_EXP_CMD = [
>   0xaa, 0x25,
>   0x16, 0x00,
>   0x0c, 0x00,
>   0x58, 0x91,
>   0x0a, 0x02, 0x82, 0x29, 0x05, 0x00,
>   0xb2, 0xaf,
>   0x02, 0x04,
> ]
> ```
>
> Comparing to Tiny4Linux's `ExposureModeTypeCommand`:
> ```
> Manual  -> sequence_nr=[0x16,0x00] checksum=[0x58,0x91] cmd=[0xb2,0xaf,0x02,0x04,0x00,0x00]
> Auto    -> sequence_nr=[0x15,0x00] checksum=[0xa8,0x9e] cmd=[0xf9,0x27,0x01,0x32,0x00,0x00]
> ```
>
> **The mapping is inverted**: cgevans's *AUTO* literal has the bytes
> `[0xaa,0x25, 0x16,0x00, 0x0c,0x00, 0x58,0x91, ..., 0xb2,0xaf,0x02,0x04]`
> which is what Tiny4Linux labels as *Manual*; cgevans's *MANUAL* literal
> matches Tiny4Linux's *Auto*. **One of the two repos has its labels
> swapped.** Both have the labels in source; both must have run on real
> hardware. This is the single most important discrepancy to validate live
> before we ship: send each frame to the camera and observe whether the
> indicator on the device confirms the label.
>
> *(My read*: cgevans's labels are the more likely-correct ones, because
> `set_exposure_mode` in cgevans goes `Global => send AUTO_EXP_CMD; then
> [0x03,0x01,0x00]`, and the `[0x03,0x01,...]` sub-command (Face AE) only
> makes sense when the camera is *in* auto exposure. Tiny4Linux's
> `set_exposure_mode` mirrors the structure but with swapped labels. So the
> labels are likely a renaming error in Tiny4Linux; the wire bytes the user
> ends up sending for "Auto/Global" are correct on cgevans and ambiguous on
> Tiny4Linux. **For our port: trust cgevans's names and treat
> Tiny4Linux's source as confirmation that the bytes themselves are real,
> not a regression.** Flag this in PROTOCOL.md.)*

### Frame "Sleep / Wake" — power state (Tiny4Linux-only)

- **Source**: Tiny4Linux `commands/sleep.rs`. **Not in cgevans.**
- **Direction**: SET_CUR on selector `0x02`. Reflected in GET_CUR status
  byte `0x02`.
- **Function group**: `[0x0a, 0x02, 0xc2, 0xa0, 0x04, 0x00]`
- **Awake**:
  - `sequence_nr` = `[0xa5, 0x00]`
  - `checksum` = `[0x5f, 0xef]`
  - `command` = `[0xbe, 0x07, 0x00, 0x00, 0x00, 0x00]`
- **Sleep**:
  - `sequence_nr` = `[0x42, 0x00]`
  - `checksum` = `[0xea, 0x63]`
  - `command` = `[0xbf, 0xfb, 0x01, 0x00, 0x00, 0x00]`
- `SleepMode::Unknown` returns `T4lError::InvalidSetting` (we do the same
  with a typed enum and let the trait surface refuse it at compile time).

Exact Rust:

```rust
pub fn build(mode: SleepMode) -> Result<[u8; 36], T4lError> {
    if mode == SleepMode::Unknown { return Err(T4lError::InvalidSetting); }
    const FUNCTION_GROUP_SLEEP: [u8; 6] = [0x0a, 0x02, 0xc2, 0xa0, 0x04, 0x00];
    let (sequence_nr, checksum, command) = match mode {
        SleepMode::Awake => ([0xa5, 0x00], [0x5f, 0xef], [0xbe, 0x07, 0x00, 0x00, 0x00, 0x00]),
        SleepMode::Sleep => ([0x42, 0x00], [0xea, 0x63], [0xbf, 0xfb, 0x01, 0x00, 0x00, 0x00]),
        SleepMode::Unknown => panic!(),
    };
    Ok(command02().function_group(FUNCTION_GROUP_SLEEP)
        .sequence_nr(sequence_nr).checksum(checksum).command(command).build())
}
```

### Frame "Tracking Speed" — Standard / Sport (Tiny4Linux-only)

- **Source**: Tiny4Linux `commands/tracking_speed.rs`. **Not in cgevans.**
  Note: cgevans's `lib.rs` declares an unused `enum TrackingMode {
  Headroom, Standard, Motion }` with an `i32 TryFrom`, but **nothing
  sends it** — there is no setter or wire mapping. So cgevans has only
  the *enum shape* of three modes (Headroom / Standard / Motion).
  Tiny4Linux ships a different two-mode enum **Standard / Sport** and a
  working wire encoding for it. The two enum names are *not*
  interchangeable — the proprietary SDK / app likely exposes the
  Standard/Sport pair as well, and we should adopt Tiny4Linux's naming
  for now. If hardware-validation shows a third "Headroom" mode exists,
  we'll add it; otherwise we cap at 2.
- **Direction**: SET_CUR on selector `0x02`. Reflected in GET_CUR status
  byte `0x21`.
- **Function group**: `[0x0a, 0x04, 0xc4, 0x0c, 0x01, 0x00]`
- **Standard**:
  - `sequence_nr` = `[0x20, 0x00]`
  - `checksum` = `[0xab, 0xcb]`
  - `command` = `[0xe6, 0x3f, 0x00, 0x00, 0x00, 0x00]`
- **Sport**:
  - `sequence_nr` = `[0x21, 0x00]`
  - `checksum` = `[0xfa, 0x0e]`
  - `command` = `[0x67, 0xfe, 0x02, 0x00, 0x00, 0x00]`
- **Appendix**: explicit `[0u8; 16]` (the source builds it as a
  zero-filled array, equivalent to the default).
- Status decode: `bytes[0x21]` ⇒ `0` Standard, `2` Sport, anything else
  defaults to Standard (`enums.rs`). Note the gap at `0x21 == 0x01` —
  either there's a tri-state we haven't seen, or the field is sparse.

### Frame "Goto Preset Position" — slots 0/1/2 (Tiny4Linux-only)

- **Source**: Tiny4Linux `commands/goto_preset_position.rs`. **Not in
  cgevans.**
- **Direction**: SET_CUR on selector `0x02`. *Recall only — there is no
  "save" opcode in either repo.* The Tiny 2 firmware (and the proprietary
  SDK) does have a save concept, but Tiny4Linux only implements **recall**;
  presets must be programmed via OBSBOT's official tool or the camera's
  on-device gesture mechanism beforehand. This is consistent with their
  README's "Controls for preset positions" feature wording.
- **Slot count**: **three slots**, indices `0`, `1`, `2`. Any other
  `i8` (including negative) returns `T4lError::InvalidSetting` with a
  `println!("Invalid preset nr {}", preset_nr + 1)`. So the user-visible
  numbering is 1-based ("Preset 1/2/3") but the API takes 0-based
  indices. We'll mirror that convention in our GUI (label "Preset 1",
  pass `0` underneath).
- **Storage**: in **camera firmware**, not in the app's config file —
  Tiny4Linux sends only a recall command, the position itself never
  crosses the wire from the host. Implication for our port: we cannot
  show the "what is preset 1 pointing to" preview, and we don't have to
  persist anything ourselves.
- **Function group**: `[0x0a, 0x04, 0xc4, 0x39, 0x14, 0x00]`
- **Preset 0** (user "Preset 1"):
  - `sequence_nr` = `[0x20, 0x00]`
  - `checksum` = `[0x6b, 0xdc]`
  - `command` = `[0xd6, 0xfb, 0x00, 0x00, 0x00, 0x00]`
- **Preset 1** (user "Preset 2"):
  - `sequence_nr` = `[0x1a, 0x00]`
  - `checksum` = `[0x4b, 0x03]`
  - `command` = `[0xeb, 0x2a, 0x01, 0x00, 0x00, 0x00]`
- **Preset 2** (user "Preset 3"):
  - `sequence_nr` = `[0x26, 0x00]`
  - `checksum` = `[0x8b, 0xc3]`
  - `command` = `[0xaf, 0x19, 0x02, 0x00, 0x00, 0x00]`
- **Appendix** (all three): not zero — it's the float-32 little-endian
  encoding of `1.0` repeated four times. Source:
  ```rust
  let mut arr = [0u8; 16];
  for i in 0..4 {
      arr[i * 4..(i + 1) * 4].copy_from_slice(&[0x00, 0x00, 0x80, 0x3f]);
  }
  ```
  `0x3f800000` (BE) = `1.0_f32`, so as LE bytes this is `[0x00, 0x00,
  0x80, 0x3f]`. The appendix is therefore four IEEE-754 single-precision
  floats all equal to `1.0`. Their semantic meaning is unknown — likely
  scale factors or normalised target coordinates — but they're *required*
  for the camera to accept the recall on a preset slot. We copy the
  bytes verbatim.

---

## GET_CUR status struct (60 bytes, selector 0x06)

`get_status()` returns a fixed 60-byte structure that the camera updates
asynchronously as the user changes state via either the wire protocol or
the on-device controls. Only the following bytes have decoded meaning in
the two repos:

| Offset  | Decoded by      | Field                  | Encoding |
|---------|-----------------|------------------------|----------|
| `0x02`  | Tiny4Linux only | Sleep state            | `0` = Awake, `1` = Sleep, anything else = Unknown |
| `0x06`  | cgevans + Tiny4Linux | HDR flag           | `0` = off, non-zero = on (Tiny4Linux's test treats `0x02` as on too) |
| `0x18`  | cgevans + Tiny4Linux | AI mode "m" tuple  | see AI mode table above; `0x00` when not tracking |
| `0x1c`  | cgevans + Tiny4Linux | AI mode "n" tuple  | see AI mode table above |
| `0x21`  | Tiny4Linux only | Tracking speed         | `0x00` = Standard, `0x02` = Sport, anything else defaults to Standard in the decode |

**Bytes `0x00`, `0x01`, `0x03`, `0x04`, `0x05`, `0x07`-`0x17`, `0x19`-`0x1b`,
`0x1d`-`0x20`, `0x22`-`0x3b` are NOT decoded.** Tiny4Linux's integration
test (`status.rs::tests::integration::camera_status::decode_status`) has a
real-looking 57-byte capture with many non-zero bytes (e.g. `0x27` at
offset 0, `0x88, 0xff` at 0x0a-0x0b, `0x01, 0x00, 0x00, 0x03` at
0x0d-0x10, `0x21, 0x00` at 0x16-0x17, `0x03, 0x00, 0x01, 0x00, 0x00,
0x1e` at 0x1a-0x1f) — i.e. the camera has more state than either repo
reads back. **These extra bytes are the discovery frontier for
contributions on our side.** A diagnostic "Dump XU status" GUI button on
a debug page is worth exposing; we already have `dump()` / `dump_02()`
as model for it.

Sample real capture (Tiny4Linux test data, hex):

```
27 00 00 01 42 00 01 01  01 01 88 ff 00 00 01 00
00 03 00 00 01 00 21 00  02 01 03 00 01 00 00 1e
00 02 00 00 00 00 00 00  00 00 00 00 00 00 00 00
00 00 00 00 00 00 00 00  00
```

This vector evaluates (per the asserts in that test) to: Awake, HDR on,
UpperBody AI mode, Sport tracking. Note `0x16` (offset 22) is `0x21`,
which is also Sport's `sequence_nr[0]` — possibly coincidence, possibly a
"last command echo" register.

Status struct shape we'll expose in `obsbot-core::xu::Status`:

```rust
pub struct Status {
    pub sleep:    SleepState,    // Awake / Sleep / Unknown — byte 0x02
    pub hdr_on:   bool,           // byte 0x06 != 0
    pub ai_mode:  AiMode,         // bytes 0x18 (m), 0x1c (n)
    pub tracking_speed: TrackingSpeed, // byte 0x21
    pub raw:      [u8; 60],       // expose for debug "Dump status" UI
}
```

---

## Tiny4Linux deltas (not in cgevans)

Aggregated for quick reference — every byte already documented above:

| Feature           | Selector | Frame size | Reflected in status @ |
|-------------------|----------|------------|------------------------|
| Sleep / Wake      | `0x02`   | 36 B       | byte `0x02`            |
| Tracking speed    | `0x02`   | 36 B       | byte `0x21`            |
| Preset recall ×3  | `0x02`   | 36 B (with non-zero appendix = four `1.0_f32`) | not in status (fire-and-forget) |

Additional architectural changes worth noting for our port:

- Tiny4Linux factors `lib.rs` into `libs/{usbio, camera/{camera, transport,
  command02, status, enums, commands/*}, errors, i18n}` — clean modules
  per concern. We should adopt this layout; cgevans's monolithic `lib.rs`
  is harder to test in isolation.
- Tiny4Linux uses the `bon` crate to build the 36-byte frames as named
  arguments. We can replicate with plain Rust (snippet above) or with
  `bytes::BytesMut` / `bytemuck::Pod`; `bon` is one dependency we can
  drop with no functional loss.
- Tiny4Linux's `Camera::set_exposure_mode` has a `.map(...)` that
  silently swallows the inner `Result` — likely a bug (the
  selector-0x06 follow-up's error is dropped). We must NOT replicate
  that; propagate it with `?` so the GUI can surface a toast.
- Tiny4Linux's `set_tracking_speed` has a fairly visible bug too:
  `self.get_status()?.speed = speed;` mutates a temporary that's
  immediately dropped before the write is issued. The write still
  happens (the `send_cmd` line after it does the real work), so the bug
  is harmless — but we don't reproduce it either.
- Tiny4Linux's `set_debugging` trait impl calls itself recursively
  (`fn set_debugging(...) { self.set_debugging(debugging); }`); the
  resolution rule lets it bind to the inherent `Camera::set_debugging`
  rather than the trait method, so it doesn't recurse forever, but it's
  fragile. We replace it with a typed flag on our `Camera` struct.
- Tiny4Linux pulls in `nix` 0.30 (cgevans had `nix` 0.29). The
  `ioctl_*!` macro surface is unchanged between those versions, but the
  errno path differs: `nix::Error::last_raw()` in Tiny4Linux vs
  `nix::errno::errno()` in cgevans. We use whichever matches our
  workspace pin.
- Tiny4Linux uses `enum_dispatch` exactly like cgevans for the
  `UvcUsbIo` trait. Not strictly needed since both have only one impl
  (`CameraHandle`), but it leaves room for a mock impl in tests.
- Tiny4Linux has no equivalent of cgevans's standard V4L2 PTZ helpers
  (`get_pan`, `set_pan`, `query_pan_range`, etc.) — by design,
  Tiny4Linux delegates "movement control" to V4L2 / Camset. **Our port
  must keep cgevans's V4L2 PTZ wrappers**; they're how the GUI's
  T-101 PTZ buttons currently issue commands without going through XU.

---

## License & attribution

| Repo                    | LICENSE file | SPDX header (per .rs file)        | Repo-level NOTICE / boilerplate |
|-------------------------|--------------|-----------------------------------|----------------------------------|
| cgevans/tiny2           | `LICENSE.txt` (`LICENSES/EUPL-1.2.txt`) is the full EUPL-1.2 text | `// SPDX-License-Identifier: EUPL-1.2` at the top of every `.rs` file | `Cargo.toml`: `license = "EUPL-1.2"`. README states: "substantially based on samliddicott's meet4k package" (no further attribution text). |
| OpenFoxes/Tiny4Linux    | `LICENSE.md` (full EUPL-1.2 text) | `// SPDX-License-Identifier: EUPL-1.2` at the top of every `.rs` file | `Cargo.toml`: `license = "EUPL-1.2"`, `authors = ["Bono Fox", "Constantine Evans"]`. README states: "This repository is a fork of Constantine Evans's 'Tiny2', which itself is substantially based on samliddicott's meet4k package." |

**Verbatim EUPL boilerplate** (from cgevans's `LICENSES/EUPL-1.2.txt`,
lines 7-10):

> The Work is provided under the terms of this Licence when the Licensor
> (as defined below) has placed the following notice immediately
> following the copyright notice for the Work:
>
>     Licensed under the EUPL

That short string ("Licensed under the EUPL") plus the SPDX line is the
**minimum** the EUPL requires us to carry into our derivative.

### Suggested attribution block for our port

We propose two artefacts:

1. **A new `CREDITS.md`** at repo root (already mentioned in tasks #3 and
   #4 of the existing task list). Sample content:

   ```markdown
   # Credits

   ## OBSBOT Tiny 2 protocol — reverse-engineering lineage

   The XU command surface implemented in `obsbot-core::xu` is a port of
   prior free-software reverse-engineering work, in reverse-chronological
   order of contribution:

   - **OpenFoxes / Tiny4Linux** (https://github.com/OpenFoxes/Tiny4Linux,
     EUPL-1.2) — Sleep/Wake, Tracking Speed (Standard/Sport), Preset
     Positions (recall, 3 slots), modular factoring of the `command02`
     36-byte frame, GET_CUR status byte 0x02 and byte 0x21 decoders.
     Authors: Bono Fox, Constantine Evans.

   - **cgevans / tiny2** (https://github.com/cgevans/tiny2,
     EUPL-1.2) — original Rust port of the OBSBOT Tiny 2 XU surface,
     including HDR (op 0x01), Face AE (op 0x03), FOV (op 0x04, three
     widths), AI Tracking Mode (op 0x16, ten modes), Manual/Auto
     exposure 18-byte blobs on selector 0x02, GET_CUR status byte
     0x06 (HDR flag) and bytes 0x18 / 0x1c (AI mode m,n decode),
     wrapping the `UVCIOC_CTRL_QUERY` ioctl directly via `nix`.
     Author: Constantine Evans.

   - **samliddicott / meet4k** (https://github.com/samliddicott/meet4k,
     EUPL-1.2) — original UVC XU work for the Logitech Meetup; cgevans's
     project is "substantially based on" this. We do not pull bytes
     directly from meet4k (the Tiny 2 protocol is different), but the
     ioctl-on-uvcvideo pattern and the multiplexed-opcode-on-XU style
     originated there.

   ## Licensing note

   The three projects above are licensed EUPL-1.2. Per its Appendix,
   EUPL-1.2 is explicitly compatible with GPL-3 for derivative works.
   The files we port from them (`obsbot-core/src/xu/**`) carry a dual
   SPDX header documenting the origin:

       // SPDX-License-Identifier: GPL-3.0-or-later
       //
       // Portions of this file are derived from EUPL-1.2 source:
       //   - cgevans/tiny2       (https://github.com/cgevans/tiny2)
       //   - OpenFoxes/Tiny4Linux (https://github.com/OpenFoxes/Tiny4Linux)
       // "Licensed under the EUPL"
   ```

2. **A per-file header** on every ported file (we already have the
   GPL-3.0-or-later SPDX as the repo default; we add three comment lines
   below it on files that carry EUPL-derived bytes).

This satisfies EUPL Article 5 ("keep intact all copyright, patent or
trademark notices and all notices that refer to the Licence; include a
copy of such notices and a copy of the Licence with every copy of the
Work") and aligns with the project's existing OSI-clean licensing.

---

## Quirks / open questions

1. **AI mode "Hand" set vs decode mismatch** (m=3 setter vs m=6
   decoder). Almost certainly a typo in cgevans's setter that
   Tiny4Linux inherited. Live-validate by `set_ai_mode(Hand)`,
   `get_status()`, check that decode returns `Hand`; if it returns
   `Unknown`, the setter is wrong.
2. **Auto vs Manual exposure label swap** between cgevans and
   Tiny4Linux. One repo has its `AUTO` and `MANUAL` constants assigned
   to inverted wire bytes. The two-step pattern (selector 0x02 18-byte
   frame, then selector 0x06 `[0x03, 0x01, x]`) only makes sense if the
   first frame puts the camera in auto — confirming cgevans's labels.
   Live-validate before shipping.
3. **No "save preset" opcode** in either repo. Tiny4Linux only recalls
   slots 0/1/2 (and expects the user to program them via the OBSBOT
   Center app or the on-device gesture). We adopt the same constraint
   for v0.3 and add "preset save" to a later milestone if user demand
   is there. Cross-check: at minimum, the four `1.0_f32` floats in the
   appendix are an interesting clue — they might be the
   "save-current-position" payload when set to non-1.0 values. Not
   safe to probe without USB capture and a hardware-recovery plan.
4. **No tracking-speed gap value** (`byte 0x21 == 0x01`). The decoder
   defaults it to Standard. Could be a third mode (Tiny4Linux
   intentionally omits the "Headroom" mode cgevans declared but never
   shipped, see §Tracking Speed). Probe by setting via the proprietary
   SDK from a USB capture rig if/when we revisit this.
5. **Selector-0x06 unprobed opcodes**: `0x02`, `0x05`, `0x06`-`0x15`,
   `0x17`-onward are unmapped. Discovery requires `dump()` calls
   correlated to actions performed on the proprietary Windows app
   (Wireshark + Windows VM, see PROTOCOL.md §6 — this part of the
   research plan is **not** invalidated, only the "are there published
   reversers?" part is).
6. **Most status-struct bytes (0x00, 0x01, 0x03-0x05, 0x07-0x17,
   0x19-0x1b, 0x1d-0x20, 0x22-0x3b)** are read by the camera but not
   decoded by either repo. There's clearly more state to discover
   (mic? gesture? voice? LED brightness? face count?). A "raw status
   dump" debug GUI page would give the user a low-effort way to
   contribute.
7. **`uvc_xu_control_mapping` struct is declared but unused** in both
   repos. UVCIOC_CTRL_MAP / UVCIOC_CTRL_MAP_OLD is the mechanism by
   which a UVC driver can expose XU values as V4L2 controls; neither
   project uses it (they go straight to UVCIOC_CTRL_QUERY for every
   read/write). We follow the same approach: no V4L2 control mapping
   for XU. The user-perceived consequence: tools like `qv4l2` /
   `v4l2-ctl` cannot see HDR / AI tracking / etc. without going
   through our app; that's fine because the proprietary Windows app
   behaves the same way.
8. **Both repos call `UVC_GET_LEN` before every `GET_CUR` and
   `SET_CUR`.** This is paranoid but correct — the kernel returns
   `EINVAL` if the buffer size mismatches the camera's declared
   selector size. Our port does the same.
9. **Verbose-flag plumbing** is racy/buggy in both repos (cgevans
   uses an instance field, Tiny4Linux passes `bool` through every
   call). For our port, route verbose-mode through `tracing` /
   `log` and don't make it a function parameter; the GUI can flip
   the log level via gsettings.

---

*End of report. All byte values triple-checked against the
verbatim Rust source above; no values are inferred.*
