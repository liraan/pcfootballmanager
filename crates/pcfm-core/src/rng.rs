//! RNG — deterministic, isolated, named random streams.
//!
//! See `modules/L0-match/01-rng.md` for the full specification.
//!
//! # Design
//!
//! Every stream is identified by a name and derived from
//! `(world_seed, stream_name)`. Two cells never share a stream. Adding a
//! new cell with a new stream never shifts the results of existing streams.
//!
//! # Algorithm
//!
//! `ChaCha20Rng` from the `rand_chacha` crate. Reproducible across platforms
//! and architectures, cryptographically strong, well-audited.
//!
//! # Seed derivation
//!
//! `stream_seed = blake3(world_seed.to_le_bytes() || stream_name.as_bytes())`
//!
//! Using a cryptographic hash ensures that changing `world_seed` changes all
//! streams, and that adding a new stream does not change existing streams.

use std::collections::BTreeMap;

use rand_chacha::ChaCha20Rng;
use rand_core::SeedableRng;

/// A named random stream.
///
/// The name is the isolation boundary: two cells with different names get
/// different streams, and adding a new stream never shifts existing ones.
///
/// See `modules/L0-match/01-rng.md` §2.3 for the naming convention
/// (`{domain}:{scope}:{purpose}`).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StreamName(String);

impl StreamName {
    /// Creates a new `StreamName` from the given name.
    ///
    /// # Errors
    ///
    /// Returns [`RngError::EmptyStreamName`] if the name is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use pcfm_core::rng::StreamName;
    ///
    /// let name = StreamName::new("match:abc:micro-decisions").unwrap();
    /// assert_eq!(name.as_str(), "match:abc:micro-decisions");
    /// ```
    pub fn new(name: impl Into<String>) -> Result<Self, RngError> {
        let name = name.into();
        if name.is_empty() {
            return Err(RngError::EmptyStreamName);
        }
        Ok(Self(name))
    }

    /// Returns the name as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Errors that can occur in the RNG module.
#[derive(Debug, thiserror::Error)]
pub enum RngError {
    /// The stream name is empty.
    #[error("stream name must not be empty")]
    EmptyStreamName,
}

/// A registry of named random streams, all derived from a single `world_seed`.
///
/// See `modules/L0-match/01-rng.md` §2.2.
///
/// # Determinism
///
/// Two `RngRegistry` with the same `world_seed` and the same `StreamName`
/// return streams that produce the same sequence of bytes.
pub struct RngRegistry {
    world_seed: u64,
    streams: BTreeMap<StreamName, ChaCha20Rng>,
}

impl RngRegistry {
    /// Creates a new `RngRegistry` with the given `world_seed`.
    #[must_use]
    pub const fn new(world_seed: u64) -> Self {
        Self {
            world_seed,
            streams: BTreeMap::new(),
        }
    }

    /// Returns the stream with the given name, creating it if it does not exist.
    ///
    /// The stream is deterministic: two `RngRegistry` with the same
    /// `world_seed` and the same `name` will return streams that produce the
    /// same sequence.
    ///
    /// Calling `stream()` multiple times with the same name is idempotent:
    /// the same stream is returned, and it continues from where it left off.
    pub fn stream(&mut self, name: &StreamName) -> &mut ChaCha20Rng {
        let world_seed = self.world_seed;
        self.streams
            .entry(name.clone())
            .or_insert_with(|| ChaCha20Rng::from_seed(derive_seed(world_seed, name)))
    }
}

/// Derives a 32-byte seed from `(world_seed, stream_name)`.
///
/// Uses BLAKE3, a cryptographic hash. The result is 32 bytes, which is the
/// seed size expected by `ChaCha20Rng::from_seed`.
fn derive_seed(world_seed: u64, name: &StreamName) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(&world_seed.to_le_bytes());
    hasher.update(name.as_str().as_bytes());
    *hasher.finalize().as_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_core::RngCore;

    /// Reads `n` bytes from the given RNG.
    fn first_n_bytes(rng: &mut ChaCha20Rng, n: usize) -> Vec<u8> {
        let mut buf = vec![0_u8; n];
        rng.fill_bytes(&mut buf);
        buf
    }

    #[test]
    fn empty_stream_name_is_rejected() {
        assert!(matches!(
            StreamName::new(""),
            Err(RngError::EmptyStreamName)
        ));
    }

    #[test]
    fn same_seed_same_sequence() {
        let name = StreamName::new("test").unwrap();
        let mut a = RngRegistry::new(42);
        let mut b = RngRegistry::new(42);
        let seq_a = first_n_bytes(a.stream(&name), 100);
        let seq_b = first_n_bytes(b.stream(&name), 100);
        assert_eq!(seq_a, seq_b);
    }

    #[test]
    fn different_seed_different_sequence() {
        let name = StreamName::new("test").unwrap();
        let mut a = RngRegistry::new(42);
        let mut b = RngRegistry::new(43);
        let seq_a = first_n_bytes(a.stream(&name), 100);
        let seq_b = first_n_bytes(b.stream(&name), 100);
        assert_ne!(seq_a, seq_b);
    }

    #[test]
    fn different_names_different_sequences() {
        let mut r = RngRegistry::new(42);
        let a = StreamName::new("a").unwrap();
        let b = StreamName::new("b").unwrap();
        let seq_a = first_n_bytes(r.stream(&a), 100);
        let seq_b = first_n_bytes(r.stream(&b), 100);
        assert_ne!(seq_a, seq_b);
    }

    #[test]
    fn adding_a_stream_does_not_shift_others() {
        let name = StreamName::new("test").unwrap();
        // Registry A: only "test".
        let mut a = RngRegistry::new(42);
        let seq_a = first_n_bytes(a.stream(&name), 100);
        // Registry B: "other" first, then "test".
        let mut b = RngRegistry::new(42);
        let other = StreamName::new("other").unwrap();
        let _ = first_n_bytes(b.stream(&other), 50);
        let seq_b = first_n_bytes(b.stream(&name), 100);
        assert_eq!(seq_a, seq_b);
    }

    #[test]
    fn stream_is_idempotent() {
        let name = StreamName::new("test").unwrap();
        let mut r = RngRegistry::new(42);
        let _ = first_n_bytes(r.stream(&name), 10);
        let seq_1 = first_n_bytes(r.stream(&name), 100);
        let mut r2 = RngRegistry::new(42);
        let _ = first_n_bytes(r2.stream(&name), 10);
        let seq_2 = first_n_bytes(r2.stream(&name), 100);
        assert_eq!(seq_1, seq_2);
    }
}
