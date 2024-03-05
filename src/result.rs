/// The result of an API calling.
pub type ApiResult<T> = std::result::Result<T, crate::error::ApiError>;

/// The result of a validation.
pub type ValidationResult<T> =
    std::result::Result<T, crate::error::ValidationError>;
