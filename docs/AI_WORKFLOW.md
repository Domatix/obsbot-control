# AI_WORKFLOW — How to collaborate with Claude Code on this project

> **Audience**: humans (you, future contributors). Explains the AI-assisted
> development workflow used here, what to expect, and which commands /
> conventions matter.

---

## 1. The setup in one paragraph

This project is built with the help of Claude Code, Anthropic's terminal-based
AI coding agent. The repository contains everything needed for Claude to pick
up the work from any state: a permanent instruction file (`CLAUDE.md`), a set
of documents that describe what we're building (`docs/SPEC.md`, etc.), a
current-state pointer (`docs/STATE.md`), and a plan with discrete tasks
(`docs/PLAN.md`). You launch Claude Code in this directory, and it reads
the right files automatically.

---

## 2. First-time setup on a new machine

1. Install Claude Code:
   ```bash
   curl -fsSL https://claude.ai/install.sh | bash
   ```
   (Check official docs at https://docs.claude.com for current install
   instructions.)

2. Have your Anthropic API key ready and run:
   ```bash
   claude
   ```
   It will prompt for the key on first run.

3. Clone the repo:
   ```bash
   git clone <repo-url> obsbot-control
   cd obsbot-control
   ```

4. Start your first session:
   ```bash
   claude
   ```

5. At the prompt, paste the contents of `INIT_PROMPT.txt` (provided in the
   scaffolding tarball, only relevant the very first time):

   > "This is a fresh project. Read CLAUDE.md, then docs/STATE.md, then
   > docs/SPEC.md, docs/ROADMAP.md, docs/PLAN.md, and the last 3 entries of
   > docs/PROGRESS.md. Then summarize where we are and propose T-001 as the
   > first task."

   After that first session, you can simply say "continue" or "let's work
   on the next task" and Claude follows the procedure documented in
   `CLAUDE.md` §0.

---

## 3. Day-to-day workflow

### 3.1 Starting a session

```bash
cd ~/proyectos/obsbot-control
claude
```

Then say something like:

- `"continue"` — Claude reads `STATE.md` and resumes whatever was active.
- `"work on the next TODO"` — same, but starts a new task if no IN_PROGRESS.
- `"work on T-014"` — jump to a specific task.

### 3.2 During a session

- Claude reports what it's doing. You can interrupt at any time.
- When Claude asks you a question (design decision, hardware action, etc.),
  reply with the option you want.
- For mechanical work you don't need to oversee, just say "go" or "proceed".

### 3.3 Ending a session

Two ways:
- **Clean end**: ask "summarize where we are and update STATE.md", then
  exit. The next session resumes seamlessly.
- **Abrupt end**: close the terminal. If Claude was mid-task, `STATE.md` and
  `PROGRESS.md` should already reflect the latest state because they're
  updated continuously (see `CLAUDE.md` §1). On resume, it picks up.

---

## 4. Claude Code slash commands

These commands work inside the Claude Code prompt (typed by you, not by
Claude):

| Command       | What it does                                                       | When to use                                                                                       |
|---------------|--------------------------------------------------------------------|---------------------------------------------------------------------------------------------------|
| `/clear`      | Wipes the current conversation context. Files on disk are untouched. | Context got polluted (failed experiments, confusion, off-topic detour). Start fresh. **NOT** a session-end; the project state survives via `STATE.md`. |
| `/compact`    | Asks Claude to summarize the current conversation, replacing the full history with the summary. | Context window is filling but you want to continue the same line of work. Cheaper than `/clear`. |
| `/cost`       | Shows cumulative tokens and approximate USD cost of this session. | Periodically, to track budget.                                                                  |
| `/help`       | Lists all available commands.                                      | When you forget the command names.                                                              |
| `/model`      | Switches between models (Opus/Sonnet/Haiku) mid-session.           | Use Haiku for routine code, Opus for hard design decisions. Sonnet is a good default.            |
| `/permissions`| Shows / changes which tools Claude can use without asking.         | If Claude is asking too often, grant blanket bash/edit permissions for this session.             |
| `/init`       | Generates a starter `CLAUDE.md` (we already have one).             | Don't use; we have a custom `CLAUDE.md`.                                                        |
| Ctrl+C        | Interrupt Claude mid-action.                                       | When you want to stop without exiting.                                                          |
| Ctrl+D / `exit` | Exit Claude Code.                                                | At end of work session.                                                                          |

### 4.1 `/clear` in detail

`/clear` is **not** a panic button. Use it when:

- You hit a wall (Claude keeps making the same mistake) and want it to
  re-read everything from disk with a clean slate.
- You spent ~30 min exploring options and now want to execute, and the
  exploration is irrelevant going forward.
- The conversation drifted into unrelated debugging that's now done.

Do **not** use `/clear` to "save tokens between tasks". Tokens are paid
either way; what matters is whether keeping context helps the next step.
Within a session, keeping context is usually a win.

After `/clear`, just say "continue" and Claude reads `STATE.md` again.

### 4.2 `/compact` in detail

`/compact` is gentler than `/clear`. It keeps the *gist* of the conversation
as a summary. Use it when:

- You've been working productively for an hour and the context is getting
  long, but the recent reasoning still matters.
- Token usage is climbing but you don't want to lose all context.

Trade-off: the summary may lose nuance. Re-state important constraints if
they get dropped.

---

## 5. The persistent-memory model

The whole point of the `docs/` setup is that **the project's memory lives on
disk, not in the chat**.

Files Claude reads at session start (per `CLAUDE.md` §0):
1. `docs/STATE.md` — where we are right now.
2. `docs/SPEC.md` — what we're building.
3. `docs/ROADMAP.md` — milestones.
4. `docs/PLAN.md` — current tasks.
5. Last 3 entries of `docs/PROGRESS.md` — what just happened.

Total token cost: ~3-5k. Cheap.

After that, Claude only reads files it needs for the current task. It does
not re-read the spec for every commit.

### 5.1 What if I want Claude to re-read something?

Just ask: `"re-read docs/ARCHITECTURE.md and tell me how Section 3.3 affects
T-014"`. Claude will.

### 5.2 What if I edit a doc myself?

Tell Claude: `"I just edited docs/SPEC.md, re-read it before continuing"`.
Claude doesn't watch the filesystem; it only knows what it's been told.

---

## 6. When to intervene

You said you wanted **minimal intervention**. Here's what that means in
practice:

### 6.1 Always intervene

- **Design decisions with trade-offs.** Claude will stop and ask. Answer
  with the option number; don't write essays.
- **Hardware actions.** USB captures, plugging the camera, firmware tests.
  Claude can't touch your hardware; you do, while Claude guides.
- **Releases.** Approving a tag for `v0.X.0`. Publishing to Flathub.
- **Scope changes.** "Actually, let's also support Tiny 2 Lite." → updates
  `SPEC.md` + `DECISIONS.md`.

### 6.2 Don't intervene (let Claude work)

- Routine implementation tasks marked clearly in `PLAN.md`.
- Refactors, formatting, lint fixes.
- Adding tests for code Claude just wrote.
- Writing documentation for public APIs Claude just wrote.
- Routine debugging of test failures.
- Updating `PROGRESS.md`, `STATE.md`, `DECISIONS.md`.
- Committing.

### 6.3 Periodically check

Even with minimal intervention, **read commits weekly**:

```bash
git log --oneline --since="1 week ago"
```

This catches drift from your vision early.

---

## 7. Reviewing work in "minimal intervention" mode

You opted for minimal intervention. Here's a low-effort review approach:

### 7.1 Per commit

Skim `git show HEAD` after each meaningful task. Look for:

- Does the change match the commit message?
- Is the task ID in the message?
- Anything that looks like a hack, `unwrap()`, or `TODO`?

### 7.2 Per task

When `PROGRESS.md` shows a task `DONE`:

- Read the entry summary.
- Look at the linked commits.
- If GUI: run the app and try the new feature.

### 7.3 Per milestone

Before approving a tag `vX.Y.0`:

- Read the milestone section in `ROADMAP.md`.
- Verify each `ROADMAP.md` checkbox is true.
- Run the smoke-test checklist (will be in `docs/QA_CHECKLIST.md` from v0.3).

---

## 8. Token-budget hygiene

Tokens cost money. Some habits:

- **Don't paste huge files** into the prompt; Claude can `view` them.
- **Don't ask "show me the whole codebase"**. Ask for specific files.
- **Run `/cost`** every hour-ish.
- **End a session** when you finish a clean task instead of leaving the
  conversation idle.
- **Use Haiku** (`/model haiku`) for routine code tweaks; Sonnet/Opus for
  design or debugging hard problems.
- **`/compact`** at ~50% context, **`/clear`** at ~80% or on confusion.

---

## 9. Common failure modes and recovery

### 9.1 Claude proposes something contradicting `SPEC.md`

Reply: `"Stop. That contradicts SPEC.md section X. Update DECISIONS.md if
this is intentional, or revert."` Claude will course-correct.

### 9.2 Claude hallucinates an API that doesn't exist

Common with niche crates. Recovery: `"That API doesn't exist. Look up the
actual one in the crate's docs (via web_search or by running cargo doc) and
retry."`

### 9.3 Tests fail and Claude keeps tweaking blindly

Stop after 2 failed attempts: `"Stop. Read the actual test output carefully
and tell me what the failure is before changing anything."`

### 9.4 Context is poisoned (Claude keeps doing the wrong thing)

`/clear`, then re-anchor: `"continue from STATE.md"`.

### 9.5 Plan and SPEC drifted out of sync

Ask: `"audit PLAN.md against SPEC.md and report inconsistencies."` Then
decide which to update.

### 9.6 You disagree with what Claude did but the commit is already made

Say so: `"I disagree with commit abc1234 because <reason>. Revert it via
`git revert abc1234` and explain in a DECISIONS.md entry why."` History
stays clean.

---

## 10. Useful prompts to keep handy

Save these for copy-paste:

**Resume**:
> "Continue from STATE.md."

**Work on a specific task**:
> "Work on T-XYZ. Follow CLAUDE.md §3 strictly."

**Audit**:
> "Audit PLAN.md against SPEC.md and ROADMAP.md. Report inconsistencies as a
> numbered list."

**Status check**:
> "Summarize current state in 5 lines: active task, last commit, blockers,
> next step, doubts."

**End session cleanly**:
> "I'm ending the session. Make sure STATE.md and PROGRESS.md are current,
> commit any pending docs changes, and tell me where we left off in 3 lines."

**Recovery from a bad path**:
> "Stop. Revert the last change. Update DECISIONS.md explaining why we
> abandoned this approach. Propose an alternative."

**Hardware capture (deferred to you)**:
> "I'll run this on my machine. Give me the exact commands and tell me what
> to paste back."

---

## 11. Sub-agents (advanced)

Claude Code supports sub-agents: ephemeral helper sessions you can launch in
parallel for tasks that would otherwise pollute your main context. Useful
when:

- You want to search the codebase for a pattern without burning main context.
- You want to draft a long document while keeping the main session focused
  on code.

For this project, we don't rely on sub-agents heavily. They're a tool to
reach for if your main session is getting cramped and the work splits
naturally.

Syntax (subject to change with Claude Code versions): `task("sub-agent
description")` or `/agents` to manage. Check `claude --help` and official
docs for current syntax.

---

## 12. Glossary of terms used here

- **Session**: one run of `claude` in your terminal, from launch to `exit`.
- **Context**: everything Claude can "remember" right now — system prompt,
  CLAUDE.md, files it has read in this session, conversation so far. Finite.
- **Token**: roughly a word fragment. Both inputs and outputs are billed in
  tokens. ~3-5 characters per token in English.
- **Task**: an atomic unit of work in `PLAN.md` with an ID like `T-014`.
- **Milestone**: a release-worthy collection of tasks, listed in
  `ROADMAP.md`, tagged `vX.Y.0`.
- **ADR**: Architecture Decision Record. One entry in `DECISIONS.md`.
- **Sub-agent**: an ephemeral helper Claude session for side-quests.

---

## 13. When things go wrong with this workflow itself

If you find yourself fighting the workflow more than working on the project:

- The workflow can change. Edit `CLAUDE.md`, `SKILLS.md`, `AI_WORKFLOW.md`.
- Add an ADR documenting why.
- Tell Claude: `"I just changed AI_WORKFLOW.md, re-read it"`.

The point of the workflow is to help you, not box you in.
