---
title: "L0.05 — Match Events (observation layer)"
level: 0
cell: "L0-match"
module_id: "L0.05"
version: "1.0"
status: "ratified"
last_updated: "2026-10-25"
supersedes: null
depends_on: ["00-foundations", "L0.01", "L0.03"]
referenced_by: ["L0.04", "L0.06", "L0.07", "L0.08", "L1-statistics", "L2-scouting"]
conforms_to: "00-foundations (v1.1)"
---

<!--
  MAINTAINER NOTE: append-only is enforced in TWO places: (1) the type does not
  expose &mut accessors, (2) the hash chain catches any external tampering.
  If you're tempted to add an "update" or "delete" method for any reason, the
  answer is no. Add a corrective event instead. That's how audit logs work.

  Also: MatchEvent must NEVER contain a latent value. It is Plane B. The
  cause_trace is a textual/structured explanation, not a latent value. If you
  need to explain "why", do it through the cause_trace, not by leaking the
  latent that produced the outcome.
-->

# L0.05 — Match Events

## 0. Purpose

Define `MatchEvent`, the **atomic unit of Plane B**. Every observable action in a match is a `MatchEvent`. Events are **append-only**: never modified, never deleted.

This is where the "observation" plane is born. Ratings, statistics, projections, scouting reports — all of it is downstream of `MatchEvent`.

## 1. Context and dependencies

- Depends on `00-foundations`, `L0.01` (ID generation), `L0.03` (entities)
- Written by `L0.04-match-engine`
- Read by `L0.06-match-output`, `L0.07-save-minimal`, and every L1/L2 module that consumes statistics
- **Never** reads `fmr-hidden`

## 2. Specification

### 2.1 The event

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchEvent {
    pub event_id: EventId,                  // generational, unique
    pub match_id: MatchId,
    pub player_id: PlayerId,

    pub timestamp_minute: u8,               // 0–120, may exceed 90 for stoppage
    pub timestamp_second: u8,
    pub minutes_played_cumulative: u16,     // for this player, this match
    pub career_minutes_cumulative: u32,     // for this player, career

    pub season_label: Option<String>,       // None at L0 (friendly)

    pub event_type: EventType,
    pub fields: serde_json::Value,          // variant-dependent

    // Audit — never contains a latent value
    pub observed_attrs_snapshot_hash: Option<Hash>,
    pub cause_trace: CauseTrace,            // why this event happened
    pub created_at: GameInstant,
}
```

**Important:** `observed_attrs_snapshot_hash` is a hash of the **observed** attributes (Plane C) at the moment of the event, not the latent attributes. It is used for audit, not for exposing latents. If the observed attributes change later, the event still references the hash of what was observed at the time.

### 2.2 Event types (L0 set)

```rust
pub enum EventType {
    // Offensive
    Shot, Pass, Cross, Dribble, KeyPass,
    // Defensive
    Tackle, Interception, Clearance, Block, Save,
    // Physical
    Duel, Foul, Card,
    // Set pieces
    Corner, FreeKick, Penalty,
    // Meta
    Substitution, Injury, Goal, Assist,
    // Advanced
    ProgressiveCarry, ProgressivePass, Pressure,
    // Random (from L0.08)
    RandomEvent,
    // Administrative
    HalfTime, FullTime, MatchAbandoned,
}
```

The set is closed at L0. `L3-registry` allows pack-provided event kinds later.

### 2.3 Cause trace

Every event carries a `CauseTrace`: the minimal, structured explanation of **why** the event happened.

```rust
pub struct CauseTrace {
    pub primary_cause: String,              // e.g. "dribble_failed_under_pressure"
    pub contributing_factors: Vec<Factor>,  // e.g. fatigue, morale, defensive pressure
    pub explanation_template: String,       // localisable, for the UI
}

pub struct Factor {
    pub name: String,                       // e.g. "fatigue"
    pub value: f32,                         // current value (observed, not latent)
    pub weight: f32,                        // contribution to the outcome
}
```

**Rule:** every event must have a cause trace. An event without a cause trace is a bug. This is how pillar #5 (transparency) is enforced at the event level.

**Rule:** `Factor.value` is an **observed** value (Plane B/C), never a latent. If the engine used `finishing_close = 17` to resolve a shot, the cause trace says `finishing_observed = 15` (the current observed estimate), not `finishing_close = 17`.

### 2.4 Append-only

- No `update`, no `delete` on `MatchEvent`
- The event store is a `Vec<MatchEvent>` plus a hash chain:

```rust
pub struct EventChain {
    events: Vec<MatchEvent>,
    hashes: Vec<Hash>,          // hashes[i] = hash(hashes[i-1] || events[i])
}
```

- The chain hash is stored in the save. Reloading verifies the chain.
- Tampering with an event invalidates the chain. This is how "never overwritten" is enforced.

### 2.5 Career minutes

`career_minutes_cumulative` at L0 is derived from the save, not from a global career. At L0 (friendly match), it equals `minutes_played_cumulative`. At L1+, it accumulates across matches.

## 3. Data model

`MatchEvent`, `EventType`, `CauseTrace`, `Factor`, `EventChain`, `EventId`.

All fields are `Serialize`/`Deserialize` for the save (see `L0.07`). None of them is a latent value.

## 4. Edge cases

- **Event at minute 0, second 0**: allowed (kickoff-related)
- **Event at minute > 120**: clamped to 120, logged
- **Event with empty `fields`**: rejected at construction for event types that require fields (e.g. `Shot` requires `xg_value`)
- **Event with cause trace missing**: rejected at construction
- **Event chain broken**: load fails, save is quarantined
- **Duplicate `event_id`**: rejected, this is a generational ID collision (bug)
- **`observed_attrs_snapshot_hash` is `None`**: allowed (the observed attributes were not snapshotted at this event; used for low-importance events)

## 5. Definition of Done

- [ ] `MatchEvent` defined, `Serialize`/`Deserialize`
- [ ] `EventType` enum complete for L0
- [ ] `CauseTrace` mandatory
- [ ] `EventChain` with hash chain
- [ ] Append-only enforced by type (no `&mut` accessor to events)
- [ ] Test: chain hash verified on reload
- [ ] Test: event without cause trace → error
- [ ] Test: no `MatchEvent` field contains a latent value (compile-time check on types)
- [ ] Test: `observed_attrs_snapshot_hash` is a hash of Plane C data, not Plane A

## 6. Risks and Mitigations

| Risk | Severity | Mitigation |
|---|---|---|
| Event without cause | High | Constructor requires it |
| Cause trace leaks a latent | Critical | `Factor.value` is observed; type system prevents latent access |
| Chain tampering | High | Hash chain + verification on load |
| Event explosion (too many) | Medium | Tick budget; low-impact events aggregated at L2, not at L0 |
| `fields` as `serde_json::Value` | Medium | Validated by event type; typed variants in L1 |

## 7. References

- [`../00-foundations.md`](../00-foundations.md) §3 — Plane B
- [`00-overview.md`](00-overview.md) — L0 cell map
- [`03-entities-minimal.md`](03-entities-minimal.md) — entities
- [`04-match-engine.md`](04-match-engine.md) — producer
- [`06-match-output.md`](06-match-output.md) — consumer
- [`07-save-minimal.md`](07-save-minimal.md) — persistence
- [`../../docs/adr/003-plane-a-enforcement.md`](../../docs/adr/003-plane-a-enforcement.md) — no latent access
- [`../../archive/Prompt_Master_v7.md`](../../archive/Prompt_Master_v7.md) §23.2
