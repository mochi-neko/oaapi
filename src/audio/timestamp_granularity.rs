use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Timestamp granularity for the result of the transcription.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimestampGranularity {
    /// Segment-level timestamps.
    Segment,
    /// Word-level timestamps.
    Word,
}

impl Default for TimestampGranularity {
    fn default() -> Self {
        Self::Segment
    }
}

impl Display for TimestampGranularity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format())
    }
}

impl FromStr for TimestampGranularity {
    type Err = crate::ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "segment" => Ok(TimestampGranularity::Segment),
            "word" => Ok(TimestampGranularity::Word),
            _ => Err(crate::ValidationError {
                type_name: "TimestampGranularity".to_string(),
                reason: format!("Unknown timestamp granularity: {}", s),
            }),
        }
    }
}

impl TimestampGranularity {
    pub(crate) fn format(self) -> &'static str {
        match self {
            TimestampGranularity::Segment => "segment",
            TimestampGranularity::Word => "word",
        }
    }
}
