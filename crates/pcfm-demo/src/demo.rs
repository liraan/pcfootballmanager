//! Demo teams for the CLI.
//!
//! For now, the teams are hardcoded: 11 starters each, all "midfielder",
//! all with the same physical and psychological state. This is enough to
//! run the engine and see the result.
//!
//! In a future step, the CLI will accept a `--home <path>` and
//! `--away <path>` to load teams from JSON. That requires a stable JSON
//! schema for `Squad`, which is a step 10+ concern.

use pcfm_core::ids::{MatchId, PlayerId, RefereeId, SquadId};
use pcfm_model::{
    Foot, FormationId, Match, MatchRules, MatchSeed, PhysicalStateMinimal, Pitch, Player,
    PsychologicalStateMinimal, Referee, RoleId, Squad,
};

use crate::error::DemoError;

/// The slot base for the home team (0..=10).
const HOME_SLOT_BASE: u32 = 0;
/// The slot base for the away team (100..=110). Disjoint from the home team.
const AWAY_SLOT_BASE: u32 = 100;

/// Builds the demo home squad.
///
/// # Errors
///
/// Returns a [`DemoError::DemoTeam`] if the demo team is malformed. By
/// construction, it is not, so this error is unreachable in practice.
pub fn home_squad() -> Result<Squad, DemoError> {
    make_squad(1, "Home", HOME_SLOT_BASE)
}

/// Builds the demo away squad.
///
/// # Errors
///
/// See [`home_squad`].
pub fn away_squad() -> Result<Squad, DemoError> {
    make_squad(2, "Away", AWAY_SLOT_BASE)
}

/// Builds the demo match.
///
/// # Errors
///
/// See [`home_squad`].
pub fn demo_match(seed: u64) -> Result<Match, DemoError> {
    Ok(Match::new(
        MatchId::new(1, 0),
        home_squad()?,
        away_squad()?,
        Pitch::default(),
        Referee::try_new(RefereeId::new(1, 0), 0.5, 0.7)
            .map_err(|e| DemoError::DemoTeam(e.to_string()))?,
        MatchSeed::from_raw(seed),
        MatchRules::default(),
    ))
}

fn make_squad(id: u32, label: &str, slot_base: u32) -> Result<Squad, DemoError> {
    let mut players = Vec::with_capacity(11);
    for i in 0..11_u32 {
        let p = Player::try_new(
            PlayerId::new(slot_base + i, 0),
            format!("{label} {i}"),
            RoleId::new("midfielder").map_err(|e| DemoError::DemoTeam(e.to_string()))?,
            Foot::Right,
            180,
            PhysicalStateMinimal::default(),
            PsychologicalStateMinimal::default(),
        )
        .map_err(|e| DemoError::DemoTeam(e.to_string()))?;
        players.push(p);
    }
    Squad::try_new(
        SquadId::new(id, 0),
        players,
        vec![],
        FormationId::new("4-4-2").map_err(|e| DemoError::DemoTeam(e.to_string()))?,
    )
    .map_err(|e| DemoError::DemoTeam(e.to_string()))
}
