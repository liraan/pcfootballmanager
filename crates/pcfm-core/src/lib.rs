//! `pcfm-core` — core infrastructure for PCFM.
//!
//! This crate is the foundation of every other crate in PCFM. It contains
//! no game logic; only infrastructure: RNG, event bus, IDs, deterministic
//! math.
//!
//! # Status
//!
//! **Step 0 of L0.** This file is intentionally minimal: it compiles and
//! passes a trivial test. The real modules (`rng`, `event_bus`, `ids`) are
//! added in Steps 1, 2, and 3.
//!
//! # Specification
//!
//! The full specification of what this crate must contain is in:
//!
//! - `modules/00-foundations.md` (doctrine)
//! - `modules/L0-match/01-rng.md` (RNG)
//! - `modules/L0-match/02-event-bus.md` (event bus)
//! - `modules/L0-match/03-entities-minimal.md` §3.1 (IDs)
//!
//! # Guarantees
//!
//! - No `unsafe` code (enforced by the workspace lints).
//! - No external dependencies yet (and none added without an ADR).
//! - The public API will grow in Steps 1–3.

#![deny(missing_docs)]

/// Returns the version of `pcfm-core`, as declared in `Cargo.toml`.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_not_empty() {
        assert!(!version().is_empty());
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
