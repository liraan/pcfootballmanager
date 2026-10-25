---
title: "L0.03 — Minimal Entities"
level: 0
cell: "L0-match"
module_id: "L0.03"
version: "1.0"
status: "ratified"
last_updated: "2026-10-25"
supersedes: null
depends_on: ["00-foundations", "L0.01"]
referenced_by: ["L0.04", "L0.05", "L0.07", "L0.08", "L0.09", "L1-roster"]
conforms_to: "00-foundations (v1.1)"
---

<!--
  MAINTAINER NOTE: the rule of this cell is "if the match engine does not need
  it, it is not here". Every time someone tries to add a field, ask: does the
  engine read it during a match? If no, it belongs to L1 or L2. If yes, it
  belongs here. The temptation to add "just one more field" is how L0 becomes
  a monolith.

  IMPORTANT: with ADR-003 (option B), latents are NOT a field of Player. They
  live in fmr-hidden, indexed by player_id. The public Player struct has only
  observed/minimal state. The match engine calls resolve_* on GroundTruth, not
  field access on Player.
-->

# L0.03 — Minimal Entities

## 0. Purpose

Define the **minimum set of entities** required to play a single match. Everything else — competition, calendar, club finances, staff, contracts, youth, market — is L1 or L2.

The rule: **if the match engine does not need it, it is not here.**

## 1. Context and dependencies

- Depends on `00-foundations` and `L0.01` (for ID generation)
- Used by `L0.04`, `L0.05`, `L0.07`, `L0.08`, `L0.09`
- The **full** `Player` entity lives in `L1-roster/01-player-attributes.md` *(planned — the L1-roster directory does not exist yet)*. L0 has a **projection** of it.
- The latent attributes of a player live in `fmr-hidden`, indexed by `player_id`. They are **not** part of the public `Player` struct. See [`../../docs/adr/003-plane-a-enforcement.md`](../../docs/adr/003-plane-a-enforcement.md).

## 2. Specification

### 2.1 Player (L0 projection)

```rust
pub struct Player {
    pub id: PlayerId,
    pub name: String,                    // display only, not localised at L0
    pub role: RoleId,                    // from registry (L3), or a hardcoded minimal set at L0
    pub preferred_foot: Foot,
    pub height_cm: u16,

    // Plane B/C — visible to the engine
    pub physical: PhysicalStateMinimal,
    pub psychological: PsychologicalStateMinimal,
}
```

**Notes:**

- There is **no** `latent` field. Latent attributes live in `fmr-hidden` and are accessed by the match engine only via `GroundTruth::resolve_*`. The engine never sees a latent number. This is the whole point of ADR-003 (option B).
- `physical` and `psychological` are minimal: only what the match engine needs.
- The `Player` struct is `Serialize`/`Deserialize`, but it contains no Plane A data, so serialising it never leaks latents.

### 2.2 PhysicalStateMinimal

```rust
pub struct PhysicalStateMinimal {
    pub condition: f32,          // 0.0–1.0, freshness
    pub match_sharpness: f32,    // 0.0–1.0
    pub fatigue: f32,            // 0.0–1.0, accumulated
}
```

No injury history, no recovery rate, no `minutes_last_7_days`. Those are `L1-roster`.

### 2.3 PsychologicalStateMinimal

```rust
pub struct PsychologicalStateMinimal {
    pub confidence: f32,         // 0.0–1.0
    pub morale: f32,             // 0.0–1.0
    pub pressure_load: f32,      // 0.0–1.0 accumulator
}
```

No `belonging`, `role_clarity`, `acute_arousal`, `leadership`, `determination`, etc. Those are `L2-psyche`.

### 2.4 Squad

```rust
pub struct Squad {
    pub id: SquadId,
    pub players: Vec<Player>,    // exactly 11 starters at kickoff
    pub bench: Vec<Player>,      // 0–7 substitutes
    pub formation: FormationId,  // from registry (L3), or hardcoded minimal at L0
}
```

### 2.5 Pitch

```rust
pub struct Pitch {
    pub length_m: u16,           // default 105
    pub width_m: u16,            // default 68
    pub condition: f32,          // 0.0–1.0, affects ball roll
    pub weather: Weather,        // Clear, Rain, Snow, Wind
}
```

### 2.6 Referee

```rust
pub struct Referee {
    pub id: RefereeId,
    pub strictness: f32,         // 0.0–1.0, affects card probability
    pub consistency: f32,        // 0.0–1.0, affects decision variance
}
```

### 2.7 Match

```rust
pub struct Match {
    pub id: MatchId,
    pub home: Squad,
    pub away: Squad,
    pub pitch: Pitch,
    pub referee: Referee,
    pub seed: MatchSeed,         // derived from world_seed + match_id
    pub rules: MatchRules,       // duration, subs allowed, etc.
}
```

### 2.8 What is **not** here

- Club, Nation, City, Stadium, Competition, Season, Contract, Staff, Manager, Board, Fan
- Those are L1 and L2. L0 does not know they exist.

**Design note:** the match engine is usable in "sandbox" mode — pass two squads directly, no club, no season. This is the test harness for L0.

## 3. Data model

`Player`, `PhysicalStateMinimal`, `PsychologicalStateMinimal`, `Squad`, `Pitch`, `Referee`, `Match`, `MatchRules`, `Foot`, `Weather`.

### 3.1 ID types

All IDs are generational (see `L0.01` and `L3-quality` *(planned)*). At L0:

```rust
pub type PlayerId = EntityId<PlayerTag>;
pub type SquadId = EntityId<SquadTag>;
pub type MatchId = EntityId<MatchTag>;
pub type RefereeId = EntityId<RefereeTag>;
```

### 3.2 Where are the latents?

They are in `fmr-hidden`. The crate exposes:

```rust
// fmr-hidden, public API
pub struct GroundTruth { /* private */ }

impl GroundTruth {
    pub fn resolve_duel(&mut self, ctx: DuelContext) -> DuelOutcome;
    pub fn resolve_shot(&mut self, ctx: ShotContext) -> ShotOutcome;
    pub fn resolve_pass(&mut self, ctx: PassContext) -> PassOutcome;
    pub fn resolve_pressure(&mut self, ctx: PressureContext) -> PressureOutcome;
    pub fn resolve_injury_risk(&mut self, ctx: InjuryContext) -> InjuryOutcome;
    pub fn resolve_save(&mut self, ctx: SaveContext) -> SaveOutcome;
    pub fn resolve_set_piece(&mut self, ctx: SetPieceContext) -> SetPieceOutcome;
    pub fn drain_observations(&mut self) -> Vec<ObservationEvent>;
}
```

The match engine holds `&mut GroundTruth` in its context (see [`04-match-engine.md`](04-match-engine.md)) and calls `resolve_*`. It never reads a latent value. This is enforced by the absence of any public accessor on `GroundTruth`.

## 4. Edge cases

- **Squad with fewer than 11 players**: error at construction, not a panic
- **Squad with more than 11 starters**: error at construction
- **Pitch with 0 length**: error
- **Condition outside [0,1]**: clamp on construction, log
- **Match with same home and away squad**: allowed (training match), but flagged
- **Referee with `strictness` outside [0,1]**: clamp

## 5. Definition of Done

- [ ] All entities defined, `#[derive(Serialize, Deserialize)]`
- [ ] Validation on construction (`try_new`, not `new`)
- [ ] No entity from L1/L2 leaks into L0
- [ ] No field of `Player` exposes latent data (type system check)
- [ ] `fmr-hidden` public API is exactly the `resolve_*` functions above
- [ ] Test: construct a match from two squads, run it, get a result
- [ ] Test: no `Player` field can be used to derive a latent value
- [ ] Test: `fmr-hidden` does not export `LatentAttributes` as a public type

## 6. Risks and Mitigations

| Risk | Severity | Mitigation |
|---|---|---|
| L0 entities grow into L1 entities | High | Hard rule: if the match engine doesn't need it, it's not here. Reviewed at PR |
| Latent leakage via `Debug` | Critical | `Player` has no latent fields (ADR-003); `GroundTruth` has a custom `Debug` that prints only `GroundTruth { .. }` |
| `fmr-hidden` accidentally exposes a latent getter | Critical | Review + type system; the crate is tiny and reviewed line by line |
| Clamping hides bugs | Medium | Log a warning when clamping occurs |
| `Squad` with 0 bench | Low | Allowed (friendly match) |

## 7. References

- [`../00-foundations.md`](../00-foundations.md) §3 — Three Planes
- [`../../docs/adr/003-plane-a-enforcement.md`](../../docs/adr/003-plane-a-enforcement.md) — `resolve_*` API, no latent fields
- [`00-overview.md`](00-overview.md) — L0 cell map
- [`04-match-engine.md`](04-match-engine.md) — consumer
- `../L1-roster/01-player-attributes.md` — full Player *(planned — the L1-roster directory does not exist yet)*
