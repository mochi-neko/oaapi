use crate::macros::impl_enum_bool_serialization;
use std::fmt::Display;

/// The stream option.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StreamOption {
    /// Return once, not stream. "false"
    ReturnOnce,
    /// Return stream. "true"
    ReturnStream,
}

impl Default for StreamOption {
    fn default() -> Self {
        Self::ReturnOnce
    }
}

impl Display for StreamOption {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | StreamOption::ReturnOnce => {
                write!(f, "false")
            },
            | StreamOption::ReturnStream => {
                write!(f, "true")
            },
        }
    }
}

impl_enum_bool_serialization!(StreamOption, ReturnStream, ReturnOnce);
