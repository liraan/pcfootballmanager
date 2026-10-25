//! IDs — generational entity identifiers.
//!
//! See `modules/L0-match/03-entities-minimal.md` §3.1 for the full
//! specification.
//!
//! # Design
//!
//! An `EntityId<T>` is a `(slot, generation)` pair, tagged by `T`. The pair
//! is unique for the lifetime of the world, even if a slot is reused after
//! an entity is deleted.
//!
//! # Why generational
//!
//! A simple `u32` counter would reuse slots after deletion, and a stale ID
//! would silently refer to a different entity. With a generation counter,
//! a stale ID is detectably stale: it has the old generation, not the new one.
//!
//! # Determinism
//!
//! IDs are pure data. They contain no pointers, no hashes, no references.
//! They are safe to use as keys in `BTreeMap` (via `Ord`) and to serialise
//! (via `Serialize`/`Deserialize`).

use core::fmt;
use core::hash::Hash;
use core::marker::PhantomData;

use serde::{Deserialize, Serialize};

/// The tag of an entity. Implemented by `PlayerTag`, `SquadTag`, etc.
///
/// The trait is intentionally narrow: it only requires what `EntityId<T>`
/// needs to be `Copy`, `Ord`, and `Hash`. It is implemented as a marker
/// trait, and it is never implemented for any type that carries data.
pub trait EntityTag:
    'static + Copy + Clone + fmt::Debug + PartialEq + Eq + PartialOrd + Ord + Hash + Send + Sync
{
}

/// A generational entity identifier.
///
/// The pair `(slot, generation)` is unique for the lifetime of the world.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct EntityId<T: EntityTag> {
    slot: u32,
    generation: u32,
    #[serde(skip)]
    _tag: PhantomData<T>,
}

impl<T: EntityTag> EntityId<T> {
    /// Creates a new `EntityId` from a slot and a generation.
    #[must_use]
    pub const fn new(slot: u32, generation: u32) -> Self {
        Self {
            slot,
            generation,
            _tag: PhantomData,
        }
    }

    /// Returns the slot.
    #[must_use]
    pub const fn slot(self) -> u32 {
        self.slot
    }

    /// Returns the generation.
    #[must_use]
    pub const fn generation(self) -> u32 {
        self.generation
    }
}

// --- Concrete tags ----------------------------------------------------------

/// The tag for `Player` entities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PlayerTag;

/// The tag for `Squad` entities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SquadTag;

/// The tag for `Match` entities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MatchTag;

/// The tag for `Referee` entities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RefereeTag;

impl EntityTag for PlayerTag {}
impl EntityTag for SquadTag {}
impl EntityTag for MatchTag {}
impl EntityTag for RefereeTag {}

// --- Type aliases -----------------------------------------------------------

/// A `Player` identifier.
pub type PlayerId = EntityId<PlayerTag>;
/// A `Squad` identifier.
pub type SquadId = EntityId<SquadTag>;
/// A `Match` identifier.
pub type MatchId = EntityId<MatchTag>;
/// A `Referee` identifier.
pub type RefereeId = EntityId<RefereeTag>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_slot_and_generation_is_equal() {
        let a = PlayerId::new(1, 0);
        let b = PlayerId::new(1, 0);
        assert_eq!(a, b);
    }

    #[test]
    fn same_slot_different_generation_is_different() {
        let a = PlayerId::new(1, 0);
        let b = PlayerId::new(1, 1);
        assert_ne!(a, b);
    }

    #[test]
    fn different_slot_is_different() {
        let a = PlayerId::new(1, 0);
        let b = PlayerId::new(2, 0);
        assert_ne!(a, b);
    }

    #[test]
    fn slot_and_generation_accessors() {
        let id = PlayerId::new(7, 3);
        assert_eq!(id.slot(), 7);
        assert_eq!(id.generation(), 3);
    }

    #[test]
    fn different_tags_are_different_types() {
        // This is a compile-time test: `PlayerId` and `MatchId` are
        // different types, even though their `(slot, generation)` may be
        // the same. The test below would not compile if uncommented:
        //
        //     let p: PlayerId = PlayerId::new(1, 0);
        //     let m: MatchId = p; // type error
        //
        // We verify it at runtime by checking that they have different
        // type names, which is a proxy for "different types".
        let p = PlayerId::new(1, 0);
        let m = MatchId::new(1, 0);
        let p_type = core::any::type_name_of_val(&p);
        let m_type = core::any::type_name_of_val(&m);
        assert_ne!(p_type, m_type);
    }

    #[test]
    fn id_is_usable_as_btreemap_key() {
        use std::collections::BTreeMap;
        let mut map: BTreeMap<PlayerId, &str> = BTreeMap::new();
        map.insert(PlayerId::new(1, 0), "a");
        map.insert(PlayerId::new(2, 0), "b");
        map.insert(PlayerId::new(1, 1), "c");
        // 3 distinct keys: same slot with different generation are different
        // keys.
        assert_eq!(map.len(), 3);
        assert_eq!(map.get(&PlayerId::new(1, 0)), Some(&"a"));
        assert_eq!(map.get(&PlayerId::new(2, 0)), Some(&"b"));
        assert_eq!(map.get(&PlayerId::new(1, 1)), Some(&"c"));
    }

    #[test]
    fn id_roundtrips_through_serde_json() {
        let id = PlayerId::new(42, 7);
        let json = serde_json::to_string(&id).unwrap();
        let back: PlayerId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, back);
    }
}
