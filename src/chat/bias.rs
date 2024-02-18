use crate::{ValidationError, ValidationResult};
use serde::{Deserialize, Serialize};

/// Bias.
///
/// ## Range
/// The value must be between -100.0 and 100.0.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Bias {
    value: f32,
}

impl Bias {
    /// Creates a new bias.
    ///
    /// ## Error
    /// - [`ValidationError`] - If the value is not between -100.0 and 100.0.
    pub fn new(value: f32) -> ValidationResult<Self> {
        if value < -100.0 || value > 100.0 {
            Err(ValidationError {
                type_name: "Bias".to_string(),
                reason: format!(
                    "The value must be between -100.0 and 100.0, but got {}.",
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
