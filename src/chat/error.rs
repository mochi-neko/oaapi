use crate::ApiError;

/// The error of a chat stream.
#[derive(Debug, thiserror::Error)]
pub enum ChatStreamError {
    /// Failed to receive chunk.
    #[error("Failed to receive chunk: {0:?}")]
    ErrorChunk(reqwest::Error),
    /// Failed to deserialize chunk.
    #[error("Failed to deserialize chunk: {0:?}, {1}")]
    DeserializeFailed(serde_json::Error, String),
}

/// The error of a chat API calling.
#[derive(Debug, thiserror::Error)]
pub enum ChatApiError {
    /// API error of an API calling.
    #[error("API error: {0:?}")]
    ApiError(ApiError),
    /// Stream option mismatch.
    #[error("Stream option mismatch")]
    StreamOptionMismatch,
}

impl From<ApiError> for ChatApiError {
    fn from(error: ApiError) -> Self {
        Self::ApiError(error)
    }
}
