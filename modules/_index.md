---
title: "Modules Index"
version: "2.0"
status: "active"
last_updated: "2026-10-25"
---

# Modules Index

The specification is **concentric**, not flat. Read [`00-foundations.md`](00-foundations.md) first, then the level that matches your task.

<!--
  MAINTAINER NOTE: this file is a navigator, not a spec. Keep it short.
  When adding a cell: add a row, bump the level's "is done when" if needed.
  When a cell moves from stub to ratified: update the status column.
-->
L0 — MATCH Two teams play one match
L1 — SEASON A league, a calendar, a world
L2 — CAREER Market, finance, youth, scouting, psyche, crowd, media, AI
L3 — META Registry, packs, generator, editor, assets, UI, statistics, quality


Dependencies flow **inward**: L2 may depend on L1 and L0, never on L3. L0 depends only on `00-foundations`.

---

## Doctrine

| File | Status |
|---|---|
| [`00-foundations.md`](00-foundations.md) | ✅ ratified |

---

## L0 — Match (core)

| ID | File | Status | Depends on |
|---|---|---|---|
| L0.00 | [`L0-match/00-overview.md`](L0-match/00-overview.md) | ✅ ratified | 00 |
| L0.01 | [`L0-match/01-rng.md`](L0-match/01-rng.md) | ✅ ratified | 00 |
| L0.02 | [`L0-match/02-event-bus.md`](L0-match/02-event-bus.md) | ✅ ratified | 00 |
| L0.03 | [`L0-match/03-entities-minimal.md`](L0-match/03-entities-minimal.md) | ✅ ratified | 00, L0.01 |
| L0.04 | [`L0-match/04-match-engine.md`](L0-match/04-match-engine.md) | 🟡 draft (ADR-003) | 00, L0.01, L0.02, L0.03 |
| L0.05 | [`L0-match/05-match-events.md`](L0-match/05-match-events.md) | ✅ ratified | 00, L0.01, L0.03 |
| L0.06 | [`L0-match/06-match-output.md`](L0-match/06-match-output.md) | ✅ ratified | 00, L0.05 |
| L0.07 | [`L0-match/07-save-minimal.md`](L0-match/07-save-minimal.md) | ✅ ratified | 00, L0.03, L0.05, L0.06 |
| L0.08 | [`L0-match/08-random-events.md`](L0-match/08-random-events.md) | 🟡 draft (ADR-003) | 00, L0.01, L0.03, L0.04, L0.05 |
| L0.09 | [`L0-match/09-context-modifiers.md`](L0-match/09-context-modifiers.md) | ✅ ratified | 00, L0.03 |

**L0 is done when:** a friendly match is playable, savable, reloadable, deterministic, and every event has a cause trace.

---

## L1 — Season (context)

| ID | File | Status | Depends on |
|---|---|---|---|
| L1.00 | `L1-season/00-overview.md` | 🚧 stub | L0 |
| L1.01 | `L1-season/01-competitions.md` | 🚧 stub | L0, L1.00 |
| L1.02 | `L1-season/02-calendar.md` | 🚧 stub | L0, L1.00, L1.01 |
| L1.03 | `L1-season/03-standings.md` | 🚧 stub | L0, L1.00, L1.01, L1.02 |
| L1.04 | `L1-world/00-overview.md` | 🚧 stub | L0 |
| L1.05 | `L1-world/01-nation.md` | 🚧 stub | L0, L1.04 |
| L1.06 | `L1-world/02-city.md` | 🚧 stub | L0, L1.04 |
| L1.07 | `L1-world/03-stadium.md` | 🚧 stub | L0, L1.04 |
| L1.08 | `L1-roster/00-overview.md` | 🚧 stub | L0 |
| L1.09 | `L1-roster/01-player-attributes.md` | 🚧 stub | L0, L1.08 |
| L1.10 | `L1-roster/02-staff.md` | 🚧 stub | L0, L1.08, L1.09 |
| L1.11 | `L1-roster/03-contracts.md` | 🚧 stub | L0, L1.08, L1.09 |

**L1 is done when:** a league season is playable, standings are visible, the world exists (nations, cities, stadiums), rosters are complete.

---

## L2 — Career (living world)

| ID | Cell | Status |
|---|---|---|
| L2.01 | `L2-tactics/` | 🚧 stub |
| L2.02 | `L2-market/` | 🚧 stub |
| L2.03 | `L2-finance/` | 🚧 stub |
| L2.04 | `L2-youth/` | 🚧 stub |
| L2.05 | `L2-scouting/` | 🚧 stub |
| L2.06 | `L2-psyche/` | 🚧 stub |
| L2.07 | `L2-crowd/` | 🚧 stub |
| L2.08 | `L2-media/` | 🚧 stub |
| L2.09 | `L2-ai/` | 🚧 stub |

**L2 is done when:** a full career is playable — transfers, budgets, youth intake, tactics, psychology, fans, media, AI.

---

## L3 — Meta (moddable)

| ID | Cell | Status |
|---|---|---|
| L3.01 | `L3-registry/` | 🚧 stub |
| L3.02 | `L3-packs/` | 🚧 stub |
| L3.03 | `L3-generator/` | 🚧 stub |
| L3.04 | `L3-editor/` | 🚧 stub |
| L3.05 | `L3-assets/` | 🚧 stub |
| L3.06 | `L3-ui/` | 🚧 stub |
| L3.07 | `L3-statistics/` | 🚧 stub |
| L3.08 | `L3-quality/` | 🚧 stub |

**L3 is done when:** the game is moddable — packs install, registry extends, generator produces worlds, editor edits, UI themes, statistics expose, quality gates run.

---

## How to read a cell

Every cell has:

- **Frontmatter** — level, cell, module_id, version, status, dependencies, doctrine compliance
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

---

## 
Level dependency graph

L3 ──┐
├──> L2 ──┐
│ ├──> L1 ──┐
│ │ ├──> L0 ──> 00-foundations
│ │ │
└─────────┴─────────┘
(read only)


A cell at level N may read the **output** of any cell at level `< N`. It may not read the internals. It may not depend on level `> N`.
