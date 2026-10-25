//! Health check for PCFM.
//!
//! Runs a series of sanity checks on the framework, using the public API
//! of `pcfm-match`, `pcfm-hidden`, `pcfm-stats`, and `pcfm-model`.
//! It does **not** reimplement any logic: it calls the framework.
//!
//! When the plugin system (ADR-012) is implemented, this module will be
//! the extension point: additional checks will validate the plugin sheets,
//! detect conflicts, and verify the LOD.

use pcfm_core::rng::RngRegistry;
use pcfm_core::time::GameInstant;
use pcfm_hidden::GroundTruth;
use pcfm_match::{
    MatchConfig, MatchContext, MatchEngine, MatchResult, MatchSave, TextMatchEngine,
};
use pcfm_model::Match;
use pcfm_stats::EventChain;

use crate::demo;
use crate::error::CliError;

/// The report of a health check.
pub struct CheckReport {
    /// The checks that passed.
    pub passed: Vec<&'static str>,
    /// The checks that failed, with a reason.
    pub failed: Vec<(&'static str, String)>,
}

impl CheckReport {
    /// Returns `true` if all checks passed.
    #[must_use]
    pub fn is_ok(&self) -> bool {
        self.failed.is_empty()
    }

    /// Returns the total number of checks.
    #[must_use]
    pub fn total(&self) -> usize {
        self.passed.len() + self.failed.len()
    }
}

/// Runs the health check.
///
/// # Errors
///
/// Returns a [`CliError`] if the check cannot be run at all (e.g. the demo
/// team cannot be constructed). A check that fails is reported in the
/// returned [`CheckReport`], not as an error.
pub fn run(seed: u64) -> Result<CheckReport, CliError> {
    let mut report = CheckReport {
        passed: Vec::new(),
        failed: Vec::new(),
    };

    // 1. Demo team is valid
    let match_ = match demo::demo_match(seed) {
        Ok(m) => {
            report.passed.push("demo teams are valid");
            m
        }
        Err(e) => {
            report.failed.push(("demo teams are valid", e.to_string()));
            return Ok(report);
        }
    };

    // 2. Same seed → same state hash
    let r1 = simulate(&match_, seed);
    let r2 = simulate(&match_, seed);
    if r1.state_hash == r2.state_hash {
        report.passed.push("same seed → same state_hash");
    } else {
        report.failed.push((
            "same seed → same state_hash",
            format!("{:?} vs {:?}", r1.state_hash, r2.state_hash),
        ));
    }

    // 3. Different seed → different state hash
    let other_match = demo::demo_match(seed.wrapping_add(1))?;
    let r3 = simulate(&other_match, seed.wrapping_add(1));
    if r1.state_hash == r3.state_hash {
        report.failed.push((
            "different seed → different state_hash",
            "hash is identical, the seed is ignored".to_string(),
        ));
    } else {
        report.passed.push("different seed → different state_hash");
    }

    // 4. Save/load roundtrip preserves state_hash
    let save = MatchSave::from_result(&r1, &match_, seed, GameInstant::from_raw(0));
    match save.to_bytes().and_then(|b| MatchSave::from_bytes(&b)) {
        Ok(loaded) => {
            if loaded.result.state_hash == r1.state_hash {
                report.passed.push("save/load roundtrip preserves state_hash");
            } else {
                report.failed.push((
                    "save/load roundtrip preserves state_hash",
                    "hash differs after roundtrip".to_string(),
                ));
            }
        }
        Err(e) => {
            report.failed.push(("save/load roundtrip", e.to_string()));
        }
    }

    // 5. Event chain is valid
    let mut chain_err: Option<String> = None;
    for ev in r1.events.events() {
        if ev.cause_trace.primary_cause.is_empty() {
            chain_err = Some("event with empty cause".to_string());
            break;
        }
        if ev.timestamp_minute > 120 {
            chain_err = Some("event with minute > 120".to_string());
            break;
        }
    }
    if chain_err.is_none() && !r1.events.verify(r1.state_hash) {
        chain_err = Some("chain hash does not verify".to_string());
    }
    match chain_err {
        None => report.passed.push("event chain is valid"),
        Some(reason) => report.failed.push(("event chain is valid", reason)),
    }

    // 6. Statistics are sane
    let (h, a) = r1.stats.possession;
    let stats_ok = (h + a - 1.0).abs() < 0.01
        && r1.stats.xg.0 >= 0.0
        && r1.stats.xg.1 >= 0.0
        && !r1.events.is_empty();
    if stats_ok {
        report.passed.push("statistics are sane");
    } else {
        report.failed.push((
            "statistics are sane",
            "possession != 1, or xG < 0, or no events".to_string(),
        ));
    }

    Ok(report)
}

/// Simulates a match with the given seed.
fn simulate(match_: &Match, seed: u64) -> MatchResult {
    let mut hidden = GroundTruth::new(seed, match_.id);
    let mut rng = RngRegistry::new(seed);
    let mut chain = EventChain::new();
    let config = MatchConfig::default();
    let ctx = MatchContext {
        match_,
        hidden: &mut hidden,
        rng: &mut rng,
        chain: &mut chain,
        config,
    };
    let mut engine = TextMatchEngine::new();
    engine.simulate(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_passes_on_default_framework() {
        let report = run(42).unwrap();
        assert!(report.is_ok(), "check failed: {:?}", report.failed);
        assert_eq!(report.total(), 6);
    }

    #[test]
    fn check_is_deterministic() {
        let a = run(42).unwrap();
        let b = run(42).unwrap();
        assert_eq!(a.passed, b.passed);
        assert_eq!(a.failed.len(), b.failed.len());
    }

    #[test]
    fn check_with_different_seed_passes() {
        let report = run(12345).unwrap();
        assert!(report.is_ok());
    }
}
