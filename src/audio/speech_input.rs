use serde::{Deserialize, Serialize};

use crate::{ValidationError, ValidationResult};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SpeechInput {
    pub(crate) value: String,
}

impl SpeechInput {
    pub fn new<S>(value: S) -> ValidationResult<Self>
    where
        S: Into<String>,
    {
        let value = value.into();
        if value.len() > 4096 {
            return Err(ValidationError {
                type_name: "SpeechInput".to_string(),
                reason: "Speech input must be at most 4096 characters"
                    .to_string(),
            });
        }

        Ok(Self {
            value,
        })
    }
}
