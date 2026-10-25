---
title: "Modules Index"
version: "2.3"
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

1. [`00-foundations.md`](00-foundations.md) — the doctrine *(v1.1)*
2. The level that matches your task (L0 → L3)
3. [`../STRUCTURE.md`](../STRUCTURE.md) — the rules of the repo
4. [`../docs/adr/`](../docs/adr/) — the decisions

---

## Current state (as of 2026-10-25)

- ✅ **Doctrine**: ratified (v1.1, with §9 concentric structure)
- ✅ **L0 — Match**: fully specified, **all 11 cells ratified** (ADR-003 accepted)
- 🚧 **L1 — Season**: planned, not started (12 cells)
- 🚧 **L2 — Career**: planned, not started (9 cells)
- 🚧 **L3 — Meta**: planned, not started (8 cells)

Cells marked *(planned)* below do not have a file yet. Their links are intentionally not clickable.

---

## Doctrine

| File | Status |
|---|---|
| [`00-foundations.md`](00-foundations.md) | ✅ ratified (v1.1) |

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

**L0 is done when:** a friendly match is playable, savable, reloadable, deterministic, and every event has a cause trace. The **specification** is complete; the **code** can now be written (ADR-003 accepted).

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

## How to read a cell

Every cell has:

- **Frontmatter** — level, cell, module_id, version, status, dependencies, doctrine compliance (see `STRUCTURE.md` §4)
- **§0 Purpose** — what the cell covers, what it does not
- **§1 Context** — what it depends on, what depends on it
- **§2 Specification** — the actual content
- **§3 Data model** — entities, if any
- **§4 Edge cases**
- **§5 Definition of Done**
- **§6 Risks and Mitigations**
- **§7 References**

If a cell lacks any of these, it is a bug. Open an issue.

---

## Doctrine compliance

Every cell declares `conforms_to: 00-foundations (vX.Y)`. When `00-foundations` is revised, every cell's `conforms_to` is checked. A cell that has not been re-checked is marked `status: review` until it is.

The current doctrine version is **v1.1**. All ratified L0 cells conform to v1.1.

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
| 🟡 draft | Being written or blocked by an open ADR |
| 🚧 planned | Cell does not exist yet; the row is a placeholder |
| ⛔ superseded | Replaced by a newer cell, kept for history |

---

## Cross-cell components

These components are referenced across L0 cells but do not (yet) have a dedicated cell. The destination is indicated.

| Component | Referenced in | Destination |
|---|---|---|
| `fmr-hidden` / `GroundTruth` | L0.03, L0.04, L0.08, ADR-003 | **L0.10** (documented here) ✅ |
| `fmr-gen` | ADR-003, L0.07, L0.10 | L3-generator *(planned)* |
| `fmr-stats` / `EventChain` | L0.05, L0.06 | `EventChain` documented in L0.05 ✅; storage at L3-statistics *(planned)* |
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
| [006](../docs/adr/006-license.md) | License | 🟡 proposed | Blocks `LICENSE.md` and `Cargo.toml` examples |
| [007](../docs/adr/007-canonical-language.md) | Canonical language | 🟡 proposed | Blocks translations |

Open ADRs: **005, 006, 007** (3).
