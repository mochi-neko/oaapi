use serde::{Deserialize, Serialize};

use crate::chat::Role;
use crate::macros::impl_display_for_serialize;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ToolMessage {
    /// The role of the messages author, in this case tool.
    pub content: String,
    /// The contents of the tool message.
    pub role: Role,
    /// Tool call that this message is responding to.
    pub tool_call_id: String,
}

impl Default for ToolMessage {
    fn default() -> Self {
        Self {
            content: "".to_string(),
            role: Role::Tool,
            tool_call_id: "".to_string(),
        }
    }
}

impl_display_for_serialize!(ToolMessage);

impl ToolMessage {
    pub fn new(
        content: String,
        tool_call_id: String,
    ) -> Self {
        Self {
            content,
            role: Role::Tool,
            tool_call_id,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialize_tool_message() {
        let json = r#"{
            "content": "Hello, world!",
            "role": "tool",
            "tool_call_id": "tool-call-id"
        }"#;
        let message: ToolMessage = serde_json::from_str(json).unwrap();
        assert_eq!(
            message,
            ToolMessage::new(
                "Hello, world!".to_string(),
                "tool-call-id".to_string(),
            ),
        );
    }

    #[test]
    fn serialize_tool_message() {
        let message = ToolMessage::new(
            "Hello, world!".to_string(),
            "tool-call-id".to_string(),
        );
        let serialized = serde_json::to_string(&message).unwrap();
        assert_eq!(
            serialized,
            "{\"content\":\"Hello, world!\",\"role\":\"tool\",\"tool_call_id\":\"tool-call-id\"}"
        );
    }
}
