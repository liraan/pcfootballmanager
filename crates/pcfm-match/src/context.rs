//! Match context and configuration.
//!
//! The `MatchContext` carries the inputs to the engine. The `MatchConfig`
//! carries the tuneable parameters (loaded from `balance/match_engine.toml`
//! in a future step; for now, defaults are enough).

use pcfm_core::rng::RngRegistry;
use pcfm_hidden::GroundTruth;
use pcfm_model::Match;
use pcfm_stats::EventChain;

/// Configuration of the match engine.
///
/// All tuneable parameters live here. Nothing in the engine is hardcoded.
#[derive(Debug, Clone, Copy)]
pub struct MatchConfig {
    /// Total duration in minutes (default 90).
    pub duration_minutes: u16,
    /// Ticks per simulated second (default 10).
    pub ticks_per_second: u8,
    /// Probability per tick that a significant action happens.
    ///
    /// This is a simplification: in a full engine, every player acts every
    /// tick. Here, we model only the "significant" actions.
    pub action_probability: f32,
}

impl Default for MatchConfig {
    fn default() -> Self {
        Self {
            duration_minutes: 90,
            ticks_per_second: 10,
            action_probability: 0.0005,
        }
    }
}

/// The inputs to `MatchEngine::simulate`.
pub struct MatchContext<'a> {
    /// The match (home squad, away squad, pitch, referee, seed, rules).
    pub match_: &'a Match,
    /// The hidden state (Plane A). Never read directly; only `resolve_*`.
    pub hidden: &'a mut GroundTruth,
    /// The RNG registry. Used for action selection, not for resolution.
    pub rng: &'a mut RngRegistry,
    /// The event chain (Plane B). Append-only.
    pub chain: &'a mut EventChain,
    /// The engine configuration.
    pub config: MatchConfig,
}
