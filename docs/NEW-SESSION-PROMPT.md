# New Session Prompt — PCFM

**Purpose:** prompt to paste as the first message of a new chat, to resume work on the project without losing context. Update this file when the project state or the operating rules change.

**Last updated:** 2026-10-27 (commit `76ae391`)

---

## How to use

1. Open a new chat.
2. Copy the content of the block below (from `# Resuming the PCFM project` to the last line, without the 4 backticks).
3. Paste it as the first message.

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

## Where we are (commit `76ae391`, 2026-10-27)

- **7 crates:** `pcfm-core`, `pcfm-model`, `pcfm-hidden`, `pcfm-stats`, `pcfm-match`, `pcfm-testkit`, `pcfm-debug`.
- **107 tests**, clippy clean (`-D warnings`), `--check` 6/6, `--list` 6 scenarios, `--coverage` 2/26 (7%).
- **14 ADRs accepted** (001-008, 012-016, 018), **1 proposed** (017, roles are labels), 1 template (000), 1 reading guide (`docs/PLUGINS-WORLDS-READING-GUIDE.md`, outside `docs/adr/`).
- **11 L0 modules** ratified + implemented.
- **16 inconsistencies** identified and resolved (all with commits).
- **Working tree clean.**

## The 3 files you MUST read first (in this order)

1. **`docs/PROJECT-IDEA-LOG.md`** (212 lines) — history, decisions, sources (DeepSeek + this conversation), what remains open. If you read only one file, read this one.
2. **`docs/REPO-SNAPSHOT.md`** (348 lines) — factual snapshot: tree, files, 7 crates with API/tests, metrics, 16 resolved inconsistencies. It also contains the complete list of files read: if a piece of content is not here, it has not been read.
3. **`docs/ARCHITECTURE.md`** (210 lines) — how the crates interact, the flow of a match, the simulation/rendering split, the testkit contract.

Then, if you need to touch the code: **`docs/CODE_TOUR.md`** (215 lines) — a reading path in 5 stages (infrastructure → entities → planes → engine → testkit).

Then the doctrine: **`modules/00-foundations.md`** (v2.0, pillar 1 = local determinism) and **`modules/_index.md`** (v2.9).

## What remains to be done (from `PROJECT-IDEA-LOG.md` §7)

In dependency order:

1. **Ratify ADR-017** — "roles are labels, not modifiers" (`proposed`). It moves the role catalogue to `L1-roster` and confirms the engine (L0) is blind to roles. This unblocks `L1-roster/01-player-attributes.md`.
2. **`ProjectionProfile`** — method for computing the confidence bands (bootstrap? Bayesian? empirical quantiles?).
3. **`ScoutReport.DeclaredBias`** — formula for applying the declared bias.
4. **`docs/performance.md`** — expected benchmarks, measurement plan.
5. **`docs/save-security.md`** — checksum, signature, backup, rollback.
6. **L1** — first module: `modules/L1-world/01-nation.md`. Implicit DoD: a playable league season, visible standings, an existing world (nations, cities, stadiums), complete rosters. See `modules/_index.md` §L1.

## Operating rules (important — we established them together)

1. **Complete files, always.** When a change concerns a file < 500 lines, give me the **complete file** to paste into the GitHub web interface. No `perl`/`sed` for small files. For files > 500 lines, ask me whether I prefer `perl` or the complete file. **Exception:** 1-line fixes in files > 500 lines may use `perl -i -pe` with a `grep -n` verification.
2. **I work on GitHub web + Git Bash on Windows.** I modify files by pasting into the web interface. I run commands (`git pull`, `cargo`, `git mv`) in Git Bash. When you give me a command, it must work on Git Bash (Windows). **No Python scripts** (Python is not installed). No Linux-only commands (`apt`, `systemctl`, etc.).
3. **Pull before every verification.** After every web change, I must run `git pull --rebase` before running `cargo`. If I forget, remind me. If the pull stops because of uncommitted local changes, stop and ask me.
4. **Mutual honesty, zero inventions.** If you have not read a file, say so. If a piece of information is not certain, say so. Do not invent numbers, commit hashes, or file contents to "fill a gap". The rule is: "I don't know" > invention.
5. **Small, atomic commits.** One commit per file (or per logical group). Clear messages in English, imperative mood.
6. **Verify after every code change:** `cargo build --release` + `cargo clippy --workspace --all-targets -- -D warnings` + `cargo test --workspace` + `cargo fmt --check` + `cargo run --release --bin pcfm-debug -- --check --seed 42`. If anything is red, **stop and we fix it**. Do not "fix" a test to make it pass: the test is the contract.
7. **No `unwrap`/`expect`/`panic!` in simulation paths.** No `HashMap` in simulation paths. No `unsafe` outside `pcfm-bridge` (future). These rules are already in the code, but it is worth remembering them.
8. **If I ask you to read a file, give me the `cat` command from Git Bash** (e.g. `for f in ...; do echo "FILE: $f"; cat "$f"; done`), do not invent the content.

## Codebase-specific rules (do not violate)

- **`pcfm-hidden` is special.** It contains Plane A. It has no public fields, it is not `Serialize`, its `Debug` is custom. The only door is `resolve_*` + `drain_observations`. **Do not add getters on latent values.** If a new module needs a latent, add a `resolve_*` (with an ADR).
- **`pcfm-testkit` has no `pcfm-*` dependencies.** It is the contract that anyone (core, plugins, tools) implements. **Do not add `pcfm-match` or other crates as dependencies.**
- **The simulation model is a prototype.** `action_probability = 0.0005`, split 60/25/12/3, `noise * 0.2` in `resolve_duel`, latents uniform in [0,1). **Do not "optimise" without an ADR.** A change to `resolve_*` or `MatchState::tick` requires a `SIM_VERSION` bump (see `CHANGELOG-SIM.md`).
- **The doctrine wins.** If a change contradicts `00-foundations.md`, a doctrine ADR is required.
- **`docs/adr/` contains only ADRs.** The reading guide is in `docs/`, not in `docs/adr/`. If you add an ADR, `Status: proposed` → discussion → `Status: accepted` (or `rejected`).

## What I expect from you in this new chat

1. **Read the 3 key documents** (`PROJECT-IDEA-LOG.md`, `REPO-SNAPSHOT.md`, `ARCHITECTURE.md`) before proposing anything.
2. **Confirm the state** with the commands I ask you for (`git pull`, `git log`, `cargo test`, etc.), without assuming it is what I describe.
3. **Propose the next concrete step** (ADR-016 or whatever I prefer) and, if needed, give me the complete file to paste.
4. **Do not invent anything.** If you don't know, ask. If you haven't read a file, say so.
5. **If a test fails, stop.** Do not "fix" the test to make it pass.

## What NOT to do

- Do not give me "lots of comments" scattered through the code: the codebase is already documented (`///`, `//!`, `# Errors`, `# Panics`, `# Example`), plus `ARCHITECTURE.md` and `CODE_TOUR.md`. If you want to add documentation, propose **where** and **why**, not "across the board".
- Do not assume that ADR-012, ADR-013, ADR-015, ADR-016, ADR-018 are "proposed". They are **accepted** (2026-10-25/26/27).
- Do not assume that `00-foundations.md` is v1.4. It is **v2.0** (pillar 1 rewritten: local determinism).
- Do not assume there are "13 ADRs". There are **14 accepted ADRs + 1 proposed (017) + 1 template (000) + 1 reading guide** (outside `docs/adr/`).
- Do not invent commit hashes or file contents you have not read.
- Do not propose Python scripts (Python is not installed). Use Bash/Git Bash.
- Do not assume CI exists: there is no `.github/workflows/`. Clippy/test/build/fmt/check are **manual**, and must be run after every change.
- Do not "optimise" the simulation model without an ADR and a `SIM_VERSION` bump.

## First step I want to take

**Ratify ADR-017** ("roles are labels", `proposed`), which moves the role catalogue to `L1-roster`. But first:

1. Read the 3 key documents.
2. Verify the current state with `git pull` + `git log --oneline -5`.
3. Give me the `cat` command to read the files you need (probably `modules/_index.md`, `modules/00-foundations.md` §7, `docs/adr/015-v1-scope.md`).
4. Then propose a draft of ADR-016 with options and your recommendations.
````
