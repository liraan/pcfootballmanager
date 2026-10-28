//! `pcfm-demo` — simulate a PCFM match and print the result.
//!
//! This is the demo binary split out of `pcfm-debug` (see ADR-020). It
//! builds two hardcoded squads, simulates a match, and renders the result
//! as text, JSON, or a self-contained HTML page.
//!
//! # Usage
//!
//! ```text
//! pcfm-demo [--seed <U64>] [--json] [--html <PATH>] [--quiet] [--help]
//! ```

mod demo;
mod error;
mod render;

use std::process::ExitCode;

use pcfm_core::rng::RngRegistry;
use pcfm_hidden::GroundTruth;
use pcfm_match::{MatchConfig, MatchContext, MatchEngine, TextMatchEngine};
use pcfm_stats::EventChain;

const HELP: &str = "\
pcfm-demo — simulate a PCFM match and print the result

USAGE:
    pcfm-demo [OPTIONS]

OPTIONS:
    --seed <U64>    RNG seed (default: 0)
    --json          Print the result as JSON
    --html <PATH>   Write a self-contained HTML page to PATH
    --quiet         Print only the final score
    --help          Print this help

EXAMPLE:
    pcfm-demo --seed 42 --html /tmp/match.html
";

/// Parsed command-line arguments.
struct Args {
    seed: u64,
    json: bool,
    html: Option<String>,
    quiet: bool,
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("pcfm-demo: {e}");
            ExitCode::FAILURE
        }
    }
}

fn parse_args() -> Result<Option<Args>, String> {
    let mut seed = 0u64;
    let mut json = false;
    let mut html = None;
    let mut quiet = false;

    let mut iter = std::env::args().skip(1);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--help" | "-h" => {
                print!("{HELP}");
                return Ok(None);
            }
            "--seed" => {
                let v = iter.next().ok_or("--seed requires a value")?;
                seed = v.parse().map_err(|_| format!("invalid seed: {v}"))?;
            }
            "--json" => json = true,
            "--html" => {
                let v = iter.next().ok_or("--html requires a path")?;
                html = Some(v);
            }
            "--quiet" => quiet = true,
            other => return Err(format!("unknown argument: {other}")),
        }
    }

    Ok(Some(Args {
        seed,
        json,
        html,
        quiet,
    }))
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let Some(args) = parse_args()? else {
        return Ok(());
    };

    let match_ = demo::demo_match(args.seed)?;
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

    if args.quiet {
        println!("{} - {}", result.score.home, result.score.away);
        return Ok(());
    }

    if args.json {
        render::print_json(&result)?;
    } else {
        render::print_text(&result)?;
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
    use crate::error::DemoError;

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
    fn demo_error_displays() {
        let e = DemoError::DemoTeam("test".to_string());
        assert!(e.to_string().contains("test"));
    }
}
