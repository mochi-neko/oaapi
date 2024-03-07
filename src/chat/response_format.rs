use crate::macros::{
    impl_display_for_serialize, impl_enum_string_serialization,
};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

/// The response format of chat.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ResponseFormat {
    /// The type of response format.
    #[serde(rename = "type")]
    pub _type: ResponseFormatType,
}

impl From<ResponseFormatType> for ResponseFormat {
    fn from(_type: ResponseFormatType) -> Self {
        Self {
            _type,
        }
    }
}

impl_display_for_serialize!(ResponseFormat);

/// The response format type of chat.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResponseFormatType {
    /// "text"
    Text,
    /// "json_object"
    Json,
}

impl Default for ResponseFormatType {
    fn default() -> Self {
        Self::Text
    }
}

impl Display for ResponseFormatType {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | ResponseFormatType::Text => {
                write!(f, "text")
            },
            | ResponseFormatType::Json => {
                write!(f, "json_object")
            },
        }
    }
}

impl_enum_string_serialization!(
    ResponseFormatType,
    Text => "text",
    Json => "json_object"
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serialization() {
        assert_eq!(
            serde_json::to_string(&ResponseFormat::from(
                ResponseFormatType::Text
            ))
            .unwrap(),
            "{\"type\":\"text\"}"
        );

        assert_eq!(
            serde_json::to_string(&ResponseFormat::from(
                ResponseFormatType::Json
            ))
            .unwrap(),
            "{\"type\":\"json_object\"}"
        );
    }

    #[test]
    fn deserialization() {
        assert_eq!(
            serde_json::from_str::<ResponseFormat>("{\"type\":\"text\"}")
                .unwrap(),
            ResponseFormatType::Text.into()
        );

        assert_eq!(
            serde_json::from_str::<ResponseFormat>(
                "{\"type\":\"json_object\"}"
            )
            .unwrap(),
            ResponseFormatType::Json.into()
        );
    }
}
