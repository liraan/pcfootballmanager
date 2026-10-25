# Repository Structure

**Version:** 1.5
**Status:** active
**Last updated:** 2026-10-25

This document explains how the repository is organised, and the rules that keep it coherent as it grows. Read it before adding a file, revising a module, or proposing a change.

---

## 1. The Documentation Model

The spec is **layered and concentric**, from abstract to concrete:

```
Doctrine  →  Modules (L0–L3)  →  ADRs  →  Code
   │              │                 │
   │              │                 └── one decision, one file
   │              └── one subsystem, one file, at one level
   └── the non-negotiable rules that everything else obeys
```

| Layer | Lives in | Purpose | Changes how often |
|---|---|---|---|
| **Doctrine** | `modules/00-foundations.md` | Why the project exists, the Three Planes, the banned things, the concentric model | Rarely. Only through a doctrine ADR. |
| **Modules** | `modules/L0-*/`, `L1-*/`, `L2-*/`, `L3-*/` | How one subsystem works, at one level | Often. Each has its own revision history. |
| **ADRs** | `docs/adr/NNN-*.md` | Why one architectural decision was made | Never edited after acceptance. Superseded by a new ADR. |
| **Code** | `crates/` | The implementation | Continuously, once it exists. |

**Rule:** a module may *reference* the doctrine but may not *restate* it. If a module contradicts the doctrine, the module is wrong, not the doctrine.

---

## 2. The Concentric Levels

The specification is not a flat list of 25 modules. It is organised in **four concentric levels**. Each level expands the previous one without replacing it. Dependencies flow **inward**: a level N may depend on levels `< N`, never on `> N`, never laterally.

```
┌─────────────────────────────────────────────────────────────┐
│  L3 — META (moddable)                                       │
│  registry, packs, generator, editor, assets, ui, stats,     │
│  quality                                                    │
└──────────────────────────┬──────────────────────────────────┘
                           │ expands
┌──────────────────────────▼──────────────────────────────────┐
│  L2 — CAREER (living world)                                 │
│  tactics, market, finance, youth, scouting, psyche,         │
│  crowd, media, ai                                           │
└──────────────────────────┬──────────────────────────────────┘
                           │ expands
┌──────────────────────────▼──────────────────────────────────┐
│  L1 — SEASON (context)                                      │
│  season, world, roster                                      │
└──────────────────────────┬──────────────────────────────────┘
                           │ expands
┌──────────────────────────▼──────────────────────────────────┐
│  L0 — MATCH (core)                                          │
│  rng, event-bus, entities-minimal, match-engine,            │
│  match-events, match-output, save-minimal, random-events,   │
│  context-modifiers, ground-truth                            │
└──────────────────────────┬──────────────────────────────────┘
                           │ grounded in
┌──────────────────────────▼──────────────────────────────────┐
│  00 — FOUNDATIONS (doctrine)                                │
│  Three Planes, no CA/PA, determinism, concentric structure  │
└─────────────────────────────────────────────────────────────┘
```

**What "the game runs" means at each level:**

- **L0:** you can play a friendly match, save it, reload it, get the same result.
- **L1:** you can play a league season. Standings, calendar, results.
- **L2:** you can play a career. Transfers, budgets, youth intake, tactics, psychology, fans.
- **L3:** the game is moddable. Registry, packs, generator, editor, UI themes.

**Rules:**

- A level N cell may **read** the output of a level `< N` cell.
- A level N cell may **not** read the internals of a level `< N` cell.
- A level N cell may **not** depend on a level `> N` cell.
- The `00-foundations` doctrine applies at all levels.
- The Three Planes apply at all levels.

---

## 3. Directory Layout

### Current state (as of 2026-10-25)

The repository contains **documentation and code**.

**Documentation:**

```
modules/
├── 00-foundations.md
├── _index.md
└── L0-match/         (11 files, 00 → 10)
docs/
├── GLOSSARY.md
├── MIGRATION-01-04.md
├── MODULE_DEVELOPER_GUIDE.md
├── adr/              (11 files, 000 → 013)
├── legal/            (.gitkeep)
└── calibration/      (.gitkeep)
archive/
├── README.md
├── prompts/          (.gitkeep)
├── doctrine/         (.gitkeep)
└── legacy/           (.gitkeep)
```

**Code (103 tests passing):**

```
crates/
├── pcfm-core/        (RNG, event bus, IDs, hash, time)
├── pcfm-model/       (minimal entities, CauseTrace)
├── pcfm-hidden/      (Plane A: GroundTruth, resolve_*)
├── pcfm-stats/       (Plane B: MatchEvent, EventChain)
├── pcfm-match/       (engine, narrative, save)
└── pcfm-debug/       (CLI debug tool)
```

**Root files:**

```
README.md
STRUCTURE.md
CONTRIBUTING.md
LICENSE.md, LICENSE-MIT, LICENSE-APACHE, LICENSE-ASSETS
CHANGELOG.md, CHANGELOG-SIM.md, CHANGELOG-REGISTRY.md
Cargo.toml, Cargo.lock, rust-toolchain.toml
.gitignore, .cargo/config.toml
.github/PULL_REQUEST_TEMPLATE.md, .github/ISSUE_TEMPLATE/
```

### Target state

The full structure, once L1–L3 are written. Items marked *(planned)* do not exist yet.

```
pc-football-manager/
├── README.md
├── STRUCTURE.md
├── CONTRIBUTING.md
├── LICENSE.md
├── LICENSE-MIT
├── LICENSE-APACHE
├── LICENSE-ASSETS
├── CHANGELOG.md
├── CHANGELOG-SIM.md
├── CHANGELOG-REGISTRY.md
├── modules/
│   ├── 00-foundations.md
│   ├── _index.md
│   ├── L0-match/
│   ├── L1-season/             # *(planned)*
│   ├── L1-world/              # *(planned)*
│   ├── L1-roster/             # *(planned)*
│   ├── L2-*/                  # *(planned)* 9 cells
│   └── L3-*/                  # *(planned)* 8 cells
├── archive/                   # historical Prompt Masters, when recovered
├── docs/
│   ├── adr/
│   ├── legal/
│   ├── calibration/
│   ├── GLOSSARY.md
│   ├── MIGRATION-01-04.md
│   └── MODULE_DEVELOPER_GUIDE.md
├── registry/                  # empty until code exists
├── crates/                    # code (see current state)
├── mods/                      # empty until user packs exist
└── data/                      # examples only, never real licensed data
```

**File naming:** `NN-kebab-case-name.md`, two-digit prefix, no Italian, no version in the filename. `04-match-engine.md`, not `Modulo 04 — Motore Partita (rev 1.1).md`. The filename is stable; the revision lives in the frontmatter.

**Crate naming:** the crates in `crates/` use the `pcfm-` prefix (e.g. `pcfm-core`, `pcfm-hidden`, `pcfm-match`, `pcfm-stats`, `pcfm-model`, `pcfm-packs`, `pcfm-assets`, `pcfm-debug`, `pcfm-pack`). The historical prefix `fmr-` (from the original "FMR project" name) is deprecated and must not be used in new files. Rust module paths follow the same convention with an underscore: `pcfm_core::rng`, `pcfm_match::simulate`, etc.

**Cells vs sub-cells:** A **cell** is a directory (`L0-match`, `L2-psyche`). A **sub-cell** is a file inside a cell (`04-match-engine.md`). A sub-sub-cell is a section (`##`) inside a file. There is no third level of directories. If a sub-cell needs its own children, it becomes a new cell at the same level, not a deeper directory.

The field `module_id` in the frontmatter is the stable identifier (`L0.04`). The field `cell` is the directory name (`L0-match`). In prose, prefer "sub-cell L0.04" over "module L0.04" when talking about a file, and "cell L0-match" when talking about a directory.

### `/archive` — the history

Every superseded document goes here. Nothing is deleted. `Prompt_Master_v5.md`, `_v6.md`, `_v7.md` will live in `archive/prompts/` once recovered. When `00-foundations.md` is revised, the previous version is archived in `archive/doctrine/`. The four legacy Italian modules (`01-architecture.md`, `02-registry.md`, `03-pack-system.md`, `04-data-model.md`) will be moved to `archive/legacy/` after migration (see `docs/MIGRATION-01-04.md`).

**Rule:** the archive is read-only. Do not edit archived files. They exist for traceability, not for maintenance.

### `/docs/adr` — the decision log

One file per architectural decision. See §5.

### `/docs/legal` — data source compliance

Licensing terms, provider constraints, what we can and cannot do with third-party data. *(planned — the directory exists but is empty)*

### `/docs/calibration` — model calibration dashboards

When the project ships, this folder holds the dashboards that compare internal models (xG, IR, projections) against real data. *(planned — the directory exists but is empty)*

### `/crates`, `/registry`, `/mods`, `/data`

`crates/` contains code (6 crates as of 2026-10-25). `registry/`, `mods/`, `data/` are empty until code or user packs exist. **Rule:** `data/` contains *examples only*. Never real licensed data. If you need to test with real data, keep it outside the repo.

---

## 4. Cell Frontmatter

Every cell (and every sub-cell) starts with:

```yaml
---
title: "L0.04 — Match Engine"
level: 0
cell: "L0-match"
module_id: "L0.04"
version: "1.2"
status: "ratified"          # draft | review | ratified | superseded
last_updated: "2026-10-25"
supersedes: null             # or "previous-version-id"
depends_on:
  - "00-foundations"
  - "L0.01"
  - "L0.02"
  - "L0.03"
  - "L0.10"
referenced_by:
  - "L0.05"
  - "L0.06"
  - "L0.08"
  - "L0.09"
  - "L1-season"
conforms_to: "00-foundations (v1.1)"
---
```

**Field reference:**

| Field | Meaning |
|---|---|
| `title` | Human-readable title, includes the module ID |
| `level` | 0, 1, 2, or 3 (see §2) |
| `cell` | The directory name (`L0-match`, `L2-psyche`, ...) |
| `module_id` | `L{level}.{NN}` for sub-cells |
| `version` | Semver-ish: bump minor for clarifications, major for invariant changes |
| `status` | `draft` (being written) / `review` (ready for comments) / `ratified` (stable) / `superseded` (replaced) |
| `last_updated` | ISO date |
| `supersedes` | The ID of the version this replaces, or `null` |
| `depends_on` | Module IDs this cell reads from |
| `referenced_by` | Module IDs that read from this cell |
| `conforms_to` | The doctrine version this cell obeys |

**Version policy:** bump the minor version for clarifications, the major version for changes that affect downstream modules. A major bump requires an ADR if it changes an invariant.

---

## 5. The ADR Process

An Architecture Decision Record is a short document that captures **one decision**, its context, the options considered, and the consequences. It is **immutable once accepted**: to change a decision, write a new ADR that supersedes the old one.

### Format

```markdown
# ADR-NNN — Title

- **Status:** proposed | accepted | rejected | superseded
- **Date:** YYYY-MM-DD
- **Supersedes:** ADR-XXX (if applicable)
- **Superseded by:** ADR-YYY (if applicable)
- **Blocks:** modules or code areas that cannot proceed until accepted

## Context
What is the problem? Why does it need a decision now?

## Options considered

### Option A — Name
- **Pro:** ...
- **Con:** ...

### Option B — Name
- **Pro:** ...
- **Con:** ...

## Decision
Which option was chosen, and why.

## Consequences
Easier / harder / locked in.

## Implementation notes
1. ...

## References
- ...
```

### Numbering

- `000-template.md` — the template
- `001` onwards — actual decisions
- Numbers are never reused. An ADR is never deleted.

### Open ADRs block code

If an ADR is `proposed` and touches an area, **no code may be written in that area until the ADR is accepted**. This is the rule that prevents the "we'll decide later" trap.

### Current ADR status

| ADR | Title | Status | Blocks |
|---|---|---|---|
| 001 | Language: Rust | ✅ accepted | — |
| 002 | Project name and scope | ✅ accepted | — |
| 003 | Plane A enforcement | ✅ accepted | — (unblocked L0.04, L0.08, L0.10) |
| 004 | Panic strategy at FFI | ✅ accepted | — |
| 005 | Qt binding | 🟡 proposed | `L3-ui`, `pcfm-bridge` |
| 006 | License | 🟡 proposed | `LICENSE.md`, every `Cargo.toml` example |
| 007 | Canonical language | 🟡 proposed | translations, language policy |
| 008 | Provider integration is external | ✅ accepted | — (reclassified `L2-ingest` as external tool) |
| 012 | Modules: The Brick Contract | 🟡 proposed | The module system (step 11+). Ratified when the framework implements it. |
| 013 | Worlds: LOD, Manifest, and Local Determinism | 🟡 proposed | The world system (step 11+). Supersedes pillar 1 of `00-foundations`. Ratified when the framework implements it. |

**Open ADRs:** 005, 006, 007, 012, 013. **Accepted:** 001, 002, 003, 004, 008.

**ADR-012 and ADR-013 are two halves of one decision.** Read them together, in order: [`012-013-reading-guide.md`](adr/012-013-reading-guide.md). ADR-013 depends on ADR-012.

---

## 6. Language Policy

**Canonical language for technical documentation: English.**

Rationale:

- The README is already English
- The project positions itself as an international community alternative
- Commit messages, code identifiers, and ADRs are English by convention
- A single language avoids the "which version is authoritative?" problem

**Translations:** Italian is the first planned translation, in `locales/it/`. Translations are maintained by volunteers, are marked with the English version they correspond to, and are **never** the source of truth. *(planned — the `locales/` directory does not exist yet)*

**This policy is itself a decision** and is ratified in **ADR-007**.

---

## 7. How to Add a New Cell

1. Pick the level (L0, L1, L2, L3) the new cell belongs to.
2. Check that the cell does not violate the dependency rules in §2.
3. Create the directory `modules/L{N}-{kebab-name}/`.
4. Add `00-overview.md` with the frontmatter (see §4). The overview is the "border" of the cell: it declares contracts, dependencies, DoD, risks.
5. Add the sub-cells as files inside the directory.
6. Add a row to `modules/_index.md`.
7. Open a PR with **one cell only**.
8. After merge, update `referenced_by` in the cells that now reference it.

## 8. How to Revise an Existing Cell

1. Bump `version` in the frontmatter. Minor for clarifications, major for invariant changes.
2. If major, write an ADR first (or link to an existing one).
3. Archive the previous version in `archive/` if the change is structural.
4. Update `last_updated`.
5. Open a PR with **one cell only**.
6. If the revision changes a dependency, update `referenced_by` in downstream cells.

## 9. What Packs Can and Cannot Extend

The distinction is between **content** and **structure**.

| Packs **can** extend | Packs **cannot** extend |
|---|---|
| The **registry** (roles, formations, instructions, attributes, metrics, index models) — see `L3-registry` *(planned)* | The **cells** of the specification (L0–L3) |
| **Assets** (logos, kits, stadium images, audio) — see `L3-assets` *(planned)* | The **levels** of the concentric structure (L0, L1, L2, L3) |
| **Data** (squads, players, competitions, nations) — as provided or generated data | The **doctrine** (`00-foundations.md`) |
| **Configuration** (`balance/*.toml`, `theme/*.toml`) | The **frontmatter schema** of a cell |
| **Scripts** in the sandboxed DSL (see `L3-packs` *(planned)*) | The **dependency rules** between levels |

**Rule:** a pack that needs a new cell is a pack that is trying to be a different game. A new cell is an **architectural decision**, not a content extension. It requires:

1. An ADR that justifies the new cell (why does it exist? why is it not a sub-cell of an existing cell? why is it not a pack-provided registry entry?).
2. A PR to the core repository that adds the cell, with frontmatter, DoD, and `_index.md` update.
3. A review by a maintainer.

This is not a restriction on moddability. It is a boundary between "modding the content" (welcome) and "modding the architecture" (welcome, but through a PR, not through a pack). The two are different activities, with different review needs.

**Examples:**

- A pack that adds a new role: **allowed** (registry extension).
- A pack that adds a new formation: **allowed** (registry extension).
- A pack that adds a new visual skin: **allowed** (asset extension).
- A pack that adds a new type of match event: **requires a core PR** (the `EventType` enum is in L0.05, and its extension is an architectural change).
- A pack that adds a new psychological axis: **requires a core PR** (the axis is in `L2-psyche`, and adding it changes the psyche model).

The last two examples are not "forbidden to modders". They are "not doable through a pack alone". The modder writes the PR, the core team reviews it, and if accepted, the new event type or axis becomes part of the core.

**Note on ADR-012 and ADR-013:** once ratified, the module system will allow a further form of extension, the **module**, which is a Rust crate that implements `MatchModule` and ships with a `sheet.toml`. See ADR-012 for the contract. The boundary above (content vs structure) is **unchanged**: a module is not a pack, and it requires a PR. A pack is a bundle of content, a module is a piece of code. Both are welcome, but they are different things.

## 10. What Does NOT Go in This Repo

- Real licensed data (Wyscout, Opta, StatsBomb, FBref scrapes)
- Credentials, API keys, tokens
- Executables, binaries, compiled artifacts
- Personal contact information of contributors
- Anything that would violate a provider's terms of service

---

## 11. The Honesty Rule

This project was drafted with AI assistance and is honest about it. That honesty applies to the documentation too:

- If a cell is a stub, say `status: draft`.
- If a decision is open, say "open" and link to the ADR.
- If a date slipped, update the date and say so in `CHANGELOG.md`.
- If you don't know something, write "unknown" rather than inventing a plausible number.

A document that lies about its own state is worse than no document.
