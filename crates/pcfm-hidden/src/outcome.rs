//! Outcome types for `resolve_*` calls.
//!
//! Outcomes are Plane B data: observed results that the engine interprets.
//! They carry no latent value. The `CauseTrace` describes the outcome using
//! observed factors, never raw latents.

use pcfm_core::ids::PlayerId;

/// A contributing factor to an outcome.
#[derive(Debug, Clone)]
pub struct Factor {
    /// The name of the factor (observed).
    pub name: String,
    /// The value of the factor (observed).
    pub value: f32,
    /// The weight of the factor in the outcome.
    pub weight: f32,
}

/// The cause of an outcome, for the `MatchEvent.cause_trace` field.
#[derive(Debug, Clone)]
pub struct CauseTrace {
    /// The primary cause.
    pub primary_cause: String,
    /// The contributing factors.
    pub contributing_factors: Vec<Factor>,
    /// The template to localise for the UI.
    pub explanation_template: String,
}

/// The outcome of a duel.
#[derive(Debug, Clone)]
pub struct DuelOutcome {
    /// The winner.
    pub winner: PlayerId,
    /// Whether the attacker retained the ball.
    pub ball_retained: bool,
    /// Whether a foul was committed, and by whom.
    pub foul_committed: Option<PlayerId>,
    /// Observed injury risk modifier (not a latent).
    pub injury_risk_modifier: f32,
    /// Observed temperament modifier (not a latent).
    pub temperament_modifier: f32,
    /// The cause trace.
    pub cause: CauseTrace,
}

/// The outcome of a shot.
#[derive(Debug, Clone)]
pub struct ShotOutcome {
    /// Whether the shot resulted in a goal.
    pub goal: bool,
    /// Whether the shot was on target.
    pub on_target: bool,
    /// The expected goals value.
    pub xg_value: f32,
    /// The cause trace.
    pub cause: CauseTrace,
}

/// The outcome of a pass.
#[derive(Debug, Clone)]
pub struct PassOutcome {
    /// Whether the pass was completed.
    pub complete: bool,
    /// The cause trace.
    pub cause: CauseTrace,
}

/// The outcome of a pressure event.
#[derive(Debug, Clone)]
pub struct PressureOutcome {
    /// Whether the pressure succeeded.
    pub success: bool,
    /// The cause trace.
    pub cause: CauseTrace,
}

/// The outcome of an injury risk resolution.
#[derive(Debug, Clone)]
pub struct InjuryOutcome {
    /// Whether the player was injured.
    pub injured: bool,
    /// The observed risk modifier (not a latent).
    pub risk_modifier: f32,
    /// The cause trace.
    pub cause: CauseTrace,
}

/// The outcome of a save.
#[derive(Debug, Clone)]
pub struct SaveOutcome {
    /// Whether the save succeeded.
    pub saved: bool,
    /// The cause trace.
    pub cause: CauseTrace,
}

/// The outcome of a set piece.
#[derive(Debug, Clone)]
pub struct SetPieceOutcome {
    /// Whether the set piece succeeded.
    pub success: bool,
    /// The cause trace.
    pub cause: CauseTrace,
}
