use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

/// A prompt for generations.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct Prompt {
    value: String,
}

impl Display for Prompt {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl FromStr for Prompt {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s))
    }
}

impl From<String> for Prompt {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl Prompt {
    /// Creates a new prompt.
    pub fn new<S>(value: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            value: value.into(),
        }
    }

    /// Returns the value of the prompt as a string.
    pub(crate) fn format(self) -> String {
        self.value
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(
            Prompt::new("Hello, world!").format(),
            "Hello, world!"
        );
    }

    #[test]
    fn serialize_prompt() {
        assert_eq!(
            serde_json::to_string(&Prompt::new("Hello, world!")).unwrap(),
            r#""Hello, world!""#
        );
    }

    #[test]
    fn deserialize_prompt() {
        assert_eq!(
            serde_json::from_str::<Prompt>("\"Hello, world!\"").unwrap(),
            Prompt::new("Hello, world!")
        );
    }
}
