---
title: "L0.08 — Random Events (in-match)"
level: 0
cell: "L0-match"
module_id: "L0.08"
version: "1.0"
status: "draft"
last_updated: "2026-10-25"
supersedes: null
depends_on: ["00-foundations", "L0.01", "L0.03", "L0.04", "L0.05"]
referenced_by: ["L0.06", "L2-psyche"]
conforms_to: "00-foundations (v1.1)"
---

<!--
  MAINTAINER NOTE: "random" here means "unpredictable to the player", not
  "non-deterministic to the system". Every event is sampled from a dedicated
  stream. Same seed + same state = same event. If someone says "but it's random,
  it can't be deterministic", they've misunderstood the design. Point them to
  this comment.

  Also: the probability function reads OBSERVED state (fatigue, condition,
  morale, confidence) and, when a latent is needed (injury_proneness), it is
  accessed via GroundTruth::resolve_injury_risk, NOT by reading a latent field.
  The example probability function below is a simplified illustration; the real
  implementation calls resolve_* and applies the returned probability modifier.
-->

# L0.08 — Random Events (in-match)

## 0. Purpose

The catalogue of **discrete, in-match events** that are not part of the normal tick-by-tick flow: injuries, cards, brawls, weather interruptions, crowd incidents, individual mistakes, moments of genius.

**"Random" here means unpredictable to the player, not non-deterministic to the system.** Every event is sampled from a dedicated RNG stream, with a probability function of the current state. Same seed + same state → same event.

**Status: `draft`.** Implementation is blocked by [`../../docs/adr/003-plane-a-enforcement.md`](../../docs/adr/003-plane-a-enforcement.md) — the engine calls `resolve_*` for the underlying checks.

## 1. Context and dependencies

- Depends on `L0.01` (RNG), `L0.03` (entities), `L0.04` (engine), `L0.05` (events)
- Triggered by the engine at each tick, with a probability function
- Produces `MatchEvent`s of type `RandomEvent`
- Reads `ContextModifiers` from `L0.09`
- Reads latents **only** via `GroundTruth::resolve_*` (ADR-003, option B)

## 2. Specification

### 2.1 Event catalogue

| Family | Examples | Triggered by | Effects |
|---|---|---|---|
| **Physical** | Muscle injury, cramp, concussion, hard knock | `fatigue`, `injury_proneness`, `condition`, `pitch.condition` | Player leaves pitch or continues with penalty |
| **Technical** | Glaring miss, brilliant solo goal, keeper howler, own goal | execution outcome from `resolve_shot` / `resolve_pass`, `pressure_load`, `confidence` | Goal or missed chance; narrative |
| **Psychological** | Loss of concentration, panic after goal conceded, on-field argument | `confidence`, `morale`, `pressure_load`, `acute_arousal` | Temporary penalty to decisions; possible card |
| **Relational** | Brawl between teammates, argument with manager, dissent | `morale`, `temperament`, `squad.chemistry` | Card, substitution, narrative |
| **Environmental** | Pitch invasion, flare, referee controversy, weather change | `crowd.hostility`, `weather`, `referee.strictness` | Match pause, morale shift, card |
| **Set-piece** | Penalty, direct free kick, corner routine | `referee.strictness`, `foul` events | Goal opportunity |

### 2.2 Event definition

Each event is a `RandomEventDef` in `balance/random_events.toml` *(planned — the `balance/` directory does not exist yet)*:

```toml
[[event]]
id = "muscle_injury_mild"
family = "physical"
description = "Mild muscle strain, player can continue with reduced condition"

trigger_conditions = [
    "player.fatigue > 0.6",
    "player.condition < 0.8",
    "pitch.condition < 0.7",
]

# The probability function is a deterministic expression of OBSERVED state.
# Latents (e.g. injury_proneness) are NOT read here; they are folded into
# the outcome by GroundTruth::resolve_injury_risk, which returns a modifier
# in [0.0, +inf). See §2.3.
probability_function = """
base = 0.0001
* (1.0 + player.fatigue * 2.0)
* (1.0 + (1.0 - player.condition) * 1.5)
* (1.0 + (1.0 - pitch.condition) * 0.5)
* injury_risk_modifier   # from resolve_injury_risk(...)
"""

effects = [
    "player.condition -= 0.15",
    "player.fatigue += 0.10",
    "emit_event:Injury { severity: Mild }",
]

narrative_template = "narrative.injury.mild"
explanation_template = "explanation.injury.mild"
```

**Rule:** the `probability_function` is a deterministic expression of the current state. It is not a constant.

**Rule:** latents (e.g. `injury_proneness`) are never referenced directly in the TOML. The engine calls `GroundTruth::resolve_injury_risk(ctx)` once per player per tick, which returns a `risk_modifier: f32`. That modifier is the only latent-derived input to the probability function.

### 2.3 Sampling

At each tick, for each player, for each eligible event:

```
risk_modifier = hidden.resolve_<family>(player_context)   // e.g. resolve_injury_risk
p = evaluate(probability_function, state, risk_modifier)
if rng.stream("match:{id}:random-events").gen::<f32>() < p {
    apply(effects)
    emit_event(...)
}
```

The stream `match:{id}:random-events` is **isolated** from `match:{id}:micro-decisions`. Adding a new random event never shifts the micro-decisions.

### 2.4 Cause trace

Every random event has a `CauseTrace`:

```rust
CauseTrace {
    primary_cause: "muscle_injury_mild",
    contributing_factors: [
        Factor { name: "fatigue", value: 0.72, weight: 0.45 },
        Factor { name: "condition", value: 0.55, weight: 0.15 },
        Factor { name: "pitch_condition", value: 0.55, weight: 0.10 },
        Factor { name: "injury_risk_modifier", value: 1.30, weight: 0.30 },
    ],
    explanation_template: "explanation.injury.mild",
}
```

The cause trace is **mandatory**. An event without a cause trace is a bug. `Factor.value` for the latent-derived factor is the **modifier returned by `resolve_*`**, not the raw latent.

### 2.5 Distribution target

The catalogue is calibrated so that:

- **Mild injuries**: ~1.5 per match (across both teams)
- **Severe injuries**: ~0.05 per match (once every 20 matches)
- **Red cards**: ~0.1 per match
- **Yellow cards**: ~4 per match
- **Penalties**: ~0.3 per match
- **On-field brawls**: ~0.02 per match (rare)

These targets are in `balance/random_events.toml` *(planned)* and tested by `L3-quality` *(planned)*. Until `L3-quality` exists, they are tested by the CI job that runs the L0.04 golden tests with distribution assertions.

### 2.6 What is **not** here

- **Pre-match events** (player had a fight with his wife) → `L2-psyche/05-life-events.md` *(planned)*
- **Training events** (injury in training) → `L2-psyche/05-life-events.md` *(planned)* (training events are psyche-adjacent at L0; a dedicated training module comes with L2)
- **Season events** (transfer request) → `L2-market` *(planned)*
- **Crowd events outside the stadium** → `L2-crowd` *(planned)*

This cell is **only** what happens *during* the match, *in* the match context.

## 3. Data model

`RandomEventDef`, `RandomEventFamily`, `CauseTrace`, `Factor`. Event kinds are the `RandomEvent` variant of `MatchEvent` (see `L0.05`).

## 4. Edge cases

- **Same player, two events in one tick**: second event is deferred to the next tick
- **Injury on a player already injured**: prevented (condition check)
- **Brawl between teammates**: both get cards, manager may substitute
- **Pitch invasion**: match paused, morale shift, resumed
- **Penalty in stoppage time**: allowed
- **Event probability > 1.0**: clamped to 1.0, logged (calibration bug)
- **Event probability < 0.0**: error at load
- **`resolve_*` returns an error**: the event is skipped for this tick, logged; the match continues

## 5. Definition of Done

- [ ] ADR-003 accepted
- [ ] Event catalogue in `balance/random_events.toml` with ≥ 30 events
- [ ] Each event has trigger, probability, effects, narrative, explanation
- [ ] Sampling uses dedicated stream
- [ ] Latents accessed only via `resolve_*`, never read directly
- [ ] Every event has a cause trace
- [ ] Distribution targets tested (10,000 matches)
- [ ] No event decides the match on its own (statistical check)
- [ ] No event without a cause trace
- [ ] Events are deterministic (same seed → same event)

## 6. Risks and Mitigations

| Risk | Severity | Mitigation |
|---|---|---|
| Events feel "cheap" | High | Cause trace mandatory; distribution calibrated |
| Event decides match | High | Statistical check: max event impact on win probability < 0.1 |
| Probability overflow | Medium | Clamp + log |
| Catalogue too small | Medium | ≥ 30 events at v1.0 |
| Stream shared with micro-decisions | Critical | Dedicated stream, tested |
| Latent leak via `probability_function` | Critical | TOML cannot reference latents; only `resolve_*` modifiers |
| `resolve_*` call cost dominates the tick | Medium | Cache the result per player per tick; call once, not per event |

## 7. References

- [`../00-foundations.md`](../00-foundations.md) §4 — determinism
- [`00-overview.md`](00-overview.md) — L0 cell map
- [`01-rng.md`](01-rng.md) — stream isolation
- [`03-entities-minimal.md`](03-entities-minimal.md) — `GroundTruth::resolve_*` API
- [`04-match-engine.md`](04-match-engine.md) — caller
- [`05-match-events.md`](05-match-events.md) — event type
- [`09-context-modifiers.md`](09-context-modifiers.md) — external state
- [`../../docs/adr/003-plane-a-enforcement.md`](../../docs/adr/003-plane-a-enforcement.md) — no latent access
- `../../archive/Prompt_Master_v7.md` §Psiche — original event families *(planned — the archive/ directory does not exist yet)*
