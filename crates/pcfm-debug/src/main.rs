//! `pcfm-debug` — simulate a PCFM match and print the result.
//!
//! See `docs/MODULE_DEVELOPER_GUIDE.md` for the design of the engine and
//! for the simulation / rendering split.
//!
//! # Usage
//!
//! ```text
//! pcfm-debug [--seed <U64>] [--json] [--html <PATH>] [--quiet]
//! ```
//!
//! # What it does
//!
//! 1. Builds two demo squads (11 starters each, no bench).
//! 2. Simulates a match using `TextMatchEngine`.
//! 3. Prints the result (text or JSON), and optionally writes an HTML file.
//!
//! # Extending it
//!
//! To load teams from JSON, add a `--home <path>` and `--away <path>` flag
//! in `args.rs`, a loader in `demo.rs`, and a stable JSON schema for
//! `Squad`. That is a step 10+ concern. The CLI is designed to be extended
//! without touching `pcfm-match`.

#![deny(missing_docs)]

mod args;
mod demo;
mod check;
mod error;
mod render;
mod features;

use std::process::ExitCode;

use pcfm_core::rng::RngRegistry;
use pcfm_hidden::GroundTruth;
use pcfm_match::{MatchConfig, MatchContext, MatchEngine, TextMatchEngine};
use pcfm_stats::EventChain;

use crate::args::{ArgError, HELP, parse};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("pcfm-debug: {e}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = match parse(std::env::args()) {
        Ok(a) => a,
        Err(ArgError::HelpRequested) => {
            print!("{HELP}");
            return Ok(());
        }
        Err(e) => {
            eprintln!("pcfm-debug: {e}");
            eprintln!();
            eprint!("{HELP}");
            return Err(Box::new(e));
        }
    };

    let match_ =
        demo::demo_match(args.seed).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    let mut hidden = GroundTruth::new(args.seed, match_.id);
    let mut rng = RngRegistry::new(args.seed);
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
    let result = engine.simulate(ctx);

    if args.check {
        let report = check::run(args.seed)?;
        println!("=== HEALTH CHECK ===");
        for p in &report.passed {
            println!("  ✅ {p}");
        }
        for (name, reason) in &report.failed {
            println!("  ❌ {name}: {reason}");
        }
        println!();
        if report.is_ok() {
            println!("  {}/{} checks passed", report.passed.len(), report.total());
            return Ok(());
        }
        return Err("health check failed".into());
    }

    if args.quiet {
        println!("{} - {}", result.score.home, result.score.away);
        return Ok(());
    }

    if args.json {
        render::print_json(&result)?;
    } else {
        render::print_text(&result).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    }

    if let Some(path) = &args.html {
        render::write_html(&result, path)?;
        eprintln!("HTML written to {path}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_match_is_valid() {
        let m = demo::demo_match(42).unwrap();
        assert_eq!(m.home.players.len(), 11);
        assert_eq!(m.away.players.len(), 11);
    }

    #[test]
    fn rendering_a_result_does_not_panic() {
        let match_ = demo::demo_match(42).unwrap();
        let mut hidden = GroundTruth::new(42, match_.id);
        let mut rng = RngRegistry::new(42);
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
        let result = engine.simulate(ctx);
        let _ = render::print_text(&result);
        let _ = render::print_json(&result);
    }

    #[test]
    fn cli_error_from_io() {
        use crate::error::CliError;
        let e = std::io::Error::new(std::io::ErrorKind::NotFound, "test");
        let cli = CliError::from(e);
        assert!(matches!(cli, CliError::Io(_)));
    }
}
