use std::fmt::Display;
use crate::macros::impl_enum_string_serialization;

/// The role of the messages author.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Role {
    /// system
    System,
    /// user
    User,
    /// assistant
    Assistant,
    /// tool
    Tool,
}

impl Default for Role {
    fn default() -> Self {
        Role::User
    }
}

impl Display for Role {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | Role::System => {
                write!(f, "system")
            },
            | Role::User => {
                write!(f, "user")
            },
            | Role::Assistant => {
                write!(f, "assistant")
            },
            | Role::Tool => {
                write!(f, "tool")
            },
        }
    }
}

impl_enum_string_serialization!(
    Role,
    System => "system",
    User => "user",
    Assistant => "assistant",
    Tool => "tool"
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn role_serialization() {
        assert_eq!(
            serde_json::to_string(&Role::System).unwrap(),
            "\"system\""
        );

        assert_eq!(
            serde_json::to_string(&Role::User).unwrap(),
            "\"user\""
        );

        assert_eq!(
            serde_json::to_string(&Role::Assistant).unwrap(),
            "\"assistant\""
        );

        assert_eq!(
            serde_json::to_string(&Role::Tool).unwrap(),
            "\"tool\""
        );
    }

    #[test]
    fn role_deserialization() {
        assert_eq!(
            serde_json::from_str::<Role>("\"user\"").unwrap(),
            Role::User
        );

        assert_eq!(
            serde_json::from_str::<Role>("\"system\"").unwrap(),
            Role::System
        );

        assert_eq!(
            serde_json::from_str::<Role>("\"assistant\"").unwrap(),
            Role::Assistant
        );

        assert_eq!(
            serde_json::from_str::<Role>("\"tool\"").unwrap(),
            Role::Tool
        );
    }
}
