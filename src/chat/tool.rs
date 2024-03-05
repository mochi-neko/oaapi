use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::macros::{
    impl_display_for_serialize, impl_enum_string_serialization,
};

/// Tool that the model may call.
#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Tool {
    /// The type of the tool. Currently, only function is supported.
    #[serde(rename = "type")]
    pub _type: ToolType,
    /// Function.
    pub function: Function,
}

impl_display_for_serialize!(Tool);

impl Tool {
    pub fn new(function: Function) -> Self {
        Self {
            _type: ToolType::Function,
            function,
        }
    }
}

impl From<Function> for Tool {
    fn from(value: Function) -> Self {
        Self::new(value)
    }
}

/// The type of the tool.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ToolType {
    /// Function.
    Function,
}

impl Default for ToolType {
    fn default() -> Self {
        ToolType::Function
    }
}

impl Display for ToolType {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | ToolType::Function => {
                write!(f, "function")
            },
        }
    }
}

impl_enum_string_serialization!(
    ToolType,
    Function => "function"
);

/// Function tha the model may call by tool.
#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Function {
    /// A description of what the function does, used by the model to choose when and how to call the function.
    pub description: Option<String>,
    /// The name of the function to be called. Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum length of 64.
    pub name: String,
    /// The parameters the functions accepts, described as a JSON Schema object. See the guide for examples, and the JSON Schema reference for documentation about the format.
    ///
    /// Omitting parameters defines a function with an empty parameter list.
    pub parameters: Option<serde_json::Map<String, serde_json::Value>>,
}

impl_display_for_serialize!(Function);
