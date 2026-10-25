//! `CauseTrace` and `Factor` — structured explanation of an event.
//!
//! These types are used by Plane B (the `MatchEvent.cause_trace` field) and
//! produced by Plane A (`pcfm-hidden::resolve_*` outcomes). They are defined
//! here, in `pcfm-model`, because both planes need them and neither plane
//! should depend on the other.

use serde::{Deserialize, Serialize};

/// A contributing factor to an outcome.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Factor {
    /// The name of the factor (observed).
    pub name: String,
    /// The value of the factor (observed).
    pub value: f32,
    /// The weight of the factor in the outcome.
    pub weight: f32,
}

/// The cause of an outcome, for the `MatchEvent.cause_trace` field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CauseTrace {
    /// The primary cause.
    pub primary_cause: String,
    /// The contributing factors.
    pub contributing_factors: Vec<Factor>,
    /// The template to localise for the UI.
    pub explanation_template: String,
}
