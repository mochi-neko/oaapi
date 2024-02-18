use crate::{ValidationError, ValidationResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Speed {
    pub(crate) value: f32,
}

impl Speed {
    pub fn new(value: f32) -> ValidationResult<Self> {
        if value < 0.25 || value > 4.0 {
            return Err(ValidationError {
                type_name: "Speed".to_string(),
                reason: "Speed must be between 0.25 and 4.0".to_string(),
            });
        }

        Ok(Self {
            value,
        })
    }
}
