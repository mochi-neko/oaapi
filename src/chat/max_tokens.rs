use crate::chat::ChatModel;
use crate::{ValidationError, ValidationResult};
use serde::{Deserialize, Serialize};

/// Max tokens count.
///
/// ## Range
/// The max tokens count must be between 1 and context window for each model.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MaxTokens {
    value: u32,
}

impl MaxTokens {
    /// Creates a new max tokens count.
    ///
    /// ## Error
    /// - [`ValidationError`] - If the max tokens count is not between 1 and context window for each model.
    pub fn new(
        value: u32,
        model: ChatModel,
    ) -> ValidationResult<Self> {
        let context_window = model.context_window();
        if value == 0 || value > context_window {
            Err(ValidationError {
                type_name: "MaxTokens".to_string(),
                reason: format!(
                    "The max tokens count must be between 1 and {}, but got {}.",
                    context_window,
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
