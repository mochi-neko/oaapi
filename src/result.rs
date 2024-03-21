/// The result of a validation.
pub type ValidationResult<T, S> = Result<T, crate::error::ValidationError<S>>;
