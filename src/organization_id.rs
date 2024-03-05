use std::env::VarError;

/// The organization ID of the OpenAI API.
#[derive(Clone, PartialEq, Eq)]
pub struct OrganizationId {
    value: String,
}

impl OrganizationId {
    /// Creates a new organization ID.
    pub fn new<S>(value: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            value: value.into(),
        }
    }

    /// Loads the organization ID from the environment variable: `OPENAI_ORG_ID`.
    pub fn from_env() -> Result<Self, VarError> {
        let key = std::env::var("OPENAI_ORG_ID")?;

        Ok(Self {
            value: key,
        })
    }

    /// Returns the HTTP header value.
    pub(crate) fn organization_header(&self) -> String {
        self.value.clone()
    }
}
