use crate::{ValidationError, ValidationResult};
use serde::{Deserialize, Serialize};

/// Top logprobs.
///
/// ## Range
/// The penalty must be between 0 and 5.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TopLogprobs {
    value: u32,
}

impl TopLogprobs {
    /// Creates a new top logprobs.
    ///
    /// ## Error
    /// - [`ValidationError`] - If the top logprobs is not between 0 and 5.
    pub fn new(value: u32) -> ValidationResult<Self, u32> {
        if value > 5 {
            Err(ValidationError {
                type_name: "TopLogprobs".to_string(),
                reason: "The top logprobs must be between 0 and 5.".to_string(),
                value,
            })
        } else {
            Ok(Self {
                value,
            })
        }
    }
}
