---
title: "L0 — Match (overview)"
level: 0
cell: "L0-match"
module_id: "L0.00"
version: "1.0"
status: "ratified"
last_updated: "2026-10-25"
depends_on: ["00-foundations"]
referenced_by: ["L1-season", "L1-roster", "L1-world", "L2-tactics", "L2-psyche"]
conforms_to: "00-foundations (v1.0)"
---

# L0 — Match (overview)

This is the **core** of the whole project. Everything else expands it. If L0 does not work, nothing else matters.

L0 answers one question: **can two teams play a football match, deterministically, and produce a result that a human can read, understand, and trust?**

It does **not** answer: who owns the club, who is the manager, what is the league table, how much a player is worth, what the fans think. Those are L1 and L2.

---

## 1. What L0 is

A single match between two squads, simulated tick by tick, producing:

- A **result** (score, events, ratings)
- An **event log** (append-only, `MatchEvent`)
- A **statistics bundle** (xG, PPDA, possession, heatmap)
- A **narrative log** (radio commentary, localisable)

L0 does **not** require:

- A season, a competition, a calendar
- A market, contracts, money
- A world, cities, nations, stadiums beyond a minimal pitch
- A manager, staff, board
- A psychology module beyond minimal morale/confidence
- A crowd beyond a single `hostility` scalar

L0 **does** require:

- A deterministic RNG
- A deterministic event bus
- Two teams with 11 players each
- A pitch
- A referee
- The `MatchEngine` trait and a text implementation
- A minimal save format

---

## 2. What "the game runs" means at L0

At the end of L0, you can:

1. Create two squads of 11 players each, with latent attributes and minimal physical state
2. Start a match with a seed
3. Watch the match unfold (textual output, event by event)
4. Get a final score, per-player ratings, and a statistics bundle
5. Save the match to disk
6. Reload the match and get **exactly the same result**
7. Read why each event happened (cause trace)

If any of those seven fails, L0 is not done.

---

## 3. Cells of L0

| ID | Cell | Purpose |
|---|---|---|
| L0.01 | [`01-rng.md`](01-rng.md) | Deterministic RNG streams |
| L0.02 | [`02-event-bus.md`](02-event-bus.md) | Deterministic event dispatch |
| L0.03 | [`03-entities-minimal.md`](03-entities-minimal.md) | Minimal entities: Player, Squad, Pitch, Referee |
| L0.04 | [`04-match-engine.md`](04-match-engine.md) | The `MatchEngine` trait and text implementation |
| L0.05 | [`05-match-events.md`](05-match-events.md) | `MatchEvent`, append-only, the observation layer |
| L0.06 | [`06-match-output.md`](06-match-output.md) | Result, ratings, statistics bundle, narrative |
| L0.07 | [`07-save-minimal.md`](07-save-minimal.md) | Save/load a single match |
| L0.08 | [`08-random-events.md`](08-random-events.md) | Discrete in-match events (injuries, cards, brawls) |
| L0.09 | [`09-context-modifiers.md`](09-context-modifiers.md) | How external state enters the engine |

---

## 4. Dependency rules

Within L0:

- `01-rng` and `02-event-bus` depend only on `00-foundations`
- `03-entities-minimal` depends on `00`, `01`
- `04-match-engine` depends on `00`, `01`, `02`, `03`
- `05-match-events` depends on `00`, `01`, `03`
- `06-match-output` depends on `00`, `05`
- `07-save-minimal` depends on `00`, `03`, `05`, `06`
- `08-random-events` depends on `00`, `01`, `03`, `04`, `05`
- `09-context-modifiers` depends on `00`, `03`

**No cell in L0 depends on L1, L2, or L3.** Ever.

The `04-match-engine` reads `GroundTruth` from `fmr-hidden` (Plane A) via the API defined in **ADR-003**. Until ADR-003 is accepted, `04-match-engine` and `08-random-events` may be specified but not implemented.

---

## 5. The Three Planes at L0

L0 is where the Three Planes are born.

| Plane | Lives in | Read by | Written by |
|---|---|---|---|
| A — Ground Truth | `fmr-hidden` | `04-match-engine`, `08-random-events` | `03-entities-minimal` (creation), generator (L3) |
| B — Observation | `fmr-stats` | `06-match-output`, L2 modules | `04-match-engine` via `05-match-events` |
| C — Estimation | not in L0 | — | L2 (index, projection) |

**Critical rule:** `06-match-output` computes ratings from `MatchEvent` only. It does not read `fmr-hidden`. This is tested in L0's DoD.

---

## 6. Determinism at L0

Every L0 cell that consumes randomness uses a **named stream** derived from `(world_seed, cell_id, match_id)`. No cell shares a stream with another. Adding a new consumer never shifts existing results.

Golden test: `same_seed → same MatchResult` bit-for-bit, across 1,000 randomly generated matches.

---

## 7. Definition of Done (L0 as a whole)

- [ ] All 10 cells specified and ratified
- [ ] ADR-003 accepted (Plane A enforcement)
- [ ] Two squads can play a match from a seed
- [ ] The match produces `MatchEvent`s, ratings, statistics, narrative
- [ ] The match is saveable and reloadable, same seed → same result
- [ ] Each event has a consultable cause trace
- [ ] No ratings computed from latents (test passes)
- [ ] No module outside `fmr-hidden` and `04-match-engine` reads Plane A (test passes)
- [ ] Performance: 90 min simulated in < 1 s on mid-range hardware
- [ ] Golden test: 1,000 matches, same seed → same hash

---

## 8. Risks and Mitigations

| Risk | Severity | Mitigation |
|---|---|---|
| Match engine becomes a monolith | High | Cell boundaries enforced; L0.04 has a hard API; random events and context modifiers are separate cells |
| Ratings leak Plane A | Critical | Type system + test of violation |
| Determinism drifts | Critical | Named streams + golden test + `SIM_VERSION` |
| Random events feel "cheap" | High | Every event has a cause trace; distribution tested |
| Performance insufficient | Medium | Profiling, tick budget, benchmarks in CI |

---

## 9. References

- [`00-foundations.md`](../00-foundations.md) — doctrine
- [`docs/adr/003-plane-a-enforcement.md`](../../docs/adr/003-plane-a-enforcement.md) — blocking
- [`archive/Prompt_Master_v7.md`](../../archive/Prompt_Master_v7.md) §12 — match engine original
