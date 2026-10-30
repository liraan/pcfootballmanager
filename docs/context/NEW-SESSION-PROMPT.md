# New Session Prompt — PCFM

**Purpose:** prompt to paste as the first message of a new chat, to resume work on the project without losing context. Update this file when the project state or the operating rules change.

**Last updated:** 2026-10-30 (commit `787e25e`)

---

## How to use

1. Open a new chat.
2. Copy the content of the block below (from `# Resuming the PCFM project` to the last line, without the 4 backticks).
3. Paste it as the first message.
4. If the chat is resumed after a long pause, read `docs/context/SESSION-STATE.md` first: it says where we are, what is open, and what the next agreed step is.

---

## Prompt

````
# Resuming the PCFM project — new session

I am resuming work on **PC Football Manager (PCFM)**. The previous chat ended at a precise point. Here is the context so we can pick up without losing anything.

## The project

- **Repository:** https://github.com/liraan/pcfootballmanager (public)
- **What it is:** a community alternative to commercial football management games. Transparent, offline-first, deterministic. Doctrine of the Three Planes (Plane A = Ground Truth, Plane B = Observation, Plane C = Estimation).
- **Language:** technical documentation in English; conversation with me in Italian.
- **Status:** L0 (match engine) is complete and implemented. L1–L3 are planned.

## Where we are (structure, not numbers)

**For current numbers (crates, tests, ADR count), read `SESSION-STATE.md` and `STRUCTURE.md` §5. This section only gives the stable structure: it does not duplicate counts that change every session.**

- **Crates:** 9 `pcfm-*` crates: `pcfm-core`, `pcfm-model`, `pcfm-hidden`, `pcfm-stats`, `pcfm-match`, `pcfm-testkit`, `pcfm-test-runner`, `pcfm-demo`, `pcfm-fixtures`.
- **`docs/context/`** holds briefing material (not normative): `REPO-SNAPSHOT.md`, `PROJECT-IDEA-LOG.md`, `NEW-SESSION-PROMPT.md`, `SESSION-STATE.md`, `AGENT-POLICY.md`, `GITHUB-SESSION-PROMPT.md`.

## Why these files matter (the commands are in the Startup ritual)

The **Startup ritual** below has the exact commands. This section explains **why** each file is read, so you know what you are looking for.

- **`docs/context/SESSION-STATE.md`** — the bookmark. What the next step is, what was done, what is open. Read it first.
- **`docs/context/PROJECT-IDEA-LOG.md`** — history and decisions, with sources. Why the project is the way it is.
- **`docs/context/IDEAS.md`** — open and rejected ideas. What is **not** decided yet; do not implement from it.
- **`docs/context/REPO-SNAPSHOT.md`** — a briefing snapshot (tree, files, crates, metrics). **Not normative, and likely stale**: read it as history, not as state. For current numbers, use `SESSION-STATE.md` and `STRUCTURE.md` §5.

Then, when the task requires it:

- **Architecture:** `docs/ARCHITECTURE.md` (how the crates interact) and `docs/CODE_TOUR.md` (reading path).
- **Vision:** `docs/WORLD-MODEL.md` (levels, taxonomy, simulation vs presentation).
- **Doctrine (non-negotiable):** `specs/00-foundations.md` (v2.0, pillar 1 = local determinism) and `specs/_index.md`.
- **Contract:** `docs/adr/019-match-engine-contract.md` (agent-first, positional, N agents: players + referee).

## What remains to be done

**The live plan is in `docs/context/SESSION-STATE.md` §"Next steps".** Do not
maintain a second list here: it diverges. **Read the bookmark before proposing
anything.** Do not assume a step is still open because it was open in a
previous session: verify with `git log` and the bookmark. Read it for the
ordered next steps, the current priority, and the files touched.

## Operating rules (important — we established them together)

**Some rules are hard stops: violating them means stop and report. They are marked 🛑: D1, D2, C2.**

**A. Mode**

- **A1 — Execution.** One command per message, one file at a time, with countdown. At the start of each multi-step plan, declare the total number of tasks; each response states `Task i/N`. When announcing a mini-session, state the expected number of commands (`3/4`) if it is predictable; if it is not, say so (`commands expected: not predictable`) and proceed in blocks. No prose around the commands. The pasted output of each command is the implicit "ok" to proceed **if the verification is clean**; if it shows a problem, stop and fix before moving on.
- **A2 — Pausa.** The word **"pausa"** may be written by the maintainer **or** by the assistant. It suspends execution (no commands, no file edits) and opens a discussion in prose. Execution resumes **only when the maintainer writes "riprendi"** (or equivalent). The assistant does not resume on its own. Pausa does not trigger the closing ritual.

**B. Editing**
- **B1 — Tools and pitfalls.** Use `sed`/`awk` for targeted edits. Delimiters: use `@` or `~`; if the text contains apostrophes, use a heredoc (`<< 'EOF'`). BRE pitfalls: `\(` `\)` are groups, use `[(]` `[)]`; a pattern with `**` fails. For multi-line replacements with backticks/em-dash/arrows, use a `c\`/`i\`/`a\` command via quoted heredoc, not a one-liner. **Heredoc limit (tested 2026-10-29): the problem is the code fence (` ``` `), not the length.** A 44-line heredoc without fences works; a 12-line heredoc with a fence breaks. Use 4-space indentation for code blocks inside heredocs, never ` ``` `. Long heredocs are fine as long as they contain no fences. To verify an anchor with invisible characters, use `sed -n 'Np' file | cat -A`. Mojibake in output is a *display* artefact: confirm with `grep -c`. Before `c`/`a`/`i`, verify the anchor with `sed -n 'Np'`; never trust line numbers from a previous state. After `git checkout --`, re-run ALL seds, not only the ones that seem missing.
- **B2 — Verify after every edit.** `sed -n 'Np' file` to confirm. `git diff file` before `git add`. If the diff shows unrelated changes, stop. If between the sentinels there is no content, the command succeeded but was silent. If the content is not what was expected, stop and report it.
- **B3 — Output sentinel.** Every command must be wrapped in the sentinel, using this exact template: `echo ""; echo -e "\033[1m► OUTPUT START\033[0m"; <command>; echo -e "\033[1m► OUTPUT END\033[0m"; echo ""`. The sentinel text is **generated by echo**, never written literally in the command block — a line `► OUTPUT START` alone in bash fails with `bash: ►: command not found`. Bold is lost on copy/paste to chat: expected, not a bug. A command without sentinel is a violation.

**C. Git and quality**

- **C1 — Commit and push.** Work in the local repo, commit in Git Bash, push. The GitHub web interface is for emergencies only. Every change: edit locally → `git add` → `git commit`. Small, atomic commits (one per file, or per logical group), clear messages in English, imperative mood. A rename is ONE logical commit, not one per occurrence. **Push at the end of the session or at a checkpoint, not after every commit.** *Phase 1 (now):* ruleset disabled, single maintainer, direct push to `main` allowed, PR optional. *Phase 2 (when an agent gets a write token):* ruleset active, branch + PR mandatory, no direct push. See `docs/context/AGENT-POLICY.md`.
- 🛑 **C2 — Quality gate (baseline, mandatory).** The pre-commit hook (`.githooks/pre-commit`, one-off install: `git config core.hooksPath .githooks`) runs fmt + clippy + test only if Rust files are staged. The hook does **not** replace the full check: also run `cargo build --release`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `cargo fmt --check`, `cargo run --release --bin pcfm-test-runner -- --check --seed 42`. If anything is red, stop: do not close, do not push, do not "fix later".
- **C2-bis — Beyond the baseline (encouraged).** The goal is a clean, optimized, secure repository, not merely passing the commands above. If your toolchain or model capabilities allow additional checks — e.g. `cargo-audit`, `cargo-deny`, `miri`, coverage (`cargo-tarpaulin`), mutation testing, fuzzing, manual review of hot paths — run them and report the result. Do not limit yourself to the baseline.
  - **Caveat (D2 applies).** "Optimized" means *verified* optimization: reduce allocations, profile, remove redundancy, **without changing behaviour**. Any change to the simulation model requires an ADR and a `SIM_VERSION` bump. Do not introduce `unsafe`, `HashMap` in simulation paths, or other banned patterns in the name of optimization.

**D. Context and doctrine**

- 🛑 **D1 — Honesty and ideas.** If you have not read a file, say so. Do not invent numbers, commit hashes, or file contents. "I don't know" > invention. **When facing ambiguity, or when the context has not been fully read, prefer completeness over speed: read the full context before acting.** New ideas go to `docs/context/IDEAS.md` (status `sketch`, with prerequisites and open questions) **before** discussion, not after. Do not implement from an idea until it has an ADR (if it touches doctrine) or a step in `SESSION-STATE.md` (if it is a task).
- 🛑 **D2 — Doctrine and codebase.** No `unwrap`/`expect`/`panic!` in simulation paths. No `HashMap` in simulation paths. No `unsafe` outside `pcfm-bridge` (future). `pcfm-hidden` contains Plane A: no public fields, not `Serialize`, custom `Debug`; the only door is `resolve_*` + `drain_observations`; never add getters on latent values. `pcfm-testkit` has no `pcfm-*` dependencies. The simulation model is a prototype: do not "optimise" without an ADR and a `SIM_VERSION` bump. The doctrine wins: if a change contradicts `00-foundations`, a doctrine ADR is required. `docs/adr/` contains only ADRs.

**E. Session lifecycle**

- **E1 — Startup ritual mandatory.** At the start of every new session, before proposing anything, execute the Startup ritual. Not optional. If a step is impossible (e.g. no network for `git pull`), report the failure and proceed with what is possible. Never invent output.
- **E2 — Closing ritual requires explicit approval.** The closing ritual is never automatic. When the work seems complete, pause (A2) and ask: *"Shall I proceed with the closing ritual?"*, listing what it will do. Execute only after an explicit "riprendi" or equivalent. Silence, an "ok" to an unrelated question, or the natural end of a task are **not** approval. An explicit request to close ("close", "checkpoint") still requires the pause and the confirmation.

**Rule precedence:** the hard stops (D1, D2, C2) win over everything else. D1 (honesty) first. In any other conflict, ask the maintainer.

## Startup ritual (mandatory — see E1)

At the start of every new chat, **before proposing anything**, ask the maintainer to run these commands in order and paste the output. Do not assume the state described in this prompt is still true: verify it.

### 1. Repo alignment

    git status --short
    git pull --ff-only
    git log --oneline -5
    git rev-parse HEAD

Check: working tree clean (or only known untracked files), HEAD matches `Last commit verified` in `SESSION-STATE.md` (semantics S1: it points to the commit *before* the bookmark update), no unexpected commits.

### 2. Read the bookmark

    sed -n '1,160p' docs/context/SESSION-STATE.md

If >160 lines, split. Extract: last commit verified, next steps, open items, files touched.

### 3. Read all analysis reports in `analysis/` (continuity)

    ls docs/context/analysis/*.md

For each report in `docs/context/analysis/` (not `analysis/archive/`), read the executive summary and any finding that is still open. If a report says "Pass" but has unverified findings, do not trust the verdict: verify each finding with `grep`/`sed` before acting.

### 4. Read the doctrine (non-negotiable)

    sed -n '1,130p' specs/00-foundations.md

At minimum: §1 (pillars), §2 (design pillars), §3 (Three Planes), §5 (banned things).

### 5. Read the operating rules

    sed -n '1,130p' docs/context/NEW-SESSION-PROMPT.md

This very file — operating rules (A1, A2, B1, B2, C1, C2, D1, D2), rule precedence, the startup and closing rituals, what NOT to do.

### 6. Confirm the numbers

    cargo test --workspace 2>/dev/null | grep "test result"
    ls crates/ | grep pcfm | wc -l
    for f in docs/adr/*.md; do sed -n '3p' "$f"; done | grep -c 'Status:\*\* accepted'

Compare with `SESSION-STATE.md` and this prompt. If they diverge, the docs are stale — report it before proceeding.

### 7. Report to the maintainer

Say explicitly:
- What you read (files, line ranges)
- What you did **not** read (and why)
- The state of the repo (aligned? clean? numbers?)
- The next step according to `SESSION-STATE.md`
- Any divergence between docs and reality

**Do not propose a step before step 7 is done.** "I have not read X" is a valid state, not a failure.

## What NOT to do

- Do not give "lots of comments" scattered through the code: the codebase is already documented.
- Do not assume the status of an ADR from memory: read `STRUCTURE.md` §5 (single source of truth). ADR-012, 013, 015, 016, 018, 019 are **accepted**, not proposed — but verify, do not assume.
- Do not assume ADR counts: they change. Use `STRUCTURE.md` §5 for the current number.
- Do not propose Python scripts (Python is not installed). Use Bash/Git Bash.
- Do not assume CI exists: there is no `.github/workflows/`. Clippy/test/build/fmt/check are **manual**, and must be run after every change (the pre-commit hook helps for Rust).
- Do not reactivate the ruleset on `main` without explicit instruction. It is disabled in Phase 1.

## Closing ritual (mandatory — see E2)

After the maintainer approves the closing (E2), run these checks in order. If any is red, stop and fix before closing.

### 1. Repo alignment and real verification

    git status --short
    git log --oneline -5
    git rev-parse HEAD

If there are local commits not pushed:

    git push
    git log --oneline -3
    git status -sb

Then the full quality gate, as defined in **C2** (baseline mandatory) and **C2-bis** (additional checks encouraged). The baseline commands are not repeated here: C2 is the single source of truth for the gate.

If anything is red: **stop**. Do not declare the session closed. Fix first.

### 2. Documentation coherence

Compare the numbers across the files that claim them:

    grep -n 'tests\|crates\|ADR' docs/context/SESSION-STATE.md | head -10
    grep -n '^| 0' STRUCTURE.md | tail -25
    grep -n '^| \[0' specs/_index.md | tail -25
    grep -n 'test\|crate\|ADR' docs/context/NEW-SESSION-PROMPT.md | head -15

All must agree. If they diverge, one is stale: fix it, and commit the fix in the same session.

If you touched a file X (renamed it, changed its path, changed its content), verify that references to X are updated everywhere:

    git diff --name-only HEAD~N..HEAD | while read f; do
      base=$(basename "$f")
      grep -rn "$base" --include='*.md' --include='*.rs' . | head -3
    done

(replace `N` with the number of commits in this session)

### 3. Analysis of `docs/context/` from a fresh-chat perspective

The test: **if tomorrow you opened a new chat, read only `NEW-SESSION-PROMPT.md` + `SESSION-STATE.md` + the last two reports in `analysis/`, would you have enough context to resume?**

If the answer is no, the bookmark is insufficient. The bookmark is the memory of the project; if the memory is incomplete, the next session starts blind.

Check explicitly:
- Does `SESSION-STATE.md` say what the next step is? Or only what was done?
- Does it list the open items, or only the closed ones?
- Does `Files touched this session` include every file modified?
- Are there sketches in `IDEAS.md` that the bookmark should point to but does not?
- Are there ADRs in `proposed` or `deferred` state that the bookmark should surface?
- Did `analysis/` receive a new report that the bookmark does not mention?

If any answer is "no", update `SESSION-STATE.md` before closing. It is cheap now, expensive tomorrow.

### 4. Bookmark update

Update `SESSION-STATE.md`:

- `Last commit verified`: the **last commit before this checkpoint** (semantics S1: it will look stale compared to HEAD; that is expected)
- `Next steps`: ordered, with the current priority, **and the first concrete action for each** (the command to run, the file to read, the question to ask). A step without a first action is a step that the next session will skip or re-derive.
- `Files touched this session`: full list, dated
- New open items from the session (verification of previous findings, decisions deferred, sketches added)

Do not update `REPO-SNAPSHOT.md` unless its own premise says it is current: it is **not normative**, and its staleness is expected.

### 5. Commit and push (before the report)

The checkpoint commit is **one logical commit**:

    git add docs/context/SESSION-STATE.md
    git commit -m "docs(session-state): checkpoint after <what happened>"
    git push
    git status -sb

`git status -sb` must show `## main...origin/main` with no `[ahead]` or `[behind]`. If it does, the session is not closed.

### 6. Final report to the maintainer

State explicitly:
- What was done (one line per commit)
- What was verified (real tool output, not claims)
- What was **not** done (and why: blocked, deferred, out of scope)
- The state of the repo (aligned, clean, quality gate green) — verified after the push
- The next step for the next session (what `SESSION-STATE.md` says)
- **The Plan table, presented in chat** (not just written in the file). The maintainer must see it before the session ends. If you skip it, say why.

If you have not run one of the checks in this ritual, say so. "I skipped X" is honest; "I ran X" when you did not is a lie.
````
