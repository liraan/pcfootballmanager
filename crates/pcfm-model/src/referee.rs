//! Referee — strictness and consistency.
//!
//! See `modules/L0-match/03-entities-minimal.md` §2.6.

use pcfm_core::ids::RefereeId;
use serde::{Deserialize, Serialize};

use crate::error::ModelError;

/// The referee.
///
/// See `modules/L0-match/03-entities-minimal.md` §2.6.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Referee {
    /// The unique identifier of the referee.
    pub id: RefereeId,
    /// The strictness, 0.0–1.0 (affects card probability).
    pub strictness: f32,
    /// The consistency, 0.0–1.0 (affects decision variance).
    pub consistency: f32,
}

impl Referee {
    /// Creates a new `Referee`, validating `strictness` and `consistency`.
    ///
    /// # Errors
    ///
    /// Returns [`ModelError::InvalidValue`] if any field is outside [0, 1]
    /// or is NaN.
    pub fn try_new(id: RefereeId, strictness: f32, consistency: f32) -> Result<Self, ModelError> {
        if !strictness.is_finite() || !(0.0..=1.0).contains(&strictness) {
            return Err(ModelError::InvalidValue {
                field: "referee.strictness",
                value: strictness,
            });
        }
        if !consistency.is_finite() || !(0.0..=1.0).contains(&consistency) {
            return Err(ModelError::InvalidValue {
                field: "referee.consistency",
                value: consistency,
            });
        }
        Ok(Self {
            id,
            strictness,
            consistency,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_referee_is_accepted() {
        let r = Referee::try_new(RefereeId::new(1, 0), 0.5, 0.7).unwrap();
        assert!((r.strictness - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn out_of_range_strictness_is_rejected() {
        let result = Referee::try_new(RefereeId::new(1, 0), 1.5, 0.7);
        assert!(matches!(
            result,
            Err(ModelError::InvalidValue {
                field: "referee.strictness",
                ..
            })
        ));
    }
}
