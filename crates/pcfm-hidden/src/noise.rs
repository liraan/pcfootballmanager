//! Internal helper for generating uniform random numbers in `[0, 1)`.
//!
//! This module exists because `clippy::cast_precision_loss` fires on the
//! `u32 -> f32` cast used to normalise the RNG output. The cast is
//! intentional and acceptable in this context: the value is a random
//! number in `[0, 1)`, and the lost precision (24 bits of the `u32`
//! mantissa vs 23 bits of `f32`) is irrelevant for the simulation.
//!
//! The `#[allow]` is local to this module: it does not propagate to the
//! rest of `pcfm-hidden`.

use rand_chacha::ChaCha20Rng;
use rand_core::RngCore;

/// Returns a uniform random number in `[0, 1)`.
#[allow(clippy::cast_precision_loss)]
pub fn next_unit(rng: &mut ChaCha20Rng) -> f32 {
    (rng.next_u32() as f32) / (u32::MAX as f32)
}
