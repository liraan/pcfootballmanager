# ADR-019 — Match engine contract

**Status:** proposed
**Date:** 2026-10-27
**Supersedes:** none (clarifies, does not replace, ADR-013)
**Superseded by:** —
**Blocks:** positional match engine implementation; first L1 module (`L1-world/01-nation.md`)
**Depends on:** ADR-012 (plugin contract), ADR-013 (LOD and local determinism)

## Context

The current L0 prototype (`MatchState::tick()`) is a per-tick probability model
(60/25/12/3). The declared goal is a positional simulation of 22 players, with a
path to 2D without rewriting the architecture. Discussion has settled on an
**agent-first** direction: state lives in players, the grid is only a
quantisation and an adjacency graph. Explicit decisions are needed on state,
rules, position, and resolution, otherwise the implementation will take them
implicitly.

## Options

**A. Grid-centric** — state lives in cells, players are derived. Simple, but
closes the door to 2D and makes defensive blocks artificial.
**B. Agent-first** — state lives in players; grid = quantisation + graph. More
expressive, compatible with 2D, requires discipline to avoid duplicating state.
**C. Pure event-driven** — events only, no persistent position. Consistent with
the prototype, but does not represent retreats or blocks.

## Decision

**D1. Agent-first (option B).** State belongs to players. The grid is a
quantisation function of `Position` and an adjacency graph; it is not an entity
with its own state.

**D2. Player functional state: closed enum.** Built-in variants only for now
(`InPossesso`, `InPressione`, `InCopertura`, `InRipiegamento`, `InCorsa`,
`InAttesa`). Extension is decided by the plugin contract (ADR-012), not by this
contract. The engine interprets only built-in variants.

**D3. Shared rules + per-player parameters.** A single rule set; individual
behaviour emerges from parameters (observable attributes, functional state,
tactical instructions, and psychological axes, passed as named parameters). The
set of active axes is configuration-defined; the set the engine understands is
implementation-defined. The interaction between tactical instructions and
psychological axes (e.g. how psychological state modulates adherence to
instructions) is deferred to L2-tactics and L2-psyche. Not independent
per-player rules: evaluation order would become semantics, with testing cost and
determinism risk.

**D4. Position: integer coordinates.** `Position { x: u16, y: u16 }` in integer
metres on a reference pitch of 105×68 (real standard dimensions). The grid is a
quantisation of `Position`, not the primary type.

**D5. Resolution: engine parameter, fixed per match, saved.**
`Resolution { width: u16, height: u16 }`, mandatory in `MatchSave`, included in
`state_hash`, verified on load and replay. Default: 105×68. The engine does not
know the platform: it receives the value and respects it.

**D6. LOD is not saved in `MatchSave`.** LOD is derivable from the module
manifest (ADR-013); saving it would duplicate state. Verifying that the manifest
exists and is coherent remains mandatory.

**D7. Two distinct parameters, single user experience (deferred).** LOD and
resolution are separate concepts (content vs granularity). The user chooses the
**modules** (from which LOD is computed) and the **resolution** freely. The
granular interface (separate choice of LOD and resolution) is **deferred**.

**D8. Zone of influence (derived, not stored).** A player's zone of influence
is a **derived relation** (player + observable attributes + functional state +
context), computed with integers or fixed-point, **not stored** as state. The
engine uses it to determine who can intervene on a position; it is not an entity
with its own state, like the grid. The specific attributes and formula are
deferred (see deferred 8).

## Deferred decisions

The following are explicitly outside this ADR and must be closed by later ADRs
or dedicated notes before implementation:

1. **Player iteration order** (determinism): proposed `EntityId`, to be
   confirmed.
2. **Off-ball pre-emption** (preventive foul, defensive block): priority rule,
   to be specified.
3. **Possession representation** (only "who has the carrier", no separate team
   flag).
4. **Granular UX**: separate choice of LOD and resolution, readable presets,
   per-platform defaults.
5. **Resolution values per platform** (mobile, ADR-014): not before mobile is
   real.
6. **Adaptive resolution** (during the match): not recommended due to
   determinism risk; to be revisited only with a dedicated ADR.
7. **LOD override** beyond the sum of weights: would require an amendment to
   ADR-013.
8. **Zone of influence formula and attributes.** Exact formula, required
   attributes, and their definition in `pcfm-model`. Deferred to a later ADR or
   a revision of `03-entities-minimal`.

## Consequences

- **`SIM_VERSION`**: bump when the contract enters into force (simulation model
  change).
- **`SCHEMA_VERSION`**: bump when `MatchSave` receives `resolution`.
- **Tests**: tests that build `MatchSave` must be updated; new test for the
  default and round-trip of `resolution`.
- **No conflict with ADR-013**: LOD remains defined as the sum of weights;
  resolution is an additional, orthogonal parameter.
- **No code changes in this phase**: the ADR is `proposed`; implementation
  follows ratification.

## Implementation notes

- The first real consumer of `resolution` will be the positional engine; until
  then the field stays in `MatchSave` with default and tests, without game logic
  using it.
- `Position` and `Resolution` are distinct types: `Position` is a player's
  position, `Resolution` is the pitch granularity. They must not be confused or
  unified.
- The grid, when it arrives, will be a type derived from `Position` +
  `Resolution`, not a primary type of the contract.

## References

- ADR-012 (Plugins: brick contract)
- ADR-013 (Worlds: LOD and local determinism)
- ADR-015 / ADR-016 (v1.0 scope)
- `modules/00-foundations.md` §3 (Three Planes), §9 (Concentric Structure)
- `modules/L0-match/04-match-engine.md` (current engine specification)
- `docs/REPO-SNAPSHOT.md` §6.5 (`pcfm-match` state)
