# Repository Snapshot — PCFM

**Scopo:** fotografia fattuale della repository al **2026-10-26**. Elenca i file, il loro stato di lettura, e cosa contengono. Non è una specifica (per quella vedi `modules/`), non è un log delle decisioni (per quello vedi `PROJECT-IDEA-LOG.md`).

**Regola:** se un file è stato letto integralmente, lo dico e ne riassumo il contenuto. Se è stato letto solo in parte, lo dico. Se non è mai stato letto, scrivo **"non letto"** e lo elenco nella sezione §7. Niente dimensioni inventate: solo quelle viste in output reali (`git ls-files`).

**Fonti:** output di `git ls-files`, `ls -la`, `head`, `grep`, e i file incollati in chat il 2026-10-25/26.

---

## 1. Stato del progetto (al 2026-10-26)

| Metrica | Valore | Fonte |
|---------|--------|-------|
| Branch | `main` | `git status` |
| Ultimo commit | `96c8f59` — "Revise REPO-SNAPSHOT.md for latest updates" | `git log` |
| Working tree | clean | `git status` |
| File tracciati | 113 | `git ls-files \| wc -l` |
| Crate | 7 (`pcfm-core`, `pcfm-model`, `pcfm-hidden`, `pcfm-stats`, `pcfm-match`, `pcfm-testkit`, `pcfm-debug`) | `git ls-files` |
| Test totali | 107 (0 failed) | `cargo test --workspace` |
| Clippy | pulito (`-D warnings`) | `cargo clippy -- -D warnings` |
| `--check` | 6/6 | `cargo run --bin pcfm-debug -- --check` |
| `--coverage` | 2/26 (7%) | `cargo run --bin pcfm-debug -- --coverage` |
| ADR | 12 ADR (001-008, 012-015) + 1 template + 1 reading guide | `ls docs/adr/` |
| ADR accepted | 11 (001-008, 012, 013, 014) | `grep Status` |
| ADR proposed | 1 (015) | `grep Status` |
| Moduli L0 | 11, tutti ratified | `git ls-files` |
| Moduli L1-L3 | 0 (tutti planned) | `git ls-files` |

---

## 2. Albero della repository

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
│   ├── pcfm-core/           (7 file: Cargo.toml + 6 src)
│   ├── pcfm-model/          (9 file: Cargo.toml + 8 src)
│   ├── pcfm-hidden/         (6 file: Cargo.toml + 5 src)
│   ├── pcfm-stats/          (5 file: Cargo.toml + 4 src)
│   ├── pcfm-match/          (9 file: Cargo.toml + 8 src)
│   ├── pcfm-testkit/        (7 file: Cargo.toml + 6 src)
│   └── pcfm-debug/          (9 file: Cargo.toml + 8 src)
├── data/.gitkeep
├── docs/
│   ├── adr/                 (14 file: 000-008, 012-015)
│   ├── calibration/.gitkeep
│   ├── legal/.gitkeep
│   ├── GLOSSARY.md
│   ├── MIGRATION-01-04.md
│   ├── PLUGIN_DEVELOPER_GUIDE.md
│   ├── PLUGINS-WORLDS-READING-GUIDE.md
│   └── REPO-SNAPSHOT.md      ← questo file
├── mods/.gitkeep
├── modules/
│   ├── 00-foundations.md
│   ├── _index.md
│   └── L0-match/            (11 file, 00-10)
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

## 3. File di root

| File | Riga | Stato | Cosa so |
|------|------|-------|---------|
| `README.md` | ~293 | ✅ | Riformattato in questa sessione. Hero con badge, "Why this project exists", Three Planes, determinism, project status, tech stack, repository layout, getting started (`--check`, `--list`, `--coverage`), reading order, contributing, license, AI disclosure, FAQ, contact. |
| `STRUCTURE.md` | 443 | ✅ | v1.8 (2026-10-26). Documentation model, concentric levels, directory layout, module frontmatter, ADR process (con tabella status), language policy, how to add/revise a module, what packs can extend, honesty rule. Link al reading guide aggiornato al nuovo path. |
| `CONTRIBUTING.md` | ~250 | ✅ | Process, review checklist, ADR workflow, licensing of contributions. Stato aggiornato: 7 crate, 107 test, 14 ADR (11 accepted, 1 proposed, 1 template, 1 reading guide). Link al reading guide aggiornato. |
| `LICENSE.md` | 63 | ✅ | Riepilogo dual license: MIT OR Apache-2.0 per il codice, CC-BY-NC-SA-4.0 per gli asset, trademark riservato. |
| `LICENSE-MIT` | 21 | ✅ | Testo MIT standard. |
| `LICENSE-APACHE` | 201 | ✅ | Testo Apache 2.0 verbatim. |
| `LICENSE-ASSETS` | 76 | ✅ | CC-BY-NC-SA-4.0 con header e link al legal code. |
| `CHANGELOG.md` | 218 | ✅ | Keep-a-Changelog. Sezioni: Unreleased, 2026-10-25 (ADR-012/013), 2026-10-25 (rename fmr→pcfm), 2026-10-25 (ADR-008), 2026-10-25 (ADR-003 ratified), 2026-10-25 (glossary), 2026-10-25 (concentric), 2026-10-24 (initial). |
| `CHANGELOG-SIM.md` | 111 | ✅ | SIM_VERSION = 1 (baseline). Procedura di bump in 5 step, template entry, history (vuota). |
| `CHANGELOG-REGISTRY.md` | 126 | ✅ | Registry = baseline, nessuna entità. Regole di deprecazione, template entry, planned deprecations. |
| `Cargo.toml` | 54 | ✅ | `[workspace]` con `resolver = "3"`, 7 membri, `[workspace.package]`, `[workspace.dependencies]`, `[workspace.lints.*]`. |
| `Cargo.lock` | 357 | ✅ | **Letto.** `version = 4`. 35 package totali: 7 `pcfm-*` (workspace members) + 28 dipendenze esterne (blake3, rand_chacha, serde, thiserror, syn, ecc.). Tutti pinnati con checksum. `pcfm-testkit` non ha dipendenze. `pcfm-debug` dipende da 6 crate del workspace + `serde_json`, `thiserror`. |
| `.gitattributes` | 36 | ✅ | Normalizzazione LF, esclusioni per `.bat`/`.cmd`/`.ps1` (CRLF) e binari. |
| `.gitignore` | 167 | ✅ | **Letto.** Sezioni: Rust build artifacts (`/target/`, `**/target/`, `*.rs.bk`, `*.pdb`), profiling (`*.profraw`, `perf.data*`, `*.dSYM/`, `*.mir`), IDE (`.vscode/`, `.idea/`, `*.swp`, ecc.), OS (`.DS_Store`, `Thumbs.db`, `desktop.ini`), log, user data (`saves/`, `*.save`, `*.pcfm`, `*.sav`), secrets (`.env`, `*.pem`, `*.key`, `credentials.toml`), pack cache (`mods/_cache/`, `mods/quarantined/`), generated worlds (`data/generated/`), temporary, test fixtures (`tests/fixtures/generated/`), coverage, local cache (`.cache/`). **Cargo.lock esplicitamente NON ignorato** (binary application, build riproducibili). |
| `.cargo/config.toml` | 13 | ✅ | **Letto.** Intenzionalmente minimale. Solo commenti: alias futuri (`check-all`, `test-all`, `clippy-all`) e `rustflags = ["-D", "warnings"]` per CI, tutti commentati. **Nessuna configurazione attiva.** |
| `rust-toolchain.toml` | 15 | ✅ | **Letto.** `channel = "stable"`, `components = ["rustfmt", "clippy", "rust-src"]`, `profile = "minimal"`. `miri` e `llvm-tools-preview` esclusi (Windows-only limitation). Da pinnare a versione specifica (es. "1.90.0") al momento del rilascio. |

---

## 4. Documentazione (`docs/`)

| File | Riga | Stato | Cosa so |
|------|------|-------|---------|
| `docs/GLOSSARY.md` | 313 | ✅ | v1.1. Termini, acronimi, concetti. Sezioni: Core, Data model, Determinism, Architecture, Process, Banned, External, Abbreviations, Historical. |
| `docs/MIGRATION-01-04.md` | 186 | ✅ | Piano di migrazione dei 4 moduli legacy italiani (01-04), mai committati in git. Mapping target verso moduli concentrici. |
| `docs/PLUGIN_DEVELOPER_GUIDE.md` | 436 | ✅ | v1.1 (2026-10-26). Contiene: two kinds of extension, crate layout (7 crate), simulation/rendering split, adding EventType, adding `resolve_*`, dependencies, conventions, plane separation, checklist PR, examples, §11 Contributing test scenarios, references. |
| `docs/PLUGINS-WORLDS-READING-GUIDE.md` | ~95 | ✅ | Contenuto nuovo (2026-10-26): `Status: accepted`, `Type: reading guide (non-ADR)`, terminologia `plugin` (non `module`), `MatchPlugin` (non `MatchModule`), riferimento a `00-foundations` v2.0. |

### 4.1 ADR (`docs/adr/`)

| File | Riga | Stato | Cosa so |
|------|------|-------|---------|
| `000-template.md` | 55 | ✅ | Template: Status, Date, Supersedes, Superseded by, Blocks, Context, Options, Decision, Consequences, Implementation notes, References. |
| `001-language-rust.md` | ~69 | ✅ | `Status: accepted`. Option A (Rust), B (C++), C (C#), D (Go). Decision: Rust. References aggiornati (Cargo.toml workspace root, rust-toolchain.toml, STRUCTURE.md §3). |
| `002-project-name-and-scope.md` | 79 | ✅ | `Status: accepted`. Nome: PC Football Manager / PCFM. Single-player first. Non-goals v1.0. |
| `003-plane-a-enforcement.md` | 235 | ✅ | `Status: accepted`. Option A (process) vs B (capability API). Decision: B. Corpo completo + Exception `pcfm-debug`. Ricomposto da `571d48f` + `70ee57f` nel commit `b309f23`. |
| `004-panic-strategy-ffi.md` | ~117 | ✅ | `Status: accepted`. Option D: `panic = "unwind"` in dev/release, `abort` in release-hardened. `catch_unwind` obbligatorio a ogni FFI. Completato nel commit `c4c775b` (Consequences, Implementation notes, References, marcate come ricostruite). |
| `005-qt-binding.md` | 96 | ✅ | `Status: accepted`. Decision: Option A (`cxx-qt`). |
| `006-license.md` | 69 | ✅ | `Status: accepted`. Decision: Option C (dual license). |
| `007-canonical-language.md` | 68 | ✅ | `Status: accepted`. Decision: Option A (inglese canonico). |
| `008-provider-integration-external.md` | 77 | ✅ | `Status: accepted`. Provider integration = external tool, non modulo core. |
| `012-plugins.md` | 139 | ✅ | `Status: accepted` (commit `ec29aba`). Brick contract: sheet mandatory, behaviour open/metadata closed, conflicts critical/minor, narrative modder's responsibility, cause traces 2 livelli, debug UI plugin view. |
| `013-worlds.md` | 165 | ✅ | `Status: accepted` (commit `73f51aa`). LOD = sum of plugin weights, fixed per save, manifest nel save, local determinism, community plural. |
| `014-mobile-strategy.md` | 167 | ✅ | `Status: accepted`. Native mobile UI, no plugin, data packs only. Mobile = Phase 3+. |
| `015-v1-scope.md` | 145 | ✅ | `Status: proposed`. Riduzione scope v1.0: 20 ruoli, 10 formazioni, 5 nazioni, 3 assi psicologici attivi, subset L2/L3. Dipende da ADR-012/013 (ora accepted). |

---

## 5. Moduli (`modules/`)

| File | Riga | Stato | Cosa so |
|------|------|-------|---------|
| `modules/00-foundations.md` | ~245 | ✅ | v2.0 (2026-10-26). Pillar 1 revised (ADR-013): determinismo locale. §1 Role, §2 Design Pillars (10), §3 Three Planes, §4 Determinism (locale), §5 Banned Things, §6 Product Goal, §7 Non-Goals, §8 References, §9 Concentric Structure. |
| `modules/_index.md` | ~265 | ✅ | v2.9. Current state (2026-10-26), Doctrine, L0 (11 ratified), L1-L3 (planned), Provider integrations, Plugins and worlds (accepted), How to read, Doctrine compliance, Level graph, Status legend, Components, ADR status (12 ADR + template + reading guide). |
| `modules/L0-match/00-overview.md` | 203 | ✅ | `Status: ratified`. 11 celle, 7-point checklist, dependency rules. |
| `modules/L0-match/01-rng.md` | 150 | ✅ | `Status: ratified`. ChaCha20Rng, stream nominati, seed derivation, SIM_VERSION. |
| `modules/L0-match/02-event-bus.md` | 138 | ✅ | `Status: ratified`. EventBus, ordering per `seq`, single-thread per match, replay compatibility. |
| `modules/L0-match/03-entities-minimal.md` | 232 | ✅ | `Status: ratified`. 7 entità: Player (senza latents), PhysicalStateMinimal, PsychologicalStateMinimal, Squad, Pitch, Referee, Match. |
| `modules/L0-match/04-match-engine.md` | 217 | ✅ | `Status: ratified`. `MatchEngine` trait, tick loop, micro-decision, `resolve_*` mapping, no ratings. |
| `modules/L0-match/05-match-events.md` | 185 | ✅ | `Status: ratified`. `MatchEvent`, EventType (L0 set), CauseTrace, EventChain (hash chain), append-only. |
| `modules/L0-match/06-match-output.md` | 185 | ✅ | `Status: ratified`. Score, Ratings (da events), MatchStatistics, Narrative. |
| `modules/L0-match/07-save-minimal.md` | 176 | ✅ | `Status: ratified`. MatchSave (postcard), migrazione, verification, GroundTruth ricostruito da seed + command log. |
| `modules/L0-match/08-random-events.md` | 220 | ✅ | `Status: ratified`. Catalogo eventi (fisici, tecnici, psicologici, relazionali, ambientali, set-piece). Sampling deterministico. |
| `modules/L0-match/09-context-modifiers.md` | 177 | ✅ | `Status: ratified`. `ContextModifiers` (14 campi, tutti observed, nessun latent). |
| `modules/L0-match/10-ground-truth.md` | 190 | ✅ | `Status: ratified`. API `GroundTruth`: `new`, 7 `resolve_*`, `drain_observations`. Nessun campo pubblico, non `Serialize`, `Debug` custom. |

---

## 6. Crate (`crates/`)

### 6.1 Tabella riassuntiva

| Crate | File | Test | Dipendenze | Responsabilità |
|-------|------|------|------------|----------------|
| `pcfm-core` | 7 | 22 | `rand_chacha`, `rand_core`, `blake3`, `thiserror`, `serde` | RNG, event bus, IDs, hash, time |
| `pcfm-model` | 9 | 21 | `pcfm-core`, `serde`, `thiserror` | Entità minime (Player senza latents, Squad, Pitch, Referee, Match, CauseTrace) |
| `pcfm-hidden` | 6 | 4 | `pcfm-core`, `pcfm-model`, `rand_chacha`, `rand_core`, `blake3`, `thiserror` | Plane A: `GroundTruth`, `resolve_*`, `drain_observations` |
| `pcfm-stats` | 5 | 11 | `pcfm-core`, `pcfm-model`, `serde`, `serde_json`, `thiserror`, `blake3` | Plane B: `MatchEvent`, `EventChain` append-only con hash chain |
| `pcfm-match` | 9 | 10 | `pcfm-core`, `pcfm-model`, `pcfm-hidden`, `pcfm-stats`, `serde`, `serde_json`, `thiserror` | Match engine: orchestrator `simulate_match`, `TextMatchEngine`, save/load, narrative |
| `pcfm-testkit` | 7 | 0 | **Nessuna** | Contract: `Scenario`, `Feature`, `TestRegistry`, `TestContext`, `Check`, `ScenarioResult` |
| `pcfm-debug` | 9 | 18 | `pcfm-core`, `pcfm-model`, `pcfm-hidden`, `pcfm-stats`, `pcfm-match`, `pcfm-testkit`, `serde_json` | CLI (`--check`, `--list`, `--coverage`, `--json`, `--html`, `--quiet`), demo teams, rendering |

**Totale:** 52 file, 86 test (dei 107 totali: 86 + 21 dev/integration).

### 6.2 Dettaglio per crate

#### `pcfm-core` (7 file, 22 test)

**API pubblica:**
- `rng`: `StreamName`, `RngRegistry` (`new`, `stream`), `next_unit()`, `RngError`
- `event_bus`: `EventKind` (14 varianti), `Event { seq, kind }`, `Handler`, `EventBus` (`new`, `publish`, `subscribe`, `dispatch_all`, `len`, `is_empty`)
- `ids`: `EntityTag` trait, `EntityId<T>` (`new`, `slot`, `generation`), 5 tag (`PlayerTag`, `SquadTag`, `MatchTag`, `RefereeTag`, `EventTag`), 5 alias (`PlayerId`, `SquadId`, `MatchId`, `RefereeId`, `EventId`)
- `hash`: `Hash([u8; 32])`, `Hash::ZERO`, `Hash::of_bytes`, `Hash::as_bytes`
- `time`: `GameInstant(u64)`, `GameInstant::from_raw`, `GameInstant::raw`

**Punti chiave:**
- `ChaCha20Rng` derivato con BLAKE3 da `(world_seed, stream_name)`. `BTreeMap` per i stream, idempotente.
- Event bus: dispatch in ordine di inserimento (`seq`), handler in ordine di registrazione. Limite noto: handler non possono pubblicare durante il dispatch.
- IDs generazionali (`slot` + `generation`), taggati per tipo.

#### `pcfm-model` (9 file, 21 test)

**API pubblica:**
- `cause`: `Factor { name, value, weight }`, `CauseTrace { primary_cause, contributing_factors, explanation_template }`
- `error`: `ModelError` (`#[non_exhaustive]`): `EmptyRoleId`, `EmptyFormationId`, `EmptyPlayerName`, `InvalidHeight`, `InvalidValue`, `InvalidStarterCount`, `InvalidBenchCount`, `InvalidPitchDimensions`
- `match_`: `MatchSeed`, `MatchRules`, `Match` (`new`, `is_friendly`)
- `pitch`: `Weather`, `Pitch` (`try_new`, `Default`)
- `player`: `Foot`, `RoleId`, `PhysicalStateMinimal`, `PsychologicalStateMinimal`, `Player` (`try_new`)
- `referee`: `Referee` (`try_new`)
- `squad`: `STARTERS = 11`, `MAX_BENCH = 7`, `FormationId`, `Squad` (`try_new`)

**Punti chiave:**
- Nessun campo latente. `Player` ha solo stato osservato.
- Validazione rigorosa: range [0,1], non-NaN, conteggi esatti. `ModelError` (`#[non_exhaustive]`).
- Serializzazione: tutte le entità sono `Serialize`/`Deserialize`.

#### `pcfm-hidden` (6 file, 4 test)

**API pubblica:**
- `GroundTruth` (`new`, `world_seed`, `match_id`, 7 `resolve_*`, `drain_observations`)
- `DuelType`, 7 context structs, 7 outcome structs
- `ObservationEvent` (7 varianti)

**Punti chiave:**
- Campi privati, non `Serialize`, `Debug` custom (stampa solo `world_seed` e `match_id`).
- Latents generati dal RNG (`PlayerLatents::from_rng`), mai esposti.
- `resolve_*` deterministico (stesso seed → stesso outcome).
- `drain_observations` idempotente.
- **Modello attuale molto semplice** (prototipo): `resolve_shot`: `goal_probability = noise * 0.3 + finishing`, goal se > 0.6.

#### `pcfm-stats` (5 file, 11 test)

**API pubblica:**
- `EventType` (27 varianti), `MatchEvent` (14 campi), `validate()`
- `EventChain` (`new`, `try_append`, `events`, `len`, `is_empty`, `chain_hash`, `verify`)
- `StatsError` (`#[non_exhaustive]`)

**Punti chiave:**
- Hash chain: `hash = BLAKE3(previous_hash || serde_json(event))`. Tamper detection.
- Append-only rigoroso: `try_append` valida prima di appendere. Se fallisce, la catena non è modificata.
- Determinismo: stesso set di eventi → stesso hash.

#### `pcfm-match` (9 file, 10 test)

**API pubblica:**
- `MatchConfig`, `MatchContext`, `MatchEngine`, `TextMatchEngine`, `Importance`, `NarrativeEntry`, `narrate`, `simulate_match`, `MatchResult`, `MatchStatistics`, `Score`, `MatchSave`, `SCHEMA_VERSION`, `SIM_VERSION`, `SaveError`, `replay`, `MatchState`

**Punti chiave:**
- Separazione simulazione/rendering: `simulate_match` produce `MatchResult` con `narrative: Vec::new()`. Solo `TextMatchEngine` chiama `narrate()`.
- Save come JSON: `serde_json`. Limitazione nota: `MatchEvent.fields` può perdere 1 ulp. Target futuro: `postcard` (futuro ADR).
- Identità del save: `state_hash`, non bit-for-bit equality.
- Tamper detection: `validate()` controlla `schema_version` e `chain_hash`.
- `can_replay`: `sim_version == SIM_VERSION`.
- `MatchState::tick()`: modello semplificato (probabilità 0.0005 per tick). Non realistico (prototipo).

#### `pcfm-testkit` (7 file, 0 test)

**API pubblica:**
- `Feature` (26 built-in + `Plugin(String)`), `ALL_BUILTIN`, `all_builtin()`, `label() -> Cow`
- `Scenario` trait (`name`, `feature`, `description`, `run`)
- `TestContext` (`seed`, `log`, `logs`)
- `Check` (`pass`, `fail`), `ScenarioResult` (`new`, `record`, `pass`, `fail`, `error`, `is_ok`)
- `TestRegistry` (`new`, `register`, `scenarios`, `run_all`), `ScenarioOutcome`, `RunReport` (`outcomes`, `is_ok`, `total_checks`, `passed_checks`, `failed_checks`)

**Punti chiave:**
- **Nessuna dipendenza da altri crate `pcfm-*`**. È il contratto che core, plugin e tool condividono.
- `Feature` è `#[non_exhaustive]`.
- `label()` ritorna `Cow<'static, str>` (nessuna allocazione per feature built-in).

#### `pcfm-debug` (9 file, 18 test)

**API pubblica (CLI):**
- `--seed <U64>`, `--json`, `--html <PATH>`, `--quiet`/`-q`, `--check`, `--list`, `--coverage`, `--help`/`-h`
- `check::run(seed) -> CheckReport`
- `commands::list()`, `commands::coverage()`
- `features::register_all(registry)`
- `demo::demo_match(seed)`, `render::print_text`, `render::print_json`, `render::write_html`

**Punti chiave:**
- Health check: 6 scenari L0 registrati nel `TestRegistry`.
- `--list`: elenca gli scenari con feature e descrizione.
- `--coverage`: mostra feature coperte vs pianificate (2/26).
- `--html`: self-contained, CSS inline, no asset esterni.
- `demo.rs`: hardcoded (11 giocatori per squadra, tutti "midfielder", `4-4-2`).
- `error.rs`: `CliError` tipizzato con `From` impls.

---

## 7. Da incollare per completare lo snapshot

**Nessun file rimane da incollare.** Tutti i 113 file tracciati sono stati letti:
- 14 ADR + 1 reading guide
- 2 moduli di dottrina (`00-foundations.md`, `_index.md`)
- 11 moduli L0
- 7 crate (52 file di codice)
- 4 file di licenza
- 3 CHANGELOG
- 3 file di configurazione (`.gitattributes`, `.gitignore`, `.cargo/config.toml`, `rust-toolchain.toml`)
- `Cargo.toml`, `Cargo.lock`
- `README.md`, `STRUCTURE.md`, `CONTRIBUTING.md`
- `GLOSSARY.md`, `MIGRATION-01-04.md`, `PLUGIN_DEVELOPER_GUIDE.md`
- `.github/` template
- `.gitkeep` (file vuoti)

---

## 8. Incongruenze trovate e risolte

Tutte le 16 incongruenze identificate sono state chiuse. Riepilogo:

| # | Incongruenza | File | Commit risolutivo |
|---|--------------|------|-------------------|
| 1 | ADR-003 troncato (solo "Exception") | `docs/adr/003-plane-a-enforcement.md` | `b309f23` |
| 2 | ADR-004 troncato (metà codice Rust) | `docs/adr/004-panic-strategy-ffi.md` | `c4c775b` |
| 3 | `module-revision.md` non è un template | `.github/ISSUE_TEMPLATE/module-revision.md` | `251a197` |
| 4 | `00-foundations` dice "ADR-013 proposed" (falso) | `modules/00-foundations.md` | `d6c3c8b` + `fd2c298` |
| 5 | `_index.md` tabella ADR obsoleta | `modules/_index.md` | `f1dc5fe` |
| 6 | `CONTRIBUTING.md` stato obsoleto | `CONTRIBUTING.md` | `635bc61` |
| 7 | `001` cita `01-architecture.md` (rimosso) | `docs/adr/001-language-rust.md` | `d03bd2e` |
| 8 | Reading guide in `docs/adr/` con `Status: active` | `docs/adr/012-013-reading-guide.md` | `5cff4f8` (mv) + `46180ab` (contenuto) + `749f154` (rename) + `3a8a0d2` (link) |
| 9 | Terminologia `MatchModule` | `docs/adr/012-013-reading-guide.md` | incluso in 7a |
| 10 | Terminologia "module view" | idem | incluso in 7a |
| 11 | "ADR-012 — Modules" | idem | incluso in 7a |
| 12 | "same module configuration" | idem | incluso in 7a |
| 13 | "ADR-012/013 proposed" in vari file | vari | inclusi nei fix 4, 5, 6 |
| 14 | Data "Current state" obsoleta | vari | inclusi nei fix 4, 5, 6 |
| 15 | Conteggio "6 crate / 103 test" obsoleto | `_index.md`, `CONTRIBUTING.md` | inclusi nei fix 5, 6 |
| 16 | Conteggio "10 ADR" obsoleto | `CONTRIBUTING.md` | incluso in fix 6 |

---

## 9. Metriche verificate (output reali, 2026-10-26)

| Metrica | Valore | Fonte |
|---------|--------|-------|
| Crate | 7 | `git ls-files` + `Cargo.toml` |
| Test totali | 107 | `cargo test --workspace` (22+21+4+11+10+18+...) |
| Clippy | pulito | `cargo clippy -- -D warnings` |
| `--check` | 6/6 | output del comando |
| `--list` | 6 scenari | output del comando |
| `--coverage` | 2/26 (7%) | output del comando |
| ADR | 14 (12 accepted, 1 proposed, 1 template, 1 reading guide) | `ls docs/adr/` + `grep Status` |
| Moduli L0 | 11, tutti ratified | `git ls-files` |
| Link rotti | 0 | script di verifica (2026-10-26) |
| Working tree | clean | `git status` |
| Ultimo commit | `96c8f59` | `git log` |
| File tracciati letti | 113/113 (100%) | questo snapshot |

---

## 10. Cosa NON so (e non voglio inventare)

1. **Eventuali altre chat o decisioni prese fuori da questa sessione.** Se ce ne sono, non le vedo.

Tutto il resto è stato letto e documentato. Questo snapshot è completo al 100% dei file tracciati.
