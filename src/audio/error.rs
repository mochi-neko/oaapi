/// The error of a speech stream.
#[derive(Debug, thiserror::Error)]
pub enum SpeechStreamError {
    /// Failed to receive chunk.
    #[error("Failed to receive chunk: {0:?}")]
    ErrorChunk(reqwest::Error),
}
