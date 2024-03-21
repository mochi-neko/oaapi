use subtp::ParseError;

use crate::ApiError;
use crate::ClientError;

/// The error of an audio API calling.
#[derive(Debug, thiserror::Error)]
pub enum AudioApiError {
    /// Client error of an API calling.
    #[error("Client error: {0:?}")]
    ClientError(#[from] ClientError),
    /// API error of an API calling.
    #[error("API error: {0:?}")]
    ApiError(#[from] ApiError),
    /// Failed to format response text of an audio API calling.
    #[error("Failed to format response text of audio API: {0:?}")]
    FormatResponseFailed(#[from] TextFormatError),
    /// Timestamp option mismatch.
    #[error("Stream option mismatch, this is only available for verbose_json response format.")]
    TimestampOptionMismatch,
}

/// The error of formatting a response text.
#[derive(Debug, thiserror::Error)]
pub enum TextFormatError {
    /// The error of formatting a response text into JSON.
    #[error("Failed to format into JSON: {error:?}, {text}")]
    FormatJsonFailed {
        /// The error of deserializing a response text.
        error: serde_json::Error,
        /// The raw response text.
        text: String,
    },
    /// The error of formatting a response text into SubRip Subtitle format.
    #[error("Failed to parse into SubRip Subtitle format: {error:?}, {text}")]
    ParseSrtFailed {
        /// The error of parsing a response text.
        error: ParseError,
        /// The raw response text.
        text: String,
    },
    /// The error of formatting a response text into WebVTT format.
    #[error("Failed to parse into WebVTT format: {error:?}, {text}")]
    ParseVttFailed {
        /// The error of parsing a response text.
        error: ParseError,
        /// The raw response text.
        text: String,
    },
}
