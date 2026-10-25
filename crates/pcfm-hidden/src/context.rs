//! Context types for `resolve_*` calls.
//!
//! Contexts are Plane B/C data: observed inputs to the resolution. They
//! carry no latent value.

use pcfm_core::ids::PlayerId;

/// The type of a duel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DuelType {
    /// Aerial duel.
    Aerial,
    /// Ground duel.
    Ground,
    /// Duel with the goalkeeper.
    WithGoalkeeper,
}

/// The context of a duel resolution.
#[derive(Debug, Clone, Copy)]
pub struct DuelContext {
    /// The attacker.
    pub attacker: PlayerId,
    /// The defender.
    pub defender: PlayerId,
    /// The type of duel.
    pub duel_type: DuelType,
}

/// The context of a shot resolution.
#[derive(Debug, Clone, Copy)]
pub struct ShotContext {
    /// The shooter.
    pub shooter: PlayerId,
}

/// The context of a pass resolution.
#[derive(Debug, Clone, Copy)]
pub struct PassContext {
    /// The passer.
    pub passer: PlayerId,
}

/// The context of a pressure resolution.
#[derive(Debug, Clone, Copy)]
pub struct PressureContext {
    /// The player under pressure.
    pub player: PlayerId,
}

/// The context of an injury risk resolution.
#[derive(Debug, Clone, Copy)]
pub struct InjuryContext {
    /// The player at risk.
    pub player: PlayerId,
}

/// The context of a save resolution.
#[derive(Debug, Clone, Copy)]
pub struct SaveContext {
    /// The goalkeeper.
    pub goalkeeper: PlayerId,
}

/// The context of a set piece resolution.
#[derive(Debug, Clone, Copy)]
pub struct SetPieceContext {
    /// The taker.
    pub taker: PlayerId,
}
