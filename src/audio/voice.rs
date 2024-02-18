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

impl_enum_string_serialization!(
    Voice,
    Alloy => "alloy",
    Echo => "echo",
    Fable => "fable",
    Onyx => "onyx",
    Nova => "nova",
    Shimmer => "shimmer"
);

impl TryFrom<String> for Voice {
    type Error = crate::ValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            | "alloy" => Ok(Self::Alloy),
            | "echo" => Ok(Self::Echo),
            | "fable" => Ok(Self::Fable),
            | "onyx" => Ok(Self::Onyx),
            | "nova" => Ok(Self::Nova),
            | "shimmer" => Ok(Self::Shimmer),
            | _ => Err(crate::ValidationError {
                type_name: "Voice".to_string(),
                reason: format!("Unknown voice: {}", value),
            }),
        }
    }
}
