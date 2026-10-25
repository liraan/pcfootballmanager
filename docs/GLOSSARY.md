# Glossary

**Version:** 1.0
**Status:** active
**Last updated:** 2026-10-25

Terms, acronyms, and concepts used throughout the specification. Every entry links to the file where the term is defined in full. This file is a navigator, not a replacement for the source.

If a term is missing, open an issue with the `glossary` label. If a definition contradicts its source, the source wins: this file is a summary.

---

## 1. Core concepts

### Three Planes

The non-negotiable architectural separation of data into three layers. Plane A (Ground Truth) is never exposed; Plane B (Observation) is computed from events; Plane C (Estimation) is what the player sees. See [`modules/00-foundations.md`](../modules/00-foundations.md) §3.

### Plane A — Ground Truth

The actual state of the world: latent attributes, physiological state, latent psychological state, internal RNG seed. Lives in `fmr-hidden`. Never exposed. Read only by `fmr-match` and `fmr-gen`, via `resolve_*` functions. See [`modules/00-foundations.md`](../modules/00-foundations.md) §3 and [`docs/adr/003-plane-a-enforcement.md`](adr/003-plane-a-enforcement.md).

### Plane B — Observation

Events and statistics computed exclusively from observable outcomes. Includes `MatchEvent`, aggregated statistics, ratings (computed from events, never from attributes), and temporal trends. Append-only. Lives in `fmr-stats`. See [`modules/L0-match/05-match-events.md`](../modules/L0-match/05-match-events.md).

### Plane C — Estimation

Valuations, projections, and indexes derived from statistical models. Includes `IndexSnapshot`, `ProjectionProfile`, visible attributes, and scouting reports. This is what the player sees. See [`modules/00-foundations.md`](../modules/00-foundations.md) §3.

### Doctrine

The set of non-negotiable rules that every module must obey. Lives in [`modules/00-foundations.md`](../modules/00-foundations.md). Changing the doctrine requires an ADR.

### Concentric structure

The specification is organised in four concentric levels (L0 → L1 → L2 → L3), each expanding the previous one without replacing it. Dependencies flow inward. See [`STRUCTURE.md`](../STRUCTURE.md) §2 and [`modules/00-foundations.md`](../modules/00-foundations.md) §9.

### Level

One of the four concentric layers: L0 (Match), L1 (Season), L2 (Career), L3 (Meta). A level N cell may read the output of a level `< N` cell, never its internals, and never depend on a level `> N` cell.

### Cell

A directory under `modules/`, containing a `00-overview.md` and one or more sub-cells. A cell is identified by a `cell` field and a `module_id` in its frontmatter (e.g. `L0-match`, `L0.04`). A **sub-cell** is a file inside a cell.

### `00-overview.md`

The "border" of a cell. It declares the cell's contracts, dependencies, Definition of Done, and risks. Every cell has one.

---

## 2. Data model

### `MatchEvent`

The atomic unit of Plane B. Every observable action in a match. Append-only: never modified, never deleted. Carries a `CauseTrace`. Defined in [`modules/L0-match/05-match-events.md`](../modules/L0-match/05-match-events.md).

### `EventChain`

The append-only store of `MatchEvent`s, with a hash chain to detect tampering. Reloading a save verifies the chain. Defined in [`modules/L0-match/05-match-events.md`](../modules/L0-match/05-match-events.md) §2.4.

### `CauseTrace`

The structured explanation carried by every `MatchEvent`, describing why the event happened. Mandatory. Contains `primary_cause`, `contributing_factors`, and an `explanation_template`. Defined in [`modules/L0-match/05-match-events.md`](../modules/L0-match/05-match-events.md) §2.3.

### `GroundTruth`

The opaque type exposed by `fmr-hidden`. Contains Plane A. Has no public fields; only the `resolve_*` methods. See [`docs/adr/003-plane-a-enforcement.md`](adr/003-plane-a-enforcement.md) and [`modules/L0-match/03-entities-minimal.md`](../modules/L0-match/03-entities-minimal.md) §3.2.

### `resolve_*`

The public API of `fmr-hidden`: `resolve_duel`, `resolve_shot`, `resolve_pass`, `resolve_pressure`, `resolve_injury_risk`, `resolve_save`, `resolve_set_piece`, `drain_observations`. The match engine calls these; it never reads a latent value. See [`modules/L0-match/03-entities-minimal.md`](../modules/L0-match/03-entities-minimal.md) §3.2.

### `IndexSnapshot`

A windowed observed measurement over time. Replaces the banned CA/PA scalar. Lives in Plane C. See [`modules/00-foundations.md`](../modules/00-foundations.md) §3.

### `ProjectionProfile`

A forecast expressed as a **confidence band**, never a single number. Replaces the banned PA. Contains `expected_band`, `confidence_curve`, `positive_drivers`, `risk_factors`. See [`modules/00-foundations.md`](../modules/00-foundations.md) §3.

### `ContextModifiers`

The struct that carries external state into the match engine. Produced by L1/L2, consumed by L0.04 and L0.08. Every field is observed, none is latent. Defined in [`modules/L0-match/09-context-modifiers.md`](../modules/L0-match/09-context-modifiers.md).

### `RandomEvent`

A discrete in-match event (injury, card, brawl, weather interruption). "Random" means unpredictable to the player, not non-deterministic to the system. Sampled from a dedicated stream. Defined in [`modules/L0-match/08-random-events.md`](../modules/L0-match/08-random-events.md).

### Entity ID

A generational identifier (`EntityId<T>`), never a hash or a pointer. Used for all entities. See [`modules/L0-match/03-entities-minimal.md`](../modules/L0-match/03-entities-minimal.md) §3.1.

### Append-only

A property of `MatchEvent`, `EventChain`, and the registry: entries are never modified, never deleted. Corrections are made by adding a new entry, not by editing an old one.

### Latent vs observed

**Latent** — a Plane A value, never exposed (e.g. `finishing_close = 17`). **Observed** — a Plane B/C value, visible to the player (e.g. `finishing_observed = 15`). The two are related but never equal, and the gap is the point of the game.

---

## 3. Determinism

### Determinism

Same seed + same inputs = same result. Always. A non-negotiable pillar. See [`modules/00-foundations.md`](../modules/00-foundations.md) §4.

### Named RNG stream

A random stream identified by a name (`match:{id}:micro-decisions`) and derived from `(world_seed, stream_name)`. Adding a new stream never shifts the results of existing streams. Implemented in `L0.01`. See [`modules/L0-match/01-rng.md`](../modules/L0-match/01-rng.md).

### `world_seed`

The top-level seed from which all streams are derived. Changing it changes the entire world. Stored in the save. See [`modules/L0-match/01-rng.md`](../modules/L0-match/01-rng.md) §2.4.

### `SIM_VERSION`

The version identifier of the simulation behaviour. Bumped whenever the outcome of a simulation changes for the same seed and inputs. Tracked in [`CHANGELOG-SIM.md`](../CHANGELOG-SIM.md). Defined in `fmr-core` (planned).

### Golden test

A test that replays a known scenario from a known seed and compares a hash of the resulting state. If the hash changes, the CI fails and asks for a `SIM_VERSION` bump. See [`modules/00-foundations.md`](../modules/00-foundations.md) §4.

### Property test

A test that asserts a property that must hold for all inputs (e.g. "money is conserved"), using `proptest`. See [`modules/00-foundations.md`](../modules/00-foundations.md) §4.

---

## 4. Architecture

### `fmr-*` crates

The Rust crates of the project. Each has a single responsibility. The mapping:
- `fmr-core` — time, IDs, RNG, event bus, deterministic math
- `fmr-hidden` — Plane A (latents, `resolve_*`)
- `fmr-match` — the match engine (orchestrator, L0.04)
- `fmr-stats` — Plane B storage (append-only)
- `fmr-model` — pure entities
- `fmr-gen` — world generator
- Others — see [`modules/00-foundations.md`](../modules/00-foundations.md) §8 and [`STRUCTURE.md`](../STRUCTURE.md) §2.

### `fmr-bridge`

The only crate that knows Qt. Contains the FFI shim. See [`docs/adr/004-panic-strategy-ffi.md`](adr/004-panic-strategy-ffi.md) and [`docs/adr/005-qt-binding.md`](adr/005-qt-binding.md).

### `fmr-hidden`

The crate that contains Plane A. Its public API is `resolve_*` and `drain_observations`. There are no public latent fields. See [`docs/adr/003-plane-a-enforcement.md`](adr/003-plane-a-enforcement.md).

### `balance/*.toml`

Configuration files for tunable parameters (e.g. `balance/match_engine.toml`, `balance/ratings.toml`, `balance/random_events.toml`). No balance value is hardcoded. *(planned — the `balance/` directory does not exist yet)*

---

## 5. Process

### ADR

Architecture Decision Record. A short document that captures one decision, its context, the options considered, and the consequences. Immutable once accepted: a change is a new ADR that supersedes the old one. Template in [`docs/adr/000-template.md`](adr/000-template.md).

### Open ADR

An ADR with `status: proposed`. It **blocks** the code in the area it touches. No code is written until it is accepted. See [`CONTRIBUTING.md`](../CONTRIBUTING.md) §"Open ADRs block code".

### Doctrine amendment

A change to `modules/00-foundations.md`. Requires a doctrine ADR. Cannot be done silently.

### Definition of Done (DoD)

The checklist that a cell or feature must satisfy before being considered complete. Every cell has one in its §5.

### Frontmatter

The YAML block at the top of every cell, between `---` markers. Declares `title`, `version`, `status`, `level`, `cell`, `module_id`, `conforms_to`, `depends_on`, `referenced_by`. See [`STRUCTURE.md`](../STRUCTURE.md) §4.

### Status

The state of a cell or ADR: `draft` (being written), `review` (ready for comments), `ratified` (stable), `superseded` (replaced). For ADRs: `proposed`, `accepted`, `rejected`, `superseded`.

### Planned

A cell or file that does not exist yet. Marked `*(planned)*` in prose, or `🚧 planned` in `_index.md`. A planned item is not a promise of a date, only a declaration of intent.

### `conforms_to`

The doctrine version a cell obeys, e.g. `00-foundations (v1.1)`. When the doctrine is revised, every cell's `conforms_to` is checked.

### `depends_on` / `referenced_by`

Frontmatter fields that declare the dependency graph. `depends_on` lists cells this cell reads from; `referenced_by` lists cells that read from this cell. A change to a cell's dependency requires updating both sides.

---

## 6. Banned things

### CA/PA

`current_ability`, `potential_ability`, and any equivalent scalar. **Banned.** Replaced by `IndexSnapshot` and `ProjectionProfile`. See [`modules/00-foundations.md`](../modules/00-foundations.md) §5.

### Truth leakage

Any path by which Plane A data reaches a module outside `fmr-hidden` / `fmr-match` / `fmr-gen`. **Banned.** See [`modules/00-foundations.md`](../modules/00-foundations.md) §3.

### `HashMap` in simulation paths

**Banned.** Use `BTreeMap`, `BTreeSet`, or explicitly sorted `Vec`. Iteration order of `HashMap` is not deterministic. See [`modules/L0-match/01-rng.md`](../modules/L0-match/01-rng.md) §2.5.

### Hardcoded balance values

**Banned.** All tunable values live in `balance/*.toml` or `registry/*.yaml`. See [`modules/00-foundations.md`](../modules/00-foundations.md) §5.

### `Proprietary`

The license of the previous draft. **Banned** (it contradicts the open-data philosophy). Replaced by `MIT OR Apache-2.0` for code and `CC-BY-NC-SA-4.0` for assets. See [`docs/adr/006-license.md`](adr/006-license.md) and [`LICENSE.md`](../LICENSE.md).

---

## 7. External

### Pack

A local expansion. Contains a `manifest.toml`, config, and optionally data. Installed in `mods/`. Validated, quarantined if invalid, rollback-able. See `L3-packs` *(planned)*.

### Registry

The central catalogue of configurable entities (roles, formations, instructions, attributes, metrics, index models). Append-only: entities are deprecated, never deleted. See `L3-registry` *(planned)* and [`CHANGELOG-REGISTRY.md`](../CHANGELOG-REGISTRY.md).

### `mods/`

The directory where packs live. Subdirectories: `installed/`, `disabled/`, `quarantined/`, `_cache/`. See [`STRUCTURE.md`](../STRUCTURE.md) §3.

### `data/`

Example data only. Never real licensed data. `data/generated/` (regenerable worlds) is gitignored. See [`STRUCTURE.md`](../STRUCTURE.md) §3 and [`.gitignore`](../.gitignore).

### `locales/`

Translations. Italian is the first planned translation, in `locales/it/`. *(planned — the `locales/` directory does not exist yet)* See [`docs/adr/007-canonical-language.md`](adr/007-canonical-language.md).

---

## 8. Abbreviations

| Abbreviation | Meaning |
|---|---|
| **PCFM** | PC Football Manager |
| **FM** | Football Manager (the commercial game we are not cloning) |
| **PC Calcio** | The Italian football management game of the 1990s, a cultural reference |
| **IR** | Index di Rendimento (Performance Index), a Plane C measurement |
| **xG / xGA** | Expected Goals / Expected Goals Against |
| **PPDA** | Passes Allowed Per Defensive Action |
| **RSTP** | FIFA Regulations on the Status and Transfer of Players |
| **DoD** | Definition of Done |
| **ADR** | Architecture Decision Record |
| **FFI** | Foreign Function Interface (Rust ↔ C++ for Qt) |
| **UB** | Undefined Behaviour |
| **MSRV** | Minimum Supported Rust Version |
| **SIM_VERSION** | The version identifier of the simulation behaviour |
| **RNG** | Random Number Generator |
| **HSV** | Hue, Saturation, Value (the colour space used for club colours) |
| **UI** | User Interface |
| **API** | Application Programming Interface |
| **PR** | Pull Request |
| **CI** | Continuous Integration |

---

## 9. Historical terms (deprecated)

These terms appeared in previous versions of the specification and are no longer used. They are kept here so that old discussions remain readable.

| Term | Why it is no longer used | Replaced by |
|---|---|---|
| **CA/PA** | Banned (see §6) | `IndexSnapshot` / `ProjectionProfile` |
| **`LatentAttributes` (public)** | Banned (see ADR-003, option B) | `GroundTruth::resolve_*` |
| **25-module structure** | Replaced by the concentric model (L0-L3) | Level + cell |
| **`01-architecture.md` … `04-data-model.md`** | Removed; content is being migrated | Distributed across L0, L1, L3 (see [`docs/MIGRATION-01-04.md`](MIGRATION-01-04.md)) |
| **`Prompt Master`** | The original source doctrine, archived | [`modules/00-foundations.md`](../modules/00-foundations.md) |
| **`modules/L1-season/00-overview.md` (link)** | The file does not exist yet | Marked `*(planned)*` in [`modules/_index.md`](../modules/_index.md) |

---

<!--
  NOTES FOR FUTURE EDITORS:

  - This is a navigator, not a source. Every definition links to the file
    where the term is defined in full. If a term is missing, add it here AND
    in its source, in that order (source first, then glossary).
  - Keep entries short. A glossary entry is one or two sentences plus a link.
    If a term needs more, it deserves its own cell.
  - Keep the sections stable: Core, Data model, Determinism, Architecture,
    Process, Banned, External, Abbreviations, Historical. If a term does not
    fit, that is a signal that the term is either too narrow (put it in the
    source's own glossary) or too broad (it is a concept, not a term).
  - Update "Last updated" when you add a term. Bump "Version" when you
    restructure a section or add more than five terms.
  - This file is linked from:
      - README.md (once the "Documentation Map" is updated to mention it)
      - CONTRIBUTING.md (reading order)
    If you move or rename this file, update those links.
-->
