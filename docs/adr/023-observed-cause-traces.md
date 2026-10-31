# ADR-023 — Observed cause traces, not latent-derived causes

- **Status:** proposed
- **Date:** 2026-10-31
- **Supersedes:** none
- **Superseded by:** none
- **Blocks:** L0 match engine (event emission), `pcfm-hidden` outcome types, `pcfm-stats` event validation, `pcfm-model::cause`, the R-01 fix and its anti-leak test

## Context

The 2026-10-30 audits (13-39 and 20-24) found a critical Plane A disclosure: `pcfm-hidden` writes raw latent values into `Factor.value`, which is `Serialize`, is carried by `MatchEvent.cause_trace`, enters `chain_hash`, and is written to save JSON. The chain is end-to-end and verified on file:line.

Verified during the 2026-10-30/31 session (not taken from the audit on trust):

- 7 leak sites, not 5: duel (`attacker_score`, `defender_score`), shot (`finishing`), pass (`passing`), injury (`injury_proneness`), save (`goalkeeping`), set piece (`set_pieces`).
- `primary_cause` has 3 real consumers (`state.rs:249` workaround, `event.rs:122` validation, `test-runner:240`), all artefacts of duplication with `EventType`.
- `explanation_template`: zero consumers. `injury_risk_modifier` / `temperament_modifier`: constants `1.0`, zero consumers. `risk_modifier`: exact linear function of `injury_proneness`, zero consumers.
- The specification requires a cause trace: `specs/L0-match/05-match-events.md:117` says "an event without a cause trace is a bug".
- The specification requires observed values in the cause trace: `05-match-events.md:117-124` and `10-ground-truth.md:96-111` say the cause trace references observed values, never latents.
- Plane A has no observations. The resolver knows only latents. It cannot produce observed causes.

The conflict: the spec asks for an observed cause trace; the resolver cannot produce one; the current code produces a latent one. The previously saved decision ("remove `CauseTrace` and `Factor` entirely", option 1b, commit `73f7ad1`) was based on incomplete premises and would violate `05-match-events.md:117`. It is superseded by this ADR.

## Options considered

### Option 1 — Remove `CauseTrace` and `Factor` entirely (1b)

- **Pro:** eliminates the leak at the root; no new type; ~2 sessions.
- **Con:** violates `05-match-events.md:117` (cause trace mandatory); loses the pillar #5 (transparency) vehicle at event level; at L1/L2 the type must be reinvented with another migration and another `SIM_VERSION` bump. Work today is thrown away.

### Option 2 — The engine builds the cause trace from observations

- **Pro:** satisfies the written specification; eliminates the leak at the root (Plane A stops producing causes); keeps pillar #5; durable — the type survives to L1/L2 and is enriched, not replaced.
- **Con:** ~4-5 sessions; at L0 the observed cause is poor (few factors, some tautological with `EventType`); D3 risk if the structure outruns its content.

### Option 3 — Defer

- **Pro:** no decision without content.
- **Con:** the verified doctrinal breach stays; every session pays the cost of reconstructing the context; a failing anti-leak test is better than no test.

## Decision

**Option 2.** The engine builds the cause trace from `ObservationEvent`, not from `GroundTruth`. Plane A stops producing causes. The type `CauseTrace` stays and is enriched over time.

Six clauses, decided with the maintainer on 2026-10-31:

1. The resolver loses `cause`; the engine builds the cause trace from `ObservationEvent`.
2. `ObservationEvent` contains only observable outcomes (booleans, IDs, categories), never continuous values derived directly from a latent (e.g. `xg_value`, `goal_probability`). If a continuous observed value is needed in future, it is a Plane C estimate (with declared uncertainty), not an exact function of the latent.
3. The cause includes only factors whose metric is active; absent factors are omitted, not represented.
4. A measured zero (`Some(0.0)`) is included; a zero from absence (`None`) is omitted.
5. `Factor` has private fields and a single constructor `observed_factor(name, Option<f32>, weight) -> Option<Factor>`. `contributing_factors: Vec<Factor>` is unchanged and may be empty. No placeholder values.
6. At L1/L2, each factor declares its source (`source: ModuleId`). Not today (D3), but the single constructor is the extension point.

Additional decisions taken with the six clauses:

- `primary_cause` is removed. `MatchEvent.event_type` is the primary cause. The 3 consumers are simplified accordingly; `EmptyCauseTrace` is removed or redefined; `validate()` no longer checks `primary_cause`.
- `CauseTrace` becomes `{ contributing_factors: Vec<Factor>, explanation_template: String }`. `explanation_template` stays (localisation, no leak, zero consumers today).
- `SIM_VERSION` is bumped (the event payload enters `chain_hash`). `SCHEMA_VERSION` is not bumped (serde ignores unknown fields).
- Old saves remain contaminated: the fix does not retroactively clean JSON written before the bump. Declared, not hidden.
- `xg_value` in `ShotOutcome` is a potentially separate leak (it is a function of `finishing`). Tracked as OI-9, out of scope here.

## Consequences

### Easier
- Plane A no longer produces a serialisable object; the leak surface at the resolver is removed.
- The specification is satisfied, not amended: `05-match-events.md:117` stays true.
- The type survives to L1/L2; the cause trace is enriched with observed factors without migration.

### Harder
- The engine must build causes for every event: more code in `state.rs`, more tests.
- The anti-leak test must cover the engine path, not only the resolver path.
- At L0 the cause trace is poor; accepted as the cost of durability.

### Locked in
- Observations must be the only input to causes. Continuous values derived from latents cannot enter `ObservationEvent`; they must go through Plane C (estimate with uncertainty) or be computed from observable features (when they exist).
- `Factor` cannot be constructed outside `observed_factor`.

## Implementation notes

1. **Test first.** Write the anti-leak test and demonstrate it fails on the pre-fix commit. Without the failure demonstration, the test proves nothing. The test is structural: (a) no `value` in `contributing_factors`; (b) no latent in `MatchEvent.fields`; (c) no `Outcome` field is an exact function of a latent — best effort, covers known linear/simple cases, not exhaustive.
2. Remove `cause` from the 7 outcomes in `pcfm-hidden/src/outcome.rs`; remove the 7 `CauseTrace` constructions in `pcfm-hidden/src/lib.rs`.
3. Remove `injury_risk_modifier`, `temperament_modifier` (constants), `risk_modifier` (exact function of a latent): dead code, zero consumers.
4. In `pcfm-match/src/state.rs`, build the cause trace from `ObservationEvent` when the event is emitted. `primary_cause` is dropped; `event_type` is the primary cause.
5. In `pcfm-stats/src/event.rs`, remove `primary_cause` from `CauseTrace`, simplify `validate()`, remove `EmptyCauseTrace` or redefine it.
6. In `pcfm-model/src/cause.rs`, make `Factor` fields private; add `observed_factor(name, Option<f32>, weight) -> Option<Factor>`. Update all construction sites (7 in the resolver, 3 in tests, 1 in `chain.rs` test).
7. Update `pcfm-stats/src/chain.rs` tests, `pcfm-match/src/narrative.rs` test fixtures, `pcfm-match/src/engine.rs` test, `pcfm-test-runner/src/features/l0_match.rs`.
8. Bump `SIM_VERSION` in `crates/pcfm-match/src/save.rs`; add the `CHANGELOG-SIM.md` entry (five-step procedure).
9. Update `specs/L0-match/10-ground-truth.md` (outcomes no longer carry `cause`); verify `05-match-events.md` stays coherent (it already asks for observed causes).
10. Fix the false doc comments: `outcome.rs:4` ("They carry no latent value") and `cause.rs:14` ("The value of the factor (observed)").

## References

- `specs/00-foundations.md` §3 (Three Planes), §5 (banned: no truth leakage, no ratings from latents)
- `specs/L0-match/05-match-events.md:117-124` (cause trace mandatory, observed values)
- `specs/L0-match/10-ground-truth.md:96-111` (outcome contract, observed values)
- `docs/context/analysis/2026-10-30-13-39-review.md` (R-01)
- `docs/context/analysis/2026-10-30-20-24-review.md` (R-01 confirmation)
- ADR-003 (Plane A enforcement, option B)
- ADR-012 (plugin contract), ADR-013 (local determinism) — for clause 6
- Commit `73f7ad1` — the superseded decision (option 1b, incomplete premises)
