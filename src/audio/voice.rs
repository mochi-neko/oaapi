use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::macros::impl_enum_string_serialization;

/// The voice of a text-to-speech.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Voice {
    /// alloy
    Alloy,
    /// echo
    Echo,
    /// fable
    Fable,
    /// onyx
    Onyx,
    /// nova
    Nova,
    /// shimmer
    Shimmer,
}

impl Default for Voice {
    fn default() -> Self {
        Self::Alloy
    }
}

impl Display for Voice {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.format())
    }
}

impl FromStr for Voice {
    type Err = crate::ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            | "alloy" => Ok(Self::Alloy),
            | "echo" => Ok(Self::Echo),
            | "fable" => Ok(Self::Fable),
            | "onyx" => Ok(Self::Onyx),
            | "nova" => Ok(Self::Nova),
            | "shimmer" => Ok(Self::Shimmer),
            | _ => Err(crate::ValidationError {
                type_name: "Voice".to_string(),
                reason: format!("Unknown voice: {}", s),
            }),
        }
    }
}

impl Voice {
    pub(crate) fn format(self) -> &'static str {
        match self {
            | Voice::Alloy => "alloy",
            | Voice::Echo => "echo",
            | Voice::Fable => "fable",
            | Voice::Onyx => "onyx",
            | Voice::Nova => "nova",
            | Voice::Shimmer => "shimmer",
        }
    }
}

impl_enum_string_serialization!(
    Voice,
    Alloy => "alloy",
    Echo => "echo",
    Fable => "fable",
    Onyx => "onyx",
    Nova => "nova",
    Shimmer => "shimmer"
);

