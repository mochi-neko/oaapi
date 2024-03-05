use std::env::VarError;

/// The API key of the OpenAI API.
#[derive(Clone, PartialEq, Eq)]
pub struct ApiKey {
    value: String,
}

impl ApiKey {
    /// Creates a new API key.
    pub fn new<S>(value: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            value: value.into(),
        }
    }

    /// Loads the API key from the environment variable: `OPENAI_API_KEY`.
    pub fn from_env() -> Result<Self, VarError> {
        let key = std::env::var("OPENAI_API_KEY")?;

        Ok(Self {
            value: key,
        })
    }

    /// Returns the HTTP authorization header value.
    pub(crate) fn authorization_header(&self) -> String {
        format!("Bearer {}", self.value)
    }
}
