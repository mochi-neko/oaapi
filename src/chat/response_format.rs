use crate::macros::{
    impl_display_for_serialize, impl_enum_string_serialization,
    impl_enum_struct_serialization,
};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

/// The response format of chat.
#[derive(Debug, Clone, PartialEq)]
pub enum ResponseFormat {
    /// Plane text
    Text(TextResponseFormat),
    /// JSON
    Json(JsonResponseFormat),
}

impl Default for ResponseFormat {
    fn default() -> Self {
        Self::Text(TextResponseFormat::default())
    }
}

impl Display for ResponseFormat {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | ResponseFormat::Text(text) => {
                write!(f, "{}", text)
            },
            | ResponseFormat::Json(json) => {
                write!(f, "{}", json)
            },
        }
    }
}

impl_enum_struct_serialization!(
    ResponseFormat,
    _type,
    Text(TextResponseFormat, "text"),
    Json(JsonResponseFormat, "json_object")
);

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

/// The response format of chat as plane text.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextResponseFormat {
    /// Must be "text".
    #[serde(rename = "type")]
    pub _type: ResponseFormatType,
}

impl Default for TextResponseFormat {
    fn default() -> Self {
        Self::new()
    }
}

impl_display_for_serialize!(TextResponseFormat);

impl TextResponseFormat {
    pub fn new() -> Self {
        Self {
            _type: ResponseFormatType::Text,
        }
    }
}

/// The response format of chat as JSON.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonResponseFormat {
    /// Must be "json_object".
    #[serde(rename = "type")]
    pub _type: ResponseFormatType,
}

impl Default for JsonResponseFormat {
    fn default() -> Self {
        Self::new()
    }
}

impl_display_for_serialize!(JsonResponseFormat);

impl JsonResponseFormat {
    pub fn new() -> Self {
        Self {
            _type: ResponseFormatType::Json,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serialization() {
        assert_eq!(
            serde_json::to_string(&ResponseFormat::Text(
                TextResponseFormat::new()
            ))
            .unwrap(),
            "{\"type\":\"text\"}"
        );

        assert_eq!(
            serde_json::to_string(&ResponseFormat::Json(
                JsonResponseFormat::new()
            ))
            .unwrap(),
            "{\"type\":\"json_object\"}"
        );
    }

    #[test]
    fn role_deserialization() {
        assert_eq!(
            serde_json::from_str::<ResponseFormat>("{\"type\":\"text\"}")
                .unwrap(),
            ResponseFormat::Text(TextResponseFormat::new())
        );

        assert_eq!(
            serde_json::from_str::<ResponseFormat>(
                "{\"type\":\"json_object\"}"
            )
            .unwrap(),
            ResponseFormat::Json(JsonResponseFormat::new())
        );
    }
}
