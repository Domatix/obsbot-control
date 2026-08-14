# HANDOFF — Start here if you are taking over this project

This is the human-facing entry point for a new developer. The
machine-readable state lives in the other `docs/` files; this page
just routes you to them and gives the 5-minute orientation.

---

## 1. What this project is

`obsbot-control` is a native GNOME application (Rust + GTK 4 +
libadwaita) to control **OBSBOT Tiny 2 family** webcams (Tiny 2,
Tiny 2 Lite) over USB / UVC / V4L2 — **without** the vendor's
proprietary SDK. The end goal is GNOME Circle + Flathub.

- **App ID**: `io.github.domatix.obsbot-control`
- **License**: GPL-3.0-or-later (metadata CC0-1.0)
- **Platform**: Linux x86_64 only
- **Repo**: `github.com/Domatix/obsbot-control` (public)

Three crates:

| Crate         | Kind | Binary                  | Role |
|---------------|------|-------------------------|------|
| `obsbot-core` | lib  | —                       | device/V4L2/UVC logic, protocol |
| `obsbot-cli`  | bin  | `obsbot-cli`            | headless control |
| `obsbot-gui`  | bin  | `obsbot-control`    | the GTK app |

---

## 2. How to resume — the one rule

This repo follows a strict persistent-memory method. **Read
`CLAUDE.md` first** (project root). It tells you to read, in order:

1. `docs/STATE.md` — ultra-compact "where we are right now" pointer.
   This is always the freshest truth. Start every session here.
2. `docs/SPEC.md` — what we are building, what is out of scope.
3. `docs/ROADMAP.md` — milestones.
4. `docs/PLAN.md` — atomic tasks (`T-001`…) with states
   (`TODO`/`IN_PROGRESS`/`DONE`/`BLOCKED`) and acceptance criteria.
5. Last 3 entries of `docs/PROGRESS.md` — append-only journal,
   newest at the top of the dated section.

Other durable docs: `docs/DECISIONS.md` (ADRs — *why* things are
the way they are; read this before reversing any choice),
`docs/PROTOCOL.md` (reverse-engineered camera protocol facts),
`docs/ARCHITECTURE.md`, `docs/GLOSSARY.md`, `docs/SKILLS.md`
(full style/convention rules).

**Nothing important lives only in chat or in someone's head — it
is all in these files, committed to git.** That is the whole point;
trust them.

---

## 3. Build, test, run

Native dev build (needs the GNOME 4 / GStreamer dev stack + Rust):

```sh
meson setup builddir          # add -Dlive-preview=true for the camera preview
meson compile -C builddir
./builddir/obsbot-control            # the GUI
cargo run -p obsbot-cli -- --help        # the CLI (cargo-only; meson does not build it)
```

Pure-cargo also works for core/cli/gui (`cargo build`, `cargo test`;
the GUI needs `--features live-preview` for the preview pipeline).

**Pre-commit gates (mandatory, any commit touching code — see
`CLAUDE.md` §2.3):**

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Packaging artifacts (all in `build-aux/`, outputs land in
`build-aux/dist/`, which is git-ignored):

- **Flatpak** (supported channel, target Flathub):
  `io.github.domatix.obsbot-control.json`, runtime
  `org.gnome.Platform//50`.
- **.deb** (Debian/Ubuntu test artifact): `build-deb.sh`.
- **Arch .pkg.tar.zst** (test artifact): `build-arch.sh` + `PKGBUILD`.

---

## 4. What is left to do (read PLAN.md for the full list)

In rough priority order:

1. **T-017b — Arch validation (first task; the boss asked for an
   Arch build).** The `PKGBUILD` is refreshed for live-preview with
   all deps incl. `blueprint-compiler` (currently `pkgver=0.4.1`;
   bump to the tag being packaged — v0.4.2 — when cut). What is
   *not* done is running it: on an Arch host run
   `./build-aux/build-arch.sh`; on a non-Arch host run the
   `docker … archlinux:latest` recipe that same script prints when
   it detects you are not on Arch. Validate makepkg → `pacman -U`
   → binary executes → `pacman -R`, and drop the `.pkg.tar.zst`
   in `build-aux/dist/`. See T-017b acceptance criteria in PLAN.md.
2. **T-202** — minor bug: a grayscale filter toggled while the
   preview is off is lost on next start. Re-apply on start, or
   disable the filter buttons while preview is off.
3. **Flathub submission prep** — see the GNOME-Circle/Flathub notes;
   needs an offline `cargo-sources.json` build, screenshots, and
   making the repo public. This is the path to v1.0.
4. **T-400** (post-v1.0) — add the OBSBOT Meet (original) to the
   model matrix.

Known hardware quirk (do not "fix" it): on Tiny 2 Lite firmware
5.10, `pan_speed`/`tilt_speed` accept writes but produce no motion;
PTZ moves via discrete `pan_absolute`/`tilt_absolute` single steps.
Details in `docs/PROTOCOL.md` and `STATE.md` known_issues.

---

## 5. Hardware safety — read before touching the device

There is a real OBSBOT Tiny 2 Lite in the loop. **Never** send
untested USB / XU commands, write unknown XU selectors, or do
anything that could brick firmware without explicit approval and a
test plan. Always cross-check reverse-engineered protocol values
against the open-source projects cited in `docs/PROTOCOL.md` before
sending them to the camera. See `CLAUDE.md` §5.5.

---

## 6. Conventions you must keep

- **Commits**: Conventional Commits, `<type>(<scope>): <subject>
  (<task-id>)`, imperative, ≤72 chars, always reference a task.
  `main` is always green. No WIP/checkpoint commits.
- **Commits must NOT mention the AI workflow** or carry AI
  attribution trailers — this repo is for public release. The
  AI-assisted workflow is documented in `README.md` /
  `docs/AI_WORKFLOW.md`, not in commit messages.
- **Languages**: code/commits/docs in English; user-facing UI
  strings via gettext. (The prior maintainer conversed with the
  AI in Spanish — that is a personal preference, not a project
  rule.)
- Update `STATE.md` / `PROGRESS.md` *as you work*, not at the end.

Welcome aboard.
