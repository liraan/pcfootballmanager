---
title: "Module 00 — Foundations"
version: "1.1"
status: "ratified"
last_updated: "2026-10-25"
supersedes: "Prompt Master v7 (archived)"
depends_on: []
referenced_by: ["all modules"]
conforms_to: "self"
---

# Module 00 — Foundations

This is the **canonical doctrine** of PC Football Manager. Every other module is downstream of it. If a module contradicts this file, the module is wrong.

The full historical version — `Prompt Master v7` — is archived in [`archive/Prompt_Master_v7.md`](../archive/Prompt_Master_v7.md). This module distils it. When in doubt, this module wins.

---

## 1. Role and Mandate

The project is developed as if by a **lead game designer + tech lead + gameplay programmer** working as a senior team. Every specification must be:

- **Executable** — not philosophy, not generic advice
- **Typed** — schemas, interfaces, contracts
- **Traced** — dependencies, inputs, outputs, parameters, test criteria
- **Trade-off aware** — when there are two options, both are presented with pro/contro

If a requirement is ambiguous, propose two options. Do not silently pick one.

---

## 2. Design Pillars

Ten non-negotiable principles. Every design decision is tested against them.

1. **Traceable determinism.** Same seed + same inputs = same result. Never compromised. See §4.

2. **Clean data / logic / UI separation.** No game rule lives in the interface. The UI is view-only.

3. **First-class moddability.** Local packs, in-game UI, validation, rollback. See Module 03.

4. **Readability before graphics.** Adjustable information density, tooltips everywhere, zero hardcoded colours.

5. **Radical transparency.** Every event has an explainable cause. "Why did I lose?", "Why isn't this kid growing?", "Why did the valuation change?" — all answerable.

6. **The academy is a game inside the game.** Not a secondary screen. Economy, human dynamics, pathways.

7. **Information is scarce and costly.** Scouting has limits, bias, cost. It is not omniscience.

8. **Living modules.** Expand, contract, version, replace — without breaking the others. Contracts are stable; implementations are not.

9. **No CA/PA.** Eliminated as scalars. Replaced by windowed observed indexes + projection profiles with confidence bands. See §3.

10. **Plane B ≠ Plane A.** Statistics are observations, not truth. The game teaches the player to tell them apart. See §3.

---

## 3. The Three Planes (Core Doctrine)

The single most important architectural rule. Violating it introduces systemic bugs that are not detectable by normal testing.

### Plane A — Ground Truth (never exposed)

**What it is:** the actual state of the world, never visible to the player.

**Contains:**
- Latent attributes per role (`passing_long`, `finishing_close`, `heading_accuracy`, ...)
- Physiological state (internal fatigue, injury risk)
- Latent psychological state (the six axes, unexposed raw)
- Internal random seed (never exposed)

**Who reads it:** only `fmr-match` (the match engine) and the world generator.

**Rule:** no external module may read Plane A. Ever. Not even for debugging.

### Plane B — Observation (generated from raw events)

**What it is:** events and statistics computed exclusively from observable outcomes.

**Contains:**
- `MatchEvent` (every action: shot, pass, duel, tackle)
- Aggregated statistics (appearances, goals, assists, xG, PPDA, completion rate)
- Ratings (computed **from raw events**, never from attributes)
- Temporal trends (last 10 matches, current season)

**Who reads it:** statistical systems, scouting, market (partially), UI.

**Critical rule:** ratings MUST be computed from raw events only. Never from latent attributes. Otherwise you have divergent feedback loops that corrupt everything.

### Plane C — Estimation (what the player sees)

**What it is:** valuations, projections, and indexes derived from statistical models.

**Contains:**
- Windowed Performance Index (IR) per context
- `ProjectionProfile` (forecasts with confidence intervals)
- "Visible" attributes (derived from statistics, updated periodically)
- Scouting reports (with declared bias and limits)

**Who reads it:** the player, the market AI, decision systems.

**Rule:** Plane C never influences Plane A. The player sees only estimates, not truth.

### Flow Diagram

┌─────────────────┐ events ┌─────────────────┐
│ PLANE A │ ─────────────> │ PLANE B │
│ (Ground Truth) │ │ (Observation) │
│ - Latents │ │ - MatchEvents │
│ - States │ │ - Stats │
└─────────────────┘ └────────┬────────┘
│
│ analysis
▼
┌─────────────────┐
│ PLANE C │
│ (Estimation) │
│ - Index IR │
│ - Projections │
│ - Reports │
└─────────────────┘
│
│ visible
▼
┌─────────────────┐
│ UI/UX │
└─────────────────┘


### Forbidden feedback loops

- ❌ Plane C → Plane A (estimates do not change reality)
- ❌ Plane A → UI directly (no truth leakage)
- ❌ Ratings using Plane A (circularity)
- ❌ Statistical models reading latents (they must use observed only)

**Enforcement:** the technical mechanism is decided in **ADR-003** (currently `proposed`). Until that ADR is accepted, no code that touches `fmr-hidden` or `fmr-match` may be written.

---

## 4. Determinism

Same seed → same world, same season, same match. Always.

Operational rules:

- **Generational IDs**, never hashes or pointers
- **Named RNG streams**: `ChaCha20Rng` derived from `(world_seed, stream_name)`, so adding a new consumer never shifts existing results
- **No `HashMap`/`HashSet` in simulation paths**: `BTreeMap`, `BTreeSet`, or explicitly sorted `Vec`
- **Portable transcendentals**: `libm` pinned, or fixed-point for the critical path
- **Deterministic parallelism**: `rayon` with fixed partitioning and ordered reduction
- **Golden tests**: 10 seasons from a known seed → state hash comparison
- **`SIM_VERSION`**: bumped whenever simulation behaviour changes, with the five-step procedure in Module 01

Full details: [`L0-match/01-rng.md`](L0-match/01-rng.md).

---

## 5. Banned Things

These are not preferences. They are prohibitions.

- **No CA/PA.** No `current_ability`, `potential_ability`, or equivalent scalar. Ever.
- **No truth leakage.** No module outside `fmr-match` and the generator reads Plane A.
- **No ratings from latents.** Ratings come from events.
- **No Plane C → Plane A feedback.**
- **No hardcoded rules, names, colours, or balance values.** Everything lives in files.
- **No invented real data** (squads, prices, regulations) presented as certain.
- **No bundling, redistributing, or scraping commercial data.**
- **No unlicensed assets.**
- **No compromising determinism for convenience.**
- **No deleting entities from the registry.** Deprecate, never delete.
- **No save-breaking changes without a migration script.**
- **No circular dependencies between modules.**
- **No statistics overwrites.** Append-only.
- **No hidden confidence intervals.** If an estimate is uncertain, the UI says so.

---

## 6. Product Goal and Target

**Single-player first.** Hotseat and asynchronous multiplayer are separate modules (future).

**Depth of FM24 + soul of PC Calcio:**
- Dense, readable interface
- Textual radio commentary
- Powerful editor
- Extra-managerial management
- Weekly sports show
- Real sackings
- Complete statistics, never overwritten
- Psyche as a game system, not decoration
- No CA/PA: everything observable, derived, windowed

**Target:**
- Platforms: Windows, Linux, macOS
- Minimum resolution: 1280×720
- Performance: 60 fps UI, full season < 60 s on mid-range hardware
- Storage: < 5 GB base install

---

## 7. Non-Goals

To keep scope honest, these are **not** in v1.0:

- 3D match engine (future crate, behind the same `MatchEngine` trait)
- Online multiplayer (separate module)
- Mobile ports
- Real licensed datasets (adapters only, user-supplied)
- Full 6-axis psychology (3 axes in v1.0, the other 3 in a pack)
- Full 25-module completeness (v1.0 ships modules 00–13 stable)

---

## 8. References

- **Historical:** [`archive/Prompt_Master_v7.md`](../archive/Prompt_Master_v7.md) — the full source doctrine
- **Architecture:** [`L0-match/04-match-engine.md`](L0-match/04-match-engine.md) — determinism enforcement
- **Data model:** [`L0-match/03-entities-minimal.md`](L0-match/03-entities-minimal.md) — no CA/PA
- **Decisions:** [`docs/adr/`](../docs/adr/) — every open question, every resolved one
- **Index of all modules:** [`_index.md`](_index.md)

---

## 9. Concentric Structure (Nest Model)

The specification is organised as **concentric levels**, not a flat list of modules. Each level expands the previous one without replacing it. Dependencies flow **inward**: a level N may depend on levels `< N`, never on `> N`, never laterally.

<!--
  MAINTAINER NOTE: this section is the formal answer to "how is the repo organised?".
  If a new module is added, it MUST declare its level in the frontmatter. If it
  doesn't fit an existing level, that's a design discussion, not a new level.
-->

L0 — MATCH (core) Two teams play one match. Deterministic, savable.
L1 — SEASON A league, a calendar, a world.
L2 — CAREER Market, finance, youth, scouting, psyche, crowd, media, AI.
L3 — META Registry, packs, generator, editor, assets, UI, statistics, quality.


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

**The full index is in [`modules/_index.md`](_index.md).**
