# SKILLS — Methodology and Conventions

> **Purpose**: Detailed rules of "how to do things" in this project. Referenced
> from `CLAUDE.md`. If a rule here conflicts with `CLAUDE.md`, `CLAUDE.md`
> wins; flag the contradiction in `DECISIONS.md`.

---

## 1. Code quality

### 1.1 Rust

- **Edition**: 2021. **MSRV**: 1.83.
- **Formatting**: `rustfmt` with default config. `cargo fmt --check` must pass
  before every commit.
- **Linting**: `cargo clippy --workspace --all-targets -- -D warnings` must
  pass before every commit. `#[allow(...)]` requires an explanatory comment.
- **Imports**: grouped (std, external, internal) and alphabetized within each
  group. `rustfmt` handles ordering; the grouping is by manual blank lines.
- **Naming**: snake_case for functions/vars, CamelCase for types, SCREAMING
  for constants, kebab-case for crates and binaries.
- **No `unwrap()`/`expect()` in non-test code** unless paired with a comment
  proving infallibility (rare). Use `?` and `Result` everywhere.
- **No `panic!()`** in library code. Binaries may panic only on
  truly-fatal-startup conditions; prefer logging + clean exit.
- **No `unsafe`** unless wrapping a C API. Required for `nix` ioctls and
  `nusb` raw transfers; isolate `unsafe` to a single function with a comment
  proving its safety invariants.

### 1.2 Errors

- **Libraries** (`obsbot-core`): use `thiserror` to define typed errors.
  Errors must be `Send + Sync + 'static` to cross thread boundaries cleanly.
- **Binaries** (`obsbot-cli`, `obsbot-gui`): use `anyhow` at the boundary;
  convert from library errors with `?` and `.context(...)`.
- **User-facing errors** in the GUI: never display raw `Debug` output. Always
  provide a localizable, user-friendly message.

### 1.3 Logging

- **Crate**: `tracing` + `tracing-subscriber`.
- **Levels**: `error` for actionable failures, `warn` for degraded state,
  `info` for major lifecycle events, `debug` for diagnostics, `trace` for
  hot paths.
- **Default level**: `info` in release builds, `debug` in dev.
- **Override**: `RUST_LOG` env var, standard `tracing-subscriber` filter.
- **Never log secrets, device serial numbers, or PII** at info level.

### 1.4 Documentation

- Every public function, struct, enum, trait method has a `///` doc comment
  with a one-line summary and at least one example for non-trivial APIs.
- Module-level docs at the top of each `mod.rs` or root file (`//!`).
- `cargo doc --workspace --no-deps` must build without warnings.

### 1.5 Tests

- Unit tests live alongside the code in a `#[cfg(test)] mod tests` block.
- Integration tests in `crates/<name>/tests/`.
- Hardware-dependent tests: `#[ignore]`d. Documented in
  `docs/QA_CHECKLIST.md` (to be created).
- Mock the `Camera` trait for unit-testing GUI logic.
- Coverage is not enforced via threshold; reviewer judgment.

---

## 2. GTK / libadwaita

### 2.1 HIG

Follow the GNOME Human Interface Guidelines
(https://developer.gnome.org/hig/) as the rulebook. When in doubt, look at
existing GNOME Circle apps (Pika Backup, Amberol, Fractal) for examples.

### 2.2 UI definition

- UI defined in **Blueprint** (`*.blp`), compiled to `*.ui` at build time.
- Composite templates use the `#[template]` attribute macro from `gtk4-rs`.
- Hand-built widgets only when the layout is genuinely dynamic (e.g. one row
  per detected camera). Even then, prefer `gtk::Box`/`adw::Bin` patterns
  over deep hand-wiring.

### 2.3 Settings

- All persistent settings go through `gio::Settings` with a schema in
  `data/<app-id>.gschema.xml`.
- One key per atomic setting.
- For complex structures (presets list), serialize JSON in a single string
  key. Document the schema in the GSettings `<description>`.
- Per-camera state is keyed by camera serial number: prefix `cameras.<serial>.`.

### 2.4 i18n

- All user-facing strings wrapped in `gettext()` (re-exported as `i18n!`
  macro for convenience).
- Source language: English. Project-maintained translation: Spanish.
- `po/POTFILES` lists every file with translatable strings.
- New translatable strings: run `xgettext` / `meson compile -C builddir
  <app-id>-pot` to update `.pot`, then update `.po`s.

### 2.5 Resources

- Icons, UI files, CSS bundled via GResource (`*.gresource.xml`).
- Bundled at build time. No runtime file lookups for app assets.

---

## 3. Git workflow

### 3.1 Branch policy

- `main` is always green.
- Small changes (≤ 200 LoC, single concern) commit directly to `main`.
- Larger changes go to a feature branch `feat/T-XYZ-short-description`.
- Feature branches merge to `main` via squash merge, preserving the
  Conventional Commits format in the squash message.
- No long-lived feature branches: rebase on `main` at least daily.

### 3.2 Commit messages

Format (from `CLAUDE.md` §2.2):

```
<type>(<scope>): <subject> (<task-id>)

<body, optional, wrap at 72 chars>

<footer, optional>
```

**Valid types**: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`,
`build`, `ci`, `perf`, `style`.

**Body**: explain *why*, not *what* (the diff shows what).

**Footer examples**: `BREAKING CHANGE: ...`, `Co-authored-by: ...`.

### 3.3 Pre-commit hooks

Configured in `build-aux/git-hooks/pre-commit`:

1. `cargo fmt --check`.
2. `cargo clippy --workspace --all-targets -- -D warnings`.
3. `cargo test --workspace`.
4. (Docs commit) markdown lint if `mdformat` or similar present.

If any step fails, the commit is rejected. No `--no-verify` allowed.

### 3.4 Releases

- Cut a release when a `ROADMAP.md` milestone is complete.
- Tag `vX.Y.0`. Patch releases `vX.Y.Z` for fixes within a milestone.
- Update `CHANGELOG.md` (generated from Conventional Commits via
  `git-cliff` or similar — TBD task in v0.6).
- Update AppStream metainfo with release notes for the new version.

---

## 4. Hardware safety

The user has a real device. Treat it carefully:

- **Never** send untested XU writes to selectors not documented in
  `PROTOCOL.md`.
- **Never** write to firmware update endpoints. We do not support firmware
  updates.
- **Always** cross-reference XU selectors against existing open projects
  (`taxfromdk/obsbot_tiny_reversing`, `samliddicott/meet4k`,
  `aaronsb/obsbot-camera-control` reference) before implementing.
- **When in doubt**, stop and ask the user to confirm a test plan.
- **Reverse engineering** captures must be sanitized of any personal data
  (serial numbers OK; broader USB context might include other devices)
  before committing.

---

## 5. Communication with the user

### 5.1 Language

- **Code, commits, docs, comments**: English.
- **Chat conversation with the user**: Spanish.
- **User-facing UI strings**: English (source), Spanish (translated).

### 5.2 When to ask the user

See `CLAUDE.md` §3.3 and §3.4 for the canonical list. Summary:
- Ask for design decisions with multiple valid answers.
- Ask before any action involving their hardware.
- Ask before changing SPEC or ROADMAP.
- Don't ask for mechanical tasks where the answer is in `PLAN.md`.

### 5.3 How to ask

- Numbered options with trade-offs, not vague questions.
- One question at a time when possible.
- Recommend a default; let the user override.

### 5.4 Response style

- Concise. The user prefers terse, structured answers.
- Bullets and short sections OK.
- Code blocks for commands, paths, code.
- Avoid filler ("Sure!", "Great question!").

---

## 6. Working pace

### 6.1 Atomic tasks

- Each task in `PLAN.md` should be doable in a single working session
  (typically 30 min — 2 h of AI work).
- If a task balloons, split it.
- One `IN_PROGRESS` task at a time per session.

### 6.2 Test-first preference

- For pure logic (`obsbot-core`), write the test alongside or before the
  implementation.
- For GUI code, manual smoke test is acceptable.

### 6.3 Refactoring

- Refactor freely within a task scope to keep code clean.
- Cross-task refactors get their own ticket; do not slip them into unrelated
  work.

---

## 7. Self-check before declaring a task done

- [ ] Code compiles (`cargo check --workspace`).
- [ ] `cargo fmt --check` passes.
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes.
- [ ] `cargo test --workspace` passes (non-ignored tests).
- [ ] Public APIs documented.
- [ ] If GUI: visible behavior matches acceptance criteria.
- [ ] If protocol: cross-referenced against existing open projects.
- [ ] `PLAN.md` updated: state = `DONE`.
- [ ] `PROGRESS.md` has a closing entry.
- [ ] `STATE.md` reflects new active state.
- [ ] `DECISIONS.md` updated if any new ADR.
- [ ] Commit(s) made with proper Conventional Commits message and task ID.
