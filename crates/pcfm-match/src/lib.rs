//! `pcfm-match` — the match engine (L0.04).
//!
//! The **orchestrator** of L0: it drives the tick loop, calls
//! `GroundTruth::resolve_*` for every resolution, publishes events to the
//! `EventChain`, and produces a `MatchResult`.
//!
//! # Architecture: simulation vs rendering
//!
//! The **simulation** is in [`orchestrator::simulate_match`]. It is the
//! **only** implementation of the tick loop and micro-decisions. Both
//! [`TextMatchEngine`] (this crate) and a future `ThreeDEngine` (in a
//! separate crate, e.g. `pcfm-match-3d`) call it. They differ only in
//! rendering.
//!
//! The **narrative** (see [`narrative`]) is **data**, not a rendered
//! string. It is filled by the renderer, not by `simulate_match`. A
//! `ThreeDEngine` can use `NarrativeEntry.text` as subtitles, or use
//! `NarrativeEntry.template_key + params` to render its own localised text,
//! or ignore narrative entirely.

#![deny(missing_docs)]

pub mod context;
pub mod engine;
pub mod narrative;
pub mod orchestrator;
pub mod result;
pub mod state;

pub use context::{MatchConfig, MatchContext};
pub use engine::{MatchEngine, TextMatchEngine};
pub use narrative::{Importance, NarrativeEntry, narrate};
pub use orchestrator::simulate_match;
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
