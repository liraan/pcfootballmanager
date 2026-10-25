//! `pcfm-match` — the match engine (L0.04).
//!
//! The **orchestrator** of L0: it drives the tick loop, calls
//! `GroundTruth::resolve_*` for every resolution, publishes events to the
//! `EventChain`, and produces a `MatchResult`.
//!
//! # The contract (ADR-003, option B)
//!
//! The engine holds `&mut GroundTruth` and calls `resolve_*`. It receives
//! an outcome and a `CauseTrace`. It cannot read a latent value. The
//! compiler enforces this: `GroundTruth` has no public fields, no getters
//! returning raw latents.
//!
//! # Scope of step 7
//!
//! This is a **minimal** engine:
//!
//! - 90 minutes, tick = 1/10 s.
//! - Micro-decisions: pass, shot, duel, foul.
//! - Events: only the "significant" ones are emitted (shot, duel, foul).
//! - Stats: possession, shots, xG, goals.
//! - Narrative: empty (filled in step 8).
//!
//! Not in this step: substitutions, cards, injuries, tactics (L2).

#![deny(missing_docs)]

pub mod context;
pub mod engine;
pub mod result;
pub mod state;

pub use context::{MatchConfig, MatchContext};
pub use engine::{MatchEngine, TextMatchEngine};
pub use result::{MatchResult, MatchStatistics, Score};
pub use state::MatchState;

/// Returns the version of `pcfm-match`, as declared in `Cargo.toml`.
#[must_use]
pub const fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_not_empty() {
        assert_ne!(version(), "");
    }
}
