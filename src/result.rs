/// The result of an API calling.
pub type ApiResult<T> = Result<T, crate::error::ApiError>;

/// The result of a validation.
pub type ValidationResult<T, S> = Result<T, crate::error::ValidationError<S>>;
