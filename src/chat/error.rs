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
