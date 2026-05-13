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

---
