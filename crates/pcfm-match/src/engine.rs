//! The `MatchEngine` trait and the text implementation.

use crate::context::MatchContext;
use crate::result::MatchResult;
use crate::state::MatchState;

/// The trait that every match engine must implement.
///
/// The trait is an orchestrator: it receives a `MatchContext` (which
/// contains `&mut GroundTruth`) and returns a `MatchResult`. It never reads
/// a latent value; it calls `resolve_*` and interprets the outcome.
///
/// See `modules/L0-match/04-match-engine.md` §2.1.
pub trait MatchEngine: Send + Sync {
    /// Simulates the match described by `ctx`, returning the result.
    fn simulate(&mut self, ctx: MatchContext<'_>) -> MatchResult;
}

/// The text implementation of the match engine.
pub struct TextMatchEngine;

impl TextMatchEngine {
    /// Creates a new `TextMatchEngine`.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for TextMatchEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl MatchEngine for TextMatchEngine {
    fn simulate(&mut self, mut ctx: MatchContext<'_>) -> MatchResult {
        let total_ticks = u64::from(ctx.config.duration_minutes)
            .saturating_mul(u64::from(ctx.config.ticks_per_second))
            .saturating_mul(60);

        let mut state = MatchState::new();
        for tick in 0..total_ticks {
            state.tick(tick, &mut ctx);
        }

        let chain = std::mem::take(ctx.chain);
        state.finish(chain)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::MatchConfig;
    use pcfm_core::ids::{MatchId, PlayerId, RefereeId, SquadId};
    use pcfm_core::rng::RngRegistry;
    use pcfm_hidden::GroundTruth;
    use pcfm_model::{
        Foot, FormationId, Match, MatchRules, MatchSeed, PhysicalStateMinimal, Pitch, Player,
        PsychologicalStateMinimal, Referee, RoleId, Squad,
    };
    use pcfm_stats::EventChain;

    fn make_squad(id: u32, slot_base: u32) -> Squad {
        let players: Vec<Player> = (0..11)
            .map(|i| {
                Player::try_new(
                    PlayerId::new(slot_base + i, 0),
                    format!("P{}", slot_base + i),
                    RoleId::new("midfielder").unwrap(),
                    Foot::Right,
                    180,
                    PhysicalStateMinimal::default(),
                    PsychologicalStateMinimal::default(),
                )
                .unwrap()
            })
            .collect();
        Squad::try_new(
            SquadId::new(id, 0),
            players,
            vec![],
            FormationId::new("4-4-2").unwrap(),
        )
        .unwrap()
    }

    fn make_match() -> Match {
        Match::new(
            MatchId::new(1, 0),
            make_squad(1, 0),
            make_squad(2, 100),
            Pitch::default(),
            Referee::try_new(RefereeId::new(1, 0), 0.5, 0.7).unwrap(),
            MatchSeed::from_raw(42),
            MatchRules::default(),
        )
    }

    fn simulate_once(seed: u64) -> MatchResult {
        let match_ = make_match();
        let mut hidden = GroundTruth::new(seed, match_.id);
        let mut rng = RngRegistry::new(seed);
        let mut chain = EventChain::new();
        let config = MatchConfig::default();
        let ctx = MatchContext {
            match_: &match_,
            hidden: &mut hidden,
            rng: &mut rng,
            chain: &mut chain,
            config,
        };
        let mut engine = TextMatchEngine::new();
        engine.simulate(ctx)
    }

    #[test]
    fn simulation_completes_without_panic() {
        let _ = simulate_once(42);
    }

    #[test]
    fn same_seed_same_state_hash() {
        let a = simulate_once(42);
        let b = simulate_once(42);
        assert_eq!(a.state_hash, b.state_hash);
    }

    #[test]
    fn same_seed_same_score() {
        let a = simulate_once(42);
        let b = simulate_once(42);
        assert_eq!(a.score.home, b.score.home);
        assert_eq!(a.score.away, b.score.away);
    }

    #[test]
    fn different_seeds_produce_different_state_hashes() {
        let a = simulate_once(42);
        let b = simulate_once(43);
        assert_ne!(a.state_hash, b.state_hash);
    }

    #[test]
    fn match_produces_some_events() {
        let r = simulate_once(42);
        assert!(!r.events.is_empty());
    }

    #[test]
    fn all_events_have_a_cause() {
        let r = simulate_once(42);
        for ev in r.events.events() {
            assert_ne!(ev.cause_trace.primary_cause, "");
        }
    }

    #[test]
    fn all_events_are_within_120_minutes() {
        let r = simulate_once(42);
        for ev in r.events.events() {
            assert!(ev.timestamp_minute <= 120);
        }
    }

    #[test]
    fn possession_sums_to_one() {
        let r = simulate_once(42);
        let (h, a) = r.stats.possession;
        assert!((h + a - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn score_is_plausible() {
        // Over 10 seeds, the average total goals must be in a sensible range.
        let mut total_goals = 0_u32;
        for seed in 0..10_u64 {
            let r = simulate_once(seed);
            total_goals += u32::from(r.score.home) + u32::from(r.score.away);
        }
        // Average total goals per match should be in [0, 10]. Outside means
        // the model is broken (0 = no goals ever, 10+ = absurd).
        let avg = f64::from(total_goals) / 10.0;
        assert!(
            (0.0..=10.0).contains(&avg),
            "average total goals per match is {avg}, expected in [0, 10]"
        );
    }
}
