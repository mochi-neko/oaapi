/// Timestamp granularity for the result of the transcription.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimestampGranularity {
    /// Segment-level timestamps.
    Segment,
    /// Word-level timestamps.
    Word,
}

impl TimestampGranularity {
    pub(crate) fn format(&self) -> &str {
        match self {
            TimestampGranularity::Segment => "segment",
            TimestampGranularity::Word => "word",
        }
    }
}
