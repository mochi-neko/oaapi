use crate::macros::impl_display_for_serialize;
use std::fmt::Display;

/// The error of the client API calling.
#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    /// HTTP request error of an API calling.
    #[error("HTTP request error: {0:?}")]
    HttpRequestError(reqwest::Error),
    /// Reading response text failed of an API calling.
    #[error("Reading response text failed: {0:?}")]
    ReadResponseTextFailed(reqwest::Error),
    /// Failed to deserialize response of an API calling.
    #[error("Failed to deserialize response as JSON: {error:?}, {text:?}")]
    ResponseDeserializationFailed {
        error: serde_json::Error,
        text: String,
    },
    /// Failed to deserialize response of an API calling.
    #[error(
        "Failed to deserialize error response as JSON: {error:?}, {text:?}"
    )]
    ErrorResponseDeserializationFailed {
        error: serde_json::Error,
        text: String,
    },
}

/// The error of an API.
#[derive(Debug, thiserror::Error)]
pub struct ApiError {
    /// The status code of the response.
    pub status_code: reqwest::StatusCode,
    /// The error response of the API calling.
    pub error_response: ErrorResponse,
}

impl Display for ApiError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "API error with status code: {}, error: {}",
            self.status_code, self.error_response,
        )
    }
}

/// The error response of an API calling.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ErrorResponse {
    #[serde(rename = "error")]
    pub error: ApiErrorBody,
}

impl_display_for_serialize!(ErrorResponse);

/// The error body of an API error response.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ApiErrorBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
    #[serde(rename = "type")]
    pub _type: String,
}

impl_display_for_serialize!(ApiErrorBody);

/// The error of a validation.
#[derive(Debug, thiserror::Error)]
pub struct ValidationError<T>
where
    T: Display,
{
    pub type_name: String,
    pub reason: String,
    pub value: T,
}

impl<T> Display for ValidationError<T>
where
    T: Display,
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "Validation error, type: {}, reason: {}, value: {}",
            self.type_name, self.reason, self.value,
        )
    }
}
