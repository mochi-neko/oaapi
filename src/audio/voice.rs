use crate::macros::impl_enum_string_serialization;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

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
        match self {
            | Voice::Alloy => {
                write!(f, "alloy")
            },
            | Voice::Echo => {
                write!(f, "echo")
            },
            | Voice::Fable => {
                write!(f, "fable")
            },
            | Voice::Onyx => {
                write!(f, "onyx")
            },
            | Voice::Nova => {
                write!(f, "nova")
            },
            | Voice::Shimmer => {
                write!(f, "shimmer")
            },
        }
    }
}

impl FromStr for Voice {
    type Err = crate::ValidationError<String>;

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
                reason: "Unknown voice".to_string(),
                value: s.to_string(),
            }),
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
