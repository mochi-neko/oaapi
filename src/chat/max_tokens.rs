use crate::chat::ChatModel;
use crate::{ValidationError, ValidationResult};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// The max tokens count.
///
/// ## Range
/// `[1, context_window_for_each_model]`
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MaxTokens {
    value: u32,
}

impl Default for MaxTokens {
    fn default() -> Self {
        Self {
            value: 1024,
        }
    }
}

impl Display for MaxTokens {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl MaxTokens {
    /// Creates a new max tokens count.
    ///
    /// ## Error
    /// - [`ValidationError`] - If the max tokens count is not between 1 and context window for each model.
    pub fn new(
        value: u32,
        model: ChatModel,
    ) -> ValidationResult<Self, u32> {
        let context_window = model.context_window();
        if value == 0 || value > context_window {
            Err(ValidationError {
                type_name: "MaxTokens".to_string(),
                reason: format!(
                    "The max tokens count must be between 1 and {}",
                    context_window,
                ),
                value,
            })
        } else {
            Ok(Self {
                value,
            })
        }
    }
}
