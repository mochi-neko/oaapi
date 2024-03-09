use crate::ApiError;

/// The error of a chat stream.
#[derive(Debug, thiserror::Error)]
pub enum ChatStreamError {
    /// Failed to read stream line.
    #[error("Failed to read stream line: {0:?}")]
    ErrorChunk(#[from] StreamLineError),
    /// Failed to deserialize chunk.
    #[error("Failed to deserialize chunk: {0:?}, {1}")]
    DeserializeFailed(serde_json::Error, String),
}

/// The error of a chat API calling.
#[derive(Debug, thiserror::Error)]
pub enum ChatApiError {
    /// API error of an API calling.
    #[error("API error: {0:?}")]
    ApiError(#[from] ApiError),
    /// Stream option mismatch.
    #[error("Stream option mismatch")]
    StreamOptionMismatch,
}

/// The error of a stream line.
#[derive(Debug, thiserror::Error)]
pub enum StreamLineError {
    #[error("Failed to read reqest stream: {0:?}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Failed to deserialize chunk to UTF-8 string: {0:?}")]
    StringDeserializationError(#[from] std::string::FromUtf8Error),
}
