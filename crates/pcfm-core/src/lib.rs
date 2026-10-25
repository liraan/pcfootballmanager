//! `pcfm-core` — core infrastructure for PCFM.
//!
//! This crate is the foundation of every other crate in PCFM. It contains
//! no game logic; only infrastructure: RNG, event bus, IDs, deterministic
//! math.
//!
//! # Modules
//!
//! - [`rng`] — deterministic, isolated, named random streams (L0.01)
//! - [`event_bus`] — deterministic in-process event dispatch (L0.02)

#![deny(missing_docs)]

pub mod event_bus;
pub mod rng;

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
