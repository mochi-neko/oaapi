use crate::macros::impl_enum_with_string_or_array_serialization;
use crate::ValidationResult;
use std::fmt::Display;

/// Stop sequence(s) option.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum StopOption {
    /// Single sequence.
    SingleSequence(String),
    /// Up to 4 sequences.
    UpTo4Sequences(Vec<String>),
}

impl Default for StopOption {
    fn default() -> Self {
        Self::SingleSequence(String::new())
    }
}

impl Display for StopOption {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | StopOption::SingleSequence(s) => {
                write!(f, "{}", s)
            },
            | StopOption::UpTo4Sequences(s) => {
                write!(f, "[")?;
                for (i, s) in s.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", s)?;
                }
                write!(f, "]")
            },
        }
    }
}

impl StopOption {
    pub fn new_single<S: Into<String>>(sequence: S) -> Self {
        Self::SingleSequence(sequence.into())
    }

    /// Creates a new stop option with up to 4 sequences.
    ///
    /// ## Error
    /// - [`ValidationError`] - If the number of sequences is greater than 4.
    pub fn new_up_to_4<S: Into<String>>(
        sequences: Vec<S>
    ) -> ValidationResult<Self> {
        if sequences.len() > 4 {
            Err(crate::ValidationError {
                type_name: "StopOption".to_string(),
                reason: format!(
                    "The number of sequences must be less than or equal to 4, but got {}.",
                    sequences.len()
                ),
            })
        } else {
            Ok(Self::UpTo4Sequences(
                sequences
                    .into_iter()
                    .map(|s| s.into())
                    .collect(),
            ))
        }
    }
}

impl_enum_with_string_or_array_serialization!(
    StopOption,
    SingleSequence(String),
    UpTo4Sequences(String)
);
