# New Session Prompt — PCFM

**Purpose:** prompt to paste as the first message of a new chat, to resume work on the project without losing context. Update this file when the project state or the operating rules change.

**Last updated:** 2026-10-29 (commit `f3fce11`)

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
- **Performance target** (`00-foundations.md` §6): 60 fps UI, full season < 60 s on mid-range hardware. Not measured yet.

## Where we are (commit `f3fce11`, 2026-10-29)

- **9 crates:** `pcfm-core`, `pcfm-model`, `pcfm-hidden`, `pcfm-stats`, `pcfm-match`, `pcfm-testkit`, `pcfm-test-runner`, `pcfm-demo`, `pcfm-fixtures`.
- **103 tests**, clippy clean (`-D warnings`), `--check` 6/6, `--list` 6 scenarios, `--coverage` 2/26 (7%).
- **17 ADRs accepted** (001-008, 012-016, 018, 019, 020, 021), **1 proposed** (017, roles are labels), 1 template (000), 1 reading guide (`docs/PLUGINS-WORLDS-READING-GUIDE.md`, outside `docs/adr/`). **19 files** in `docs/adr/` total.
- **ADR-019 (match engine contract)** is `accepted`: agent-first, positional, N agents (players + referee). Deferred 1 (spatial iteration order), 2 (tick-cost pre-emption, selective foul), and 3 (possession representation) resolved. Deferred 4–9 open.
- **11 L0 modules** ratified + implemented.
- **`docs/context/`** holds briefing material (not normative): `REPO-SNAPSHOT.md`, `PROJECT-IDEA-LOG.md`, `NEW-SESSION-PROMPT.md`, `SESSION-STATE.md`, `AGENT-POLICY.md`.
- **`STRUCTURE.md` §5** is the single source of truth for ADR status. `modules/_index.md` shows a derived view.
- **Pre-commit hook** (`.githooks/pre-commit`, versioned): runs fmt, clippy, test only if Rust files are staged. Install with `git config core.hooksPath .githooks` (one-off).
- **Ruleset on `main`:** configured (PR required, force-push and deletion blocked, 0 approvals) but **currently disabled** (Phase 1). Reactivate before granting a write token to any agent.
- **Working tree clean.**

## The 4 files you MUST read first (in this order)

1. **`docs/context/SESSION-STATE.md`** — the bookmark: last commit, next agreed step, open items. Read this first if the chat is resumed after a pause.
2. **`docs/context/PROJECT-IDEA-LOG.md`** — history, decisions, sources, what remains open.
3. **`docs/context/REPO-SNAPSHOT.md`** — briefing snapshot: tree, files, crates, metrics. **Not normative** (see its own premise).
4. **`docs/context/IDEAS.md`** — open and rejected ideas. Read it to know what is not yet decided; do not implement from it.

Then, if you need the architecture: **`docs/ARCHITECTURE.md`** (how the crates interact) and **`docs/CODE_TOUR.md`** (reading path).

Then the vision: **`docs/WORLD-MODEL.md`** (levels, taxonomy, simulation vs presentation).

Then the doctrine: **`modules/00-foundations.md`** (v2.0, pillar 1 = local determinism, §1 football fidelity) and **`modules/_index.md`** (v2.9).

Then the contract: **`docs/adr/019-match-engine-contract.md`** (agent-first, positional, N agents: players + referee).

## What remains to be done

**The live plan is in `docs/context/SESSION-STATE.md` §"Next steps".** Do not
maintain a second list here: it diverges. Read the bookmark for the ordered
next steps, the current priority, and the files touched.

## Operating rules (important — we established them together)

0. **Work in the local repo, commit in Git Bash, push.** The GitHub web interface is for emergencies only. Every change: edit locally → `git add` → `git commit`. Commit per file (or per logical group); **push at the end of the session or at a checkpoint (rule 10), not after every commit.** Phase 1 (now): direct push to `main` allowed. Phase 2 (ruleset active): branch + PR, no direct push — see rule 1.
1. **Phase 1 vs Phase 2.** *Phase 1 (now):* ruleset disabled, single maintainer, direct push to `main` allowed, PR optional. *Phase 2 (when an agent gets a write token):* ruleset active, branch + PR mandatory, no direct push. See `docs/context/AGENT-POLICY.md`.
2. **Small, targeted edits in Git Bash.** Use `sed`/`awk` for targeted modifications. Do not send the full file in chat when it exceeds ~120 lines: it truncates. For longer files, give the exact `sed`/`awk` commands, one section at a time. A `git diff` pasted in chat truncates at ~80 lines: use `git diff | head -80` + `git diff | tail -60`, not the full diff.
3. **`sed`/`awk` delimiters.** The replacement text must not contain the delimiter. Use `@` or `~` by default. If the text contains apostrophes, use a heredoc (`<< 'EOF'`) to avoid shell quoting issues.
3bis. **`sed` pitfalls (learned the hard way).** In BRE, `\(` and `\)` are
capturing groups, not literal parentheses: use `[(]` and `[)]`. A pattern
containing `**` fails with `Invalid preceding regular expression`. For
multi-line replacements with backticks, em-dash, or arrows, use a
`c\`/`i\`/`a\` command fed via a quoted heredoc (`<< 'EOF'`), not a
one-liner. Mojibake in terminal output is a *display* artefact, not file
corruption: confirm with `grep -c` before assuming the worst.
Before a line-address change (`c`), append (`a`), or insert (`i`)
command, verify the anchor with `sed -n 'Np'` — never trust line numbers
deduced from a previous state. After `git checkout --`, re-run ALL seds,
not only the ones that seem missing.
4. **Verify after every edit.** `sed -n 'Np' file` to confirm the change. `git diff file` before `git add`. If the diff shows unrelated changes, stop.
5. **One file at a time, one command at a time, with countdown.** In the current file: edit, verify (`sed -n`, `git diff`), next step. The pasted output of each command is the implicit "ok" to proceed **if the verification is clean**; if it shows a problem, stop and fix before moving on. At the start of each multi-step plan, declare the total number of tasks; each response states `Task i/N`.
6. **Mutual honesty, zero inventions.** If you have not read a file, say so. Do not invent numbers, commit hashes, or file contents. "I don't know" > invention.
7. **Small, atomic commits.** One commit per file (or per logical group). Clear messages in English, imperative mood. A rename is ONE logical commit, not one per occurrence: splitting it leaves false sentences in `main` between commits.
8. **Verify after every code change.** The pre-commit hook (`.githooks/pre-commit`, one-off install: `git config core.hooksPath .githooks`) runs fmt + clippy + test only if Rust files are staged. The hook does **not** replace the full check: also run `cargo build --release`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `cargo fmt --check`, `cargo run --release --bin pcfm-test-runner -- --check --seed 42`. If anything is red, stop and we fix it.
9. **No `unwrap`/`expect`/`panic!` in simulation paths.** No `HashMap` in simulation paths. No `unsafe` outside `pcfm-bridge` (future).
10. **Bookmark and checkpoint.** Checkpoint at the end of the session (or at a multi-step plan completion): update `SESSION-STATE.md` with last commit, ordered next steps, files touched. Commit and push. **Bookmark S1:** `Last commit verified` in `SESSION-STATE.md` and `Last commit` in `REPO-SNAPSHOT.md` point to the last commit *before* the bookmark/snapshot update, not to HEAD; they will look stale — that is expected, do not "fix" them. "Pausa" does not trigger the checkpoint (see rule 11).
11. **"Pausa" — discursive mode.** When the maintainer writes "pausa", switch to discursive mode: prose answers, no commands, no file edits. Use it to amend, rectify, or remove parts of the workflow discursively. Resume when the maintainer says so.
12. **New ideas go to `IDEAS.md` before discussion, not after.** When a proposal is not yet decided, write it in `docs/context/IDEAS.md` (status `sketch`, with prerequisites and open questions), then discuss. Do not implement from an idea until it has an ADR (if it touches doctrine) or a step in `SESSION-STATE.md` (if it is a task).
13. **One command at a time, no extra prose.** By default, send **one command per message** and wait for the maintainer to paste the output before the next. Do not add explanations, headers, or "why" around the command unless the maintainer writes "pausa". Especially in the evening: the goal is to make the session light, not to explain every step. If a step needs context, wrap it in "pausa" (rule 11) first, then resume with the command.

## Codebase-specific rules (do not violate)

- **`pcfm-hidden` is special.** It contains Plane A. It has no public fields, it is not `Serialize`, its `Debug` is custom. The only door is `resolve_*` + `drain_observations`. **Do not add getters on latent values.** If a new module needs a latent, add a `resolve_*` (with an ADR).
- **`pcfm-testkit` has no `pcfm-*` dependencies.** It is the contract that anyone (core, plugins, tools) implements. **Do not add `pcfm-match` or other crates as dependencies.**
- **The simulation model is a prototype.** `action_probability = 0.0005`, split 60/25/12/3, `noise * 0.2` in `resolve_duel`, latents uniform in [0,1). **Do not "optimise" without an ADR.** A change to `resolve_*` or `MatchState::tick` requires a `SIM_VERSION` bump (see `CHANGELOG-SIM.md`).
- **The doctrine wins.** If a change contradicts `00-foundations.md`, a doctrine ADR is required.
- **`docs/adr/` contains only ADRs.** The reading guide is in `docs/`, not in `docs/adr/`. If you add an ADR, `Status: proposed` → discussion → `Status: accepted` (or `rejected`).

## What I expect from you in this new chat

1. **Read `docs/context/SESSION-STATE.md` first**, then the 4 key documents. Do not propose anything before you have the context.
2. **Confirm the state** with the commands I ask you for (`git pull`, `git log`, `cargo test`, etc.), without assuming it is what I describe.
3. **Propose the next concrete step** (whatever I prefer) and, if needed, give me the exact `sed`/`awk` command. Do not send the full file in chat if it exceeds ~120 lines: it truncates.
4. **Do not invent anything.** If you don't know, ask. If you haven't read a file, say so.
5. **If a test fails, stop.** Do not "fix" the test to make it pass.

## Startup ritual — the exact commands to ask for

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

### 3. Read the latest analysis reports (continuity)

    ls -t docs/context/analysis/*.md | head -3

For each of the 3 most recent reports, read the executive summary and any finding that is still open. If a report says "Pass" but has unverified findings, do not trust the verdict: verify each finding with `grep`/`sed` before acting.

### 4. Read the doctrine (non-negotiable)

    sed -n '1,130p' modules/00-foundations.md

At minimum: §1 (pillars), §2 (design pillars), §3 (Three Planes), §5 (banned things).

### 5. Read the operating rules

    sed -n '1,130p' docs/context/NEW-SESSION-PROMPT.md

This very file — rules 0-12, codebase-specific rules, what NOT to do.

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

- Do not give me "lots of comments" scattered through the code: the codebase is already documented.
- Do not assume that ADR-012, ADR-013, ADR-015, ADR-016, ADR-018, ADR-019 are "proposed". They are **accepted**.
- Do not assume there are "17 ADRs". There are **19 files in `docs/adr/`**: 17 accepted (001-008, 012-016, 018, 019, 020, 021), 1 proposed (017), 1 template (000). Plus 1 reading guide outside `docs/adr/`.
- Do not paste a full file > 120 lines in chat: it truncates. Use `sed`/`awk` commands, or split into consecutive parts with no text in between.
- Do not invent commit hashes or file contents you have not read.
- Do not propose Python scripts (Python is not installed). Use Bash/Git Bash.
- Do not assume CI exists: there is no `.github/workflows/`. Clippy/test/build/fmt/check are **manual**, and must be run after every change (the pre-commit hook helps for Rust).
- Do not "optimise" the simulation model without an ADR and a `SIM_VERSION` bump.
- Do not push directly to `main` in Phase 2. In Phase 1, direct push is allowed (see `AGENT-POLICY.md`).
- Do not reactivate the ruleset on `main` without explicit instruction. It is disabled in Phase 1.

## First step I want to take

**Whatever `docs/context/SESSION-STATE.md` says at step 0.** Read the bookmark
before proposing anything. Do not assume a step is still open because it was
open in a previous session: verify with `git log` and the bookmark.

## Closing ritual — the exact commands to run before ending a session

At the end of every session (or at a multi-step plan completion), **before declaring the session closed**, run these checks. If any is red, stop and fix before closing.

### 1. Repo alignment and real verification

    git status --short
    git log --oneline -5
    git rev-parse HEAD

If there are local commits not pushed:

    git push
    git log --oneline -3
    git status -sb

Then the full quality gate (not just the pre-commit hook):

    cargo build --release
    cargo clippy --workspace --all-targets -- -D warnings
    cargo test --workspace
    cargo fmt --check
    cargo run --release --bin pcfm-test-runner -- --check --seed 42

If anything is red: **stop**. Do not declare the session closed. Fix first.

### 2. Documentation coherence

Compare the numbers across the files that claim them:

    grep -n 'tests\|crates\|ADR' docs/context/SESSION-STATE.md | head -10
    grep -n '^| 0' STRUCTURE.md | tail -25
    grep -n '^| \[0' modules/_index.md | tail -25
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
- `Next steps`: ordered, with the current priority
- `Files touched this session`: full list, dated
- New open items from the session (verification of previous findings, decisions deferred, sketches added)

Do not update `REPO-SNAPSHOT.md` unless its own premise says it is current: it is **not normative**, and its staleness is expected.

### 5. Final report to the maintainer

State explicitly:
- What was done (one line per commit)
- What was verified (real tool output, not claims)
- What was **not** done (and why: blocked, deferred, out of scope)
- The state of the repo (aligned, clean, quality gate green)
- The next step for the next session (what `SESSION-STATE.md` says)

If you have not run one of the checks in this ritual, say so. "I skipped X" is honest; "I ran X" when you did not is a lie.

### 6. Commit and push

The checkpoint commit is **one logical commit**:

    git add docs/context/SESSION-STATE.md
    git commit -m "docs(session-state): checkpoint after <what happened>"
    git push
    git status -sb

`git status -sb` must show `## main...origin/main` with no `[ahead]` or `[behind]`. If it does, the session is not closed.
````
