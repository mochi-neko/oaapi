use std::fmt::Display;

/// The error of an API calling.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// IO error with an API calling.
    #[error("IO error: {0}")]
    IOError(std::io::Error),
    /// HTTP request error of an API calling.
    #[error("HTTP request error: {0}")]
    HttpRequestError(reqwest::Error),
    /// Reading response text failed of an API calling.
    #[error("Reading response text failed: {0}")]
    ReadResponseTextFailed(reqwest::Error),
    /// Failed to format response of an audio API calling.
    #[error("Failed to format response of audio: {0:?}")]
    FormatResponseFailed(crate::audio::TextFormatError),
    /// Failed to deserialize response of an API calling.
    #[error("Failed to deserialize response as JSON: {error:?}, {text}")]
    ResponseDeserializationFailed {
        error: serde_json::Error,
        text: String,
    },
    /// Failed to deserialize response of an API calling.
    #[error("Failed to deserialize error response as JSON: {error:?}, {text}")]
    ErrorResponseDeserializationFailed {
        error: serde_json::Error,
        text: String,
    },
    /// API error of an API calling.
    #[error("API error: {status_code}, {error_response:?}")]
    ApiError {
        status_code: reqwest::StatusCode,
        error_response: ApiError,
    },
    /// Stream option mismatch.
    #[error("Stream option mismatch")]
    StreamOptionMismatch,
    /// Timestamp option mismatch.
    #[error("Stream option mismatch, this is only available for verbose_json response format.")]
    TimestampOptionMismatch,
}

/// The error format of an API error.
#[derive(serde::Deserialize, Debug)]
pub struct ApiError {
    #[serde(rename = "error")]
    pub error: ApiErrorBody,
}

/// The error body of an API error.
#[derive(serde::Deserialize, Debug)]
pub struct ApiErrorBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
    #[serde(rename = "type")]
    pub _type: String,
}

/// The error of a validation.
#[derive(Debug, thiserror::Error)]
pub struct ValidationError {
    pub type_name: String,
    pub reason: String,
}

impl Display for ValidationError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "Validation error, type: {}, reason: {}",
            self.type_name, self.reason
        )
    }
}

