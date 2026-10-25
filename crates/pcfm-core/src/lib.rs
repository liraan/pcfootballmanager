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

#![deny(missing_docs)]

/// Returns the version of `pcfm-core`, as declared in `Cargo.toml`.
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

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
