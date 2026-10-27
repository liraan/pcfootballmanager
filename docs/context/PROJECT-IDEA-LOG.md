# Project Idea Log — PCFM

> **Update 2026-10-26:** ADR-012, ADR-013, and ADR-015 have been ratified. See `docs/context/REPO-SNAPSHOT.md` for the current state. This log keeps the original entries for traceability.
>
> **Update 2026-10-27:** ADR-019 (match engine contract) has been ratified. See `docs/context/SESSION-STATE.md` for the current working state and `docs/context/REPO-SNAPSHOT.md` for the briefing snapshot. This log keeps the original entries for traceability.

**Purpose:** record the ideas, decisions, and open questions of the project, with the source of each. This is not a status document for the code (see `REPO-SNAPSHOT.md`). It is not a specification (see `modules/`).

**Rule of this document:** every statement has a source. If a decision was made, there is a commit or message that contains it. If an idea is only sketched, it is marked as "idea". If we do not know something, we write "I don't know" instead of inventing.

**Sources:**
- **D** = DeepSeek conversation (`chat.deepseek.com/share/ubaqxjcf8loa3jvy9s`)
- **C** = this conversation (GitHub web + Git Bash, from 2026-10-24 to 2026-10-26)
- **R** = repository (`git log`, `git ls-files`, files read)

---

## 1. Origin of the project

The project was born from the desire to build a community alternative to commercial football management games, with three non-negotiable principles (stated in `README.md` and `modules/00-foundations.md`):

1. **Transparency** — every formula, every rule, every data source is public.
2. **Privacy** — no telemetry, no accounts, no network.
3. **Tactical depth** — an engine that rewards understanding, not clicks.

The doctrine of the **Three Planes** (Plane A = Ground Truth, Plane B = Observation, Plane C = Estimation) is the architectural differentiator: scouting is not a lookup, it is *an estimate of an observation of the truth*. (Source: `README.md`, `modules/00-foundations.md` §3; C)

---

## 2. Recommendations from the DeepSeek chat (source D)

The DeepSeek conversation ends with **5 recommendations**, **3 technical gaps**, and **2 missing documents**. I report them because they are the basis for the subsequent decisions.

### 2.1 Recommendations

| # | Recommendation | Status at 2026-10-26 |
|---|----------------|----------------------|
| 1 | Reduce the v1.0 scope: 20 roles (not 56), 10 formations (not 20), 5 nations (not 20), 3 active psychological axes (not 6) | ⏳ formalised in ADR-015 (`accepted`, 2026-10-26) |
| 2 | Write code before completing the 25 modules | ✅ done: L0 implemented, 107 tests |
| 3 | Ratify ADR-003 and ADR-005 before writing code that touches them | ✅ done: ADR-003 accepted (with `pcfm-debug` exception), ADR-005 accepted |
| 4 | Align document versions and unify the language | ✅ done: ADR-007 accepted, `00-foundations.md` v2.0, documentation in English |
| 5 | The 3 technical gaps and the 2 missing documents | ⏳ open (see §2.2 and §2.3) |

### 2.2 The three technical gaps (source D)

| # | Gap | What is missing | Status |
|---|-----|-----------------|--------|
| A | 6-axis psychology | The doctrine says 3 axes in v1.0, but the data model has 6. The `active: false` flag must be made explicit on the 3 non-v1.0 axes. | ⏳ open, tracked in ADR-015 (§Deferred decisions) |
| B | `ProjectionProfile` | The "expected band" has no computation method (bootstrap? Bayesian? empirical quantiles?). Without a specification, the band is arbitrary. | ⏳ open, not yet addressed |
| C | `ScoutReport.DeclaredBias` | The declared bias has no application formula. Is it just a label, or does it enter the calculation? | ⏳ open, not yet addressed |

### 2.3 The two missing documents (source D)

| Document | What it should contain | Status |
|----------|------------------------|--------|
| `docs/performance.md` | Expected benchmarks (season < 60 s on mid-range hardware), measurement plan, computational cost per tick | ❌ does not exist |
| `docs/save-security.md` (or a section in `L0-match/07-save-minimal.md`) | Checksum, signature, automatic backup, rollback for corrupted saves | ❌ does not exist |

---

## 3. Decisions taken in this conversation (source C + R)

In chronological order. Each decision has the commit that recorded it (when available).

### 3.1 License (ADR-006)

- **Decision:** dual license. Code `MIT OR Apache-2.0`, assets `CC-BY-NC-SA-4.0`.
- **Files created:** `LICENSE-MIT`, `LICENSE-APACHE`, `LICENSE-ASSETS`, `LICENSE.md`.
- **ADR-006:** from `proposed` to `accepted` (2026-10-25).
- **Commit:** `99c8946` (LICENSE-ASSETS), `8871ab4` (LICENSE.md), plus the ADR-006 ratification commits.

### 3.2 Qt binding (ADR-005)

- **Decision:** `cxx-qt` (option A). The only binding that covers both QWidgets (data grid) and QML (skins).
- **ADR-005:** from `proposed` to `accepted` (2026-10-25).
- **Commit:** ratified together with the ADR batch.

### 3.3 Canonical language (ADR-007)

- **Decision:** English canonical for code, commits, ADRs, technical documentation. Italian as a translation in `locales/it/` (planned).
- **ADR-007:** from `proposed` to `accepted` (2026-10-25).
- **Commit:** ratified together with the ADR batch.

### 3.4 Doctrine (00-foundations.md)

- **Decision:** remove the "under review" notes on ADR-013. The doctrine was v1.4, pillar 1 (global determinism) stable. ADR-013 was cited as `proposed, not ratified` (later ratified on 2026-10-26, doctrine updated to **v2.0** with pillar 1 rewritten to local determinism).
- **File:** `modules/00-foundations.md` v2.0, `status: "ratified"`, `last_updated: 2026-10-26`.
- **Commit:** `d6c3c8b` + `fd2c298` (v2.0).

### 3.5 ADR-003 (Plane A enforcement)

- **Decision:** document the `pcfm-debug` exception. The crate may depend on `pcfm-hidden` only to build `GroundTruth` to pass to `pcfm-match`, never to read latents. The exception is confined to `demo.rs` and `check.rs`.
- **File:** `docs/adr/003-plane-a-enforcement.md`, section "Exception: pcfm-debug". Full body recomposed from `571d48f` + `70ee57f`.
- **Commit:** `b309f23` (full recomposition).

### 3.6 v1.0 scope (ADR-015)

- **Decision:** formalise the scope reduction (20 roles, 10 formations, 5 nations, 3 active psychological axes, L2/L3 subset).
- **File:** `docs/adr/015-v1-scope.md` (renamed from `014-v1-scope.md` because 014 was taken by `014-mobile-strategy.md`).
- **Status:** `accepted` (2026-10-26). **Depended on ADR-012 and ADR-013**, both ratified on 2026-10-25.
- **Commit:** `1a59b81` (rename), `8d8d347` (original creation), `4d826dc` (ratification), `4c93214` (propagation).
- **Note:** the ADR contains 7 **Deferred decisions**. They were resolved in ADR-016 ("v1.0 content selections", accepted 2026-10-27).

### 3.7 Coretest framework (`pcfm-testkit`)

- **Decision:** create a public crate `pcfm-testkit` that defines the `Scenario` / `Feature` / `TestRegistry` contract, **with no dependencies on other `pcfm-*` crates**. Plugins can implement `Scenario` without depending on `pcfm-debug`.
- **Files created:** `crates/pcfm-testkit/` (7 files), `crates/pcfm-debug/src/features/` (2 files), `crates/pcfm-debug/src/commands.rs`.
- **Files modified:** `args.rs` (added `--list`, `--coverage`), `main.rs` (dispatch), `check.rs` (thin wrapper over the registry).
- **Commit:** series `928f734`, `aff7a13`, `ff13f2d`, `5bdcb78` and subsequent.
- **Status:** working. `--check` 6/6, `--list` 6 scenarios, `--coverage` 2/26 (7%).

### 3.8 Documentation updated (2026-10-26)

- **README.md:** reformatted for readability, added `--list` / `--coverage`, updated L0 status ("ratified + implemented + tested"), FAQ updated ("All decisions are now accepted").
- **STRUCTURE.md:** v1.7 → v1.8. 7 crates, 15 ADRs, ADR-005/006/007 accepted, ADR-014/015 added.
- **PLUGIN_DEVELOPER_GUIDE.md:** v1.0 → v1.1. Added §11 "Contributing test scenarios", fixed `fmr-pack` → `pcfm-pack`, updated crate table.
- **`_index.md`:** v2.8 → v2.9. ADR-012/013/014 accepted, doctrine v2.0, 7 crates / 107 tests, 12 accepted ADRs.
- **`CONTRIBUTING.md`:** status updated (7 crates, 107 tests, 12 accepted, 0 proposed).
- **Commit:** `7c98612`, `8c67ec8`, `be929c4`, `635bc61`, `f1dc5fe`, `4c93214`.

### 3.9 Architecture and code tour documentation (2026-10-26)

- **Decision:** add two missing documents: `ARCHITECTURE.md` (how the crates interact, flow of a match, simulation/rendering split, testkit contract) and `CODE_TOUR.md` (reading path in 5 stages: infrastructure, entities, planes, engine, testkit). Plus 5 targeted comments in the cryptic points of the code (`action_probability = 0.0005`, 60/25/12/3 split in `MatchState::tick`, `noise * 0.2` in `resolve_duel`, uniformity of latents, choice of the 6 L0 scenarios).
- **Files created:** `docs/ARCHITECTURE.md` (210 lines), `docs/CODE_TOUR.md` (215 lines).
- **Files modified:** `context.rs`, `state.rs`, `hidden/lib.rs`, `latents.rs`, `l0_match.rs` (comments only, no behaviour change).
- **Commit:** `e35d15e` (the 5 files + 2 documents), `0a329d4` (fix `DoD` in backticks for `clippy::doc_markdown`).
- **Rationale:** the codebase had `///` everywhere, but lacked an architectural map and a reading path. The original idea ("many comments in all files") was discarded because comments that repeat the code age and duplicate the specification. Only the genuinely cryptic points were commented.

### 3.10 Session 2026-10-27 (ADR-019, agent policy, docs/context)

- **ADR-019 (match engine contract)** — `accepted` (2026-10-27). 8 decisions (D1–D8) + 9 deferred. Deferred 1 (spatial iteration order) and deferred 2 (tick-cost pre-emption, selective foul) resolved the same day. Deferred 3–9 remain open.
- **`AGENT-POLICY.md`** — created in `docs/context/`. Defines what an agent may do autonomously, what requires confirmation, what it must never do, and how to revoke. Phase 1 (ruleset disabled, single maintainer) vs Phase 2 (ruleset active, PR mandatory).
- **Pre-commit hook** — `.githooks/pre-commit` (versioned). Runs fmt, clippy, test only if Rust files are staged. Installation: `git config core.hooksPath .githooks` (one-off, per clone).
- **`docs/context/`** — `REPO-SNAPSHOT.md`, `PROJECT-IDEA-LOG.md`, `NEW-SESSION-PROMPT.md`, `SESSION-STATE.md`, `AGENT-POLICY.md`. Briefing material, not normative. `STRUCTURE.md` §5 remains the ADR status source of truth.
- **Ruleset** — configured on `main` (PR required, force-push blocked, deletion blocked, 0 approvals). **Currently disabled** (Phase 1). Reactivate before granting a write token to any agent.
- **Doctrine** — added "Football fidelity" as a tie-breaker criterion in `modules/00-foundations.md` §1.
- **Commits** — `88d20b4`, `67c2098`, `952c5a4`, `b1196c3`, `23add74`, `c82e310`, `a261439`, `b4ff405`, `eb71bda`, `25cb73c`, `7f3a7a6`, and subsequent.

---

## 4. Ratified decisions (at 2026-10-26)

### 4.1 ADR-012 (Plugins: The Brick Contract)

- **Status:** `accepted` (2026-10-25).
- **What it decides:** the module (plugin) contract: what a module is, what it declares (`sheet.toml`), how conflicts are handled.
- **Blocks:** the plugin system (step 11+), and indirectly ADR-015 (because "in a pack" is not a safe answer until the module contract exists).
- **Ratified:** 2026-10-25.

### 4.2 ADR-013 (Worlds: LOD, Manifest, and Local Determinism)

- **Status:** `accepted` (2026-10-25).
- **What it decides:** the world system (how the bricks compose into a world), the Level of Detail, the save manifest, and — if ratified — it supersedes pillar 1 of the doctrine (determinism from global to local).
- **Depends on:** ADR-012.
- **Blocks:** ADR-015, the world system, local determinism.
- **Ratified:** 2026-10-25.

### 4.3 ADR-015 (v1.0 scope)

- **Status:** `accepted` (2026-10-26).
- **Depended on:** ADR-012 and ADR-013, both ratified on 2026-10-25.
- **To do:** the 7 Deferred decisions (which 3 psychological axes? which 20 roles? which 10 formations? which 5 nations? which L2/L3 in v1.0?) were resolved in **ADR-016** (accepted 2026-10-27). The role catalogue moved to L1-roster in ADR-017 (proposed); crowd and media deferred to v1.1 in ADR-018 (accepted).

### 4.4 ADR-019 (Match engine contract)

- **Status:** `accepted` (2026-10-27).
- **What it decides:** agent-first state in players; grid as quantisation + adjacency graph (not an entity); integer position (metres); resolution as a saved engine parameter; zone of influence as a derived relation. Defines the target contract for the positional engine.
- **Deferred:** 9 items. Deferred 1 (spatial iteration order) and deferred 2 (tick-cost pre-emption) resolved the same day. Deferred 3–9 remain open (possession, UX, resolution per platform, adaptive resolution, LOD override, zone formula, role→mechanic).
- **Blocks:** the positional engine implementation (replaces the current probability-per-tick prototype).
- **Commits:** `64c9b91` (creation), `de208bd` (ratification), `67c2098` (clarifications), `eb71bda` (deferred 2), and subsequent.

---

## 5. What I do NOT know (and do not want to invent)

All tracked files have been read (see `REPO-SNAPSHOT.md` §7). The only thing outside is:

1. **Any other chats or decisions taken outside this session.** If there are any, I do not see them.

---

## 6. Next steps (in dependency order)

1. **Resolve the remaining ADR-019 deferred decisions** (4–9): granular UX (4), resolution per platform (5), adaptive resolution (6), LOD override (7), zone-of-influence formula (8), role→mechanic chain (9). Deferred 1 (spatial order), deferred 2 (pre-emption), and deferred 3 (possession) are resolved.
2. **Ratify ADR-017** ("roles are labels, not modifiers", `proposed`): move the role catalogue to L1-roster and confirm the engine is blind to roles.
3. **Address the 2 technical gaps** (`ProjectionProfile`, `ScoutReport.DeclaredBias`). The third (psyche) is inside ADR-016.
4. **Write the 2 missing documents** (`performance.md`, `save-security.md`).
5. **Start writing L1** (first module: `L1-world/01-nation.md`). Unblocks the first L1 scenarios in the coretest.
6. **Agent setup (Warp)** — Phase 2. Reactivate the ruleset on `main` before granting a write token. See `docs/context/AGENT-POLICY.md`.
