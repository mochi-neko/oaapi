use crate::{ValidationError, ValidationResult};
use serde::{Deserialize, Serialize};

/// Top_p of generation.
///
/// ## Range
/// The top_p must be between 0.0 and 1.0.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TopP {
    value: f32,
}

impl TopP {
    /// Creates a new top_p.
    ///
    /// ## Error
    /// - [`ValidationError`] - If the top_p is not between 0.0 and 1.0.
    pub fn new(value: f32) -> ValidationResult<Self, f32> {
        if value < 0.0 || value > 1.0 {
            Err(ValidationError {
                type_name: "TopP".to_string(),
                reason: "The top_p must be between 0.0 and 1.0".to_string(),
                value,
            })
        } else {
            Ok(Self {
                value,
            })
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        assert!(TopP::new(0.0).is_ok());
        assert!(TopP::new(0.5).is_ok());
        assert!(TopP::new(1.0).is_ok());

        assert!(TopP::new(-0.1).is_err());
        assert!(TopP::new(1.1).is_err());
    }

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::to_string(&TopP::new(0.0).unwrap()).unwrap(),
            "0.0"
        );
        assert_eq!(
            serde_json::to_string(&TopP::new(0.5).unwrap()).unwrap(),
            "0.5"
        );
        assert_eq!(
            serde_json::to_string(&TopP::new(1.0).unwrap()).unwrap(),
            "1.0"
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            serde_json::from_str::<TopP>("0.0").unwrap(),
            TopP::new(0.0).unwrap()
        );
        assert_eq!(
            serde_json::from_str::<TopP>("0.5").unwrap(),
            TopP::new(0.5).unwrap()
        );
        assert_eq!(
            serde_json::from_str::<TopP>("1.0").unwrap(),
            TopP::new(1.0).unwrap()
        );
    }
}
