use serde::{Deserialize, Serialize};

use crate::chat::Role;
use crate::macros::impl_display_for_serialize;
use crate::Prompt;

/// A system message.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct SystemMessage {
    /// The contents of the system message.
    content: String,
    /// The role of the messages author, in this case system.
    role: Role,
    /// An optional name for the participant.
    /// Provides the model information to differentiate between participants of the same role.
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

impl Default for SystemMessage {
    fn default() -> Self {
        Self {
            content: "".to_string(),
            role: Role::System,
            name: None,
        }
    }
}

impl_display_for_serialize!(SystemMessage);

impl SystemMessage {
    pub fn new<S>(
        content: S,
        name: Option<String>,
    ) -> Self
    where
        S: Into<String>,
    {
        Self {
            content: content.into(),
            role: Role::System,
            name,
        }
    }

    pub fn from_prompt(
        prompt: Prompt,
        name: Option<String>,
    ) -> Self {
        Self {
            content: prompt.format(),
            role: Role::System,
            name,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialize_system_message() {
        let json = r#"{
            "content": "Hello, world!",
            "role": "system",
            "name": "System"
        }"#;
        let message: SystemMessage = serde_json::from_str(json).unwrap();
        assert_eq!(
            message,
            SystemMessage::new(
                "Hello, world!".to_string(),
                Some("System".to_string()),
            ),
        );
    }

    #[test]
    fn deserialize_system_message_without_optional() {
        let json = r#"{
            "content": "Hello, world!",
            "role": "system"
        }"#;
        let message: SystemMessage = serde_json::from_str(json).unwrap();
        assert_eq!(
            message,
            SystemMessage::new("Hello, world!".to_string(), None,),
        );
    }

    #[test]
    fn serialize_system_message() {
        let message = SystemMessage::new(
            "Hello, world!".to_string(),
            Some("System".to_string()),
        );
        let json = serde_json::to_string(&message).unwrap();
        assert_eq!(
            json,
            r#"{"content":"Hello, world!","role":"system","name":"System"}"#,
        );
    }

    #[test]
    fn serialize_system_message_without_optional() {
        let message = SystemMessage::new("Hello, world!".to_string(), None);
        let json = serde_json::to_string(&message).unwrap();
        assert_eq!(
            json,
            r#"{"content":"Hello, world!","role":"system"}"#,
        );
    }
}
