use crate::audio::TextFormatError;
use crate::ApiError;

/// The error of an audio API calling.
#[derive(Debug, thiserror::Error)]
pub enum AudioApiError {
    /// API error of an API calling.
    #[error("API error: {0:?}")]
    ApiError(#[from] ApiError),
    /// IO error with an API calling.
    #[error("IO error: {0:?}")]
    IOError(#[from] std::io::Error),
    /// Failed to format response text of an audio API calling.
    #[error("Failed to format response text of audio API: {0:?}")]
    FormatResponseFailed(#[from] TextFormatError),
    /// Timestamp option mismatch.
    #[error("Stream option mismatch, this is only available for verbose_json response format.")]
    TimestampOptionMismatch,
}
