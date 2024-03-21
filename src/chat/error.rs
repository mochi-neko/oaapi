use crate::ApiError;
use crate::ClientError;

/// The error of a chat API calling.
#[derive(Debug, thiserror::Error)]
pub enum ChatApiError {
    /// Client error of an API calling.
    #[error("Client error: {0:?}")]
    ClientError(#[from] ClientError),
    /// API error of an API calling.
    #[error("API error: {0:?}")]
    ApiError(#[from] ApiError),
    /// Stream option mismatch.
    #[error("Stream option mismatch")]
    StreamOptionMismatch,
}

/// The error of a chunk of chat stream.
#[derive(Debug, thiserror::Error)]
pub enum ChatChunkError {
    /// Stream error.
    #[error("Stream error: {0:?}")]
    StreamError(#[from] reqwest::Error),
    /// Failed to decode chunk of stream to UTF-8 string.
    #[error("Failed to decode chunk of stream to UTF-8 string: {0:?}")]
    StringDecodingError(#[from] std::string::FromUtf8Error),
    /// Data prefix missing.
    #[error("Data prefix missing: {0}")]
    DataPrefixMissing(String),
    /// Failed to deserialize chunk.
    #[error("Failed to deserialize chunk: {0:?} from: {1}")]
    DeserializeFailed(serde_json::Error, String),
}
