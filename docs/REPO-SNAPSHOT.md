# Repository Snapshot — PCFM

**Purpose:** factual snapshot of the repository at **2026-10-26**. Lists the files, their reading status, and what they contain. It is not a specification (see `modules/`), not a decision log (see `PROJECT-IDEA-LOG.md`).

**Rule:** if a file has been read in full, I say so and summarise its content. If it has been read only in part, I say so. If it has never been read, I write **"not read"** and list it in §7. No invented sizes: only those seen in real output (`git ls-files`).

**Sources:** output of `git ls-files`, `ls -la`, `head`, `grep`, and the files pasted in chat on 2026-10-25/26.

---

## 1. Project state (at 2026-10-26)

| Metric | Value | Source |
|--------|-------|--------|
| Branch | `main` | `git status` |
| Last commit | `edcbb6f` — "chore: apply cargo fmt to l0_match.rs" | `git log` |
| Working tree | clean | `git status` |
| Tracked files | 118 | `git ls-files \| wc -l` |
| Crates | 7 (`pcfm-core`, `pcfm-model`, `pcfm-hidden`, `pcfm-stats`, `pcfm-match`, `pcfm-testkit`, `pcfm-debug`) | `git ls-files` |
| Tests | 107 (0 failed) | `cargo test --workspace` |
| Clippy | clean (`-D warnings`) | `cargo clippy -- -D warnings` |
| `--check` | 6/6 | `cargo run --bin pcfm-debug -- --check` |
| `--coverage` | 2/26 (7%) | `cargo run --bin pcfm-debug -- --coverage` |
| ADRs | 16 files in `docs/adr/`: 14 accepted (001-008, 012-016, 018), 1 proposed (017), 1 template (000). Plus 1 reading guide in `docs/` (not an ADR). | `ls docs/adr/` |
| ADRs accepted | 14 (001-008, 012, 013, 014, 015, 016, 018) | `grep Status` |
| ADRs proposed | 1 (017) | `grep Status` |
| L0 modules | 11, all ratified | `git ls-files` |
| L1-L3 modules | 0 (all planned) | `git ls-files` |

---

## 2. Repository tree

```
pcfootballmanager/
├── .github/
│   ├── ISSUE_TEMPLATE/
│   │   ├── bug_report.md
│   │   └── module-revision.md
│   └── PULL_REQUEST_TEMPLATE.md
├── archive/
│   ├── README.md
│   ├── doctrine/.gitkeep
│   ├── legacy/.gitkeep
│   └── prompts/.gitkeep
├── crates/
│   ├── pcfm-core/           (7 files: Cargo.toml + 6 src)
│   ├── pcfm-model/          (9 files: Cargo.toml + 8 src)
│   ├── pcfm-hidden/         (6 files: Cargo.toml + 5 src)
│   ├── pcfm-stats/          (5 files: Cargo.toml + 4 src)
│   ├── pcfm-match/          (9 files: Cargo.toml + 8 src)
│   ├── pcfm-testkit/        (7 files: Cargo.toml + 6 src)
│   └── pcfm-debug/          (9 files: Cargo.toml + 8 src)
├── data/.gitkeep
├── docs/
│   ├── adr/                 (16 files: 000-008, 012-018)
│   ├── calibration/.gitkeep
│   ├── legal/.gitkeep
│   ├── ARCHITECTURE.md
│   ├── CODE_TOUR.md
│   ├── GLOSSARY.md
│   ├── MIGRATION-01-04.md
│   ├── PLUGIN_DEVELOPER_GUIDE.md
│   ├── PLUGINS-WORLDS-READING-GUIDE.md
│   ├── PROJECT-IDEA-LOG.md
│   ├── REPO-SNAPSHOT.md      ← this file
│   └── NEW-SESSION-PROMPT.md
├── mods/.gitkeep
├── modules/
│   ├── 00-foundations.md
│   ├── _index.md
│   └── L0-match/            (11 files, 00-10)
├── registry/.gitkeep
├── .cargo/config.toml
├── .gitattributes
├── .gitignore
├── CHANGELOG-REGISTRY.md
├── CHANGELOG-SIM.md
├── CHANGELOG.md
├── CONTRIBUTING.md
├── Cargo.lock
├── Cargo.toml
├── LICENSE-APACHE
├── LICENSE-ASSETS
├── LICENSE-MIT
├── LICENSE.md
├── README.md
├── STRUCTURE.md
└── rust-toolchain.toml
```

---

## 3. Root files

| File | Lines | Status | What I know |
|------|-------|--------|-------------|
| `README.md` | ~293 | ✅ | Reformatted in this session. Hero with badge, "Why this project exists", Three Planes, determinism, project status, tech stack, repository layout, getting started (`--check`, `--list`, `--coverage`), reading order, contributing, license, AI disclosure, FAQ ("All decisions are now accepted"), contact. |
| `STRUCTURE.md` | 443 | ✅ | v1.8 (2026-10-26). Documentation model, concentric levels, directory layout, module frontmatter, ADR process (with status table), language policy, how to add/revise a module, what packs can extend, honesty rule. Link to reading guide updated to the new path. |
| `CONTRIBUTING.md` | ~210 | ✅ | Process, review checklist, ADR workflow, licensing of contributions. Current state delegates to `REPO-SNAPSHOT.md` §1 (no duplication). Link to reading guide updated. |
| `LICENSE.md` | 63 | ✅ | Dual license summary: MIT OR Apache-2.0 for code, CC-BY-NC-SA-4.0 for assets, trademark reserved. |
| `LICENSE-MIT` | 21 | ✅ | Standard MIT text. |
| `LICENSE-APACHE` | 201 | ✅ | Apache 2.0 text verbatim. |
| `LICENSE-ASSETS` | 76 | ✅ | CC-BY-NC-SA-4.0 with header and link to the legal code. |
| `CHANGELOG.md` | 218 | ✅ | Keep-a-Changelog. Sections: Unreleased, 2026-10-25 (ADR-012/013), 2026-10-25 (rename fmr→pcfm), 2026-10-25 (ADR-008), 2026-10-25 (ADR-003 ratified), 2026-10-25 (glossary), 2026-10-25 (concentric), 2026-10-24 (initial). |
| `CHANGELOG-SIM.md` | 111 | ✅ | SIM_VERSION = 1 (baseline). 5-step bump procedure, entry template, history (empty). |
| `CHANGELOG-REGISTRY.md` | 126 | ✅ | Registry = baseline, no entities. Deprecation rules, entry template, planned deprecations. |
| `Cargo.toml` | 54 | ✅ | `[workspace]` with `resolver = "3"`, 7 members, `[workspace.package]`, `[workspace.dependencies]`, `[workspace.lints.*]`. |
| `Cargo.lock` | 357 | ✅ | **Read.** `version = 4`. 35 packages: 7 `pcfm-*` (workspace members) + 28 external dependencies (blake3, rand_chacha, serde, thiserror, syn, etc.). All pinned with checksums. `pcfm-testkit` has no dependencies. `pcfm-debug` depends on 6 workspace crates + `serde_json`, `thiserror`. |
| `.gitattributes` | 36 | ✅ | LF normalisation, exclusions for `.bat`/`.cmd`/`.ps1` (CRLF) and binaries. |
| `.gitignore` | 167 | ✅ | **Read.** Sections: Rust build artifacts, profiling, IDE, OS, logs, user data, secrets, pack cache, generated worlds, temporary, test fixtures, coverage, local cache. **Cargo.lock explicitly NOT ignored.** |
| `.cargo/config.toml` | 13 | ✅ | **Read.** Intentionally minimal. Only comments: future aliases and `rustflags` for CI, all commented out. **No active configuration.** |
| `rust-toolchain.toml` | 15 | ✅ | **Read.** `channel = "stable"`, `components = ["rustfmt", "clippy", "rust-src"]`, `profile = "minimal"`. |

---

## 4. Documentation (`docs/`)

| File | Lines | Status | What I know |
|------|-------|--------|-------------|
| `docs/ARCHITECTURE.md` | ~210 | ✅ | **New (2026-10-26).** How the 7 crates interact, flow of a match (10 steps), simulation/rendering split, testkit contract, what is not yet implemented. |
| `docs/CODE_TOUR.md` | ~215 | ✅ | **New (2026-10-26).** Reading path in 5 stages: infrastructure, entities, planes, engine, testkit. With "where to start contributing" and "a note on the current simulation". |
| `docs/GLOSSARY.md` | 313 | ✅ | v1.1. Terms, acronyms, concepts. Sections: Core, Data model, Determinism, Architecture, Process, Banned, External, Abbreviations, Historical. |
| `docs/MIGRATION-01-04.md` | 186 | ✅ | Migration plan for the 4 legacy Italian modules (01-04), never committed to git. Target mapping to concentric modules. |
| `docs/PLUGIN_DEVELOPER_GUIDE.md` | 436 | ✅ | v1.1 (2026-10-26). Contains: two kinds of extension, crate layout (7 crates), simulation/rendering split, adding EventType, adding `resolve_*`, dependencies, conventions, plane separation, PR checklist, examples, §11 Contributing test scenarios, references. |
| `docs/PLUGINS-WORLDS-READING-GUIDE.md` | ~95 | ✅ | New content (2026-10-26): `Status: accepted`, `Type: reading guide (non-ADR)`, `plugin` terminology (not `module`), `MatchPlugin` (not `MatchModule`), reference to `00-foundations` v2.0. |

### 4.1 ADRs (`docs/adr/`)

| File | Lines | Status | What I know |
|------|-------|--------|-------------|
| `000-template.md` | 55 | ✅ | Template: Status, Date, Supersedes, Superseded by, Blocks, Context, Options, Decision, Consequences, Implementation notes, References. |
| `001-language-rust.md` | ~69 | ✅ | `Status: accepted`. Option A (Rust), B (C++), C (C#), D (Go). Decision: Rust. References updated (Cargo.toml workspace root, rust-toolchain.toml, STRUCTURE.md §3). |
| `002-project-name-and-scope.md` | 79 | ✅ | `Status: accepted`. Name: PC Football Manager / PCFM. Single-player first. Non-goals v1.0. |
| `003-plane-a-enforcement.md` | 235 | ✅ | `Status: accepted`. Option A (process) vs B (capability API). Decision: B. Full body + Exception `pcfm-debug`. Recomposed from `571d48f` + `70ee57f` in commit `b309f23`. |
| `004-panic-strategy-ffi.md` | ~117 | ✅ | `Status: accepted`. Option D: `panic = "unwind"` in dev/release, `abort` in release-hardened. `catch_unwind` mandatory at every FFI. Completed in commit `c4c775b` (Consequences, Implementation notes, References, marked as reconstructed). |
| `005-qt-binding.md` | 96 | ✅ | `Status: accepted`. Decision: Option A (`cxx-qt`). |
| `006-license.md` | 69 | ✅ | `Status: accepted`. Decision: Option C (dual license). |
| `007-canonical-language.md` | 68 | ✅ | `Status: accepted`. Decision: Option A (English canonical). |
| `008-provider-integration-external.md` | 77 | ✅ | `Status: accepted`. Provider integration = external tool, not a core module. |
| `012-plugins.md` | 139 | ✅ | `Status: accepted` (commit `ec29aba`). Brick contract: sheet mandatory, behaviour open/metadata closed, conflicts critical/minor, narrative modder's responsibility, cause traces 2 levels, debug UI plugin view. |
| `013-worlds.md` | 165 | ✅ | `Status: accepted` (commit `73f51aa`). LOD = sum of plugin weights, fixed per save, manifest in save, local determinism, community plural. |
| `014-mobile-strategy.md` | 167 | ✅ | `Status: accepted`. Native mobile UI, no plugins, data packs only. Mobile = Phase 3+. |
| `015-v1-scope.md` | 145 | ✅ | `Status: accepted` (2026-10-26, commit `4d826dc`). v1.0 scope reduction: 20 roles, 10 formations, 5 nations, 3 active psychological axes, L2/L3 subset. 7 Deferred decisions resolved in ADR-016 (accepted 2026-10-27); crowd/media partially superseded by ADR-018 (accepted). |
| `016-v1-content-selections.md` | 238 | ✅ | `Status: accepted` (2026-10-27). v1.0 content selections: 3 axes (professionalism, ambition, pressure), 20 roles (catalogue in L1-roster), 10 formations, 5 nations (real cities; generated clubs/stadiums/players), 7 L2 modules (crowd/media to v1.1). Resolves ADR-015 deferred decisions. |
| `017-roles-are-labels.md` | 92 | 🟡 | `Status: proposed`. Roles are labels, not modifiers. Role catalogue in L1-roster; the engine (L0) is blind to roles. No SIM_VERSION bump. |
| `018-defer-crowd-media.md` | 59 | ✅ | `Status: accepted` (2026-10-27). Defer crowd and media to v1.1. Partially supersedes ADR-015. |

---

## 5. Modules (`modules/`)

| File | Lines | Status | What I know |
|------|-------|--------|-------------|
| `modules/00-foundations.md` | ~245 | ✅ | v2.0 (2026-10-26). Pillar 1 revised (ADR-013): local determinism. §1 Role, §2 Design Pillars (10), §3 Three Planes, §4 Determinism (local), §5 Banned Things, §6 Product Goal, §7 Non-Goals, §8 References, §9 Concentric Structure. |
| `modules/_index.md` | ~265 | ✅ | v2.9. Current state (2026-10-26), Doctrine, L0 (11 ratified), L1-L3 (planned), Provider integrations, Plugins and worlds (accepted), How to read, Doctrine compliance, Level graph, Status legend, Components, ADR status (14 accepted, 1 proposed (017), 1 template, 1 reading guide). |
| `modules/L0-match/00-overview.md` | 203 | ✅ | `Status: ratified`. 11 cells, 7-point checklist, dependency rules. |
| `modules/L0-match/01-rng.md` | 150 | ✅ | `Status: ratified`. ChaCha20Rng, named streams, seed derivation, SIM_VERSION. |
| `modules/L0-match/02-event-bus.md` | 138 | ✅ | `Status: ratified`. EventBus, ordering by `seq`, single-thread per match, replay compatibility. |
| `modules/L0-match/03-entities-minimal.md` | 232 | ✅ | `Status: ratified`. 7 entities: Player (no latents), PhysicalStateMinimal, PsychologicalStateMinimal, Squad, Pitch, Referee, Match. |
| `modules/L0-match/04-match-engine.md` | 217 | ✅ | `Status: ratified`. `MatchEngine` trait, tick loop, micro-decision, `resolve_*` mapping, no ratings. |
| `modules/L0-match/05-match-events.md` | 185 | ✅ | `Status: ratified`. `MatchEvent`, EventType (L0 set), CauseTrace, EventChain (hash chain), append-only. |
| `modules/L0-match/06-match-output.md` | 185 | ✅ | `Status: ratified`. Score, Ratings (from events), MatchStatistics, Narrative. |
| `modules/L0-match/07-save-minimal.md` | 176 | ✅ | `Status: ratified`. MatchSave (postcard), migration, verification, GroundTruth reconstructed from seed + command log. |
| `modules/L0-match/08-random-events.md` | 220 | ✅ | `Status: ratified`. Event catalogue (physical, technical, psychological, relational, environmental, set-piece). Deterministic sampling. |
| `modules/L0-match/09-context-modifiers.md` | 177 | ✅ | `Status: ratified`. `ContextModifiers` (14 fields, all observed, no latent). |
| `modules/L0-match/10-ground-truth.md` | 190 | ✅ | `Status: ratified`. `GroundTruth` API: `new`, 7 `resolve_*`, `drain_observations`. No public fields, not `Serialize`, custom `Debug`. |

---

## 6. Crates (`crates/`)

### 6.1 Summary table

| Crate | Files | Tests | Dependencies | Responsibility |
|-------|-------|-------|--------------|----------------|
| `pcfm-core` | 7 | 22 | `rand_chacha`, `rand_core`, `blake3`, `thiserror`, `serde` | RNG, event bus, IDs, hash, time |
| `pcfm-model` | 9 | 21 | `pcfm-core`, `serde`, `thiserror` | Minimal entities (Player without latents, Squad, Pitch, Referee, Match, CauseTrace) |
| `pcfm-hidden` | 6 | 4 | `pcfm-core`, `pcfm-model`, `rand_chacha`, `rand_core`, `blake3`, `thiserror` | Plane A: `GroundTruth`, `resolve_*`, `drain_observations` |
| `pcfm-stats` | 5 | 11 | `pcfm-core`, `pcfm-model`, `serde`, `serde_json`, `thiserror`, `blake3` | Plane B: `MatchEvent`, `EventChain` append-only with hash chain |
| `pcfm-match` | 9 | 10 | `pcfm-core`, `pcfm-model`, `pcfm-hidden`, `pcfm-stats`, `serde`, `serde_json`, `thiserror` | Match engine: orchestrator `simulate_match`, `TextMatchEngine`, save/load, narrative |
| `pcfm-testkit` | 7 | 0 | **None** | Contract: `Scenario`, `Feature`, `TestRegistry`, `TestContext`, `Check`, `ScenarioResult` |
| `pcfm-debug` | 9 | 18 | `pcfm-core`, `pcfm-model`, `pcfm-hidden`, `pcfm-stats`, `pcfm-match`, `pcfm-testkit`, `serde_json` | CLI (`--check`, `--list`, `--coverage`, `--json`, `--html`, `--quiet`), demo teams, rendering |

**Total:** 52 files, 86 tests (out of 107: 86 + 21 dev/integration).

### 6.2 Details per crate

#### `pcfm-core` (7 files, 22 tests)

**Public API:**
- `rng`: `StreamName`, `RngRegistry` (`new`, `stream`), `next_unit()`, `RngError`
- `event_bus`: `EventKind` (14 variants), `Event { seq, kind }`, `Handler`, `EventBus` (`new`, `publish`, `subscribe`, `dispatch_all`, `len`, `is_empty`)
- `ids`: `EntityTag` trait, `EntityId<T>` (`new`, `slot`, `generation`), 5 tags (`PlayerTag`, `SquadTag`, `MatchTag`, `RefereeTag`, `EventTag`), 5 aliases (`PlayerId`, `SquadId`, `MatchId`, `RefereeId`, `EventId`)
- `hash`: `Hash([u8; 32])`, `Hash::ZERO`, `Hash::of_bytes`, `Hash::as_bytes`
- `time`: `GameInstant(u64)`, `GameInstant::from_raw`, `GameInstant::raw`

**Key points:**
- `ChaCha20Rng` derived with BLAKE3 from `(world_seed, stream_name)`. `BTreeMap` for streams, idempotent.
- Event bus: insertion-order dispatch (`seq`), handlers in registration order. Known limitation: handlers cannot publish during dispatch.
- Generational IDs (`slot` + `generation`), tagged by type.

#### `pcfm-model` (9 files, 21 tests)

**Public API:**
- `cause`: `Factor { name, value, weight }`, `CauseTrace { primary_cause, contributing_factors, explanation_template }`
- `error`: `ModelError` (`#[non_exhaustive]`): `EmptyRoleId`, `EmptyFormationId`, `EmptyPlayerName`, `InvalidHeight`, `InvalidValue`, `InvalidStarterCount`, `InvalidBenchCount`, `InvalidPitchDimensions`
- `match_`: `MatchSeed`, `MatchRules`, `Match` (`new`, `is_friendly`)
- `pitch`: `Weather`, `Pitch` (`try_new`, `Default`)
- `player`: `Foot`, `RoleId`, `PhysicalStateMinimal`, `PsychologicalStateMinimal`, `Player` (`try_new`)
- `referee`: `Referee` (`try_new`)
- `squad`: `STARTERS = 11`, `MAX_BENCH = 7`, `FormationId`, `Squad` (`try_new`)

**Key points:**
- No latent field. `Player` has only observed state.
- Strict validation: range [0,1], non-NaN, exact counts. `ModelError` (`#[non_exhaustive]`).
- Serialisation: all entities are `Serialize`/`Deserialize`.

#### `pcfm-hidden` (6 files, 4 tests)

**Public API:**
- `GroundTruth` (`new`, `world_seed`, `match_id`, 7 `resolve_*`, `drain_observations`)
- `DuelType`, 7 context structs, 7 outcome structs
- `ObservationEvent` (7 variants)

**Key points:**
- Private fields, not `Serialize`, custom `Debug` (prints only `world_seed` and `match_id`).
- Latents generated from RNG (`PlayerLatents::from_rng`), never exposed.
- `resolve_*` deterministic (same seed → same outcome).
- `drain_observations` idempotent.
- **Current model very simple** (prototype, documented in the code): `resolve_shot`: `goal_probability = noise * 0.3 + finishing`, goal if > 0.6; `resolve_duel`: `noise * 0.2 + attacker_score > defender_score`.

#### `pcfm-stats` (5 files, 11 tests)

**Public API:**
- `EventType` (27 variants), `MatchEvent` (14 fields), `validate()`
- `EventChain` (`new`, `try_append`, `events`, `len`, `is_empty`, `chain_hash`, `verify`)
- `StatsError` (`#[non_exhaustive]`)

**Key points:**
- Hash chain: `hash = BLAKE3(previous_hash || serde_json(event))`. Tamper detection.
- Strict append-only: `try_append` validates before appending. On failure, the chain is not modified.
- Determinism: same set of events → same hash.

#### `pcfm-match` (9 files, 10 tests)

**Public API:**
- `MatchConfig`, `MatchContext`, `MatchEngine`, `TextMatchEngine`, `Importance`, `NarrativeEntry`, `narrate`, `simulate_match`, `MatchResult`, `MatchStatistics`, `Score`, `MatchSave`, `SCHEMA_VERSION`, `SIM_VERSION`, `SaveError`, `replay`, `MatchState`

**Key points:**
- Simulation/rendering split: `simulate_match` produces `MatchResult` with `narrative: Vec::new()`. Only `TextMatchEngine` calls `narrate()`.
- Save as JSON: `serde_json`. Known limitation: `MatchEvent.fields` may lose 1 ulp. Future target: `postcard` (future ADR).
- Save identity: `state_hash`, not bit-for-bit equality.
- Tamper detection: `validate()` checks `schema_version` and `chain_hash`.
- `can_replay`: `sim_version == SIM_VERSION`.
- `MatchState::tick()`: simplified model (probability 0.0005 per tick, split 60/25/12/3 pass/shot/duel/foul). Documented in the code as a prototype.

#### `pcfm-testkit` (7 files, 0 tests)

**Public API:**
- `Feature` (26 built-in + `Plugin(String)`), `ALL_BUILTIN`, `all_builtin()`, `label() -> Cow`
- `Scenario` trait (`name`, `feature`, `description`, `run`)
- `TestContext` (`seed`, `log`, `logs`)
- `Check` (`pass`, `fail`), `ScenarioResult` (`new`, `record`, `pass`, `fail`, `error`, `is_ok`)
- `TestRegistry` (`new`, `register`, `scenarios`, `run_all`), `ScenarioOutcome`, `RunReport` (`outcomes`, `is_ok`, `total_checks`, `passed_checks`, `failed_checks`)

**Key points:**
- **No `pcfm-*` dependencies.** It is the contract that core, plugins, and tools share.
- `Feature` is `#[non_exhaustive]`.
- `label()` returns `Cow<'static, str>` (no allocation for built-in features).

#### `pcfm-debug` (9 files, 18 tests)

**Public API (CLI):**
- `--seed <U64>`, `--json`, `--html <PATH>`, `--quiet`/`-q`, `--check`, `--list`, `--coverage`, `--help`/`-h`
- `check::run(seed) -> CheckReport`
- `commands::list()`, `commands::coverage()`
- `features::register_all(registry)`
- `demo::demo_match(seed)`, `render::print_text`, `render::print_json`, `render::write_html`

**Key points:**
- Health check: 6 L0 scenarios registered in the `TestRegistry`.
- `--list`: lists the scenarios with feature and description.
- `--coverage`: shows covered vs planned features (2/26).
- `--html`: self-contained, inline CSS, no external assets.
- `demo.rs`: hardcoded (11 players per squad, all "midfielder", `4-4-2`).
- `error.rs`: typed `CliError` with `From` impls.

---

## 7. To be pasted to complete the snapshot

**No files remain to be pasted.** All 118 tracked files have been read:
- 13 ADRs + 1 template
- 1 reading guide in `docs/`
- 2 doctrine modules (`00-foundations.md`, `_index.md`)
- 11 L0 modules
- 2 architecture documents (`ARCHITECTURE.md`, `CODE_TOUR.md`)
- 7 crates (52 code files)
- 4 license files
- 3 CHANGELOGs
- 4 configuration files (`.gitattributes`, `.gitignore`, `.cargo/config.toml`, `rust-toolchain.toml`)
- `Cargo.toml`, `Cargo.lock`
- `README.md`, `STRUCTURE.md`, `CONTRIBUTING.md`
- `GLOSSARY.md`, `MIGRATION-01-04.md`, `PLUGIN_DEVELOPER_GUIDE.md`
- `PROJECT-IDEA-LOG.md`, `REPO-SNAPSHOT.md`, `NEW-SESSION-PROMPT.md`
- `.github/` templates
- `.gitkeep` (empty files)

---

## 8. Inconsistencies found and resolved

All 16 identified inconsistencies have been closed. Summary:

| # | Inconsistency | File | Resolving commit |
|---|---------------|------|------------------|
| 1 | ADR-003 truncated (only "Exception") | `docs/adr/003-plane-a-enforcement.md` | `b309f23` |
| 2 | ADR-004 truncated (half Rust code) | `docs/adr/004-panic-strategy-ffi.md` | `c4c775b` |
| 3 | `module-revision.md` is not a template | `.github/ISSUE_TEMPLATE/module-revision.md` | `251a197` |
| 4 | `00-foundations` says "ADR-013 proposed" (false) | `modules/00-foundations.md` | `d6c3c8b` + `fd2c298` |
| 5 | `_index.md` ADR table stale | `modules/_index.md` | `f1dc5fe` |
| 6 | `CONTRIBUTING.md` status stale | `CONTRIBUTING.md` | `635bc61` |
| 7 | `001` cites `01-architecture.md` (removed) | `docs/adr/001-language-rust.md` | `d03bd2e` |
| 8 | Reading guide in `docs/adr/` with `Status: active` | `docs/adr/012-013-reading-guide.md` | `5cff4f8` (mv) + `46180ab` (content) + `749f154` (rename) + `3a8a0d2` (links) |
| 9 | Terminology `MatchModule` | `docs/adr/012-013-reading-guide.md` | included in 7a |
| 10 | Terminology "module view" | idem | included in 7a |
| 11 | "ADR-012 — Modules" | idem | included in 7a |
| 12 | "same module configuration" | idem | included in 7a |
| 13 | "ADR-012/013 proposed" in various files | various | included in fixes 4, 5, 6 |
| 14 | "Current state" date stale | various | included in fixes 4, 5, 6 |
| 15 | "6 crates / 103 tests" count stale | `_index.md`, `CONTRIBUTING.md` | included in fixes 5, 6 |
| 16 | "10 ADRs" count stale | `CONTRIBUTING.md` | included in fix 6 |

**ADR-015 propagation** (commits `4d826dc` + `4c93214`): `_index.md`, `CONTRIBUTING.md`, `README.md`, `PROJECT-IDEA-LOG.md` updated to reflect `accepted`, 0 proposed.

**Architecture documentation** (commits `e35d15e` + `0a329d4`): `ARCHITECTURE.md` and `CODE_TOUR.md` added; 5 targeted comments in the cryptic points (fix `DoD` in backticks for clippy).

**English translation pass** (commits after `d7e2e65`): `docs/adr/004-panic-strategy-ffi.md` (maintainer note), `docs/PROJECT-IDEA-LOG.md`, `docs/REPO-SNAPSHOT.md`, `docs/adr/015-v1-scope.md` (citation + English translation). All non-English content removed from the repository.

---

## 9. Verified metrics (real output, 2026-10-26)

| Metric | Value | Source |
|--------|-------|--------|
| Crates | 7 | `git ls-files` + `Cargo.toml` |
| Total tests | 107 | `cargo test --workspace` (22+21+4+11+10+18+...) |
| Clippy | clean | `cargo clippy -- -D warnings` |
| `--check` | 6/6 | command output |
| `--list` | 6 scenarios | command output |
| `--coverage` | 2/26 (7%) | command output |
| ADRs | 16 files in `docs/adr/`: 14 accepted (001-008, 012-016, 018), 1 proposed (017), 1 template (000). Plus 1 reading guide in `docs/`. | `ls docs/adr/` + `grep Status` |
| Reading guide | 1 in `docs/` (`PLUGINS-WORLDS-READING-GUIDE.md`) | `ls docs/` |
| L0 modules | 11, all ratified | `git ls-files` |
| Broken links | 0 | verification script (2026-10-26) |
| Working tree | clean | `git status` |
| Last commit | `edcbb6f` (HEAD at snapshot time) | `git log` |
| Tracked files read | 118/118 (100%) | this snapshot |

---

## 10. What I do NOT know (and do not want to invent)

1. **Any other chats or decisions taken outside this session.** If there are any, I do not see them.

Everything else has been read and documented. This snapshot is 100% complete for tracked files.
