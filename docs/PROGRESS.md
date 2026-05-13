# PROGRESS — Chronological Journal

> **Purpose**: Continuous append-only log of work. Updated as work happens,
> not at the end of sessions. Last 3 entries are read by Claude Code at
> session start (see `CLAUDE.md` §0).
>
> **Format**: each entry is timestamped (UTC, ISO 8601), tagged with task ID,
> and describes one observable action. Never edit past entries; correct via
> a new entry that supersedes.

---

## 2026-05-12

### [2026-05-12T00:00:00Z] [bootstrap] Project scaffolding generated

Initial scaffolding generated externally (via planning conversation) and
delivered as a tarball. Repository contents:

- `CLAUDE.md`, `README.md`, `.gitignore`.
- `docs/` with 11 documentation files (STATE, SPEC, ARCHITECTURE, ROADMAP,
  PLAN, PROGRESS, DECISIONS, SKILLS, AI_WORKFLOW, PROTOCOL, GLOSSARY).
- Empty directories: `crates/`, `data/`, `build-aux/`, `po/` (each preserved
  with a `.gitkeep` placeholder).
- `INIT_PROMPT.txt` for the first Claude Code session.

No code yet. No git history yet. T-001 will create the first commit.

### [2026-05-12T10:30:00Z] [T-001] Started

Plan: validate the scaffolding against `ARCHITECTURE.md` §2, record the
validation outcome as an ADR, set `PLAN.md`/`STATE.md` to reflect T-001,
then `git init` on `main` and make the single bootstrap commit
`chore: initial scaffolding (T-001)`. The four intentional `.gitkeep` files
in `crates/`, `data/`, `build-aux/`, `po/` stay (user-confirmed
microconvention — they preserve §2's directory skeleton and are removed
naturally when later tasks fill those dirs).

### [2026-05-12T10:30:00Z] [T-001] Structure validated against ARCHITECTURE §2

Compared the working tree against `ARCHITECTURE.md` §2. Present: `CLAUDE.md`,
`README.md`, `.gitignore`, `docs/` (11 files), and the four `.gitkeep`'d
directories. Justifiably absent (each mapped to a later PLAN task in
[[ADR-0010]]): `Cargo.toml`, `meson.build`, `LICENSE`, all Rust source under
`crates/`, all assets under `data/`, gettext files in `po/`, and the Flatpak
manifest in `build-aux/`. Acceptance criterion #3 satisfied.

### [2026-05-12T10:30:00Z] [T-001] DONE — repository initialized

`git init -b main` ran in the project root. `git add` staged the entire
scaffolding (respecting `.gitignore`). `git commit` produced the single
bootstrap commit `chore: initial scaffolding (T-001)`. `git status` is
clean afterwards. All T-001 acceptance criteria met. Next: propose T-002
(namespace + license decisions) to the user.

### [2026-05-12T10:35:00Z] [T-002] Started

Plan: gather the two deferred decisions from [[ADR-0009]] — OSI license
and reverse-DNS namespace — by asking the user sequentially (user
preference: one decision at a time). License first (independent), then
namespace (needs the user's GitHub handle). Each decision will become its
own ADR superseding ADR-0009; a `LICENSE` file will land at the root;
project-wide search-and-replace of `io.github.<ns>` follows. Final commit
`chore: set namespace and license (T-002)`.

### [2026-05-12T10:40:00Z] [T-002] License decided: GPL-3.0-or-later

User selected GPL-3.0-or-later from a 4-option list (GPLv3+, GPLv2+,
LGPLv3+, MIT/Apache-2.0). Rationale (to be expanded in the upcoming ADR):
GNOME Circle precedent (used by Pika Backup, Fractal, Amberol, Shortwave,
Loupe), strong copyleft alignment with [[ADR-0002]] (no proprietary
derivatives), redistributable in Debian main, Fedora, and Arch.
ADR + `LICENSE` file will land together with the namespace ADR at task
close. Now asking for the namespace.

### [2026-05-12T10:50:00Z] [T-002] Namespace decided: io.github.domatix.ObsbotCamControl

User specified: hosting under GitHub organization `Domatix`
(`https://github.com/Domatix`), product name `ObsbotCamControl`. Resolved
the full identity per Flathub conventions (org segment lowercased):
- App ID: `io.github.domatix.ObsbotCamControl`
- Display name: "Obsbot Cam Control"
- Copyright line: "© 2026 Domatix and contributors"
- Local folder + crate names unchanged (`obsbot-control` umbrella with
  crates `obsbot-core`, `obsbot-cli`, `obsbot-gui`).
- Trademark-risk disclaimer to be reinforced in metainfo at T-009.

### [2026-05-12T10:55:00Z] [T-002] DONE — ADRs + LICENSE + placeholder sweep

Wrote [[ADR-0011]] (license) and [[ADR-0012]] (namespace + identity),
both formally superseding [[ADR-0009]]. Installed `LICENSE` at the repo
root with the verbatim GNU GPL-3.0 text fetched from `gnu.org/licenses/
gpl-3.0.txt` (674 lines). Swept `<ns>`, `<app-id>`, `<username>`, and
`ObsbotControl` placeholders across `ARCHITECTURE.md`, `PLAN.md`,
`SKILLS.md`, `GLOSSARY.md`, and `README.md`; left historical references
inside `PROGRESS.md`, `ADR-0009`, and `ADR-0010` untouched per the
append-only rule. README title now reads "Obsbot Cam Control" with the
App ID, license, and Domatix hosting surfaced at the top, and the
license section replaced with the GPL-3.0-or-later notice + SPDX
identifier. Commit `chore: set namespace and license (T-002)` (fce4f06)
follows.

### [2026-05-12T11:00:00Z] [T-003] BLOCKED — handed off hardware capture to user

User opted to advance T-003 and T-004 in parallel. T-003 cannot proceed
without `lsusb -v` and `v4l2-ctl` output captured against the user's
plugged-in OBSBOT Tiny 2 (Claude has no hardware access on this machine).
A clean copy-paste command block was given to the user covering: device
identification (`lsusb | grep -i obsbot`), full USB descriptor capture
(`lsusb -v -d <vid>:<pid>` to `/tmp/obsbot-lsusb.txt`), V4L2 device
enumeration, and per-`/dev/videoN` `--all` / `--list-ctrls-menus`
captures. Returning to IN_PROGRESS once the user pastes back the outputs.

### [2026-05-12T11:00:00Z] [T-004] Started — Cargo workspace scaffolding

Plan: create root `Cargo.toml` declaring `[workspace]` with the three
member crates (initially empty paths; T-005/T-006/T-007 will populate),
a shared `[workspace.package]` block (edition 2021, MSRV 1.83, license,
authors, repository), and `[workspace.dependencies]` pinning the runtime
deps from `ARCHITECTURE §1`. Validate via `cargo check --workspace` and
`cargo fmt --check`. Commit `build: create cargo workspace (T-004)`.
First action: verify Rust toolchain availability on this machine.

### [2026-05-12T11:05:00Z] [T-004] Toolchain probe — Rust absent

Neither `cargo`, `rustc`, nor `rustup` are in `PATH`. `apt-cache policy`
reports Debian trixie offers `rustc 1.85.0+dfsg3-1` (>= MSRV 1.83) plus
`rust-clippy`. Asked user which install path to use — they chose apt.
`sudo` requires interactive password (`sudo -n` fails), so handed the
exact command to the user for execution with the `!` prefix:
`sudo apt install -y rustc cargo rustfmt rust-clippy`. Validation paused
until install confirmed.

### [2026-05-12T11:10:00Z] [T-004] Workspace draft written (unvalidated)

While waiting for the toolchain install, drafted the root `Cargo.toml`
with `resolver = "2"`, `members = ["crates/*"]` (glob expands to empty
until T-005 lands), `[workspace.package]` shared metadata (version
"0.1.0" tracking the in-progress v0.1 milestone), and
`[workspace.dependencies]` pinning the runtime stack from ARCHITECTURE
§1: gtk4 ^0.9, libadwaita ^0.7, glib/gio ^0.20, gstreamer family ^0.23,
v4l ^0.14, nusb ^0.1, nix ^0.29, tracing ^0.1, tracing-subscriber ^0.3
(+env-filter), thiserror ^2, anyhow ^1, clap ^4 (+derive), async-channel
^2, gettext-rs ^0.7. Release profile: lto thin, codegen-units 1, strip
symbols. Also modified `.gitignore` to unhide `Cargo.lock` (this is a
binary-shipping workspace; the lockfile must be tracked for
reproducible Flathub/distro builds — inline comment in `.gitignore`
links to the Cargo FAQ). Pending: `cargo check --workspace`,
`cargo fmt --check`, commit.

### [2026-05-12T11:32:33Z] [T-004] Session paused by user

User requested to stop and power off the machine. Following the
controlled-pause protocol (CLAUDE.md §4.1): no WIP commit made.
Working tree at pause snapshot (`git status --short`):
  M  .gitignore
  M  docs/PLAN.md
  M  docs/PROGRESS.md
  M  docs/STATE.md
  ?? Cargo.toml
Last green commit on `main`: `fce4f06 chore: set namespace and license
(T-002)`. The drafted `Cargo.toml` and `.gitignore` changes survive in
the working tree across the power-off (they're on disk, not staged).
Resume protocol now lives in `STATE.md` under `resume_protocol:`.

---

## 2026-05-13

### [2026-05-13T09:57:27Z] [T-004] Session resumed — toolchain still absent

User said "continúa desde STATE.md". Re-read STATE.md, SPEC.md,
ROADMAP.md, PLAN.md, and the last entries of PROGRESS.md per CLAUDE.md
§0. Working tree matches the 2026-05-12T11:32:33Z pause snapshot
(unchanged on disk across the power-cycle): `.gitignore`, `docs/PLAN.md`,
`docs/PROGRESS.md`, `docs/STATE.md` modified; `Cargo.toml` untracked.
Probed for the Rust toolchain again — still absent: no `cargo`/`rustc`/
`rustfmt` in PATH, no `~/.cargo/bin`, no `~/.rustup`, no
`/usr/local/bin/{cargo,rustc}`, and `dpkg -l` reports rustc/cargo/
rustfmt/rust-clippy as not installed. T-004 stays IN_PROGRESS, blocked
on the user-side `sudo apt install -y rustc cargo rustfmt rust-clippy`
already prescribed in STATE.md's resume protocol.

### [2026-05-13T10:00:00Z] [T-004] Toolchain available — apt install completed

User ran `sudo apt install -y rustc cargo rustfmt rust-clippy` in a
separate interactive shell (this session's sudo cannot prompt for a
password). Re-probe: `/usr/bin/cargo` (cargo 1.85.0), `/usr/bin/rustc`
(rustc 1.85.0, built from source tarball, 2025-02-17), `/usr/bin/rustfmt`
(rustfmt 1.8.0), `/usr/bin/cargo-clippy` (clippy 0.1.85). Toolchain
satisfies MSRV 1.83 ([[ADR-0003]]). T-004 returns to active validation.

### [2026-05-13T10:05:00Z] [T-004] Validation hit virtual-workspace dead end → [[ADR-0013]]

First validation attempts failed: `cargo check --workspace` and
`cargo fmt --all --check` both error with `manifest is virtual, and
the workspace has no members` because `members = ["crates/*"]`
expands to the empty set (the `crates/` directory only has its
`.gitkeep`). The original T-004 acceptance criteria implicitly assumed
≥1 stub member existed, which is a T-005 deliverable. Three options
weighed: merge T-004+T-005 (rejected — breaks atomicity), throwaway
placeholder crate (rejected — clutter), or amend criteria
([[ADR-0013]]: T-004 validated by `cargo metadata`; full check/fmt
gate moved to T-005). Chose the third. [[ADR-0013]] written; PLAN.md
T-004 and T-005 criteria amended in place.

### [2026-05-13T10:10:18Z] [T-004] DONE — workspace manifest validated

`cargo metadata --no-deps --format-version 1` exit 0; output reports
`workspace_root=/home/alvaro/Documentos/proyectos/obsbot-control`,
`workspace_members=0`, `packages=0` (expected — members glob still
empty until T-005). `cargo verify-project --manifest-path Cargo.toml`
returns `{"success":"true"}`. Both [[ADR-0013]]-amended acceptance
criteria satisfied. T-004 set to DONE in [[PLAN.md]]. Commit
`build: create cargo workspace (T-004)` follows, bundling: root
`Cargo.toml`, `.gitignore` (Cargo.lock now committed), `docs/PLAN.md`
(T-004 DONE + criteria amendment, T-005 inheritance note),
`docs/DECISIONS.md` ([[ADR-0013]]), `docs/STATE.md` (active task
→ T-005), and this PROGRESS section. Next: propose T-005 to the user
(stub `obsbot-core` crate); T-003 stays BLOCKED on hardware capture.

### [2026-05-13T10:11:32Z] [T-004] Commit landed — 921fb57

`build: create cargo workspace (T-004)` committed as `921fb57` on
`main` (6 files, +287/-20). Working tree clean. STATE.md updated
to record the real SHA (was a placeholder during the commit-prep
window). Trailing docs-only delta to follow.

### [2026-05-13T10:20:00Z] [T-003] User correction → resuming with direct hardware probe

User flagged that the "Claude cannot touch hardware" framing in the
2026-05-12T11:00Z entry was wrong: `lsusb`/`v4l2-ctl` are read-only
USB / V4L2 queries that I run on the user's local machine like any
other shell tool. The original hand-off was unnecessary friction.
T-003 returns to active work; the SPEC change of [[ADR-0014]] follows
directly from the first capture.

### [2026-05-13T10:21:00Z] [T-003] lsusb capture — device is Tiny 2 *Lite*, not Tiny 2

`lsusb` shows `Bus 001 Device 006: ID 3564:fef9 Remo Tech Co., Ltd.
OBSBOT Tiny 2 Lite`. The regular Tiny 2 ships as `3564:fef8`
(linuxtv-commits patch). SPEC.md was authored assuming the regular
Tiny 2. Three options weighed: (A) declare a "Tiny 2 family" primary
target covering both regular and Lite; (B) flip primary to Lite,
downgrade regular to best-effort; (C) keep SPEC pointing at regular
Tiny 2 and treat the Lite as a development proxy. User chose (A).
[[ADR-0014]] records the rationale and the doc deltas.

### [2026-05-13T10:21:30Z] [T-003] Doc sweep for ADR-0014 — SPEC / ROADMAP / README updated

`docs/SPEC.md` §3 (target users), §5 (out of scope), §7 (constraints),
§10 (references) reworded to name the family and both PIDs.
`docs/ROADMAP.md` v0.1 goal, v0.4 prerequisites, beyond-v1.0 ideas
updated. `README.md` "Goals" and "Supported cameras" sections rewritten
with both PIDs and a pointer to ADR-0014. Crate names, App ID, repo
name, GUI binary name remain unchanged (all family-neutral).

### [2026-05-13T10:21:50Z] [T-003] PROTOCOL.md §1 + §3 filled from the Lite capture

`docs/PROTOCOL.md` §1.1 documents the Lite's full device descriptor,
VideoControl interface (INPUT_TERMINAL bmControls = 0x00023e3a,
PROCESSING_UNIT bUnitID=3 bmControls = 0x0000f7df, EXTENSION_UNIT
bUnitID=2 GUID `9a1e7291-6843-4683-6d92-39bc7906ee49` bmControls =
ff ff 3f 00 bNumControls=19), and VideoStreaming / Audio interface
shapes. §1.2 keeps regular Tiny 2 (`3564:fef8`) but explicitly marks
it as speculative pending community capture. §3.1 records the Lite
XU table; §3.2 notes the regular Tiny 2 XU as TBD. §5 captures a
critical finding: `iSerial = 0` on the Lite — per-device settings
persistence (T-105) cannot key off USB serial as SPEC.md §4.1 implied;
flagged for T-105 decision.

### [2026-05-13T10:22:00Z] [T-003] V4L2 half held by /dev/video* permissions

`/dev/video0` and `/dev/video1` belong to `root:video` with ACL only
extending to `Debian-gdm`. User `alvaro` is not in the `video` group.
Result: `v4l2-ctl -d /dev/video0 --all` returns `Permission denied`.
Two paste-blocks handed to the user in chat: (1) `sudo usermod -aG
video alvaro` for the long-term fix (also unblocks future
`cargo run -p obsbot-gui` from T-007 onwards), (2) four `sudo v4l2-ctl
... > /tmp/obsbot-v4l2-*.txt` redirects to seed PROTOCOL.md §2
immediately. Both blocks are also archived as the canonical procedure
inside PROTOCOL.md §2. T-003 stays IN_PROGRESS until /tmp/*.txt land
and the V4L2 table goes into PROTOCOL.md §2.

### [2026-05-13T10:22:12Z] [T-003] Commit prep — splitting into two commits

ADR-0014 + SPEC/ROADMAP/README and the T-003 lsusb capture are causally
one change but conceptually two: a scope decision and a data point.
Splitting per ADR-0008 (commit granularity):
  1. `docs: expand primary target to Tiny 2 family (ADR-0014)` —
     `docs/DECISIONS.md`, `docs/SPEC.md`, `docs/ROADMAP.md`, `README.md`.
  2. `docs: capture Tiny 2 Lite USB descriptor (T-003)` —
     `docs/PROTOCOL.md`, `docs/PLAN.md`, `docs/STATE.md`, `docs/PROGRESS.md`.

### [2026-05-13T10:40:00Z] [T-003] V4L2 captures landed — four files in /tmp

User ran the `sudo usermod -aG video alvaro` + four `sudo v4l2-ctl`
redirects from PROTOCOL.md §2. Files in `/tmp/`:
  obsbot-v4l2-all-0.txt    (90 lines — full /dev/video0 dump)
  obsbot-v4l2-all-1.txt    (34 lines — /dev/video1, metadata node)
  obsbot-v4l2-ctrls-0.txt  (34 lines — control table)
  obsbot-v4l2-ctrls-1.txt  (0 lines — metadata node has no controls)
Driver `uvcvideo` 6.12.73, bus `usb-0000:00:14.0-7` (matches sysfs
`1-7`), hardware revision `0x00000510 (1296)` = decimal match for the
descriptor's `bcdDevice 5.10` (corroborates the "firmware 5.10"
hypothesis in §1.1, but doesn't *prove* it's the firmware revision
rather than a hardware revision label; formal confirmation would need
OBSBOT Center's own version readout, deferred). Media-graph confirms
the kernel mounts XU 2 as entity `Extension 2 (Video Pixel Formatter)`
linked to the capture node via pad `0x100000a` — `UVCIOC_CTRL_QUERY`
will work against `bUnitID=2` once selector semantics are known.

### [2026-05-13T10:42:00Z] [T-003] PROTOCOL.md §2 populated — 24 controls + 3 quirks

§2.1 (User Controls) tabulates 13 entries: brightness/contrast/
saturation/hue (all 0..100), gain (1..64), red/blue_balance (0..2048),
white_balance_automatic + white_balance_temperature (2000..10000 K),
power_line_frequency menu {Disabled, 50, 60}, sharpness, backlight_
compensation. §2.2 (Camera Controls) tabulates 11 entries:
auto_exposure menu {Auto, Manual, Aperture Priority — note absent
value 2 = Shutter Priority}, exposure_time_absolute (1..2500),
pan/tilt_absolute (±130° / ±90° in UVC's degrees × 3600 units, step
3600 = 1°), focus_absolute + focus_automatic_continuous, zoom_absolute,
zoom_continuous, pan_speed (−1..160), tilt_speed (−1..120).

§2.3 documents 3 observed quirks the v0.2 GUI design must accommodate:
  Q1 — power_line_frequency default=3 outside menu max=2 (use 0
       Disabled as canonical default, ignore device-reported default).
  Q2 — zoom_continuous can read back beyond its advertised max=100
       (snapshot showed value=245). GUI clamps display; whether to
       surface this control at all is a T-102 decision.
  Q3 — gamma absent from PROCESSING_UNIT bmControls; treat as
       XU-only on this family until proven otherwise.

§2.4 leaves a TODO for `v4l2-ctl --list-formats-ext` (full format /
size / framerate matrix) which can wait until v0.3 / T-200 preview
work needs it. §1.1 unchanged; §3.1 gained a cross-check sentence
that the kernel mounts XU 2 (media-graph confirmation).

### [2026-05-13T10:45:37Z] [T-003] DONE — closing task and prepping commit

All three acceptance criteria satisfied for the Lite ([[PLAN T-003]]
Outcome). Regular Tiny 2 entries remain speculative; that gap is
recorded in PROTOCOL.md §1.2 and §3.2 and does not block T-003
closure given the family scope of [[ADR-0014]] (a community capture
will close it later). T-003 set to DONE in [[PLAN.md]]. STATE.md
goes idle (no active task), pending_user_actions trimmed to a single
optional reminder (log out / log back in to pick up the new `video`
group membership before T-013). Commit
`docs: capture Tiny 2 Lite V4L2 controls (T-003)` follows, bundling
PROTOCOL.md + PLAN.md + STATE.md + this PROGRESS section.

### [2026-05-13T10:49:16Z] [T-005] Started — scaffold obsbot-core

Plan: create the first workspace member, `crates/obsbot-core/`,
populating it minimally to satisfy [[PLAN T-005]] + [[ARCHITECTURE §3.1]]:
  Cargo.toml — package metadata pulling from `[workspace.package]`,
    a single `[dependencies]` block consuming `thiserror` and
    `tracing` from `[workspace.dependencies]`.
  src/error.rs — `Error` enum (`#[derive(Debug, Error)]`,
    `#[non_exhaustive]`) with variants for Unsupported, OutOfRange,
    Busy(PathBuf), Disconnected, and a transparent `Io` from
    `std::io::Error`. `pub type Result<T> = core::result::Result<T, Error>`.
  src/camera.rs — `CameraInfo` (vendor, product, vid, pid, serial,
    firmware, video_path), `Capabilities` (bool struct of ~25 feature
    flags covering SPEC.md §4.1), enums `AntiFlicker`, `ExposureMode`
    (matching the V4L2 menu observed in PROTOCOL.md §2.2 — no
    `ShutterPriority` variant, see §3.3 doc comment), `Fov`, and
    `AutoFramingMode`. The `Camera` trait itself: `Send + Sync` (per
    §3.1 verbatim), two required methods `info()` and `capabilities()`,
    ~50 default methods returning `Err(Error::Unsupported)` covering
    image controls / PTZ / WB / exposure / focus / anti-flicker / gain
    / backlight + vendor XU (HDR, FOV, auto-framing, face AE/AF,
    gesture). Default impls keep future backends concise.
  src/lib.rs — module declarations, `pub use` re-exports, crate-level
    doc comment.
SPDX header + short GPL boilerplate on every .rs per [[ADR-0011]].
Single unit test: a `DummyCamera` impl with stub `info()`/`capabilities()`
asserts that a default method (`brightness()`) returns `Error::Unsupported`.
Delete `crates/.gitkeep` (no longer needed — the directory acquires
real content). Validate via the four gates listed in
[[PLAN T-005]] acceptance criteria. Commit
`feat(core): scaffold Camera trait and types (T-005)`.

### [2026-05-13T10:51:00Z] [T-005] Clippy pedantic flagged `struct_excessive_bools`

First clippy run failed: `Capabilities` has 26 bool fields and
`clippy::struct_excessive_bools` (a pedantic-tier lint enabled by the
crate's `[lints.clippy] pedantic = warn`) suggests refactoring to a
state machine or two-variant enums. That advice is wrong for this
struct: the fields are independent feature flags, not interlocking
states. Added a focused `#[allow(clippy::struct_excessive_bools)]`
with a multi-line justification comment citing [[ARCHITECTURE §3.1]]
("Capabilities is a struct of bools"). Per [[CLAUDE.md §5.2]] every
allow must carry an in-comment justification.

### [2026-05-13T10:52:00Z] [T-005] All four gates green

  cargo fmt --all --check                                 → exit 0
  cargo check --workspace --all-targets                   → exit 0
  cargo clippy -p obsbot-core --all-targets -- -D warnings → exit 0
  cargo test -p obsbot-core                               → 3 unit + 1 doc, all pass

The doc-test is the `no_run` example in the lib.rs crate-level
comment — it compiles and links against the trait, so it's a real
shape check, not just prose. Final code stats: 4 source files
(Cargo.toml, lib.rs, camera.rs, error.rs), ~470 lines including
GPL boilerplate, comments, and tests. Cargo resolved 11 transitive
deps (thiserror 2.0.18 + tracing 0.1.44 trees), `Cargo.lock` now
present at repo root.

### [2026-05-13T11:00:09Z] [scope] Stakeholder request → ADR-0015 (.deb + Arch test packages)

User relayed a stakeholder ask: "cuando llegues a la app de control
de la cámara, que te haga un paquete .deb para que la pruebes tú y
otro para arch" — i.e., once a runnable build exists, the toolchain
should also emit a `.deb` (for the user's Debian trixie machine) and
an Arch `pkg.tar.zst` (for an Arch-using stakeholder) so both can
sideload-test the same revision. This contradicts the original
[[SPEC.md §4.5]] stance that Deb/RPM packaging is a non-goal.
Read of intent: **internal test distribution**, not Debian-policy /
AUR-grade upstreaming.

Recorded as [[ADR-0015]]. SPEC.md §4.5 amended to add a "test
artifact" tier (Flatpak stays primary). ROADMAP.md v0.1 "Includes"
gains the .deb + Arch lines. PLAN.md gains T-016 (`.deb` via
`cargo-deb`, depends on T-007 + T-013) and T-017 (Arch `PKGBUILD`,
same deps). T-015 (CI) acceptance criteria extended to cover the
test-artifact jobs. Nothing about T-005 or T-006 changes; the scope
addition is purely additive at the v0.1 tail.

Commit `docs: add .deb + Arch test-package scope (ADR-0015)` follows
before T-006 starts.

### [2026-05-13T11:01:00Z] [T-006] Started — scaffold obsbot-cli

Plan: `crates/obsbot-cli/Cargo.toml` (package metadata pulled from
`[workspace.package]`; `[[bin]] name = "obsbot-cli"`; depends only on
`clap` from `[workspace.dependencies]` for now — `obsbot-core`
dependency deferred to T-012 when the `list` subcommand actually
needs it; same `[lints]` block as obsbot-core); `src/main.rs` with a
minimal `#[derive(clap::Parser)]` empty struct carrying
`#[command(name = "obsbot-cli", version, about = "...")]`, `main()`
calls `.parse()` and prints `obsbot-cli v{CARGO_PKG_VERSION}`. SPDX
header per [[ADR-0011]]. Validate: `cargo run -p obsbot-cli --
--version` shows `obsbot-cli 0.1.0` (clap's auto-render), bare `cargo
run -p obsbot-cli` shows `obsbot-cli v0.1.0`. Then four workspace
gates as for T-005. Commit `feat(cli): scaffold CLI binary (T-006)`.

### [2026-05-13T12:30:00Z] [T-007] Code lands; objective smoke-test passes

`cargo build -p obsbot-gui` succeeded (1m 05s first build pulling
GTK4 + libadwaita Rust bindings + transitives). Four workspace gates
green after a `clippy::doc_markdown` fix (`GLib` needed backticks in
the application.rs doc comment). Two minor design choices hardened
into the source:
  * Aliasing strategy: `use gtk4 as gtk;` / `use libadwaita as adw;`
    declared per-module (the gtk-rs idiom). Tried first via
    `package = "..."` workspace-rename in Cargo.toml — Cargo rejects
    that combination (looks up the consumer-side name in
    `workspace.dependencies`, not the package field). The per-module
    `use ... as ...` is what every gtk-rs example does.
  * Cargo.toml `[[bin]] name = "obsbot-cam-control"` per
    [[ADR-0012]]; `cargo run -p obsbot-gui` still works because `-p`
    selects the package.

Objective window-appearance check via xwininfo while the binary ran
in the background (X11 session, DISPLAY=:10.0): root window tree
includes `0x2600004 "Obsbot Cam Control": ("obsbot-cam-control"
"obsbot-cam-control") 842x662+539+231` — i.e. the app maps a window
titled "Obsbot Cam Control" with WM_CLASS `obsbot-cam-control` and
near-default 720×540 geometry (Mutter expanded for decoration). The
process accepted SIGTERM with exit 143 (= 128 + 15), the standard
GTK behaviour without an explicit signal handler.

Visual + interactive checks (window appearance, Ctrl+Q, close
button) handed to the user — Claude cannot drive keyboard input.
T-007 stays IN_PROGRESS until the user reports back; once confirmed,
closes with `feat(gui): scaffold libadwaita application (T-007)`.

### [2026-05-13T12:36:03Z] [T-008] Started — Meson orchestration scaffold

Plan: top-level `meson.build` declaring the project (`obsbot-cam-control`
0.1.0, `license: 'GPL-3.0-or-later'`, `meson_version: '>= 1.0'`),
runtime-lib presence + minimum-version checks
(`dependency('gtk4', '>= 4.14')`, `libadwaita-1 >= 1.6`,
`glib-2.0 >= 2.74`, `gio-2.0 >= 2.74` per [[ARCHITECTURE §1]]), a
single `custom_target('cargo-build', …)` wrapping cargo via a
build-aux shim that:
  1. `cargo build -p obsbot-gui [--release]` against
     `meson.project_build_root()/cargo` as `--target-dir`,
  2. copies the produced `obsbot-cam-control` binary to `@OUTPUT@`.
The custom target is `build_by_default: true`, `install: true`,
`install_dir: bindir`. PLAN T-008 mentions handling `.desktop.in` /
`.metainfo.xml.in` / GSettings schema but those don't exist yet —
T-009 (metainfo + .desktop), T-010 (icons), T-105 (schema) introduce
them and will extend meson.build then. T-008 intentionally scopes
itself to the cargo-orchestration spine plus a hook comment for
each future addition.

Note: `cargo-build.sh` will be marked executable (`chmod +x`); meson
finds it via `find_program('build-aux/cargo-build.sh', required:
true)`. Validation: `meson setup builddir`, `meson compile -C
builddir`, `meson install -C builddir --destdir /tmp/install-test`,
plus an inspection of `/tmp/install-test` confirming the binary
lands at the right place under the prefix. Commit `build: set up
Meson orchestration (T-008)`.

### [2026-05-13T12:38:00Z] [T-008] meson.build details + 'rust' language dropped

`meson.build` declared the project without `'rust'` as a project
language. Rationale: meson never compiles Rust directly; cargo does
everything via a `custom_target`. Declaring `'rust'` makes meson hunt
for a rustc and prepare Rust-specific machinery we never use — strict
noise. Pika Backup follows the same pattern.

Runtime-lib `dependency(...)` checks (gtk4 ≥ 4.14, libadwaita-1 ≥ 1.6,
glib-2.0 ≥ 2.74, gio-2.0 ≥ 2.74) are belt-and-suspenders against the
cargo-side gtk4-sys / libadwaita-sys link: they fail at
`meson setup` with a clear pkg-config error if dev headers are
missing, instead of failing mid-cargo-build with a less obvious
linker message. On the user's machine they all resolved cleanly:
gtk4 4.18.6, libadwaita-1 1.7.6, glib-2.0 2.84.4, gio-2.0 2.84.4.

`build-aux/cargo-build.sh` is a 50-line bash wrapper:
`set -euo pipefail`, 6 positional args (manifest, target-dir,
profile, package, binary name, output path), profile-case dispatch
to either `--release` or default debug, `install -m 755` of the
produced binary to the meson-provided output slot. Trapping
non-existent binary paths with a clear error message (`exit 65`).

### [2026-05-13T12:39:00Z] [T-008] Gates green

  meson setup builddir                            → exit 0 (all 4 runtime
                                                    deps resolved, cargo found,
                                                    shim discovered)
  meson compile -C builddir                       → exit 0 (cargo build --release
                                                    -p obsbot-gui, 1m 22s on
                                                    first run, produces
                                                    builddir/cargo/release/
                                                    obsbot-cam-control, shim
                                                    installs to
                                                    builddir/obsbot-cam-control)
  meson install -C builddir --destdir /tmp/install-test
                                                  → exit 0; tree:
                                                    /tmp/install-test/usr/local/
                                                    bin/obsbot-cam-control
                                                    (424 448 bytes,
                                                    stripped ELF, mode 0755)
  /tmp/install-test/usr/local/bin/obsbot-cam-control --help
                                                  → exit 0 (GLib option-group
                                                    help in Spanish locale —
                                                    confirms the binary loads
                                                    and resolves GTK at
                                                    runtime, even from the
                                                    DESTDIR location since
                                                    libgtk is at the system
                                                    location).

### [2026-05-13T12:39:26Z] [T-008] DONE — closing task and prepping commit

PLAN.md T-008 set to DONE with the Outcome block (in-task scope
correction noted: `.desktop.in`/`.metainfo.xml.in`/schema deferred
to their owning tasks). STATE.md goes idle with T-009 (AppStream
metainfo + .desktop file) as the natural next task. Commit
`build: set up Meson orchestration (T-008)` follows, bundling
`meson.build` (new), `build-aux/cargo-build.sh` (new, +x mode
preserved by `git add`), `build-aux/.gitkeep` (deleted —
directory now has real content, matching the [[T-005]] precedent
for `crates/`), and the three docs files.

### [2026-05-13T12:42:08Z] [session-end] Clean checkpoint at end of session

User asked to end the session. No active task, no partial work, no
uncommitted changes. Eight v0.1 tasks closed this session (T-003
USB+V4L2 capture, T-004 Cargo workspace, T-005 obsbot-core stub,
T-006 obsbot-cli stub, T-007 obsbot-gui stub, T-008 Meson
orchestration) plus two scope decisions (ADR-0014 Tiny 2 family
target, ADR-0015 .deb + Arch test-package tier). Next session
resumes with T-009 (AppStream metainfo + `.desktop` file +
i18n scaffolding). Tiny fix-up commit follows to record the real
T-008 SHA in STATE.md (it was a "pending in this turn" placeholder)
and to leave this checkpoint marker.

### [2026-05-13T12:30:42Z] [T-007] DONE — user confirmed visual + interactive

User ran `cargo run -p obsbot-gui` and reported all three acceptance
paths green: window appears with the "Obsbot Cam Control" header
bar + the Adwaita status-page placeholder, Ctrl+Q quits cleanly,
and the window close button quits cleanly. PLAN.md T-007 set to
DONE with the Outcome block. STATE.md goes idle with T-008 (Meson
orchestration) as the natural next task. Commit `feat(gui): scaffold
libadwaita application (T-007)` follows, bundling crates/obsbot-gui/
(four new files) + Cargo.lock + docs/PLAN.md + docs/STATE.md + this
PROGRESS section.

### [2026-05-13T12:21:13Z] [T-007] Started — scaffold obsbot-gui

Plan: third workspace member. `crates/obsbot-gui/Cargo.toml` with
the same `[lints]` block as obsbot-core/obsbot-cli, `[[bin]] name =
"obsbot-cam-control"` (per [[ADR-0012]] GUI binary name; `cargo run
-p obsbot-gui` still works because `-p` selects the package, not the
bin), and dependencies on `gtk4 + libadwaita + gio + glib` from
`[workspace.dependencies]` (plus a `path` dep on `obsbot-core` so
the trait is already imported for T-013 to wire to). Source split per
[[ARCHITECTURE §2]]:
  src/main.rs — bootstrap: APP_ID constant
    (`io.github.domatix.ObsbotCamControl`), call into
    `application::run(APP_ID)`, return `glib::ExitCode`.
  src/application.rs — `pub fn run(app_id) -> glib::ExitCode` that
    builds an `adw::Application`, wires `connect_activate` to spawn
    a `window::ObsbotWindow`, registers an `app.quit` SimpleAction
    and binds it to `<primary>q` so Ctrl+Q closes the app.
  src/window.rs — minimal `pub fn build(app: &adw::Application) ->
    adw::ApplicationWindow` that constructs the window with a header
    bar carrying the literal `"Obsbot Cam Control"` title plus a
    placeholder Adwaita status page ("nothing here yet, T-013 will
    fill this") so the empty window looks intentional rather than
    broken.
SPDX header on every .rs per [[ADR-0011]]. Validate: `cargo check
--workspace`, `cargo fmt --all --check`, `cargo clippy --workspace
--all-targets -- -D warnings`, `cargo test --workspace`, and a `cargo
build -p obsbot-gui` followed by a brief `cargo run -p obsbot-gui`
spot-launch (window appearance / Ctrl+Q quit have to be confirmed by
the user — Claude can't see the screen). Commit `feat(gui): scaffold
libadwaita application (T-007)`.

### [2026-05-13T11:02:07Z] [T-006] DONE — gates green, behaviour verified

  cargo run -p obsbot-cli -- --version → `obsbot-cli 0.1.0` (clap), exit 0
  cargo run -p obsbot-cli              → `obsbot-cli v0.1.0` (println), exit 0
  cargo fmt --all --check              → exit 0
  cargo check --workspace --all-targets → exit 0
  cargo clippy --workspace --all-targets -- -D warnings → exit 0
  cargo test --workspace               → 3 unit + 1 doc, all pass (obsbot-core); obsbot-cli has no tests yet (none added — would be premature for a `println!` stub)

`Cargo.lock` picks up clap 4.6.1 along with its transitives
(clap_builder 4.6.0, clap_derive 4.6.1, anstream 1.0.0, anstyle-parse
1.0.0, strsim 0.11.1). PLAN.md T-006 set to DONE with outcome block.
STATE.md idle, T-007 next. Commit `feat(cli): scaffold CLI binary
(T-006)` follows.

### [2026-05-13T10:52:32Z] [T-005] DONE — closing task and prepping commit

All acceptance criteria satisfied (see [[PLAN T-005]] Outcome
block). PLAN.md T-005 set to DONE with detailed criteria checkboxes.
STATE.md goes idle with T-006 as the natural next task (clap-based
CLI binary scaffold; depends only on T-005). PROGRESS.md captures
this T-005 chronicle. Commit
`feat(core): scaffold Camera trait and types (T-005)` follows,
bundling: `crates/obsbot-core/{Cargo.toml, src/lib.rs, src/camera.rs,
src/error.rs}` (new), `crates/.gitkeep` (deleted), `Cargo.lock` (new,
ships per the [[T-004]] / `.gitignore` decision), `docs/PLAN.md`,
`docs/STATE.md`, and `docs/PROGRESS.md`.

---
