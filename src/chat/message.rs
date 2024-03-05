use crate::chat::AssistantMessage;
use crate::chat::SystemMessage;
use crate::chat::ToolMessage;
use crate::chat::UserMessage;
use crate::macros::{
    impl_display_for_serialize, impl_enum_struct_serialization,
};

/// A message of chat.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Message {
    /// System message.
    System(SystemMessage),
    /// User message.
    User(UserMessage),
    /// Assistant message.
    Assistant(AssistantMessage),
    /// Tool message.
    Tool(ToolMessage),
}

impl Default for Message {
    fn default() -> Self {
        Self::User(UserMessage::default())
    }
}

impl_display_for_serialize!(Message);

impl_enum_struct_serialization!(
    Message,
    role,
    System(SystemMessage, "system"),
    User(UserMessage, "user"),
    Assistant(AssistantMessage, "assistant"),
    Tool(ToolMessage, "tool")
);

#[cfg(test)]
mod tests {
    use serde_json;

    use crate::chat::MessageContent;

    use super::*;

    #[test]
    fn deserialize_system_message() {
        let json = r#"
        {
            "role": "system",
            "content": "System message"
        }"#;

        let deserialized = serde_json::from_str::<Message>(json).unwrap();
        assert_eq!(
            deserialized,
            Message::System(SystemMessage::new(
                "System message".to_string(),
                None,
            ))
        );

        let serialized = serde_json::to_string(&deserialized).unwrap();
        assert_eq!(
            serialized,
            "{\"content\":\"System message\",\"role\":\"system\"}"
        );
    }

    #[test]
    fn deserialize_user_message() {
        let json = r#"
        {
            "role": "user",
            "content": "User message"
        }"#;

        let deserialized = serde_json::from_str::<Message>(json).unwrap();
        assert_eq!(
            deserialized,
            Message::User(UserMessage::new(
                MessageContent::Text("User message".to_string()),
                None,
            ))
        );

        let serialized = serde_json::to_string(&deserialized).unwrap();
        assert_eq!(
            serialized,
            "{\"content\":\"User message\",\"role\":\"user\"}"
        );
    }

    #[test]
    fn deserialize_assistant_message() {
        let json = r#"
        {
            "role": "assistant"
        }"#;

        let deserialized = serde_json::from_str::<Message>(json).unwrap();
        assert_eq!(
            deserialized,
            Message::Assistant(AssistantMessage::new(None, None, None))
        );

        let serialized = serde_json::to_string(&deserialized).unwrap();
        assert_eq!(serialized, "{\"role\":\"assistant\"}");
    }

    #[test]
    fn deserialize_tool_message() {
        let json = r#"
        {
            "role": "tool",
            "content": "Tool message",
            "tool_call_id": "tool_call_id"
        }"#;

        let deserialized = serde_json::from_str::<Message>(json).unwrap();
        assert_eq!(
            deserialized,
            Message::Tool(ToolMessage::new(
                "Tool message".to_string(),
                "tool_call_id".to_string(),
            ))
        );

        let serialized = serde_json::to_string(&deserialized).unwrap();
        assert_eq!(serialized, "{\"content\":\"Tool message\",\"role\":\"tool\",\"tool_call_id\":\"tool_call_id\"}");
    }
}
