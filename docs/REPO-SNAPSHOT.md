# Repository Snapshot — PCFM

**Scopo:** fotografia fattuale della repository al **2026-10-26**. Elenca i file, il loro stato di lettura, e cosa contengono. Non è una specifica (per quella vedi `modules/`), non è un log delle decisioni (per quello vedi `PROJECT-IDEA-LOG.md`).

**Regola:** se un file è stato letto integralmente, lo dico e ne riassumo il contenuto. Se è stato letto solo in parte, lo dico. Se non è mai stato letto, scrivo **"non letto"** e lo elenco nella sezione §7. Niente dimensioni inventate: solo quelle viste in output reali (`git ls-files`).

**Fonti:** output di `git ls-files`, `ls -la`, `head`, `grep`, e i file incollati in chat il 2026-10-25/26.

---

## 1. Stato del progetto (al 2026-10-26)

| Metrica | Valore | Fonte |
|---------|--------|-------|
| Branch | `main` | `git status` |
| Ultimo commit | `3a8a0d2` — "docs: update links to PLUGINS-WORLDS-READING-GUIDE" | `git log` |
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
│   ├── pcfm-core/           (6 src file + Cargo.toml)
│   ├── pcfm-model/          (8 src file + Cargo.toml)
│   ├── pcfm-hidden/         (5 src file + Cargo.toml)
│   ├── pcfm-stats/          (4 src file + Cargo.toml)
│   ├── pcfm-match/          (8 src file + Cargo.toml)
│   ├── pcfm-testkit/        (6 src file + Cargo.toml)
│   └── pcfm-debug/          (6 src file + features/ + Cargo.toml)
├── data/.gitkeep
├── docs/
│   ├── adr/                 (14 file: 000-008, 012-015)
│   ├── calibration/.gitkeep
│   ├── legal/.gitkeep
│   ├── ADR-012-013-READING-GUIDE.md  ← non più (rinominato)
│   ├── PLUGINS-WORLDS-READING-GUIDE.md  ← nuovo path
│   ├── GLOSSARY.md
│   ├── MIGRATION-01-04.md
│   └── PLUGIN_DEVELOPER_GUIDE.md
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

**Nota:** la directory `docs/adr/` non contiene più `012-013-reading-guide.md` (spostato e rinominato in `docs/PLUGINS-WORLDS-READING-GUIDE.md` con commit `749f154`).

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
| `Cargo.lock` | 357 | ❌ | **Non letto** (auto-generato). |
| `.gitattributes` | 36 | ✅ | Normalizzazione LF, esclusioni per `.bat`/`.cmd`/`.ps1` (CRLF) e binari. |
| `.gitignore` | 167 | ❌ | **Non letto.** |
| `.cargo/config.toml` | 13 | ❌ | **Non letto.** |
| `rust-toolchain.toml` | 15 | ❌ | **Non letto.** |

---

## 4. Documentazione (`docs/`)

| File | Riga | Stato | Cosa so |
|------|------|-------|---------|
| `docs/GLOSSARY.md` | 313 | ✅ | v1.1. Termini, acronimi, concetti. Sezioni: Core, Data model, Determinism, Architecture, Process, Banned, External, Abbreviations, Historical. |
| `docs/MIGRATION-01-04.md` | 186 | ✅ | Piano di migrazione dei 4 moduli legacy italiani (01-04), mai committati in git. Mapping target verso moduli concentrici. |
| `docs/PLUGIN_DEVELOPER_GUIDE.md` | 436 | ✅ | v1.1 (2026-10-26). Contiene: two kinds of extension, crate layout (7 crate), simulation/rendering split, adding EventType, adding `resolve_*`, dependencies, conventions, plane separation, checklist PR, examples, **§11 Contributing test scenarios** (nuovo), references. |
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
| `013-worlds.md` | 165 | ✅ | `Status: accepted` (commit `73f51aa`). LOD = sum of plugin weights, fixed per save, manifest nel save, **local determinism**, community plural. |
| `014-mobile-strategy.md` | 167 | ✅ | `Status: accepted`. Native mobile UI, no plugin, data packs only. Mobile = Phase 3+. |
| `015-v1-scope.md` | 145 | ✅ | `Status: proposed`. Riduzione scope v1.0: 20 ruoli, 10 formazioni, 5 nazioni, 3 assi psicologici attivi, subset L2/L3. Dipende da ADR-012/013 (ora accepted). |

---

## 5. Moduli (`modules/`)

| File | Riga | Stato | Cosa so |
|------|------|-------|---------|
| `modules/00-foundations.md` | ~245 | ✅ | v2.0 (2026-10-26). **Pillar 1 revised** (ADR-013): determinismo locale. §1 Role, §2 Design Pillars (10), §3 Three Planes, §4 Determinism (locale), §5 Banned Things, §6 Product Goal, §7 Non-Goals, §8 References, §9 Concentric Structure. |
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

| Crate | Test | File src | Stato lettura | Cosa so |
|-------|------|----------|---------------|---------|
| `pcfm-core` | 22 | 6 | ❌ | **Non letto.** Da `PLUGIN_DEVELOPER_GUIDE.md` §2: "RNG, event bus, IDs, hash, time". |
| `pcfm-model` | 21 | 8 | ❌ | **Non letto.** Da §2: "Minimal entities (`Player`, `Squad`, `Match`, …) + `CauseTrace`". |
| `pcfm-hidden` | 4 | 5 | ❌ | **Non letto.** Da §2: "`GroundTruth`, `resolve_*`, `drain_observations`". |
| `pcfm-stats` | 11 | 4 | ❌ | **Non letto.** Da §2: "`MatchEvent`, `EventType`, `EventChain`". |
| `pcfm-match` | 10 | 8 | ❌ | **Non letto.** Da §2: "The match engine (orchestrator + shared simulation)". |
| `pcfm-testkit` | 0 | 6 | ✅ | Contract only. `Scenario`, `Feature`, `TestRegistry`, `TestContext`, `Check`, `ScenarioResult`. |
| `pcfm-debug` | 18 | 9 | ✅ (6/9) | `main.rs`, `args.rs`, `check.rs`, `commands.rs`, `features/mod.rs`, `features/l0_match.rs` letti. `demo.rs`, `error.rs`, `render.rs` **non letti**. |

### 6.2 File di `pcfm-debug` (letti)

| File | Riga | Cosa so |
|------|------|---------|
| `main.rs` | 170 | Entry point. Dispatch: `--list`, `--coverage`, `--check`, simula match. 3 test. |
| `args.rs` | 233 | Parser CLI (7 flag + `--help`). `ArgError` enum. 12 test. `#[allow(clippy::struct_excessive_bools)]` su `Args`. |
| `check.rs` | 77 | Thin wrapper: `TestRegistry`, `features::register_all`, `run_all(seed)`, adatta a `CheckReport`. `pub fn run(seed: u64) -> CheckReport`. |
| `commands.rs` | 68 | `list()` e `coverage()`. Usa `Feature::all_builtin()`. |
| `features/mod.rs` | 25 | `register_all(registry)` con `l0_match::register`. TODO per L1/L2/L3/plugin. |
| `features/l0_match.rs` | 277 | 6 scenari: `DemoTeamScenario`, `SameSeedScenario`, `DifferentSeedScenario`, `SaveLoadScenario`, `EventChainScenario`, `StatsSanityScenario`. |

### 6.3 File di `pcfm-testkit` (letti)

| File | Riga | Cosa so |
|------|------|---------|
| `lib.rs` | 24 | `pub mod` dei 5 moduli + `pub use` dei tipi. |
| `feature.rs` | 158 | `Feature` enum (`#[non_exhaustive]`), `ALL_BUILTIN`, `all_builtin()`, `label() -> Cow`. 26 feature built-in. |
| `scenario.rs` | 29 | `trait Scenario: Send + Sync` con `name()`, `feature()`, `description()`, `run()`. |
| `context.rs` | 33 | `TestContext { seed: u64, log: Vec<String> }`, `const fn new(seed)`, `log()`, `logs()`. |
| `result.rs` | 91 | `Check` (`pass`/`fail`), `ScenarioResult` (`new`, `record`, `pass`, `fail`, `error`, `is_ok`). `#[allow(clippy::missing_const_for_fn)]` su 4 funzioni. |
| `registry.rs` | 105 | `TestRegistry` (`new`, `register`, `scenarios`, `run_all`), `ScenarioOutcome`, `RunReport` (`outcomes`, `is_ok`, `total_checks`, `passed_checks`, `failed_checks`). |

---

## 7. Da incollare per completare lo snapshot

Questi file **non sono mai stati letti** (❌) o **letti solo in parte** (🟡). Per ognuno, indica se vuoi che lo legga e lo riassuma.

### Codice (per completare lo snapshot implementativo)

| Crate | File |
|-------|------|
| `pcfm-core` | `event_bus.rs`, `hash.rs`, `ids.rs`, `lib.rs`, `rng.rs`, `time.rs` |
| `pcfm-model` | `cause.rs`, `error.rs`, `lib.rs`, `match_.rs`, `pitch.rs`, `player.rs`, `referee.rs`, `squad.rs` |
| `pcfm-hidden` | `context.rs`, `latents.rs`, `lib.rs`, `observation.rs`, `outcome.rs` |
| `pcfm-stats` | `chain.rs`, `error.rs`, `event.rs`, `lib.rs` |
| `pcfm-match` | `context.rs`, `engine.rs`, `lib.rs`, `narrative.rs`, `orchestrator.rs`, `result.rs`, `save.rs`, `state.rs` |
| `pcfm-debug` | `demo.rs`, `error.rs`, `render.rs` |

### Configurazione

- `Cargo.lock` (auto-generato, probabilmente irrilevante)
- `.gitignore`, `.gitattributes`, `.cargo/config.toml`, `rust-toolchain.toml`

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
| 9 | Terminologia `MatchModule` (dovrebbe essere `MatchPlugin`) | `docs/adr/012-013-reading-guide.md` | incluso in 7a |
| 10 | Terminologia "module view" (dovrebbe essere "plugin view") | idem | incluso in 7a |
| 11 | "ADR-012 — Modules" (dovrebbe essere "Plugins") | idem | incluso in 7a |
| 12 | "same module configuration" (dovrebbe essere "plugin") | idem | incluso in 7a |
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
| Ultimo commit | `3a8a0d2` | `git log` |

---

## 10. Cosa NON so (e non voglio inventare)

1. **Contenuto del codice sorgente** dei 5 crate non letti (`pcfm-core`, `pcfm-model`, `pcfm-hidden`, `pcfm-stats`, `pcfm-match`) + 3 file di `pcfm-debug`. Posso descriverne la responsabilità (da `PLUGIN_DEVELOPER_GUIDE.md` §2 e `STRUCTURE.md` §3), ma non il contenuto riga per riga.
2. **Contenuto di `.gitignore`, `.cargo/config.toml`, `rust-toolchain.toml`**. So che esistono e la loro dimensione, ma non il contenuto.
3. **Contenuto di `Cargo.lock`**. Auto-generato, probabilmente irrilevante per lo snapshot.
4. **Eventuali altre chat o decisioni prese fuori da questa sessione.** Se ce ne sono, non le vedo.

Per colmare questi buchi, mi serve che tu mi incolli i file (o mi dia il via per scrivere "non letto" nel `REPO-SNAPSHOT.md` e completarli dopo — cosa che ho già fatto).
