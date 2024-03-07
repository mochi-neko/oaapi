use crate::{ValidationError, ValidationResult};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// The penalty.
///
/// ## Range
/// `[-2.0, 2.0]`
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Penalty {
    value: f32,
}

impl Display for Penalty {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Penalty {
    /// Creates a new penalty.
    ///
    /// ## Error
    /// - [`ValidationError`] - If the penalty is not between -2.0 and 2.0.
    pub fn new(value: f32) -> ValidationResult<Self, f32> {
        if value < -2.0 || value > 2.0 {
            Err(ValidationError {
                type_name: "Penalty".to_string(),
                reason: "The penalty must be between -2.0 and 2.0.".to_string(),
                value,
            })
        } else {
            Ok(Self {
                value,
            })
        }
    }
}
