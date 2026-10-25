---
title: "L0.04 — Match Engine"
level: 0
cell: "L0-match"
module_id: "L0.04"
version: "1.0"
status: "draft"
last_updated: "2026-10-25"
supersedes: null
depends_on: ["00-foundations", "L0.01", "L0.02", "L0.03"]
referenced_by: ["L0.05", "L0.06", "L0.08", "L0.09", "L1-season"]
conforms_to: "00-foundations (v1.1)"
---

<!--
  MAINTAINER NOTE: this file is intentionally `draft` until ADR-003 is accepted.
  Do not write code against this spec yet. The spec is complete; the code waits.
  If you're tempted to "just start with a stub", stop. The interface is what
  ADR-003 is deciding. A stub written before the decision will need to be
  rewritten. Wait.

  Once ADR-003 is accepted, change `status: draft` to `status: ratified` and
  begin implementation.
-->

# L0.04 — Match Engine

## 0. Purpose

Define the `MatchEngine` trait and specify its text implementation. The match engine is the **orchestrator** of L0: it drives the tick loop, calls `GroundTruth::resolve_*` for every resolution, publishes events, and produces a `MatchResult`.

**Status: `draft`.** Implementation is blocked by [`../../docs/adr/003-plane-a-enforcement.md`](../../docs/adr/003-plane-a-enforcement.md). The specification below is complete; the code waits.

## 1. Context and dependencies

- Depends on `00-foundations`, `L0.01` (RNG), `L0.02` (event bus), `L0.03` (entities)
- Reads `GroundTruth` from `fmr-hidden` via `resolve_*` (ADR-003, option B)
- Publishes events to `L0.02-event-bus`
- Produces `MatchEvent` (Plane B) consumed by `L0.05` and `L0.06`
- Reads `ContextModifiers` from `L0.09` for external state
- Triggers `L0.08-random-events` when conditions are met
- **Never** reads latent values directly. It has no access to latent fields; it only receives `resolve_*` outcomes.

## 2. Specification

### 2.1 The trait

```rust
pub trait MatchEngine: Send + Sync {
    fn simulate(&mut self, ctx: MatchContext) -> MatchResult;
}

pub struct MatchContext<'a> {
    pub match_: Match,                       // from L0.03
    pub hidden: &'a mut GroundTruth,         // from fmr-hidden, opaque
    pub rng: &'a mut RngRegistry,            // from L0.01
    pub bus: &'a mut EventBus,               // from L0.02
    pub modifiers: ContextModifiers,         // from L0.09
    pub config: MatchConfig,                 // from balance/match_engine.toml
}

pub struct MatchResult {
    pub score: Score,
    pub events: Vec<MatchEvent>,
    pub stats: MatchStatistics,
    pub narrative: Vec<NarrativeEntry>,
    pub state_hash: Hash,                    // for golden tests
}
```

`GroundTruth` is opaque. The engine can only call its `resolve_*` methods (see `L0.03` §3.2 and ADR-003). It cannot read a latent value, cannot iterate over internal state, cannot `Debug` it into a log that leaks data.

### 2.2 Tick loop

```
for tick in 0..(90 * 10) {                    // 90 minutes at 1/10 s
    for player in all_on_pitch() {
        perception → decision → execution
    }
    bus.dispatch_all();
    random_events.check();                     // L0.08
    context_modifiers.update();                // L0.09
    if tick % 300 == 0 { statistics.snapshot(); }
}
```

The tick loop is **strictly sequential within a match**. There is no internal parallelism. Parallelism is at the level of multiple matches running concurrently (each with its own `GroundTruth`, `RngRegistry`, `EventBus`).

### 2.3 Micro-decision model

For each player, each tick:

1. **Perception**: what does the player see? A function of role, position, physical state, and the current `ContextModifiers`. Perception is expressed as a `PerceptionContext` passed to `resolve_*`.
2. **Decision**: what does the player want to do? A function of perception, role, tactics (L2), and minimal psychological state. At L0, tactics are a fixed default; the `manager_tactical_clarity` modifier adjusts variance.
3. **Execution**: what actually happens? The engine **calls `hidden.resolve_*`** with the appropriate context and receives an outcome (`DuelOutcome`, `ShotOutcome`, `PassOutcome`, ...). The engine does **not** compute the outcome from latents. It only interprets the outcome.

Each step has an error term. Errors are drawn from the match stream `match:{id}:micro-decisions`.

### 2.4 Sub-models

The engine calls the following `resolve_*` methods. The mapping from sub-model to method is fixed:

| Sub-model | Method |
|---|---|
| Duels (aerial, ground) | `resolve_duel` |
| Shots | `resolve_shot` |
| Passes | `resolve_pass` |
| Pressure | `resolve_pressure` |
| Injury risk | `resolve_injury_risk` |
| Goalkeeper saves | `resolve_save` |
| Set pieces | `resolve_set_piece` |

Additional sub-models (collective positioning, coordinated pressing, offside trap) are implemented **inside** `fmr-hidden` and exposed through the `resolve_*` functions above. The match engine does not know how they work; it only sees the outcomes.

### 2.5 Ratings

**Ratings are computed in `L0.06-match-output`, from `MatchEvent` only.** The engine emits events, not ratings. This is a hard rule to prevent circularity (see `00-foundations` §3, Plane B rule).

### 2.6 Determinism

- Seeds: `match:{id}:micro-decisions`, `match:{id}:set-pieces`, `match:{id}:random-events`
- No `HashMap` in the engine
- No floating-point in the critical path where avoidable (use `libm` if needed)
- State hash after each tick, compared across runs
- Golden test: 1,000 matches, same seed → same hash

### 2.7 What the engine does NOT do

- It does not know about clubs, seasons, money, contracts
- It does not compute ratings (those are in `L0.06`)
- It does not read `fmr-hidden` values directly (it calls `resolve_*`)
- It does not compute `ContextModifiers` (those come from `L0.09`)
- It does not implement `resolve_*` logic (that is `fmr-hidden`'s job)
- It does not implement random events (those are `L0.08`)

### 2.8 Configuration

All tuneable parameters live in `balance/match_engine.toml` *(planned — the `balance/` directory does not exist yet)*:

```toml
[tick]
duration_minutes = 90
ticks_per_second = 10
half_time_minute = 45
extra_time_minutes = 0          # L0 does not use extra time

[execution]
base_error_rate = 0.05
pressure_multiplier = 0.10
fatigue_multiplier = 0.08

[stats]
snapshot_interval_ticks = 300   # every 30 simulated seconds
```

Nothing in the engine is hardcoded. Changing a parameter requires no recompile (hot-reload at the start of a match).

## 3. Data model

`MatchContext`, `MatchResult`, `MatchConfig`, `Score`.

The engine produces `MatchEvent`s (see `L0.05`) and consumes `ContextModifiers` (see `L0.09`). It has no data model of its own.

## 4. Edge cases

- **Player sent off**: engine continues with 10 players, no restart
- **All substitutes used + injury**: engine plays with 10, no crash
- **Match abandoned** (weather, crowd): rare, produces `MatchAbandoned` event
- **Extra time**: not in L0 (friendly match), only in L1 (cup matches)
- **Penalty shootout**: not in L0, only in L1
- **`GroundTruth::resolve_*` returns an error**: panic (caught at the call site per ADR-004), the match is abandoned, the save is preserved
- **`EventBus` handler panics**: caught at the handler boundary, the event is logged as failed, the match continues

## 5. Definition of Done

- [ ] ADR-003 accepted
- [ ] Trait `MatchEngine` implemented for text mode
- [ ] Tick loop deterministic (state hash stable across 100 runs)
- [ ] Micro-decision model documented and implemented
- [ ] Sub-models implemented inside `fmr-hidden`
- [ ] No ratings computed in the engine
- [ ] No latent value read outside `resolve_*` (compile-time check: no field access on `GroundTruth`)
- [ ] Performance: 90 min < 1 s on mid-range hardware
- [ ] Golden test: 1,000 matches, same seed → same hash
- [ ] All tuneable parameters in `balance/match_engine.toml`
- [ ] `catch_unwind` at the `simulate` entry point (ADR-004)

## 6. Risks and Mitigations

| Risk | Severity | Mitigation |
|---|---|---|
| Engine becomes a monolith | High | Hard API; `L0.08` and `L0.09` are separate cells; sub-models live in `fmr-hidden` |
| Ratings leak latents | Critical | Type system: ratings only accept `&[MatchEvent]`; test of violation |
| Determinism drift | Critical | Golden test + `SIM_VERSION` bump procedure |
| Performance insufficient | Medium | Tick budget, profiling, benchmark in CI |
| Match "always wins the strong team" | High | Calibration in `L3-quality` *(planned)* |
| `fmr-hidden` API too coarse | Medium | `resolve_*` methods are the contract; adding a new resolution type is additive, not breaking |
| `fmr-hidden` API too fine | Medium | Fewer, larger `resolve_*` methods are preferred; granularity is a design choice, documented per method |

## 7. References

- [`../00-foundations.md`](../00-foundations.md) §3 (Three Planes), §4 (determinism)
- [`00-overview.md`](00-overview.md) — L0 cell map
- [`01-rng.md`](01-rng.md) — RNG
- [`02-event-bus.md`](02-event-bus.md) — event bus
- [`03-entities-minimal.md`](03-entities-minimal.md) — entities and `GroundTruth` API
- [`05-match-events.md`](05-match-events.md) — event type
- [`08-random-events.md`](08-random-events.md) — discrete events
- [`09-context-modifiers.md`](09-context-modifiers.md) — external state
- [`../../docs/adr/003-plane-a-enforcement.md`](../../docs/adr/003-plane-a-enforcement.md) — `resolve_*` API
- [`../../docs/adr/004-panic-strategy-ffi.md`](../../docs/adr/004-panic-strategy-ffi.md) — `catch_unwind`
- `../../archive/Prompt_Master_v7.md` §12 — original match engine *(planned — the archive/ directory does not exist yet)*
