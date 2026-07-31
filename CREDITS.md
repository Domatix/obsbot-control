# Credits

This project ships under `GPL-3.0-or-later`. Some files in
`crates/obsbot-core/src/xu/` are derivative works of upstream
free-software reverse-engineering projects licensed under the
European Union Public Licence v1.2 (EUPL-1.2), which is
explicitly compatible with GPL-3 per its Appendix. Each ported
file carries an SPDX line plus a short attribution block
documenting the origin (see § *Per-file headers* below).

---

## OBSBOT Tiny 2 protocol — reverse-engineering lineage

The UVC Extension Unit (XU) command surface implemented in
`obsbot-core::xu` is a port of prior free-software work, in
reverse-chronological order of contribution:

### OpenFoxes / Tiny4Linux

- Repo: <https://github.com/OpenFoxes/Tiny4Linux>
- Licence: EUPL-1.2
- Authors: Bono Fox, Constantine Evans
- Contributions ported into `obsbot-core::xu`:
  - Sleep / Wake control (selector `0x02`, function-group
    `0x0a 0x02 0xc2 0xa0 0x04 0x00`).
  - Tracking Speed (Standard / Sport, selector `0x02`, function-
    group `0x0a 0x04 0xc4 0x0c 0x01 0x00`).
  - Preset position recall (3 slots, selector `0x02`, function-
    group `0x0a 0x04 0xc4 0x39 0x14 0x00`).
  - Modular factoring of the 36-byte `command02` frame
    (`FRAME_ID + sequence_nr + SEGMENT_SIZE + checksum +
    function_group + command + appendix`).
  - Decode of GET_CUR status bytes `0x02` (sleep state) and
    `0x21` (tracking speed).
  - Test vectors used as fixtures in our `obsbot-core::xu`
    unit tests.

### cgevans / tiny2

- Repo: <https://github.com/cgevans/tiny2>
- Licence: EUPL-1.2
- Author: Constantine Evans
- Contributions ported into `obsbot-core::xu`:
  - Original Rust wrapping of `UVCIOC_CTRL_QUERY` against
    `/dev/videoN` (no libusb path).
  - HDR (selector `0x06` op `0x01`).
  - Face Auto-Exposure (selector `0x06` op `0x03`).
  - Field of View, three widths Wide / Normal / Narrow
    (selector `0x06` op `0x04`).
  - AI Tracking Mode, ten modes (selector `0x06` op `0x16`,
    `(m, n)` tuple encoding).
  - Manual / Auto exposure 18-byte frames (selector `0x02`,
    function-group `0x0a 0x02 0x82 0x29 0x05 0x00`).
  - Decode of GET_CUR status bytes `0x06` (HDR), `0x18` and
    `0x1c` (AI mode `(m, n)`).
  - V4L2 standard PTZ wrappers (`V4L2_CID_PAN_ABSOLUTE`,
    `V4L2_CID_TILT_ABSOLUTE`, `V4L2_CID_ZOOM_ABSOLUTE` and
    relative variants) — re-used to back the T-101 PTZ
    buttons.

### samliddicott / meet4k

- Repo: <https://github.com/samliddicott/meet4k>
- Licence: EUPL-1.2
- Contribution: cgevans's project is *substantially based on*
  meet4k. We do not pull bytes directly from meet4k (the Tiny 2
  protocol is different from the Logitech Meetup's), but the
  pattern of *opcode-multiplexed XU over `uvcvideo` ioctl* and
  the `nix::ioctl_readwrite_buf!` shape originated there.

---

## Licensing note

The three projects above are licensed EUPL-1.2. Per its
Appendix, EUPL-1.2 is compatible with GPL-3 for derivative
works. Files we port from them carry a dual SPDX header
documenting the origin:

```rust
// SPDX-License-Identifier: GPL-3.0-or-later
//
// Portions of this file are derived from EUPL-1.2 source:
//   - cgevans/tiny2        (https://github.com/cgevans/tiny2)
//   - OpenFoxes/Tiny4Linux (https://github.com/OpenFoxes/Tiny4Linux)
// "Licensed under the EUPL"
```

The literal `"Licensed under the EUPL"` line is the only
attribution string the EUPL itself requires (Article 5,
"Obligations of the Licensee"). The two URL pointers are an
editorial courtesy.

---

## Per-file headers

Files in `crates/obsbot-core/src/xu/**` that contain ported
byte sequences from EUPL-1.2 sources carry the block above.
Files that are entirely original (e.g. the GTK page wiring in
`obsbot-gui`) keep the plain `GPL-3.0-or-later` SPDX line
without the attribution block.

If you are auditing for licence compliance, the canonical list
of EUPL-derived files is anything matching:

```
crates/obsbot-core/src/xu/commands/*.rs
crates/obsbot-core/src/xu/transport.rs
crates/obsbot-core/src/xu/status.rs
crates/obsbot-core/src/xu/enums.rs
crates/obsbot-core/src/xu/command02.rs
```

See `docs/DECISIONS.md` ADR-0020 for the full rationale.
