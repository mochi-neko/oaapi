use crate::macros::impl_enum_bool_serialization;
use std::fmt::Display;

/// The logprobs option.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LogprobsOption {
    /// Do not return logprobs. "false"
    NotReturnLogprobs,
    /// Return logprobs. "true"
    ReturnLogprobs,
}

impl Default for LogprobsOption {
    fn default() -> Self {
        LogprobsOption::NotReturnLogprobs
    }
}

impl Display for LogprobsOption {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | LogprobsOption::NotReturnLogprobs => {
                write!(f, "false")
            },
            | LogprobsOption::ReturnLogprobs => {
                write!(f, "true")
            },
        }
    }
}

impl_enum_bool_serialization!(
    LogprobsOption,
    ReturnLogprobs,
    NotReturnLogprobs
);
