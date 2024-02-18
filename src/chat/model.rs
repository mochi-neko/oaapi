use crate::macros::impl_enum_string_serialization;

/// The model to use for the chat.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ChatModel {
    // GPT-3.5-turbo models
    /// gpt-3.5-turbo-1106
    Gpt35Turbo1106,
    /// gpt-3.5-turbo-0613
    Gpt35Turbo0613,
    /// gpt-3.5-turbo
    Gpt35Turbo,
    /// gpt-3.5-turbo-16k
    Gpt35Turbo16k,
    /// gpt-3.5-turbo-instruct
    Gpt35TurboInstruct,

    // GPT-4 models
    /// gpt-4-0125-preview
    Gpt40125Preview,
    /// gpt-4-1106-vision-preview
    Gpt41106VisionPreview,
    /// gpt-4-1106-preview
    Gpt41106Preview,
    /// gpt-4-vision-preview
    Gpt4VisionPreview,
    /// gpt-4
    Gpt4,
    /// gpt-4-32k
    Gpt432k,
    /// gpt-4-0613
    Gpt40613,
    /// gpt-4-32k-0613
    Gpt432k0613,
}

impl ChatModel {
    pub fn context_window(&self) -> u32 {
        match self {
            | ChatModel::Gpt35Turbo1106 => 16358,
            | ChatModel::Gpt35Turbo0613 => 4096,
            | ChatModel::Gpt35Turbo => 4096,
            | ChatModel::Gpt35Turbo16k => 16358,
            | ChatModel::Gpt35TurboInstruct => 4096,
            | ChatModel::Gpt40125Preview => 8192,
            | ChatModel::Gpt41106VisionPreview => 128000,
            | ChatModel::Gpt41106Preview => 128000,
            | ChatModel::Gpt4VisionPreview => 128000,
            | ChatModel::Gpt4 => 8192,
            | ChatModel::Gpt432k => 32768,
            | ChatModel::Gpt40613 => 8192,
            | ChatModel::Gpt432k0613 => 32768,
        }
    }
}

impl_enum_string_serialization!(
    ChatModel,
    Gpt35Turbo1106 => "gpt-3.5-turbo-1106",
    Gpt35Turbo0613 => "gpt-3.5-turbo-0613",
    Gpt35Turbo => "gpt-3.5-turbo",
    Gpt35Turbo16k => "gpt-3.5-turbo-16k",
    Gpt35TurboInstruct => "gpt-3.5-turbo-instruct",
    Gpt40125Preview => "gpt-4-0125-preview",
    Gpt41106VisionPreview => "gpt-4-1106-vision-preview",
    Gpt41106Preview => "gpt-4-1106-preview",
    Gpt4VisionPreview => "gpt-4-vision-preview",
    Gpt4 => "gpt-4",
    Gpt432k => "gpt-4-32k",
    Gpt40613 => "gpt-4-0613",
    Gpt432k0613 => "gpt-4-32k-0613"
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialize_chat_model() {
        assert_eq!(
            serde_json::from_str::<ChatModel>("\"gpt-3.5-turbo-1106\"")
                .unwrap(),
            ChatModel::Gpt35Turbo1106
        );

        assert_eq!(
            serde_json::from_str::<ChatModel>("\"gpt-3.5-turbo-0613\"")
                .unwrap(),
            ChatModel::Gpt35Turbo0613
        );

        assert_eq!(
            serde_json::from_str::<ChatModel>("\"gpt-3.5-turbo\"").unwrap(),
            ChatModel::Gpt35Turbo
        );

        assert_eq!(
            serde_json::from_str::<ChatModel>("\"gpt-3.5-turbo-16k\"").unwrap(),
            ChatModel::Gpt35Turbo16k
        );

        assert_eq!(
            serde_json::from_str::<ChatModel>("\"gpt-3.5-turbo-instruct\"")
                .unwrap(),
            ChatModel::Gpt35TurboInstruct
        );

        assert_eq!(
            serde_json::from_str::<ChatModel>("\"gpt-4-0125-preview\"")
                .unwrap(),
            ChatModel::Gpt40125Preview
        );

        assert_eq!(
            serde_json::from_str::<ChatModel>("\"gpt-4-1106-vision-preview\"")
                .unwrap(),
            ChatModel::Gpt41106VisionPreview
        );

        assert_eq!(
            serde_json::from_str::<ChatModel>("\"gpt-4-1106-preview\"")
                .unwrap(),
            ChatModel::Gpt41106Preview
        );

        assert_eq!(
            serde_json::from_str::<ChatModel>("\"gpt-4-vision-preview\"")
                .unwrap(),
            ChatModel::Gpt4VisionPreview
        );

        assert_eq!(
            serde_json::from_str::<ChatModel>("\"gpt-4\"").unwrap(),
            ChatModel::Gpt4
        );

        assert_eq!(
            serde_json::from_str::<ChatModel>("\"gpt-4-32k\"").unwrap(),
            ChatModel::Gpt432k
        );

        assert_eq!(
            serde_json::from_str::<ChatModel>("\"gpt-4-0613\"").unwrap(),
            ChatModel::Gpt40613
        );

        assert_eq!(
            serde_json::from_str::<ChatModel>("\"gpt-4-32k-0613\"").unwrap(),
            ChatModel::Gpt432k0613
        );
    }

    #[test]
    fn serialize_chat_model() {
        assert_eq!(
            serde_json::to_string(&ChatModel::Gpt35Turbo1106).unwrap(),
            "\"gpt-3.5-turbo-1106\""
        );

        assert_eq!(
            serde_json::to_string(&ChatModel::Gpt35Turbo0613).unwrap(),
            "\"gpt-3.5-turbo-0613\""
        );

        assert_eq!(
            serde_json::to_string(&ChatModel::Gpt35Turbo).unwrap(),
            "\"gpt-3.5-turbo\""
        );

        assert_eq!(
            serde_json::to_string(&ChatModel::Gpt35Turbo16k).unwrap(),
            "\"gpt-3.5-turbo-16k\""
        );

        assert_eq!(
            serde_json::to_string(&ChatModel::Gpt35TurboInstruct).unwrap(),
            "\"gpt-3.5-turbo-instruct\""
        );

        assert_eq!(
            serde_json::to_string(&ChatModel::Gpt40125Preview).unwrap(),
            "\"gpt-4-0125-preview\""
        );

        assert_eq!(
            serde_json::to_string(&ChatModel::Gpt41106VisionPreview).unwrap(),
            "\"gpt-4-1106-vision-preview\""
        );

        assert_eq!(
            serde_json::to_string(&ChatModel::Gpt41106Preview).unwrap(),
            "\"gpt-4-1106-preview\""
        );

        assert_eq!(
            serde_json::to_string(&ChatModel::Gpt4VisionPreview).unwrap(),
            "\"gpt-4-vision-preview\""
        );

        assert_eq!(
            serde_json::to_string(&ChatModel::Gpt4).unwrap(),
            "\"gpt-4\""
        );

        assert_eq!(
            serde_json::to_string(&ChatModel::Gpt432k).unwrap(),
            "\"gpt-4-32k\""
        );

        assert_eq!(
            serde_json::to_string(&ChatModel::Gpt40613).unwrap(),
            "\"gpt-4-0613\""
        );

        assert_eq!(
            serde_json::to_string(&ChatModel::Gpt432k0613).unwrap(),
            "\"gpt-4-32k-0613\""
        );
    }
}
