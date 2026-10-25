//! The result of a match simulation.

use pcfm_core::hash::Hash;
use pcfm_stats::EventChain;

/// The score of a match.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Score {
    /// The home team's goals.
    pub home: u8,
    /// The away team's goals.
    pub away: u8,
}

/// The statistics of a match.
///
/// For step 7, only the basic statistics are computed. The full bundle
/// (heatmap, momentum, pass completion, PPDA) is filled in step 8.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct MatchStatistics {
    /// Possession, as `(home, away)`, sums to 1.0.
    pub possession: (f32, f32),
    /// Shots, as `(home, away)`.
    pub shots: (u16, u16),
    /// Expected goals, as `(home, away)`.
    pub xg: (f32, f32),
    /// Goals, as `(home, away)`.
    pub goals: (u8, u8),
}

/// The result of a match simulation.
pub struct MatchResult {
    /// The score.
    pub score: Score,
    /// The append-only event chain.
    pub events: EventChain,
    /// The statistics.
    pub stats: MatchStatistics,
    /// The state hash, equal to `events.chain_hash()`.
    ///
    /// Used by golden tests: two simulations with the same seed must
    /// produce the same `state_hash`.
    pub state_hash: Hash,
    /// The narrative, empty in step 7.
    pub narrative: Vec<String>,
}
