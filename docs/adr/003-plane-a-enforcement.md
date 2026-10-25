# ADR-003 — Plane A Enforcement

- **Status:** proposed
- **Date:** 2026-10-25
- **Supersedes:** none
- **Superseded by:** none
- **Blocks:** L0.04 (match engine), L0.08 (random events), all L2 modules that touch latents

<!--
  MAINTAINER NOTE: this is the single most consequential ADR in the project.
  It decides HOW Plane A (Ground Truth) is protected from the rest of the code.
  Two options are on the table. Option B is recommended, but the project lead
  must ratify. Until ratified, no code that touches fmr-hidden may be written.
-->

## Context

The Three Planes doctrine (`00-foundations` §3) says:

- **Plane A** (Ground Truth) contains latent attributes, physiological state, latent psychological state.
- **Plane A is never exposed to the player, to the UI, or to any module outside the match engine and the generator.**
- **Plane B** (Observation) is computed from events, never from latents.
- **Plane C** (Estimation) is what the player sees.

The doctrine is clear on the *what*. It is not yet clear on the *how*. Two enforcement strategies are viable, and they produce **different crate architectures**. Choosing late means rewriting the match engine.

Concretely: `L0.04-match-engine` cannot be implemented until this ADR is accepted, because the shape of the `fmr-hidden` API is exactly what this ADR decides.

## Options considered

### Option A — Doctrine + process enforcement

`fmr-hidden` is a normal crate that exposes `LatentAttributes` as public types. The match engine (`fmr-match`) imports `fmr-hidden` and reads latent values directly.

Enforcement is:
- **Dependency hygiene**: only `fmr-match` and `fmr-gen` declare `fmr-hidden` in their `Cargo.toml`.
- **CI check**: a script (`scripts/check-plane-a.sh`) fails the build if any other crate declares `fmr-hidden`.
- **Review discipline**: a human reviewer rejects PRs that leak latents.

**Pro:**
- Simple. Direct. The match engine can read `player.latent.finishing` and compute.
- No refactor of the match engine API. The engine calls the fields it needs.
- Lower initial cost.

**Con:**
- The enforcement is **social and process-based**, not type-based. A distracted refactor can add `fmr-hidden` to a `Cargo.toml` and the CI is the only guard.
- Once a crate depends on `fmr-hidden`, removing the dependency is a breaking change.
- The `LatentAttributes` type is public. It can be logged, printed, serialised, sent over the wire — all things that "never expose Plane A" forbids. Guarding every `Debug` impl, every `Serialize`, every `Display` is whack-a-mole.
- A developer debugging a UI issue can `dbg!(player.latent)` and the truth leaks into a screenshot. That's not hypothetical; it happens on every project that takes this route.

### Option B — Capability API: latents never cross the crate boundary

`fmr-hidden` exposes **only computations**, not values. The match engine asks questions and receives answers, but never sees the numbers that produced the answer.

```rust
// fmr-hidden — public API
pub struct GroundTruth {
    // fields are private, no public accessor
}

impl GroundTruth {
    /// The match engine asks: "how does this duel resolve?"
    /// It receives an outcome, not the numbers behind it.
    pub fn resolve_duel(&mut self, ctx: DuelContext) -> DuelOutcome;

    /// "Does this shot go in?"
    pub fn resolve_shot(&mut self, ctx: ShotContext) -> ShotOutcome;

    /// "Is this player injured by this challenge?"
    pub fn resolve_injury_risk(&mut self, ctx: InjuryContext) -> InjuryOutcome;

    /// The only readable output: Plane B data.
    pub fn drain_observations(&mut self) -> Vec<ObservationEvent>;
}
