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

### [2026-05-13T16:18:00Z] [scope] T-013 split into T-013a/b/c/d via [[ADR-0016]]

T-013 as written bundled four unrelated mechanisms (initial-scan
camera list, hot-plug listener, V4L2 control sub-page, Blueprint
pipeline) into one task ID, which would have made the IN_PROGRESS
state cover several days of work and violated the "atomic functional
change" granularity that worked well for T-001..T-012. Split into:

* T-013a — initial camera list in GUI (active now; hand-coded GTK,
  no hot-plug).
* T-013b — hot-plug listener (first-pass polling per ADR-0016).
* T-013c — V4L2 control sub-page (read-only).
* T-013d — Blueprint pipeline (introduced once T-013c has a UI
  with enough named children to make the .blp pipeline pay for
  itself).

The `video` group membership pending action graduates from a v0.1
prerequisite to a T-013c-specific prerequisite, and is in any case
already satisfied on this machine: `groups` returns `alvaro sudo
video users`, `/dev/video0` is `crw-rw---- root:video`, so V4L2
reads will work whenever T-013c arrives. T-013a / T-013b touch
sysfs only and need no group membership at all.

T-014, T-016, T-017 dependency lines updated in PLAN.md to point at
T-013a (the moment the GUI shows a real camera) instead of the
parent T-013.

### [2026-05-13T18:05:00Z] [session-end] Clean checkpoint at end of session

User asked to wrap the session. No active task, no partial work, no
uncommitted changes. Six v0.1 atoms closed this session
(T-012 CLI `list`, T-013a initial camera list, T-013b hot-plug
listener, T-013c V4L2 control sub-page, T-014 Flatpak manifest +
fixes) plus one scope decision ([[ADR-0017]] deferring T-013d
Blueprint pipeline to v0.2 as new T-099). v0.1 remaining:

* **T-015 (CI)** — BLOCKED until the repo is published on GitHub
  per its own PLAN note. Mark BLOCKED when reached.
* **T-016 (.deb via cargo-deb)** — autonomous; depends on cargo-deb
  install + `[package.metadata.deb]` block in obsbot-gui/Cargo.toml.
* **T-017 (Arch PKGBUILD)** — autonomous; text file + makepkg path,
  no host dep beyond a contributor on Arch (or fakeroot).

Plus the deferred GNOME-48-EOL bump (pre-v1.0 readiness task) and
the deferred T-099 Blueprint pipeline (v0.2 prerequisite). Next
session resumes with T-016 unless the user opts to push the repo
to GitHub first to unblock T-015.

### [2026-05-13T17:55:00Z] [T-014] DONE — three fixes after first runtime probe

User ran the apt + flatpak install sequence and ~1-2 GB of GNOME 48
runtime+SDK + rust-stable extension downloaded cleanly. First
`flatpak-builder` invocation then surfaced two manifest issues; a
third minor fix (gitignore) appeared during cleanup. All three are
in the manifest / repo state now.

**Fix 1 — `libclang.so` missing from the GNOME 48 SDK sandbox.**
First build failed at `v4l2-sys-mit-0.3.0` build-script with
`bindgen-0.65.1: Unable to find libclang`. Root cause: the GNOME
SDK ships clang binaries but not the libclang shared library
bindgen looks for. Standard Flathub pattern for Rust+bindgen apps
is to add `org.freedesktop.Sdk.Extension.llvmNN` as a second
sdk-extension and export `LIBCLANG_PATH`. Available extensions for
runtime 24.08 are llvm18 / llvm19 / llvm20 (probed via `flatpak
search org.freedesktop.Sdk.Extension.llvm`); picked llvm19 as the
stable mid-ground. The manifest now adds
`"org.freedesktop.Sdk.Extension.llvm19"` to `sdk-extensions`, and
the `build-options` block grows:

* `append-path`: `/usr/lib/sdk/rust-stable/bin:/usr/lib/sdk/llvm19/bin`
* `prepend-ld-library-path`: `/usr/lib/sdk/llvm19/lib`
* `env.LIBCLANG_PATH`: `/usr/lib/sdk/llvm19/lib`

**Fix 2 — Flatpak's icon validator rejected the symbolic SVG.**
Second build cleared the compile + meson install phases but failed
at the `Exporting … to repo` stage with `flatpak-validate-icon:
"Format not recognized"` on the symbolic SVG. The scalable SVG
(same author, same shape, same toolchain) passed at 128×128.
Bisected the difference by writing a minimal-valid `<svg>` and
diffing against ours: the three SVG comments sitting between the
`<?xml ... ?>` declaration and the `<svg>` root element trigger
the validator's stricter symbolic loader at 16×16, even though the
regular SVG loader at 128×128 accepts the same shape. The
comments restated SPDX/copyright info that already lives in the
project LICENSE file and the docs, so they were aspirational
rather than load-bearing. Both files cleaned (defense-in-depth on
the scalable side too); the body of each is now strictly
`<?xml...?>` → `<svg>` → drawing → `</svg>`. Manual
`flatpak-validate-icon --sandbox` confirmed both pass at
128×128 / 16×16 respectively before re-running the build.

**Fix 3 — `.gitignore` for the flatpak-builder output dir.**
`flatpak-builder ... build-flatpak ...` writes its working ostree
repo to `build-flatpak/`. Already had `.flatpak-builder/` ignored
(the cache dir) but not the output dir. Single-line addition.

Third `flatpak-builder` run succeeded end-to-end in ~3 minutes
(cargo cache warm; first cold-cache run was closer to 5 minutes).
The app installs as `io.github.domatix.ObsbotCamControl 0.1.0
master` in the user's flatpak. `flatpak run io.github.domatix.
ObsbotCamControl` opens the same `842x662 "Obsbot Cam Control"`
window xwininfo has seen since T-007. User-confirmed via
AskUserQuestion: the camera row, drill-down with 22 V4L2 controls,
and hot-plug all work identically to the native binary — confirming
`--device=all` is the right finish-arg for V4L2 access from the
sandbox.

**EOL warning noted, deferred.** The Flatpak install surfaced a
prominent `Info: org.gnome.Platform 48 is end-of-life` warning
(GNOME 48 EOL'd 2026-03-24, ~50 days ago as of today). The runtime
still functions for local-build verification and Flathub
submission is a v1.0 goal, so we don't bump now. A future task
(pre-v1.0 readiness check) will migrate to the then-current
supported GNOME runtime. Recorded in T-014's outcome and STATE.md.

PLAN.md T-014 → DONE. STATE.md returns to idle. v0.1 remaining:
T-015 (CI — BLOCKED until repo public per its own PLAN note),
T-016 (.deb test artifact), T-017 (Arch PKGBUILD). The three
fixes go in a single follow-up commit (manifest + SVGs + gitignore
+ docs).

### [2026-05-13T17:05:00Z] [T-014] Started — initial Flatpak manifest

Pre-flight probe: neither `flatpak` nor `flatpak-builder` is
installed on the host. The full runtime validation
(`flatpak-builder --user --install ...`) requires ~1-2 GB of GNOME
48 runtime+SDK download on the user's side, so the autonomous
deliverable for this turn is the manifest itself (a structured JSON
that can be written correctly and reviewed against Flathub patterns
without running it). The two acceptance criteria that need the
running flatpak toolchain stay PENDING_USER until they install the
environment.

Plan:

* `build-aux/io.github.domatix.ObsbotCamControl.json` — Flatpak
  manifest with the canonical GNOME-Circle shape:
    - `runtime`: `org.gnome.Platform` 48 + `org.gnome.Sdk` 48.
    - `sdk-extensions`: `org.freedesktop.Sdk.Extension.rust-stable`
      so cargo is available during the meson build (the GNOME SDK
      itself does not ship rustc).
    - `command`: `obsbot-cam-control` (the binary name from
      [[ADR-0012]]).
    - `finish-args`: `--share=ipc`, `--socket=wayland`,
      `--socket=fallback-x11`, `--device=all`. The
      `--device=all` is required to reach `/dev/video*` plus the
      raw USB device for future XU work (no narrower Flatpak
      device filter exists for V4L2 + UVC controls).
    - `build-options`: `append-path` for the rust-stable extension's
      `/usr/lib/sdk/rust-stable/bin`, `--share=network` build-arg
      so cargo can fetch crates, `CARGO_HOME` pointed at the
      Flatpak build dir.
    - Single module `obsbot-cam-control` of buildsystem `meson`,
      source `type=dir path=..` for local-build (CI / Flathub will
      replace this with a `type=git` block at release time).

* No code or test changes — the manifest is data, the existing
  `meson` orchestration from T-008 already produces a runnable
  binary.

* README update: a `## Flatpak (local build)` subsection with the
  three-line install sequence
  (`sudo apt install flatpak flatpak-builder`, `flatpak install
  --user flathub org.gnome.Platform//48 …`, `flatpak-builder --user
  --install --force-clean build-flatpak build-aux/…json`).

Cargo gates: unchanged (no Rust touched). Will run `cargo fmt
--all --check`, `cargo clippy --workspace --all-targets --
-D warnings`, `cargo test --workspace` as a regression check before
the commit.

Commit: `build: initial Flatpak manifest (T-014)`.

### [2026-05-13T17:03:00Z] [scope] T-013d deferred to v0.2 per [[ADR-0017]]

ADR-0016's premise — "Blueprint pipeline lands in T-013d because
T-013c will have many named children" — did not materialise. The
V4L2 detail page from T-013c renders entirely from a dynamic
`Vec<ControlDescriptor>` (22 entries on the user's hardware), the
only static widget tree in the GUI is the 5-line `AdwApplication
Window → AdwNavigationView → AdwToolbarView → AdwHeaderBar +
AdwBin` shell in `window.rs`. Setting up `blueprint-compiler` +
`glib-build-tools` + `build.rs` + GResource for that shell would be
~150 lines of new glue with one consumer — the textbook "design for
hypothetical future requirements" CLAUDE.md warns against. The
pipeline lands in v0.2 as the new T-099, before any T-100+ task
that introduces a static widget tree (slider forms, PTZ pad, etc.).

PLAN.md T-013d state moved to `DEFERRED` with the acceptance
criteria preserved verbatim for the absorbing v0.2 task. v0.2
hints gain a `T-099 Blueprint pipeline` entry at the top so it is
the first task on the v0.2 backlog. v0.1 milestone DOD shrinks
from "all of T-013a/b/c/d + T-014..T-017" to "all of T-013a/b/c +
T-014..T-017"; T-013a/b/c are already DONE, so v0.1 closes when
T-014..T-017 are DONE. Commit `ce6206e` records the decision.

### [2026-05-13T16:58:00Z] [T-013c] DONE — backend + GUI, drill-down user-confirmed

Implementation came in with three in-flight design / environment
deltas worth a paper trail beyond the inline comments:

1. **`home@0.5.12` blocked the v4l dep chain on MSRV 1.85.** When
   I added `v4l.workspace = true` to `obsbot-core/Cargo.toml`,
   `cargo check` failed with `home@0.5.12 requires rustc 1.88`.
   The dep chain is `obsbot-core → v4l → v4l2-sys-mit → bindgen →
   which → home`. Two options weighed: (a) bump our MSRV to 1.88
   (would invalidate [[ADR-0003]]'s 1.83 minimum and orphan Debian
   trixie's stock `rustc 1.85` — bad for distro packaging T-016)
   or (b) `cargo update -p home --precise 0.5.11` to pin the
   pre-1.88 variant. Chose (b). The pin lives in `Cargo.lock`;
   no ADR needed since it's a mechanical workaround, not a scope
   change, and reverts naturally when we eventually do bump MSRV.

2. **PROTOCOL §2's "13+11=24" overcount.** First hardware-test
   run failed at `assert!(controls.len() >= 24, …)` with
   `got 22`. Cross-checked via `v4l2-ctl --list-ctrls -d
   /dev/video0` and got the same 22 (12 User + 10 Camera). The
   2-control gap is in PROTOCOL.md §2's tabulation — it appears
   to have counted the `User Controls` and `Camera Controls`
   class headers as controls. `v4l2-ctl` and the V4L2 kernel
   enumeration both agree on 22; the kernel is authoritative.
   Test threshold relaxed to `>= 22` with an inline comment
   pointing at the discrepancy. PROTOCOL.md edit deferred (it's
   a docs-tier fix; do it bundled with T-013d or a later docs
   pass).

3. **`non_exhaustive` consumption ergonomics.** Both
   `ControlClass` and `ControlKind` carry `#[non_exhaustive]`
   (by design — future variants must be a non-breaking add).
   The GUI's `match` on `ctrl.class` and `ctrl.kind` in
   `controls_view.rs` therefore needs wildcard arms for the
   compiler's exhaustiveness check, even though `Other(_)` /
   `Other(String)` already catch the open cases. Two `_ => …`
   arms added explicitly (one with `Other(_)`-style fallback,
   one with `(unsupported)` text) — visible and intentional,
   not a clippy suppression.

Final shape:

* **`crates/obsbot-core/src/controls.rs`** (new, ~190 lines).
  Public surface:
  ```
  pub struct ControlDescriptor { name, class, kind }
  pub enum ControlClass { User, Camera, Other(u32) }
  pub enum ControlKind {
      Integer { current, min, max, step },
      Boolean { current },
      Menu { current_label, options },
      Other(String),
  }
  pub fn read_controls(video_path: &Path) -> Result<Vec<ControlDescriptor>>
  ```
  Private helpers: `classify(id)` (3 unit tests), `build_kind`,
  `read_integer`. All four types have `#[derive(Debug, Clone,
  PartialEq, Eq)]` so consumers can diff snapshots. `Result`
  surface inherits from `crate::Error`: open / ioctl failures
  collapse into `Error::Io(io::Error)` via the existing `#[from]`
  on the error enum.
* **`crates/obsbot-core/tests/hardware.rs`** picks up
  `reads_v4l2_controls_from_connected_unit` (`#[ignore]`d).
  Asserts ≥22 controls, both classes present, Brightness is an
  integer-typed User control. Run via `cargo test -p obsbot-core
  -- --ignored` → 2 passed (the T-011 enumerate test stays
  green).
* **`crates/obsbot-gui/src/controls_view.rs`** (new, ~130 lines).
  `build_controls_page(&CameraInfo) -> adw::NavigationPage`.
  Wraps an `AdwToolbarView` (its own `AdwHeaderBar` — the back
  button is auto-provided by the outer `AdwNavigationView`) with
  an `AdwPreferencesPage` of grouped controls (User / Camera /
  Other). Error paths render as `AdwStatusPage` so the UI never
  panics on a partial read.
* **`crates/obsbot-gui/src/window.rs`** rewritten (~150 lines).
  Top-level structure is now `AdwApplicationWindow → AdwNavigation
  View → root NavigationPage("cameras") → ToolbarView → Bin
  (body_slot)`. The hot-plug timer's closure also weak-captures
  `nav_view` so the camera-row factory can wire `connect_
  activated` to `nav_view.push(&build_controls_page(&cam))`
  without an Rc cycle. Each camera row becomes
  `activatable(true)` with a `go-next-symbolic` suffix icon to
  hint at the drill-down.

Gate summary:

```
cargo fmt --all --check                                → exit 0
cargo clippy --workspace --all-targets -- -D warnings  → exit 0
cargo test --workspace                                 → 14 unit
                                                         (8 enumerate
                                                          + 3 controls
                                                          + 3 camera
                                                          ↑ obsbot-core)
                                                         + 2 ignored
                                                         hardware
                                                         + 1 doctest
                                                         + 3 CLI render
                                                         = 23 total
                                                         pass
cargo test -p obsbot-core --test hardware -- --ignored → 2 passed
                                                         (the new
                                                         controls test
                                                         joined the
                                                         T-011 enumerate
                                                         test)
cargo build -p obsbot-gui                              → exit 0
./target/debug/obsbot-cam-control (background)         → maps the
                                                         842x662
                                                         window with
                                                         drill-down
                                                         wired in.
```

User-confirmed drill-down via AskUserQuestion: "Sub-página
correcta" — the user tapped the Tiny 2 Lite row, saw the 22
controls grouped under "User Controls" / "Camera Controls" with
their live values + ranges in the subtitles, and confirmed the
back button works.

PLAN.md T-013c → DONE with the Outcome block. STATE.md returns to
idle with T-013d (Blueprint pipeline) named as the next task. Commit
`feat: V4L2 control sub-page (T-013c)` follows, bundling:
`Cargo.lock` (v4l + transitives + home pin), `crates/obsbot-core/
Cargo.toml`, `crates/obsbot-core/src/controls.rs` (new),
`crates/obsbot-core/src/lib.rs`, `crates/obsbot-core/tests/
hardware.rs`, `crates/obsbot-gui/src/main.rs`, `crates/obsbot-gui/
src/controls_view.rs` (new), `crates/obsbot-gui/src/window.rs`,
plus the three docs files.

### [2026-05-13T16:40:00Z] [T-013c] Started — V4L2 control sub-page

Plan: two-side change.

* Backend (`obsbot-core`): new module `controls.rs` exposing
  `read_controls(video_path: &Path) -> Result<Vec<ControlDescriptor>>`
  built on top of `v4l 0.14` (workspace dep). Reshape the v4l
  crate's `Description` / `Value` types into obsbot-core-owned
  `ControlDescriptor { name, class, kind }` so consumers never
  depend on the v4l API directly. Skip `CtrlClass` entries and
  any flag-disabled / write-only controls. Re-export from
  `lib.rs`. Add `v4l.workspace = true` to obsbot-core's
  `[dependencies]`. Three unit tests on the `classify()` helper
  (User / Camera / unknown class IDs) plus a new `#[ignore]`d
  hardware integration test asserting the 24 controls from
  PROTOCOL §2.
* GUI (`obsbot-gui`): wrap the existing camera list in an
  `AdwNavigationView`. Each `AdwActionRow` becomes `activatable
  (true)` and `connect_activated` pushes the detail page returned
  by a new module `controls_view::build_controls_page(&cam)` onto
  the nav-view. The detail page = `AdwToolbarView + AdwHeaderBar +
  AdwPreferencesPage` with one `AdwPreferencesGroup` per V4L2
  class. Each control shown as an `AdwActionRow` with the
  value + range / "Yes-No" / "<label> · N options" in the subtitle.

Synchronous read on the GTK main thread for the first pass — the
~24 ioctls take well under 100 ms on the user's hardware. Async
lift deferred to a future task if profiling demands it. No new
unit tests on the GUI side (GUI is not auto-tested per
[[CLAUDE.md §5.4]]); acceptance is the user tapping the row and
confirming the controls appear with sensible values.

Commit: `feat: V4L2 control sub-page (T-013c)`.

### [2026-05-13T16:36:00Z] [T-013b] DONE — gates green, both hot-plug paths verified

Implementation came in as planned. Final `window.rs` adds 32 lines
to the T-013a shape:

* `const POLL_INTERVAL: Duration = Duration::from_secs(2);` —
  defined as `Duration` rather than `u32` so the call site uses
  `timeout_add_local` (Duration-typed) instead of the seconds
  helper, which keeps sub-second tuning a one-line change.
* `start_hotplug_poll(body_slot: &adw::Bin, initial:
  Vec<CameraInfo>)` factored out of `build()` for readability.
  Closure shape:

  ```rust
  glib::timeout_add_local(
      POLL_INTERVAL,
      glib::clone!(
          #[weak] body_slot,
          #[upgrade_or] glib::ControlFlow::Break,
          move || {
              let latest = enumerate_cameras();
              let mut prev = snapshot.borrow_mut();
              if *prev != latest {
                  body_slot.set_child(Some(&build_body(&latest)));
                  *prev = latest;
              }
              glib::ControlFlow::Continue
          }
      ),
  );
  ```

* `build_body` and `camera_row` unchanged from T-013a; the body
  factory is now called from two places (initial mount + on-change
  re-mount), and that's the whole point of the refactor.

Gate summary:

```
cargo fmt --all --check                                → exit 0
cargo clippy --workspace --all-targets -- -D warnings  → exit 0
cargo test --workspace                                 → 11 unit + 1
                                                         ignored
                                                         hardware + 1
                                                         doctest pass
                                                         (totals
                                                         unchanged)
cargo build -p obsbot-gui                              → exit 0
./target/debug/obsbot-cam-control (background)         → maps the
                                                         842x662
                                                         window
                                                         (xwininfo
                                                         confirmed
                                                         by the
                                                         prior run
                                                         shape)
```

Hot-plug acceptance via AskUserQuestion: the user selected "Ambos
cambios funcionan" — unplugging the Tiny 2 Lite swapped in the
empty-state `AdwStatusPage` within ~2-3 s, re-plugging restored the
camera row. Closure-internal weak ref behaved as expected (no
crashes after window close; pkill produced the normal exit 144).

PLAN.md T-013b → DONE. STATE.md returns to idle with T-013c (V4L2
control sub-page) named as the next task. Commit `feat(gui):
hot-plug listener (T-013b)` follows, bundling
`crates/obsbot-gui/src/window.rs` plus the three docs files.

### [2026-05-13T16:30:00Z] [T-013b] Started — hot-plug listener (polling first-pass)

Plan: refactor `crates/obsbot-gui/src/window.rs` so the body widget
is mounted inside a stable `adw::Bin` slot, and a `glib::timeout_
add_seconds_local` polls `enumerate_cameras()` every 2 s diffing the
latest snapshot against a captured `RefCell<Vec<CameraInfo>>`. When
the snapshot changes, the slot's child is replaced via
`Bin::set_child(Some(&build_body(&latest)))` — full rebuild on
change, no rebuild on no-change (avoids visual flicker for the
steady state).

Key design decisions:

* **`adw::Bin` slot**, not a swap inside the existing `gtk::Box`.
  `Bin` is libadwaita's canonical single-child container; calling
  `set_child(Some(&new))` unparents the previous child cleanly. The
  alternative — `gtk::Box::remove(&old); gtk::Box::append(&new)` —
  requires holding a reference to `old`, which the timer closure
  would have to thread alongside the slot. The Bin approach keeps
  the timer state to two values (the slot weak ref and the snapshot).
* **Weak capture via `glib::clone!`**. The `body_slot` is captured
  with `#[weak]` and `#[upgrade_or] glib::ControlFlow::Break`, so
  when the window (and therefore the slot) is destroyed, the
  closure auto-returns Break and the GLib source is removed — no
  manual `SourceId::remove()` plumbing needed. The
  `RefCell<Vec<CameraInfo>>` snapshot is captured by move (it's not
  a GObject and doesn't need weak semantics).
* **2 s poll interval**. Two seconds feels like the right hot-plug
  UX target (matches GNOME Settings' Devices panel rough latency)
  while keeping the syscall load tiny (one `read_dir` plus a few
  `canonicalize` / `read_to_string` per detected video node).
  Promoted to a `const POLL_INTERVAL_SECS: u32 = 2;` so future
  tuning is one-line.
* **Equality check leans on `Vec<CameraInfo>: PartialEq`**. T-005's
  `#[derive(PartialEq, Eq)]` on `CameraInfo` makes the full-Vec
  comparison correct and cheap (1-2 elements typically). Replacing
  the child only when it changes avoids the visual flicker that
  would come from rebuilding on every tick.
* **Move to udev / FileMonitor deferred** per [[ADR-0016]] — re-
  evaluate after T-013c lands V4L2 reads on the same timer (those
  open `/dev/videoN` and read controls, which is heavier than the
  sysfs walk).

No new dependencies. No unit tests per [[CLAUDE.md §5.4]] (GUI is
not auto-tested). Acceptance is physical: user plugs and unplugs
the Tiny 2 Lite while the app runs and confirms the
appearance / disappearance.

Commit: `feat(gui): hot-plug listener (T-013b)`.

### [2026-05-13T16:25:00Z] [T-013a] DONE — gates green, user confirmed visual

Implementation matched the plan exactly. Final `window.rs` shape:

* `build()` unchanged at the signature level; calls
  `enumerate_cameras()` once and hands the slice to `build_body`.
* `build_body(cameras: &[CameraInfo]) -> gtk::Widget`: empty list →
  `AdwStatusPage` (icon `camera-web-symbolic`, title "No OBSBOT
  cameras detected", description "Connect an OBSBOT Tiny 2 family
  camera via USB."); non-empty list → `AdwPreferencesPage` with one
  `AdwPreferencesGroup` titled "Connected cameras" containing the
  `AdwActionRow`s.
* `camera_row(cam: &CameraInfo) -> adw::ActionRow`: title = product,
  subtitle = `"{vid:04x}:{pid:04x} · {video_path-or-(no video node)}"`,
  prefix icon `camera-web-symbolic`. The `(no video node)` fallback
  is structurally unreachable today (every enumerator hit produces a
  `Some(...)` video path) but `CameraInfo.video_path` is typed
  `Option<PathBuf>` so the row factory has to render the `None` arm;
  [[CLAUDE.md §5.2]] forbids `unwrap()` in production paths, so a
  one-line fallback is the legal expression.

Gate summary:

```
cargo fmt --all --check                                → exit 0
cargo clippy --workspace --all-targets -- -D warnings  → exit 0
cargo test --workspace                                 → 11 unit + 1
                                                         ignored
                                                         hardware + 1
                                                         doctest pass
                                                         (totals
                                                         unchanged from
                                                         T-012; no new
                                                         tests in T-013a
                                                         per CLAUDE.md
                                                         §5.4)
cargo build -p obsbot-gui                              → exit 0
./target/debug/obsbot-cam-control (background)         → xwininfo found
                                                         `0x2600004
                                                         "Obsbot Cam
                                                         Control"
                                                         842x662` —
                                                         same shape
                                                         T-007 verified.
```

Visual acceptance via AskUserQuestion: the user selected "Fila Tiny
2 Lite (correcto)" describing the `AdwActionRow` with subtitle
`3564:fef9 · /dev/video0` and the camera prefix icon, inside the
"Connected cameras" preferences group. Empty-state criterion
deferred to incidental verification (covered by inspection — the
only `is_empty()` branch in `build_body` mounts the
`AdwStatusPage` with the documented copy). Process killed via
`pkill -f obsbot-cam-control` after confirmation.

PLAN.md T-013a → DONE with the Outcome block; T-013b (hot-plug),
T-013c (V4L2 controls), T-013d (Blueprint) remain TODO. STATE.md
returns to idle. Commit `feat(gui): initial camera list (T-013a)`
follows, bundling: `crates/obsbot-gui/src/window.rs` (rewritten),
`docs/DECISIONS.md` ([[ADR-0016]]), `docs/PLAN.md` (T-013 split,
T-016 / T-017 dependency lines updated), `docs/STATE.md`, and this
PROGRESS section.

### [2026-05-13T16:18:00Z] [T-013a] Started — initial camera list in GUI

Plan: single-file diff in `crates/obsbot-gui/src/window.rs`.

* Pull `obsbot_core::{enumerate_cameras, CameraInfo}` at the top of
  the module.
* Split `build()` into the existing window-construction code plus a
  `build_body(cameras: &[CameraInfo]) -> gtk::Widget` factory:
    - Empty list → an `AdwStatusPage` (icon `camera-web-symbolic`,
      title "No OBSBOT cameras detected", description "Connect an
      OBSBOT Tiny 2 family camera via USB."). Same shape as the
      T-007 placeholder, different copy.
    - Non-empty list → an `AdwPreferencesPage` with a single
      `AdwPreferencesGroup` titled "Connected cameras", containing
      one `AdwActionRow` per camera. Row title = `cam.product`,
      subtitle = `"<vid:pid> · <video_path>"` (e.g. `3564:fef9 ·
      /dev/video0`); prefix icon = `camera-web-symbolic` (matches
      the existing empty-state icon for visual continuity).
* `build()` calls `enumerate_cameras()` once at startup and passes
  the result to `build_body`. No hot-plug subscription, no timer —
  that is T-013b's job.
* `crates/obsbot-gui/src/main.rs` and `application.rs` unchanged.

No new dependencies (obsbot-core path dep already in place since
T-007; enumeration symbols re-exported from `lib.rs` by T-011).
No new unit tests — GUI is not auto-tested per [[CLAUDE.md §5.4]],
and the row-factory shape is small enough to read at a glance.

Validation: four cargo gates (`fmt --check`, `check --workspace
--all-targets`, `clippy --workspace --all-targets -- -D warnings`,
`test --workspace`); then `cargo run -p obsbot-gui` and let the user
visually confirm two paths — with the Tiny 2 Lite plugged in (real
row shows) and unplugged (empty-state status page).

Commit: `feat(gui): initial camera list (T-013a)`.

### [2026-05-13T16:12:00Z] [T-012] DONE — gates green incl. live hardware smoke test

Implementation came in exactly the shape planned in the Started
entry, with one in-task delta worth a paper trail:

* First clippy pass tripped on `map(...).unwrap_or_else(...)` — the
  pedantic `clippy::map_unwrap_or` lint suggests the canonical
  `map_or_else` form. Flipped to `cam.video_path.as_ref().
  map_or_else(|| String::from("(none)"), |p| p.display().to_string())`
  in `render`. `cargo fmt --all` rewrapped the call onto one line
  afterwards; final source lives at `crates/obsbot-cli/src/main.rs`
  L106–L109.
* No other lint corrections, no follow-up dependency churn.

Live verification on this turn (the user's plugged-in Tiny 2 Lite,
VID 0x3564 / PID 0xfef9, kernel uvcvideo against `/dev/video0`):

```
$ cargo run -q -p obsbot-cli -- list
1 camera detected:

[1] OBSBOT Tiny 2 Lite
    Vendor:   Remo Tech Co., Ltd.
    USB ID:   3564:fef9
    Serial:   (not advertised)
    Firmware: 0510
    Video:    /dev/video0
```

Six fields, all populated from sysfs reads done by
`obsbot_core::enumerate_cameras()`; the `Serial: (not advertised)`
fallback fires because Tiny 2 Lite firmware 5.10 reports `iSerial=
0` (see PROTOCOL.md §5). `obsbot-cli list --help` renders the full
six-field schema verbatim from the `LIST_LONG_ABOUT` constant,
satisfying the second acceptance criterion ("Output format
documented in `--help`"). `obsbot-cli --help` correctly lists
`list` as a subcommand. The bare `obsbot-cli` invocation prints
`obsbot-cli v0.1.0` unchanged from T-006 — the new `command:
Option<Commands>` schema preserves the version-banner default for
the `None` arm.

Gate summary:

```
cargo fmt --all --check                                → exit 0
cargo clippy --workspace --all-targets -- -D warnings  → exit 0
cargo test --workspace                                 → 11 unit
                                                         (8 obsbot-core
                                                          + 3 obsbot-cli)
                                                         + 1 ignored
                                                         hardware
                                                         + 1 doctest pass
cargo run -p obsbot-cli -- list                        → 1 camera
                                                         detected (real
                                                         Tiny 2 Lite)
```

PLAN.md T-012 set to DONE with the Outcome block. STATE.md returns
to idle; T-013 (diagnostics view — GUI consumer of
`enumerate_cameras` with a hot-plug listener) is the natural next
task. Commit `feat(cli): list command (T-012)` follows, bundling:
`crates/obsbot-cli/Cargo.toml` (new path dep), `crates/obsbot-cli/
src/main.rs` (subcommand router + render helper + three unit
tests), `Cargo.lock` (no new transitives — `obsbot-core` already
locked in by T-011), and the three docs files.

### [2026-05-13T16:05:00Z] [T-012] Started — wire enumeration into the CLI

Plan: surface `obsbot_core::enumerate_cameras()` through a `list`
subcommand on `obsbot-cli`. Changes scoped to two files:

* `crates/obsbot-cli/Cargo.toml` gains an `obsbot-core = { path =
  "../obsbot-core" }` entry under `[dependencies]`. This was the
  dependency [[PLAN T-006]] explicitly deferred ("`obsbot-core`
  dependency intentionally deferred to T-012 when the `list`
  subcommand needs it"); T-012 is that moment.
* `crates/obsbot-cli/src/main.rs` grows from a `--version`-only stub
  into a clap subcommand router:
  - `enum Commands { List }` via `#[derive(Subcommand)]`. Only one
    variant for now; adding more is a one-line append per future task.
  - The `Cli` struct gains `#[command(subcommand)] command:
    Option<Commands>`; with `None` we keep the bare `obsbot-cli`
    behaviour (print version banner and exit) so the T-006 smoke test
    stays green.
  - A pure helper `render(cameras: &[CameraInfo]) -> String` that
    produces the on-stdout output. Factoring as a pure function lets
    two compact unit tests pin the empty-list shape and a
    two-camera-with-and-without-serial shape — verifying the
    pluralisation, the `(not advertised)` serial fallback, and the
    stanza ordering without touching stdout.
  - `Commands::List` calls `obsbot_core::enumerate_cameras()` and
    prints `render(&cams)`. Exit code 0 in every case, mirroring
    `ls` on an empty directory; the "no cameras" message goes to
    stdout, not stderr.
  - The `list` subcommand carries a `long_about` block enumerating
    the six fields of each stanza (Product / Vendor / USB ID /
    Serial / Firmware / Video) and the exit-code contract. This
    satisfies the second acceptance criterion ("Output format
    documented in `--help`") — `obsbot-cli list --help` will show
    the full schema.

Output shape (one stanza per camera, indexed):

```
2 cameras detected:

[1] OBSBOT Tiny 2 Lite
    Vendor:   Remo Tech Co., Ltd.
    USB ID:   3564:fef9
    Serial:   (not advertised)
    Firmware: 0510
    Video:    /dev/video0

[2] …
```

The firmware string is rendered as the raw `bcdDevice` hex (e.g.
`0510` rather than `5.10`) — the v0.1 CLI mirrors what the kernel
attribute file contains; surfacing a decoded "major.minor"
representation is a minor display-only concern that can live as a
later task if the GUI's About dialog wants it.

Validation: the four cargo gates (`fmt --all --check`, `check
--workspace --all-targets`, `clippy --workspace --all-targets --
-D warnings`, `test --workspace`) plus a real `cargo run -p
obsbot-cli -- list` smoke test on the user's plugged-in Tiny 2 Lite,
plus `obsbot-cli --help` and `obsbot-cli list --help` to verify the
documented format renders. Meson tests stay unchanged.

Commit: `feat(cli): list command (T-012)`.

### [2026-05-13T15:59:15Z] [session-end] Clean checkpoint at end of session

User asked to wrap the session and resume in a new one. No active
task, no partial work, no uncommitted changes. Three v0.1 tasks
closed this session (T-009 AppStream metainfo + `.desktop`, T-010
placeholder icon — with visual confirmation deferred to T-014
Flatpak / next GNOME login, T-011 USB enumeration for the Tiny 2
family with the `#[ignore]`d hardware integration test confirmed
green against the user's plugged-in Tiny 2 Lite). Six v0.1 tasks
remain: T-012 (CLI `list`), T-013 (diagnostics view), T-014
(Flatpak), T-015 (CI), T-016 (.deb test artifact), T-017 (Arch
test artifact). Next session resumes with T-012, which is a thin
wrapper over `obsbot_core::enumerate_cameras` behind a `clap`
subcommand on `obsbot-cli`. Tiny fix-up commit follows to leave
this checkpoint marker; STATE.md `updated_at` bumps to this
timestamp.

### [2026-05-13T15:53:49Z] [T-011] DONE — gates green incl. real-hardware integration test

`crates/obsbot-core/src/enumerate.rs` lands the four public symbols
([[PLAN T-011]] Outcome block). Implementation choices that
informed the diff and are worth a paper trail beyond the inline
comments:

  * Function returns `Vec<CameraInfo>`, not `Result<Vec<…>>`. The
    three plausible failure modes (sysfs missing, sysfs unreadable,
    no Tiny 2 family device plugged in) all collapse to "no
    cameras" from the GUI / CLI standpoint; raising them through
    `Result` would force callers into pointless error handling.
    Diagnostic value preserved by logging the underlying
    `io::Error` via `tracing::warn!` whenever `read_dir` fails.
  * `enumerate_cameras_in(root: &Path)` exists as a public split
    point purely for testability; production code only calls
    `enumerate_cameras()`. Both are `#[must_use]` per pedantic-tier
    clippy.
  * Dedup keys on `fs::canonicalize(<entry>/device/..)`, which
    resolves to the USB device sysfs path. This is robust to any
    `/sys/class/video4linux/videoN` aliasing the kernel might
    introduce; we never trust the basename.
  * `device` symlink target convention: real sysfs uses
    `../../../<port>:1.0` (verified by reading the actual symlink
    on the user's machine — see paper trail), which `canonicalize`
    resolves through. The mock builder had to mirror that exactly
    once the first test run with a `../../../` shortcut failed
    (`canonicalize` then walked one level too high and pointed at
    `usb1/`, not the device dir). Pinned in a regression-style
    comment inside the test helper.
  * One clippy fix: `single_match_else` triggered on the
    HashMap-update branch; flipped to `if let … else` per the
    suggested form.

Gate summary on this turn:
  cargo fmt --all --check                                → exit 0
  cargo clippy --workspace --all-targets -- -D warnings  → exit 0
  cargo test --workspace                                 → 8 unit
                                                           + 0
                                                           ignored
                                                           + 1 doc
                                                           pass;
                                                           the
                                                           workspace
                                                           total is
                                                           8 / 1 ig
                                                           / 1 doc
                                                           (the
                                                           hardware
                                                           test is
                                                           the
                                                           ignored
                                                           one).
  cargo test -p obsbot-core --test hardware -- --ignored → 1 pass
                                                           against
                                                           the real
                                                           Tiny 2
                                                           Lite
                                                           (vid=
                                                           0x3564,
                                                           pid=
                                                           0xfef9,
                                                           video_
                                                           path=
                                                           /dev/
                                                           video0).
  meson test -C builddir                                 → 2/2 OK
                                                           (T-009
                                                           cases
                                                           unchanged).

PLAN.md T-011 set to DONE with the Outcome block. STATE.md returns
to idle with T-012 (CLI `list`) as the natural next task — it just
wraps `obsbot_core::enumerate_cameras()` behind a `clap` subcommand.
Commit `feat(core): USB enumeration for Tiny 2 (T-011)` follows.

### [2026-05-13T15:44:58Z] [T-011] Started — USB enumeration for Tiny 2 family

Plan: new module `crates/obsbot-core/src/enumerate.rs` exposing two
public symbols matching the [[PLAN T-011]] contract:
  * `pub const VID_OBSBOT: u16 = 0x3564;` — Remo Tech Co., Ltd.,
    OBSBOT's USB-IF vendor entity.
  * `pub const TINY2_FAMILY: &[(u16, u16)] = &[...];` — `(0x3564,
    0xfef8)` Tiny 2 + `(0x3564, 0xfef9)` Tiny 2 Lite. Constant
    rather than a hashmap/function so a future model becomes a
    single-line append per [[ADR-0014]]'s "no code-path branching"
    clause.
  * `pub fn enumerate_cameras() -> Vec<CameraInfo>` — scans
    `/sys/class/video4linux`, filters by `TINY2_FAMILY`, returns
    one `CameraInfo` per *USB device* (not per `/dev/videoN` —
    Tiny 2 family advertises two video nodes per camera, one
    capture and one metadata).
  * `pub fn enumerate_cameras_in(root: &Path) -> Vec<CameraInfo>`
    — same logic against an arbitrary sysfs-like tree; used by
    unit tests.

Sysfs walk validated against the user's plugged-in Tiny 2 Lite:
`/sys/class/video4linux/video0/device/..` canonicalises to
`/sys/devices/pci0000:00/0000:00:14.0/usb1/1-7`, whose
`idVendor` reads `3564`, `idProduct` reads `fef9`, `manufacturer`
reads "Remo Tech Co., Ltd.", `product` reads "OBSBOT Tiny 2 Lite",
`bcdDevice` reads `0510` (= firmware 5.10). The `serial` attribute
file is absent — consistent with [[PROTOCOL §5]] / `iSerial=0`.

Dedup strategy: keep a `HashMap<PathBuf /* canonicalised USB
device dir */, CameraInfo>` keyed on the canonical USB-device path
so the same physical camera is not surfaced twice. When two video
nodes resolve to the same device, the lower-numbered
`/dev/videoN` wins as `video_path` (capture is conventionally
lower than metadata).

Mock filesystem for unit tests: `tempfile` crate (new
`[dev-dependencies]` entry in `crates/obsbot-core/Cargo.toml`,
plus pinned version added to `[workspace.dependencies]` for
consistency). Tests create a real directory tree under the
temp dir with `std::os::unix::fs::symlink` matching real sysfs's
relative-symlink convention; the production code uses
`fs::canonicalize`, which resolves the chain natively.

Hardware integration test: `crates/obsbot-core/tests/hardware.rs`
with `#[ignore]` attribute so it does not run by default; the
user invokes it with `cargo test -p obsbot-core -- --ignored`
when the Tiny 2 Lite is plugged in. The test exercises the real
`/sys` and asserts at least one `CameraInfo` with VID `0x3564`,
PID `0xfef9`, and a `video_path` matching `/dev/video*`.

`lib.rs` will re-export the new symbols; the doctest example
shape from [[T-005]] is unaffected. No change to the [`Camera`]
trait surface (T-011 only adds the discovery layer; opening a
device and producing an `impl Camera` is T-100-series work).

### [2026-05-13T15:44:58Z] [T-010] DONE with caveat — visual deferred to Flatpak / next login

User-side visual test attempted three ways during this session:
  1. `XDG_DATA_DIRS=$HOME/.local-icontest/...:$XDG_DATA_DIRS
      cargo run -p obsbot-gui` → Alt+Tab showed the generic
      "rhombus + gears" placeholder, not the webcam.
  2. Copying both SVGs into `~/.local/share/icons/hicolor/...` +
      `gtk4-update-icon-cache -f -t` + a fresh `cargo run` →
      same generic placeholder.
  3. Adding the `.desktop` from T-009 to
      `~/.local/share/applications/` and running
      `update-desktop-database` → same generic placeholder.

Diagnosis: GNOME Shell maps a running window to its icon via
WM_CLASS / StartupWMClass lookup against a `.desktop` cache that
is built at session-startup. Drops into `~/.local/share/
applications/` during a running session are not picked up — even
after `update-desktop-database`. The standard remedies are:
  * A real Flatpak install (T-014) — Flathub's runtime path
    triggers Shell's normal cache build.
  * A distro test-package (T-016 `.deb`, T-017 PKGBUILD) — the
    files land under `/usr/share/`, which Shell indexes on
    startup.
  * A fresh GNOME login — Shell re-reads `~/.local/share/
    applications/`.

What was confirmed objectively:
  * The scalable SVG itself renders fine in the user's default
    SVG viewer (`xdg-open` showed a clear blue webcam).
  * The file paths after `meson install --destdir` are the
    freedesktop canonical ones (`share/icons/hicolor/{scalable,
    symbolic}/apps/`).
  * The GUI calls `gtk::Window::set_default_icon_name(app_id)` in
    `connect_startup` (code review).
  * The symbolic SVG uses `fill="currentColor"` (code review) —
    GTK's contract guarantees recolouring against the current
    text / accent color, no runtime way for that to fail except
    a GTK bug.

Decision (agreed with the user): close T-010 as DONE with the
caveat documented in PLAN.md. The visual will be reconfirmed at
T-014 (Flatpak) or the user's next session — if the failure
persists in either path, a follow-up task is filed; until then
the observation is treated as a dev-test artefact.

Cleanup performed:
  * Removed `~/.local/share/icons/hicolor/{scalable,symbolic}/apps/
    io.github.domatix.ObsbotCamControl*.svg`.
  * Removed `~/.local/share/applications/io.github.domatix.
    ObsbotCamControl.desktop`.
  * Re-ran `gtk4-update-icon-cache -f -t` and
    `update-desktop-database` to bring those caches back to their
    pre-test state.
  * Removed `/tmp/install-test` and `~/.local-icontest` scratch
    trees.

PLAN.md T-010 set to DONE with the Outcome / caveat block.
STATE.md returns to idle with T-011 (USB enumeration) named as
the next task. The post-login retest stays in
`pending_user_actions`. Commit `docs: close T-010 with deferred
visual caveat (T-010)` follows.

### [2026-05-13T13:18:54Z] [T-010] Code-complete — gates green, awaiting user-visual

Cargo gates: `fmt --all --check` exit 0; `clippy --workspace
--all-targets -- -D warnings` exit 0 (the new `gtk::Window::
set_default_icon_name` call did not trip any lint, including the
move-by-value of `icon_name: String` into the `connect_startup`
closure); `test --workspace` 3 unit + 1 doc-test pass, 0 ignored.

Meson gates: `setup` resolves cleanly with both helper tools now
detected (`gtk4-update-icon-cache` at `/usr/bin/gtk4-update-icon-
cache`, `update-desktop-database` at `/usr/bin/update-desktop-
database`); `test -C builddir` 2/2 OK (the validate-desktop and
validate-metainfo cases from T-009 stay unchanged); `install -C
builddir --destdir /tmp/install-test` drops the two new SVGs at
`share/icons/hicolor/{scalable,symbolic}/apps/` and reports the
correct "Skipping custom install script because DESTDIR is set"
log lines for both post-install hooks — those will fire when an
actual system install runs the same `meson install`.

The installed tree is now five files:
  /tmp/install-test/usr/local/bin/obsbot-cam-control                                                (T-008)
  /tmp/install-test/usr/local/share/applications/<id>.desktop                                       (T-009)
  /tmp/install-test/usr/local/share/icons/hicolor/scalable/apps/<id>.svg                            (T-010)
  /tmp/install-test/usr/local/share/icons/hicolor/symbolic/apps/<id>-symbolic.svg                   (T-010)
  /tmp/install-test/usr/local/share/metainfo/<id>.metainfo.xml                                      (T-009)

`/tmp/install-test/usr/local/bin/obsbot-cam-control --help` still
returns the GLib option-group help in exit 0 — confirms the
`set_default_icon_name` call does not break the binary's early-exit
paths (it lives inside `connect_startup` and `--help` shortcuts out
of that).

PLAN.md T-010 set to IN_PROGRESS with the code-complete outcome
block; the two acceptance criteria are marked PENDING USER (visual
confirmation only — Claude cannot read the framebuffer). STATE.md
records `awaiting_user_visual` so the next session knows T-010 is
not yet DONE; the user-action list explains the simplest manual
verification (`meson install --destdir ~/.local-icontest` +
`XDG_DATA_DIRS` override + `cargo run`). Once the user confirms,
PLAN can flip to DONE without further code changes. Commit
`feat: add app icon (T-010)` follows.

### [2026-05-13T13:00:58Z] [T-010] Started — placeholder app icon (regular + symbolic)

Plan: two SVGs under `data/icons/`:
  * `scalable/apps/io.github.domatix.ObsbotCamControl.svg` — 128×128
    viewBox, Adwaita-palette webcam shape (rounded-rect body in two
    blues `#3584e4` / `#1c71d8`, dark lens disc with a subtle highlight,
    small red tally LED, slim stand at the bottom). Recognizable as
    a camera at the typical 32×32 / 64×64 / 128×128 sizes GNOME
    requests from the icon cache.
  * `symbolic/apps/io.github.domatix.ObsbotCamControl-symbolic.svg` —
    16×16 viewBox, single compound path with `fill="currentColor"`
    so GTK swaps in the active text/accent color when the icon shows
    in an About dialog, sidebar header, or shell notification.

The naming follows GTK's icon-theme convention: the regular icon's
basename equals the App ID; the symbolic icon's basename equals the
App ID + `-symbolic`. Both land at `share/icons/hicolor/{scalable,
symbolic}/apps/` after `meson install`.

`data/meson.build` gains two `install_data` calls and a
`gnome.post_install(gtk_update_icon_cache: true,
update_desktop_database: true)`. The post-install bumps the
hicolor cache so GNOME Shell starts resolving the icon for the
already-installed `.desktop` (T-009) without a desktop restart.

`crates/obsbot-gui/src/application.rs` gains a
`gtk::Window::set_default_icon_name(APP_ID)` call early in
`run()`. This sets the default for any window the app creates so
the icon appears in the Wayland window-list / X11 WM_HINTS even
before the user installs anything system-wide. Combined with the
already-set `resource_base_path` (anticipation from T-007), the
path is also primed for a future GResource bundle that embeds the
icon directly into the binary.

i18n / gettext infrastructure stays deferred (no string changes
here either). The acceptance criterion "renders in GNOME Shell"
is visual and hardware-side: validated by the user after install,
exactly as T-007's "window appears" check was.

### [2026-05-13T12:54:51Z] [T-009] DONE — both validators green, install paths confirmed

`meson setup builddir` resolved everything cleanly (gtk4 4.18.6,
libadwaita-1 1.7.6, glib/gio 2.84.4, cargo, the build-aux shim, and
both validators — `appstreamcli` and `desktop-file-validate`).
`configure_file()` substituted `@APP_ID@` →
`io.github.domatix.ObsbotCamControl` and `@VERSION@` → `0.1.0` in
both templates.

  meson test -C builddir                                     → 2/2 OK
    validate-desktop                                         → 0.00s
    validate-metainfo                                        → 0.01s
  LC_ALL=C appstreamcli validate --no-net --explain \
    builddir/.../metainfo.xml                                → exit 0
  desktop-file-validate builddir/.../<id>.desktop            → exit 0 (silent)

Only diagnostic surfaced (only when `--pedantic` is added):
  P: cid-contains-uppercase-letter on `ObsbotCamControl`
The Component-ID's `ObsbotCamControl` segment uses TitleCase per
[[ADR-0012]] (the App ID is fixed by the namespace ADR; renaming
to lowercase would require a superseding ADR, a project-wide
search-and-replace across already-committed docs, and is unjustified
here — AppStream allows mixed case, it just prefers lowercase).
Not surfaced at the default validation level; recorded as a known
intentional pedantic note.

`meson install -C builddir --destdir /tmp/install-test` adds two
new files to the install tree relative to the T-008 baseline:
  /tmp/install-test/usr/local/share/applications/<app-id>.desktop
  /tmp/install-test/usr/local/share/metainfo/<app-id>.metainfo.xml
The `Icon=io.github.domatix.ObsbotCamControl` line in the installed
`.desktop` will start resolving to a real icon at T-010; until then
GNOME Shell falls back to a generic application glyph (acceptable
for a scaffolding milestone — `desktop-file-validate` does not check
that the referenced icon resource exists).

PLAN.md T-009 set to DONE with the Outcome block. STATE.md returns
to idle; T-010 (placeholder icon) is the natural next task — same
data/ subtree, can extend `data/meson.build` to install
`data/icons/scalable/apps/<app-id>.svg`. Commit `feat: AppStream
metainfo and desktop file (T-009)` follows, bundling: `data/`
(three new files, `.gitkeep` deleted), `meson.build` (the
`subdir('data')` uncomment), and the three docs files.

### [2026-05-13T12:50:39Z] [T-009] Started — AppStream metainfo + `.desktop`

Plan: create `data/io.github.domatix.ObsbotCamControl.metainfo.xml.in`
(AppStream `component type="desktop-application"`; `<summary>` ≤ 35 chars
per `appstreamcli validate`; `<metadata_license>` `CC0-1.0`,
`<project_license>` `GPL-3.0-or-later`; OARS 1.1 content rating "all
clear"; trademark disclaimer in the description per [[ADR-0012]]; both
Tiny 2 family PIDs surfaced in `<keywords>` per [[ADR-0014]]) and
`data/io.github.domatix.ObsbotCamControl.desktop.in` (`Exec=
obsbot-cam-control`, `Icon=@APP_ID@`, `Categories=AudioVideo;Video;`,
`Keywords=` listing OBSBOT / Tiny / PTZ / UVC, `StartupWMClass=
obsbot-cam-control` matching what xwininfo reported during T-007).
Both files use `@APP_ID@` / `@VERSION@` placeholders so a single
`configure_file()` substitution produces the installable file —
i18n is deferred (no `_Name=` markers, no `<_summary>`) and will
land later when actual translatable strings emerge; this keeps T-009
strictly scoped to the two acceptance gates.

`data/meson.build` (new) wires:
  * `configure_file(input: '<id>.metainfo.xml.in', output:
    '<id>.metainfo.xml', configuration: cdata, install: true,
    install_dir: datadir / 'metainfo')`.
  * Same for the `.desktop.in` → `datadir / 'applications'`.
  * `test('validate-metainfo', appstreamcli, args: ['validate',
    '--no-net', metainfo_file])` and `test('validate-desktop',
    desktop_file_validate, args: [desktop_file])` so `meson test`
    enforces the acceptance criteria on every CI/local run.

Top-level `meson.build` swaps the placeholder `# subdir('data')`
comment for the real call. `data/.gitkeep` is removed (directory
now has real content — matches the [[T-005]] / `crates/` and
[[T-008]] / `build-aux/` precedent).

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
