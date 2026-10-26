//! Latent attributes (Plane A).
//!
//! These are the values that must **never** leave `pcfm-hidden`.

use pcfm_core::rng::next_unit;
use rand_chacha::ChaCha20Rng;

/// Latent attributes of a player.
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
    ///
    /// # Why uniform in `[0, 1)`
    ///
    /// Each latent is drawn uniformly from `[0, 1)`. This is a
    /// **prototype**: real player attributes are not uniform (they are
    /// roughly bell-shaped, with a long tail of elite players). A
    /// realistic model would use a distribution derived from real data,
    /// or a separate "world generator" (`pcfm-gen`, future) that produces
    /// correlated attributes (e.g. a striker with high `finishing` also
    /// tends to have high `aerial`).
    ///
    /// The latents are **not** saved: `GroundTruth` is reconstructed from
    /// `(world_seed, match_id)` on load, so the same seed always produces
    /// the same latents. This is what makes `replay` possible.
    ///
    /// See `docs/CODE_TOUR.md` "A note on the current simulation".
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
