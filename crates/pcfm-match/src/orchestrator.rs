//! `simulate_match` — the shared simulation.
//!
//! This module contains the **only** implementation of the tick loop and
//! micro-decisions. Both `TextMatchEngine` (in `pcfm-match`) and a future
//! `ThreeDEngine` (in a separate crate, e.g. `pcfm-match-3d`) call
//! `simulate_match` and differ only in rendering.
//!
//! # Adding a 3D engine
//!
//! A 3D engine is a new crate that:
//!
//! 1. Depends on `pcfm-match` (for `simulate_match` and `MatchEngine`).
//! 2. Implements `MatchEngine::simulate` by calling
//!    `pcfm_match::simulate_match` and then rendering the returned
//!    `MatchResult` to a 3D scene.
//! 3. Does **not** touch `pcfm-match`. Zero changes.
//!
//! This is the whole point of separating orchestration from rendering.

use crate::context::MatchContext;
use crate::result::MatchResult;
use crate::state::MatchState;

/// Runs the simulation and returns the `MatchResult`.
///
/// The returned `MatchResult` contains only data: score, events, statistics,
/// state hash, and an **empty** narrative. Rendering (text or 3D) is the
/// responsibility of the caller.
///
/// This is the function that a `ThreeDEngine` will call. The simulation does
/// not change when a new renderer is added.
pub fn simulate_match(ctx: &mut MatchContext<'_>) -> MatchResult {
    let total_ticks = u64::from(ctx.config.duration_minutes)
        .saturating_mul(u64::from(ctx.config.ticks_per_second))
        .saturating_mul(60);

    let mut state = MatchState::new();
    for tick in 0..total_ticks {
        state.tick(tick, ctx);
    }

    let chain = std::mem::take(ctx.chain);
    state.finish(chain)
}
