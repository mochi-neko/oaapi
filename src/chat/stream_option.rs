use crate::macros::impl_enum_bool_serialization;

/// Stream option.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StreamOption {
    /// Return once, not stream. "false"
    ReturnOnce,
    /// Return stream. "true"
    ReturnStream,
}

impl_enum_bool_serialization!(StreamOption, ReturnOnce, ReturnStream);
