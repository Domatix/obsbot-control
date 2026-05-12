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
identifier. Commit `chore: set namespace and license (T-002)` follows.

---
