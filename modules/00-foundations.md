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

