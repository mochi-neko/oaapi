use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::chat::Role;
use crate::macros::impl_enum_struct_serialization;
use crate::macros::impl_enum_with_string_or_array_serialization;
use crate::macros::{
    impl_display_for_serialize, impl_enum_string_serialization,
};
use crate::{ValidationError, ValidationResult};

/// A user message.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct UserMessage {
    /// The contents of the user message.
    pub content: MessageContent,
    /// The role of the messages author, in this case user.
    pub role: Role,
    /// An optional name for the participant.
    /// Provides the model information to differentiate between participants of the same role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Default for UserMessage {
    fn default() -> Self {
        Self {
            content: MessageContent::Text("".to_string()),
            role: Role::User,
            name: None,
        }
    }
}

impl_display_for_serialize!(UserMessage);

impl UserMessage {
    pub fn new(
        content: MessageContent,
        name: Option<String>,
    ) -> Self {
        Self {
            content,
            role: Role::User,
            name,
        }
    }
}

/// The content of a user message.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MessageContent {
    /// The text contents of the message.
    Text(String),
    /// An array of content parts with a defined type, each can be of type text or image_url when passing in images.
    /// You can pass multiple images by adding multiple image_url content parts.
    /// Image input is only supported when using the gpt-4-visual-preview model.
    Array(Vec<MessageContentPart>),
}

impl Default for MessageContent {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl_display_for_serialize!(MessageContent);

impl_enum_with_string_or_array_serialization!(
    MessageContent,
    Text(String),
    Array(MessageContentPart)
);

/// Content part of a message.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MessageContentPart {
    /// Text content part.
    Text(TextContentPart),
    /// Image content part.
    Image(ImageContentPart),
}

impl Default for MessageContentPart {
    fn default() -> Self {
        Self::Text(TextContentPart::new(String::new()))
    }
}

impl_display_for_serialize!(MessageContentPart);

impl_enum_struct_serialization!(
    MessageContentPart,
    type,
    Text(TextContentPart, "text"),
    Image(ImageContentPart, "image_url")
);

/// Text content part of a message.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct TextContentPart {
    /// The type of the content part, in this case `text`.
    #[serde(rename = "type")]
    pub _type: String,
    /// The text content.
    pub text: String,
}

impl Default for TextContentPart {
    fn default() -> Self {
        Self {
            _type: "text".to_string(),
            text: String::new(),
        }
    }
}

impl_display_for_serialize!(TextContentPart);

impl TextContentPart {
    pub fn new<S>(text: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            _type: "text".to_string(),
            text: text.into(),
        }
    }
}

/// Image content part of a message.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ImageContentPart {
    /// The type of the content part, in this case `image_url`.
    #[serde(rename = "type")]
    pub _type: String,
    /// The image url.
    pub image_url: ImageUrl,
}

impl Default for ImageContentPart {
    fn default() -> Self {
        Self {
            _type: "image_url".to_string(),
            image_url: ImageUrl {
                url: String::new(),
                detail: None,
            },
        }
    }
}

impl_display_for_serialize!(ImageContentPart);

impl ImageContentPart {
    pub fn new(image_url: ImageUrl) -> Self {
        Self {
            _type: "image_url".to_string(),
            image_url,
        }
    }
}

/// Image URL of a message content part.
#[derive(Debug, Clone, Eq, PartialEq, Default, Serialize, Deserialize)]
pub struct ImageUrl {
    /// The URL of the image.
    pub url: String,
    /// The detail of the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<ImageDetail>,
}

impl_display_for_serialize!(ImageUrl);

impl ImageUrl {
    /// Specify full URL.
    pub fn url(
        url: String,
        detail: Option<ImageDetail>,
    ) -> Self {
        Self {
            url,
            detail,
        }
    }

    /// Upload a Base64 encoded image.
    pub fn upload_base64(
        base64: String,
        format: ImageFormat,
        detail: Option<ImageDetail>,
    ) -> Self {
        let url = format!(
            "data:image/{};base64,{}",
            format.to_string(),
            base64
        );

        Self {
            url,
            detail,
        }
    }
}

/// Image format.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ImageFormat {
    /// PNG.
    Png,
    /// JPEG.
    Jpeg,
    /// WEBP
    Webp,
    /// Non-animated GIF
    Gif,
}

impl Default for ImageFormat {
    fn default() -> Self {
        ImageFormat::Png
    }
}

impl Display for ImageFormat {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | ImageFormat::Png => {
                write!(f, "png")
            },
            | ImageFormat::Jpeg => {
                write!(f, "jpeg")
            },
            | ImageFormat::Webp => {
                write!(f, "webp")
            },
            | ImageFormat::Gif => {
                write!(f, "gif")
            },
        }
    }
}

impl ImageFormat {
    pub fn from_path(
        path: std::path::PathBuf
    ) -> ValidationResult<Self, String> {
        let extension = path
            .extension()
            .ok_or_else(|| ValidationError {
                type_name: "ImageFormat".to_string(),
                reason: "Extension is not found".to_string(),
                value: path
                    .to_string_lossy()
                    .to_string(),
            })?
            .to_str()
            .ok_or_else(|| ValidationError {
                type_name: "ImageFormat".to_string(),
                reason: "Extension is not found".to_string(),
                value: path
                    .to_string_lossy()
                    .to_string(),
            })?;

        match extension {
            | "png" => Ok(ImageFormat::Png),
            | "jpg" | "jpeg" => Ok(ImageFormat::Jpeg),
            | "webp" => Ok(ImageFormat::Webp),
            | "gif" => Ok(ImageFormat::Gif),
            | _ => Err(ValidationError {
                type_name: "ImageFormat".to_string(),
                reason: "Not supported extension".to_string(),
                value: extension.to_string(),
            }),
        }
    }
}

/// Image detail to control over how the model processes the image and generates its textual understanding.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ImageDetail {
    /// `auto`
    Auto,
    /// `low`, disable the “high res” model.
    Low,
    /// `High`, enable the “high res” model.
    High,
}

impl Default for ImageDetail {
    fn default() -> Self {
        ImageDetail::Auto
    }
}

impl_display_for_serialize!(ImageDetail);

impl_enum_string_serialization!(
    ImageDetail,
    Auto => "auto",
    Low => "low",
    High => "high"
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialize_user_message() {
        let json = r#"{
            "content": "Hello, how are you?",
            "role": "user",
            "name": "John"
        }"#;
        let message: UserMessage = serde_json::from_str(json).unwrap();
        assert_eq!(
            message,
            UserMessage {
                content: MessageContent::Text(
                    "Hello, how are you?".to_string()
                ),
                role: Role::User,
                name: Some("John".to_string()),
            }
        );
    }

    #[test]
    fn deserialize_user_message_without_optional() {
        let json = r#"{
            "content": "Hello, how are you?",
            "role": "user"
        }"#;
        let message: UserMessage = serde_json::from_str(json).unwrap();
        assert_eq!(
            message,
            UserMessage {
                content: MessageContent::Text(
                    "Hello, how are you?".to_string()
                ),
                role: Role::User,
                name: None,
            }
        );
    }

    #[test]
    fn serialize_user_message() {
        let message = UserMessage {
            content: MessageContent::Text("Hello, how are you?".to_string()),
            role: Role::User,
            name: Some("John".to_string()),
        };
        let json = serde_json::to_string(&message).unwrap();
        assert_eq!(
            json,
            r#"{"content":"Hello, how are you?","role":"user","name":"John"}"#,
        );
    }

    #[test]
    fn serialize_user_message_without_optional() {
        let message = UserMessage {
            content: MessageContent::Text("Hello, how are you?".to_string()),
            role: Role::User,
            name: None,
        };
        let json = serde_json::to_string(&message).unwrap();
        assert_eq!(
            json,
            r#"{"content":"Hello, how are you?","role":"user"}"#,
        );
    }

    #[test]
    fn deserialize_user_message_with_image_content_part() {
        let json = r#"{
            "content": [
                {
                    "type": "text",
                    "text": "Hello, how are you?"
                },
                {
                    "type": "image_url",
                    "image_url": {
                        "url": "https://images.unsplash.com/photo-1622839418057-8e9e6b6b0b0f",
                        "detail": "auto"
                    }
                }
            ],
            "role": "user",
            "name": "John"
        }"#;
        let message: UserMessage = serde_json::from_str(json).unwrap();
        assert_eq!(
            message,
            UserMessage {
                content: MessageContent::Array(vec![
                    MessageContentPart::Text(TextContentPart::new(
                        "Hello, how are you?".to_string()
                    )),
                    MessageContentPart::Image(ImageContentPart::new(
                        ImageUrl::url(
                            "https://images.unsplash.com/photo-1622839418057-8e9e6b6b0b0f".to_string(),
                            Some(ImageDetail::Auto),
                        ),
                    )),
                ]),
                role: Role::User,
                name: Some("John".to_string()),
            }
        );
    }

    #[test]
    fn serialize_user_message_with_image_content_part() {
        let message = UserMessage {
            content: MessageContent::Array(vec![
                MessageContentPart::Text(TextContentPart::new(
                    "Hello, how are you?".to_string()
                )),
                MessageContentPart::Image(ImageContentPart::new(
                    ImageUrl::url(
                        "https://images.unsplash.com/photo-1622839418057-8e9e6b6b0b0f".to_string(),
                        Some(ImageDetail::Auto),
                    ),
                )),
            ]),
            role: Role::User,
            name: Some("John".to_string()),
        };
        let json = serde_json::to_string(&message).unwrap();
        assert_eq!(
            json,
            r#"{"content":[{"type":"text","text":"Hello, how are you?"},{"type":"image_url","image_url":{"url":"https://images.unsplash.com/photo-1622839418057-8e9e6b6b0b0f","detail":"auto"}}],"role":"user","name":"John"}"#,
        );
    }
}
