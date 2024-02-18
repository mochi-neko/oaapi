/// The result of an API calling.
pub type Result<T> = std::result::Result<T, crate::error::Error>;

/// The result of a validation.
pub type ValidationResult<T> = std::result::Result<T, crate::error::ValidationError>;