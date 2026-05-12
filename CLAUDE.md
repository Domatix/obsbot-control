# CLAUDE.md — Operating instructions for Claude Code

This file is read automatically by Claude Code at session start. It is the
single source of truth for **how** to work on this project. The **what** lives
in `docs/`.

---

## 0. Read this first, every session

When a new session starts, before doing anything else, read these files in
order. Stop and report back to the user after step 5:

1. `docs/STATE.md` — current state (tiny file, ~200 tokens). Where we are *right now*.
2. `docs/SPEC.md` — what we are building and what is out of scope.
3. `docs/ROADMAP.md` — milestone overview.
4. `docs/PLAN.md` — atomic tasks; locate the `IN_PROGRESS` task and the next `TODO`.
5. Last 3 entries of `docs/PROGRESS.md` — what happened recently.

Then summarize to the user in **3 short lines**:
- Active task (ID + one-line description).
- Last completed step.
- Proposed next step.

Wait for the user's confirmation before continuing.

If `STATE.md` says "no active task", propose the next `TODO` from `PLAN.md`.

---

## 1. The persistent-memory contract

All durable knowledge lives in files committed to git. Nothing important lives
only in the chat context. The rules:

### 1.1 `STATE.md` — the ultra-compact pointer
- Always reflects the current state, **continuously updated**.
- Maximum 30 lines.
- Updated at: task start, every significant sub-step, task end, on interruption.
- Format is fixed (see file). Machine-readable.

### 1.2 `PROGRESS.md` — append-only journal
- One entry per sub-step, not per session.
- Update **as work happens**, not at the end.
- Each entry: timestamp (UTC ISO 8601), task ID, action, outcome.
- Never edit past entries. To correct, add a new entry that supersedes.

### 1.3 `PLAN.md` — task list with states
- Tasks have IDs (`T-001`, `T-002`, …) and states (`TODO`, `IN_PROGRESS`, `DONE`, `BLOCKED`).
- Update state **before** starting work, not after.
- Adding sub-tasks mid-flight is allowed; document why in `DECISIONS.md`.

### 1.4 `DECISIONS.md` — append-only ADRs
- Add an entry whenever:
  - A non-obvious technical choice is made.
  - The plan is changed (scope, order, technology).
  - A contradiction with `SPEC.md` is found.
  - The user gives an instruction that overrides the documented approach.
- Format: date, context, decision, consequence.

### 1.5 `PROTOCOL.md` — research findings
- Append every protocol fact discovered (XU selectors, USB captures, V4L2 CID mappings).
- Cite sources. Mark unverified facts as such.

---

## 2. Git policy — autonomous and independent of doc updates

Commits and doc updates are **two separate disciplines** and neither waits for
the other.

### 2.1 When to commit
- After every atomic functional change that compiles and passes tests.
- After every documentation-only change of meaningful size.
- Never commit work-in-progress that breaks `main`.
- Never commit "checkpoint" or "wip" commits. If unfinished, leave in working
  tree and reflect status in `STATE.md` and `PROGRESS.md`.

### 2.2 Commit message format — Conventional Commits, mandatory
```
<type>(<scope>): <subject> (<task-id>)

<body, optional>

<footer, optional>
```

- `type`: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`, `build`, `ci`, `perf`, `style`.
- `scope`: `core`, `cli`, `gui`, `meson`, `flatpak`, `ci`, `docs`, or specific module.
- `subject`: imperative mood, no period, ≤72 chars.
- `task-id`: always reference the task being worked on, e.g. `(T-014)`.

Examples:
- `feat(core): add V4L2 device enumeration (T-007)`
- `docs: update PROGRESS for T-014 mid-task checkpoint (T-014)`
- `test(core): cover brightness clamp edge cases (T-015)`
- `fix(gui): release camera handle on window destroy (T-022)`

### 2.3 Pre-commit checks (mandatory before any commit touching code)
1. `cargo fmt --check` — must pass.
2. `cargo clippy --workspace --all-targets -- -D warnings` — must pass.
3. `cargo test --workspace` — must pass.

If any fails, fix before committing. Do not skip. Do not `--no-verify`.

For docs-only commits, only run a markdown lint if configured.

### 2.4 Branches
- `main` is always green: compiles, tests pass.
- Small changes go straight to `main`.
- Large or risky changes go to `feat/T-XYZ-short-name` branch and merge via PR
  (squash merge preserving conventional commit format).
- Never force-push `main`.

### 2.5 Tags
- Release tags follow semver: `v0.1.0`, `v0.2.0`, `v1.0.0`.
- A release is cut when a milestone in `ROADMAP.md` is complete.

---

## 3. Working on a task

### 3.1 Lifecycle
1. Read `PLAN.md`, pick `IN_PROGRESS` or next `TODO`.
2. Update `PLAN.md`: set state to `IN_PROGRESS`, add timestamp.
3. Update `STATE.md`: set active task.
4. Append to `PROGRESS.md`: "Started T-XYZ at <timestamp>: <one-line plan>".
5. Do the work, one sub-step at a time.
6. After each sub-step: append to `PROGRESS.md` ("T-XYZ: implemented X, tests
   green"), update `STATE.md` last-step, commit if appropriate.
7. When acceptance criteria of `PLAN.md` are met: set `DONE`, append summary to
   `PROGRESS.md`, final commit, update `STATE.md`.
8. Suggest the next task to the user.

### 3.2 Acceptance criteria
- Every task in `PLAN.md` must have explicit acceptance criteria.
- Do not mark `DONE` unless every criterion is verified.
- If a criterion cannot be verified (e.g. requires hardware the user must
  operate), mark `BLOCKED` and explain in `PROGRESS.md`.

### 3.3 When to STOP and ask the user
Stop and ask, do not proceed, when:
- A contradiction between code/spec is found.
- A decision has multiple reasonable answers with different consequences.
- An action would touch the user's hardware in a way that requires their
  presence (USB capture, plugging the camera, etc.).
- A change to `SPEC.md` or `ROADMAP.md` is needed.
- Estimated context usage of next step exceeds remaining budget.
- The user previously asked to be consulted on a class of decisions.

When asking, give the user a numbered list of options with trade-offs. Do not
ask vague open questions.

### 3.4 When NOT to ask
Do not ask, just do, when:
- Implementing a task whose acceptance criteria are clear.
- Fixing failing tests in code you just wrote.
- Refactoring within a module to satisfy clippy.
- Adding documentation to public APIs you just wrote.
- Adding test cases that increase coverage of code you just wrote.

The user asked for "minimal intervention". Honor that for mechanical work,
break that for design decisions.

---

## 4. Interruptions and resumption

### 4.1 If the user says "stop", "para", "wait":
1. Finish the current tool call cleanly (do not abort mid-write).
2. Update `STATE.md` and `PROGRESS.md` with **exact current state**: what
   was being done, in which file, on which function, what the next step would be.
3. Do **not** commit partial work.
4. Report status to user in ≤5 lines.

### 4.2 If a session ends abruptly (network, /clear, etc.):
The next session starts by reading `STATE.md` (see Section 0). The previous
session's last `PROGRESS.md` entries must contain enough detail to resume.
This is enforced by the discipline of updating per sub-step, not per session.

### 4.3 If the user starts a fresh session mid-task:
Read `STATE.md`. If it says `IN_PROGRESS`, do not start from scratch. Read the
relevant `PROGRESS.md` entries to understand last position, then propose:
"I see T-XYZ is in progress, last step was <X>. Resume?"

---

## 5. Project conventions (refer to `docs/SKILLS.md` for full rules)

### 5.1 Languages
- **Code**: English (identifiers, comments, docstrings).
- **Commits**: English.
- **Docs in `docs/`**: English (project is intended for the global GNOME community).
- **Conversation with the user**: Spanish (the user's preferred language).
- **User-facing UI strings**: English, externalized via gettext for translation.

### 5.2 Rust style
- `rustfmt` defaults. No custom formatting.
- `clippy` with `-D warnings`. No allow without justification in comment.
- Errors via `thiserror` for library crates, `anyhow` for binaries.
- No `unwrap()` or `expect()` in production paths. Tests may use them.
- Public APIs documented with `///` doc comments and at least one example.
- Module-level docs at the top of each `mod.rs` or root file.

### 5.3 GTK / libadwaita style
- Follow GNOME HIG (https://developer.gnome.org/hig/).
- UI defined in Blueprint (`.blp`) compiled to `.ui`; never hand-build widgets
  in code unless dynamic.
- Composite templates for widgets with `#[template]` macro.
- Use `adw::Application` and `adw::ApplicationWindow`.
- Settings via `gio::Settings` with a schema in `data/`.
- All user-facing strings wrapped in `gettext()` / `i18n!`.

### 5.4 Testing
- Unit tests live next to the code (`#[cfg(test)] mod tests`).
- Integration tests in `tests/` directory of each crate.
- GUI is not auto-tested (industry standard for GTK apps).
- A test that depends on hardware must be `#[ignore]`d and runnable with
  `cargo test -- --ignored`.

### 5.5 Hardware safety
- The user has a real OBSBOT Tiny 2. Never write code that could brick
  firmware, write to unknown XU selectors, or send untested USB commands
  without explicit user approval.
- Reverse-engineering protocol values: always cross-check against existing
  open-source projects (cited in `PROTOCOL.md`) before sending to the device.
- For unverified commands, propose a test plan first; ask before executing.

---

## 6. Token-budget awareness

The user pays per token. Be efficient:

- Read only what you need from large files. Use `view` with line ranges.
- Do not re-read files unchanged within the same session unless context was
  cleared.
- Prefer `grep`/`rg` over reading whole files when searching.
- Do not generate large outputs the user did not ask for.
- If approaching context limits, propose `/compact` or starting a new session
  after updating `STATE.md` and `PROGRESS.md`.

When in doubt about whether to do extra work: don't. Ask.

---

## 7. Definition of "done" for a milestone

A `ROADMAP.md` milestone is done when:
1. All its tasks in `PLAN.md` are `DONE`.
2. `cargo test --workspace` passes locally on the user's machine.
3. `cargo clippy --workspace --all-targets -- -D warnings` passes.
4. The Flatpak builds successfully (if applicable to the milestone).
5. `README.md` reflects current capabilities accurately.
6. A git tag `vX.Y.0` exists.
7. `PROGRESS.md` has a "Milestone vX.Y.0 reached" entry.

---

## 8. What this file does NOT cover

- Project scope and features → `docs/SPEC.md`
- Technical architecture → `docs/ARCHITECTURE.md`
- Concrete tasks → `docs/PLAN.md`
- Style and conventions in detail → `docs/SKILLS.md`
- Working with the AI as a human → `docs/AI_WORKFLOW.md`
- Domain terms → `docs/GLOSSARY.md`

If something is unclear or contradicts itself, **stop and ask the user**.
