use crate::{ValidationError, ValidationResult};
use serde::{Deserialize, Serialize};

/// Penalty.
///
/// ## Range
/// The penalty must be between -2.0 and 2.0.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Penalty {
    value: f32,
}

impl Penalty {
    /// Creates a new penalty.
    ///
    /// ## Error
    /// - [`ValidationError`] - If the penalty is not between -2.0 and 2.0.
    pub fn new(value: f32) -> ValidationResult<Self> {
        if value < -2.0 || value > 2.0 {
            Err(ValidationError {
                type_name: "Penalty".to_string(),
                reason: format!(
                    "The penalty must be between -2.0 and 2.0, but got {}.",
                    value
                ),
            })
        } else {
            Ok(Self {
                value,
            })
        }
    }
}
