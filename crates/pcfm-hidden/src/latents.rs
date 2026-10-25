//! Latent attributes (Plane A).
//!
//! These are the values that must **never** leave `pcfm-hidden`.

use rand_chacha::ChaCha20Rng;

use crate::noise::next_unit;

/// Latent attributes of a player.
///
/// All values are in [0, 1]. They are never exposed. They are used only
/// inside `resolve_*` to compute outcomes.
#[derive(Debug, Clone, Copy)]
pub struct PlayerLatents {
    pub finishing: f32,
    pub passing: f32,
    pub tackling: f32,
    pub aerial: f32,
    pub goalkeeping: f32,
    pub set_pieces: f32,
    pub injury_proneness: f32,
}

impl PlayerLatents {
    /// Initialises a `PlayerLatents` from the RNG. Deterministic.
    pub fn from_rng(rng: &mut ChaCha20Rng) -> Self {
        Self {
            finishing: next_unit(rng),
            passing: next_unit(rng),
            tackling: next_unit(rng),
            aerial: next_unit(rng),
            goalkeeping: next_unit(rng),
            set_pieces: next_unit(rng),
            injury_proneness: next_unit(rng),
        }
    }
}
