use crate::macros::impl_enum_bool_serialization;

/// Logprobs option.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LogprobsOption {
    /// Do not return logprobs. "false"
    NotReturnLogprobs,
    /// Return logprobs. "true"
    ReturnLogprobs,
}

impl_enum_bool_serialization!(
    LogprobsOption,
    ReturnLogprobs,
    NotReturnLogprobs
);
