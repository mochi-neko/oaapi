use crate::{ValidationError, ValidationResult};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// The speed to speech.
/// 
/// ## Range
/// `[0.25, 4.0]`
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Speed {
    pub(crate) value: f32,
}

impl Default for Speed {
    fn default() -> Self {
        Self {
            value: 1.0,
        }
    }
}

impl Display for Speed {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Speed {
    pub fn new(value: f32) -> ValidationResult<Self, f32> {
        if value < 0.25 || value > 4.0 {
            return Err(ValidationError {
                type_name: "Speed".to_string(),
                reason: "Speed must be between 0.25 and 4.0".to_string(),
                value,
            });
        }

        Ok(Self {
            value,
        })
    }
}
