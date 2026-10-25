---
title: "Modules Index"
version: "2.7"
status: "active"
last_updated: "2026-10-25"
---

# Modules Index

The specification is **concentric**, not flat. Read [`00-foundations.md`](00-foundations.md) first, then the level that matches your task.

```
L0 — MATCH       Two teams play one match
L1 — SEASON      A league, a calendar, a world
L2 — CAREER      Market, finance, youth, scouting, psyche, crowd, media, AI
L3 — META        Registry, packs, generator, editor, assets, UI, statistics, quality
```

Dependencies flow **inward**: L2 may depend on L1 and L0, never on L3. L0 depends only on `00-foundations`.

**Reading order:**

1. [`00-foundations.md`](00-foundations.md) — the doctrine *(v1.4, pillar 1 under review)*
2. The level that matches your task (L0 → L3)
3. [`../STRUCTURE.md`](../STRUCTURE.md) — the rules of the repo
4. [`../docs/adr/`](../docs/adr/) — the decisions

---

## Current state (as of 2026-10-25)

- ✅ **Doctrine**: ratified (v1.4). **Pillar 1 is under review** (ADR-013, proposed).
- ✅ **L0 — Match**: fully specified, **all 11 cells ratified** (ADR-003 accepted)
- 🚧 **L1 — Season**: planned, not started (12 cells)
- 🚧 **L2 — Career**: planned, not started (9 cells)
- 🚧 **L3 — Meta**: planned, not started (8 cells)
- 🔌 **Provider integrations** (Wyscout, StatsBomb, Opta): **external tools, not core cells** (ADR-008)
- 🧱 **Modules and worlds**: ADR-012 and ADR-013 are **proposed**. The framework does not yet implement the module system. The determinism is **global** today; the move to **local** determinism is under review.

Cells marked *(planned)* below do not have a file yet. Their links are intentionally not clickable.

---

## Doctrine

| File | Status |
|---|---|
| [`00-foundations.md`](00-foundations.md) | ✅ ratified (v1.4) — pillar 1 under review (ADR-013) |

**Note on `conforms_to`:** the doctrine is **v1.4**. All cells declare `conforms_to: "00-foundations (v1.1)"` because **v1.1 is the last version that changed an invariant**. v1.2 added one sentence to §7 (Non-goals). v1.3 added a cross-reference to the module developer guide. v1.4 added the "pillar 1 under review" note and the §4 status note. None of these changes an invariant, so the cells do not need to be re-checked. When the pillar is rewritten (ADR-013 ratified), the doctrine will bump to v2.0, and each cell will be re-checked against it.

---

## L0 — Match (core)

| ID | File | Status | Depends on |
|---|---|---|---|
| L0.00 | [`L0-match/00-overview.md`](L0-match/00-overview.md) | ✅ ratified | 00 |
| L0.01 | [`L0-match/01-rng.md`](L0-match/01-rng.md) | ✅ ratified | 00 |
| L0.02 | [`L0-match/02-event-bus.md`](L0-match/02-event-bus.md) | ✅ ratified | 00 |
| L0.03 | [`L0-match/03-entities-minimal.md`](L0-match/03-entities-minimal.md) | ✅ ratified | 00, L0.01 |
| L0.04 | [`L0-match/04-match-engine.md`](L0-match/04-match-engine.md) | ✅ ratified | 00, L0.01, L0.02, L0.03, L0.10 |
| L0.05 | [`L0-match/05-match-events.md`](L0-match/05-match-events.md) | ✅ ratified | 00, L0.01, L0.03 |
| L0.06 | [`L0-match/06-match-output.md`](L0-match/06-match-output.md) | ✅ ratified | 00, L0.05 |
| L0.07 | [`L0-match/07-save-minimal.md`](L0-match/07-save-minimal.md) | ✅ ratified | 00, L0.01, L0.03, L0.05, L0.06, L0.10 |
| L0.08 | [`L0-match/08-random-events.md`](L0-match/08-random-events.md) | ✅ ratified | 00, L0.01, L0.03, L0.04, L0.05, L0.10 |
| L0.09 | [`L0-match/09-context-modifiers.md`](L0-match/09-context-modifiers.md) | ✅ ratified | 00, L0.03 |
| L0.10 | [`L0-match/10-ground-truth.md`](L0-match/10-ground-truth.md) | ✅ ratified | 00, L0.01, L0.03 |

**L0 is done when:** a friendly match is playable, savable, reloadable, deterministic, and every event has a cause trace. The **specification** is complete; the **code** is complete (6 crates, 103 tests). See `STRUCTURE.md` §3.

---

## L1 — Season (context)

*(planned — none of these files exist yet)*

| ID | Cell | Status | Depends on |
|---|---|---|---|
| L1.00 | L1-season/00-overview.md | 🚧 planned | L0 |
| L1.01 | L1-season/01-competitions.md | 🚧 planned | L0, L1.00 |
| L1.02 | L1-season/02-calendar.md | 🚧 planned | L0, L1.00, L1.01 |
| L1.03 | L1-season/03-standings.md | 🚧 planned | L0, L1.00, L1.01, L1.02 |
| L1.04 | L1-world/00-overview.md | 🚧 planned | L0 |
| L1.05 | L1-world/01-nation.md | 🚧 planned | L0, L1.04 |
| L1.06 | L1-world/02-city.md | 🚧 planned | L0, L1.04 |
| L1.07 | L1-world/03-stadium.md | 🚧 planned | L0, L1.04 |
| L1.08 | L1-roster/00-overview.md | 🚧 planned | L0 |
| L1.09 | L1-roster/01-player-attributes.md | 🚧 planned | L0, L1.08 |
| L1.10 | L1-roster/02-staff.md | 🚧 planned | L0, L1.08, L1.09 |
| L1.11 | L1-roster/03-contracts.md | 🚧 planned | L0, L1.08, L1.09 |

**L1 is done when:** a league season is playable, standings are visible, the world exists (nations, cities, stadiums), rosters are complete.

---

## L2 — Career (living world)

*(planned — none of these directories exist yet)*

| ID | Cell | Status |
|---|---|---|
| L2.01 | L2-tactics/ | 🚧 planned |
| L2.02 | L2-market/ | 🚧 planned |
| L2.03 | L2-finance/ | 🚧 planned |
| L2.04 | L2-youth/ | 🚧 planned |
| L2.05 | L2-scouting/ | 🚧 planned |
| L2.06 | L2-psyche/ | 🚧 planned |
| L2.07 | L2-crowd/ | 🚧 planned |
| L2.08 | L2-media/ | 🚧 planned |
| L2.09 | L2-ai/ | 🚧 planned |

**L2 is done when:** a full career is playable — transfers, budgets, youth intake, tactics, psychology, fans, media, AI.

---

## L3 — Meta (moddable)

*(planned — none of these directories exist yet)*

| ID | Cell | Status |
|---|---|---|
| L3.01 | L3-registry/ | 🚧 planned |
| L3.02 | L3-packs/ | 🚧 planned |
| L3.03 | L3-generator/ | 🚧 planned |
| L3.04 | L3-editor/ | 🚧 planned |
| L3.05 | L3-assets/ | 🚧 planned |
| L3.06 | L3-ui/ | 🚧 planned |
| L3.07 | L3-statistics/ | 🚧 planned |
| L3.08 | L3-quality/ | 🚧 planned |

**L3 is done when:** the game is moddable — packs install, registry extends, generator produces worlds, editor edits, UI themes, statistics expose, quality gates run.

---

## Provider integrations (external, not core)

| Former ID | What it was | What it is now |
|---|---|---|
| ~~L2-ingest~~ | A core module for Wyscout / StatsBomb / Opta integration | **🔌 External tool**, produces packs, not a core cell. See [ADR-008](../docs/adr/008-provider-integration-external.md). |

The core does not talk to providers. A tool (team, community, or third party) talks to the provider, produces a pack, and the user installs the pack. This keeps the core offline-first, sandboxed, and lightweight.

---

## Modules and worlds (proposed)

ADR-012 (Modules: The Brick Contract) and ADR-013 (Worlds: LOD, Manifest, and Local Determinism) define the module system and the world system. They are **proposed**: the framework does not yet implement them.

ADR-012 defines the brick contract (sheet, conflicts, narrative, cause traces, debug UI module view). ADR-013 defines how the bricks compose into worlds (LOD = sum of modules, LOD fixed per save, manifest in save, **local determinism**, community plural and aware).

ADR-013 **supersedes pillar 1** of `00-foundations` §2: the determinism becomes **local** (same save + same match + same module configuration = same result) instead of **global** (same seed regardless of configuration). The pillar is currently marked "under review" in the doctrine.

Read the two ADRs together, in order: [`012-013-reading-guide.md`](../docs/adr/012-013-reading-guide.md). ADR-013 depends on ADR-012.

For the current status of the two ADRs, see the **ADR status** table below.

---

## How to read a cell

Every cell has:

- **Frontmatter** — level, cell, module_id, version, status, dependencies, doctrine compliance (see `STRUCTURE.md` §4)
- **§0 Purpose** — what the cell covers, what it does not
- **§1 Context** — what it depends on, what depends on it
- **§2 Specification** — the actual content
- **§3 Data model** — entities, if any
- **§4 Edge cases**
- **§5 Definition of Done** (split into "Specification" and "Implementation" once code starts)
- **§6 Risks and Mitigations**
- **§7 References**

If a cell lacks any of these, it is a bug. Open an issue.

---

## Doctrine compliance

Every cell declares `conforms_to: 00-foundations (vX.Y)`. The `conforms_to` field means **"the cell has been verified against this version"**, not **"the cell is compatible with any later version"**. When `00-foundations` is revised, a cell that has not been re-verified is marked `status: review` until it is.

The doctrine is currently **v1.4**. Cells conform to **v1.1**, which is the last version that changed an invariant. v1.2 → v1.4 added clarifications and a status note (pillar 1 under review), none of which change an invariant. The cells do not need to be re-checked. When the pillar is rewritten (ADR-013 ratified), the doctrine will bump to v2.0, and all cells will be re-verified.

---

## Level dependency graph

```
L3 ──┐
     ├──> L2 ──┐
     │         ├──> L1 ──┐
     │         │         ├──> L0 ──> 00-foundations
     │         │         │
     └─────────┴─────────┘
        (read only)
```

A cell at level N may read the **output** of any cell at level `< N`. It may not read the internals. It may not depend on level `> N`.

---

## Status legend

| Symbol | Meaning |
|---|---|
| ✅ ratified | Accepted, stable, downstream cells may depend on it |
| 🟡 draft / proposed | Being written or blocked by an open ADR |
| 🚧 planned | Cell does not exist yet; the row is a placeholder |
| 🔌 external | Not a core cell; a tool or pack outside the repository |
| ⛔ superseded | Replaced by a newer cell, kept for history |

---

## Components

Some components are referenced across cells but do not (yet) have a dedicated cell. This section is the map: for each component, where it is currently defined and where its dedicated cell (if any) will live.

### Infrastructure components

These are the `pcfm-*` crates that provide infrastructure to multiple cells. They are not domain cells; they are services. Their full specification is **fragmented across the cells that use them**, until a dedicated cell is written.

| Component | Currently defined in | Dedicated cell (planned) | Notes |
|---|---|---|---|
| `pcfm-core` | L0.01 §2.2 (RNG), L0.02 §2.1 (event bus), L0.03 §3.1 (IDs), `00-foundations` §4 (fixed-point) | — | The core crate. Used by every other crate. |
| `pcfm-stats` | L0.05 §2.4 (`EventChain`), L0.06 (consumer) | `L3-statistics` | Storage of `MatchEvent`, query engine. |
| `pcfm-metrics` | L0.04 §2.4 (sub-models), L0.06 §2.3 (`MatchStatistics`) | `L3-statistics` | xG, PPDA, progressive actions. Minimal version at L0. |
| `pcfm-data` | L0.07 §2.3 (migration scripts) | `L3-quality` | Loader, validator, migrations. |
| `pcfm-bridge` | `docs/adr/004-panic-strategy-ffi.md`, `docs/adr/005-qt-binding.md` | `L3-ui` | Qt FFI shim. The only crate that knows Qt. |

**Rule:** when a new cell needs `pcfm-core` (or any other infrastructure crate), it does not re-specify the crate. It links to the existing definition. The dedicated cell, when written, will be the single source of truth; until then, the "Currently defined in" column is the map.

### Cross-cell components

These are components that are referenced across cells but are not crates. They are either configuration, data, or external concepts.

| Component | Referenced in | Destination |
|---|---|---|
| `pcfm-hidden` / `GroundTruth` | L0.03, L0.04, L0.07, L0.08, ADR-003 | **L0.10** (documented here) ✅ |
| `pcfm-gen` | ADR-003, L0.07, L0.10 | L3-generator *(planned)* |
| `balance/*.toml` | L0.04, L0.06, L0.08 | L3-quality *(planned)* |
| `registry/*.yaml` | L0.03, L0.05 | L3-registry *(planned)* |
| `locales/*` | L0.06 | L3-assets *(planned)*; translations at `locales/it/` *(planned)* |

---

## ADR status

| ADR | Title | Status | Effect |
|---|---|---|---|
| [001](../docs/adr/001-language-rust.md) | Language: Rust | ✅ accepted | — |
| [002](../docs/adr/002-project-name-and-scope.md) | Project name and scope | ✅ accepted | — |
| [003](../docs/adr/003-plane-a-enforcement.md) | Plane A enforcement | ✅ accepted | Unblocked L0.04, L0.08, L0.10 |
| [004](../docs/adr/004-panic-strategy-ffi.md) | Panic strategy at FFI | ✅ accepted | — |
| [005](../docs/adr/005-qt-binding.md) | Qt binding | 🟡 proposed | Blocks L3-ui |
| [006](../docs/adr/006-license.md) | License | 🟡 proposed | Blocks `LICENSE.md` (currently marked `proposed`) and `Cargo.toml` examples |
| [007](../docs/adr/007-canonical-language.md) | Canonical language | 🟡 proposed | Blocks translations, language policy in `STRUCTURE.md` §6 |
| [008](../docs/adr/008-provider-integration-external.md) | Provider integration is external | ✅ accepted | Reclassified `L2-ingest` as external tool |
| [012](../docs/adr/012-plugins.md) | Modules: The Brick Contract | 🟡 proposed | Defines the module sheet, conflicts, narrative, cause traces, debug UI module view |
| [013](../docs/adr/013-worlds.md) | Worlds: LOD, Manifest, and Local Determinism | 🟡 proposed | Defines LOD, manifest, local determinism; supersedes pillar 1 |

Open ADRs: **005, 006, 007, 012, 013** (5).
