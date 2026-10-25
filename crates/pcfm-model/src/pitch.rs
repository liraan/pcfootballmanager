//! Pitch — dimensions, condition, weather.
//!
//! See `modules/L0-match/03-entities-minimal.md` §2.5.

use serde::{Deserialize, Serialize};

use crate::error::ModelError;

/// The weather conditions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Weather {
    /// Clear weather.
    Clear,
    /// Rain.
    Rain,
    /// Snow.
    Snow,
    /// Wind.
    Wind,
}

/// The pitch.
///
/// See `modules/L0-match/03-entities-minimal.md` §2.5.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Pitch {
    /// The length in metres (default 105).
    pub length_m: u16,
    /// The width in metres (default 68).
    pub width_m: u16,
    /// The condition, 0.0–1.0 (affects ball roll).
    pub condition: f32,
    /// The weather.
    pub weather: Weather,
}

impl Pitch {
    /// Creates a new `Pitch`, validating dimensions and condition.
    ///
    /// # Errors
    ///
    /// - [`ModelError::InvalidPitchDimensions`] if `length_m` or `width_m` is zero.
    /// - [`ModelError::InvalidValue`] if `condition` is outside [0, 1] or is NaN.
    pub fn try_new(
        length_m: u16,
        width_m: u16,
        condition: f32,
        weather: Weather,
    ) -> Result<Self, ModelError> {
        if length_m == 0 || width_m == 0 {
            return Err(ModelError::InvalidPitchDimensions {
                length: length_m,
                width: width_m,
            });
        }
        if !condition.is_finite() || !(0.0..=1.0).contains(&condition) {
            return Err(ModelError::InvalidValue {
                field: "pitch.condition",
                value: condition,
            });
        }
        Ok(Self {
            length_m,
            width_m,
            condition,
            weather,
        })
    }
}

impl Default for Pitch {
    fn default() -> Self {
        Self {
            length_m: 105,
            width_m: 68,
            condition: 1.0,
            weather: Weather::Clear,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_pitch_is_valid() {
        let p = Pitch::default();
        assert_eq!(p.length_m, 105);
        assert_eq!(p.width_m, 68);
    }

    #[test]
    fn zero_length_is_rejected() {
        let result = Pitch::try_new(0, 68, 1.0, Weather::Clear);
        assert!(matches!(
            result,
            Err(ModelError::InvalidPitchDimensions { length: 0, .. })
        ));
    }

    #[test]
    fn out_of_range_condition_is_rejected() {
        let result = Pitch::try_new(105, 68, 2.0, Weather::Clear);
        assert!(matches!(
            result,
            Err(ModelError::InvalidValue {
                field: "pitch.condition",
                ..
            })
        ));
    }
}
