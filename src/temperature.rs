use crate::{ValidationError, ValidationResult};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// The temperature for generations.
///
/// ## Range
/// `[0.0, 1.0]`
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Temperature {
    value: f32,
}

impl Display for Temperature {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Temperature {
    /// Creates a new temperature.
    ///
    /// ## Error
    /// - [`ValidationError`] - If the temperature is not between 0.0 and 1.0.
    pub fn new(value: f32) -> ValidationResult<Self, f32> {
        if value < 0.0 || value > 1.0 {
            Err(ValidationError {
                type_name: "Temperature".to_string(),
                reason: "The temperature must be between 0.0 and 1.0."
                    .to_string(),
                value,
            })
        } else {
            Ok(Self {
                value,
            })
        }
    }

    /// Returns the value of the temperature as a string.
    pub(crate) fn format(self) -> String {
        self.value.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        assert!(Temperature::new(0.0).is_ok());
        assert!(Temperature::new(0.5).is_ok());
        assert!(Temperature::new(1.0).is_ok());

        assert!(Temperature::new(-0.1).is_err());
        assert!(Temperature::new(1.1).is_err());
    }

    #[test]
    fn test_format() {
        assert_eq!(
            Temperature::new(0.0)
                .unwrap()
                .format(),
            "0"
        );
        assert_eq!(
            Temperature::new(0.5)
                .unwrap()
                .format(),
            "0.5"
        );
        assert_eq!(
            Temperature::new(1.0)
                .unwrap()
                .format(),
            "1"
        );
    }

    #[test]
    fn serialize_temperature() {
        assert_eq!(
            serde_json::to_string(&Temperature::new(0.0).unwrap()).unwrap(),
            "0.0"
        );
        assert_eq!(
            serde_json::to_string(&Temperature::new(0.5).unwrap()).unwrap(),
            "0.5"
        );
        assert_eq!(
            serde_json::to_string(&Temperature::new(1.0).unwrap()).unwrap(),
            "1.0"
        );
    }

    #[test]
    fn deserialize_temperature() {
        assert_eq!(
            serde_json::from_str::<Temperature>("0.0").unwrap(),
            Temperature::new(0.0).unwrap()
        );
        assert_eq!(
            serde_json::from_str::<Temperature>("0.5").unwrap(),
            Temperature::new(0.5).unwrap()
        );
        assert_eq!(
            serde_json::from_str::<Temperature>("1.0").unwrap(),
            Temperature::new(1.0).unwrap()
        );
    }
}
