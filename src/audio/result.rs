use bytes::Bytes;
use crate::audio::error::{AudioApiError, SpeechStreamError};

/// The result of a speech stream.
pub type SpeechStreamResult =
    Result<Bytes, SpeechStreamError>;

/// The result of an audio API calling.
pub type AudioApiResult<T> = Result<T, AudioApiError>;
