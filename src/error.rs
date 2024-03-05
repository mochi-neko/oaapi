use std::fmt::Display;

/// The error of an API calling.
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
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
    /// API response error of an API calling.
    #[error("API error: {status_code:?}, {error_response:?}")]
    ApiResponseError {
        status_code: reqwest::StatusCode,
        error_response: ErrorResponse,
    },
}

/// The error response of an API calling.
#[derive(serde::Deserialize, Debug)]
pub struct ErrorResponse {
    #[serde(rename = "error")]
    pub error: ApiErrorBody,
}

/// The error body of an API error response.
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
