use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::chat::ToolType;
use crate::macros::{
    impl_display_for_serialize, impl_enum_string_serialization,
};

/// The tool choice.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolChoice {
    /// Options.
    Option(TooChoiceOption),
    /// Specified tool.
    Specified(SpecifiedTool),
}

impl Default for ToolChoice {
    fn default() -> Self {
        Self::Option(TooChoiceOption::default())
    }
}

impl_display_for_serialize!(ToolChoice);

/// The tool choice option.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TooChoiceOption {
    /// none, means the model will not call a function and instead generates a message.
    None,
    /// auto, means the model can pick between generating a message or calling a function.
    Auto,
}

impl Default for TooChoiceOption {
    fn default() -> Self {
        TooChoiceOption::Auto
    }
}

impl Display for TooChoiceOption {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | TooChoiceOption::None => {
                write!(f, "none")
            },
            | TooChoiceOption::Auto => {
                write!(f, "auto")
            },
        }
    }
}

impl_enum_string_serialization!(
    TooChoiceOption,
    None => "none",
    Auto => "auto"
);

impl From<TooChoiceOption> for ToolChoice {
    fn from(value: TooChoiceOption) -> Self {
        Self::Option(value)
    }
}

impl From<SpecifiedTool> for ToolChoice {
    fn from(value: SpecifiedTool) -> Self {
        Self::Specified(value)
    }
}

/// The specified tool.
#[derive(Debug, Clone, Eq, PartialEq, Default, Serialize, Deserialize)]
pub struct SpecifiedTool {
    /// The type of the tool. Currently, only function is supported.
    #[serde(rename = "type")]
    pub _type: ToolType,
    /// Specified function.
    pub function: SpecifiedFunction,
}

impl_display_for_serialize!(SpecifiedTool);

/// The specified function.
#[derive(Debug, Clone, Eq, PartialEq, Default, Serialize, Deserialize)]
pub struct SpecifiedFunction {
    /// The name of function.
    pub name: String,
}

impl_display_for_serialize!(SpecifiedFunction);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialize() {
        assert_eq!(
            serde_json::from_str::<ToolChoice>("\"none\"").unwrap(),
            ToolChoice::Option(TooChoiceOption::None)
        );

        assert_eq!(
            serde_json::from_str::<ToolChoice>("\"auto\"").unwrap(),
            ToolChoice::Option(TooChoiceOption::Auto)
        );

        assert_eq!(
            serde_json::from_str::<ToolChoice>(
                r#"{
                    "type": "function",
                    "function": {
                        "name": "my_function"
                    }
                }"#
            )
            .unwrap(),
            ToolChoice::Specified(SpecifiedTool {
                _type: ToolType::Function,
                function: SpecifiedFunction {
                    name: "my_function".to_string(),
                },
            })
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::to_string(&ToolChoice::Option(
                TooChoiceOption::None
            ))
            .unwrap(),
            "\"none\""
        );

        assert_eq!(
            serde_json::to_string(&ToolChoice::Option(
                TooChoiceOption::Auto
            ))
            .unwrap(),
            "\"auto\""
        );

        assert_eq!(
            serde_json::to_string(&ToolChoice::Specified(SpecifiedTool {
                _type: ToolType::Function,
                function: SpecifiedFunction {
                    name: "my_function".to_string(),
                },
            }))
            .unwrap(),
            r#"{"type":"function","function":{"name":"my_function"}}"#
        );
    }
}
