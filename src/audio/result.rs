use bytes::Bytes;

use crate::audio::error::AudioApiError;

/// The result of a speech stream.
pub type SpeechStreamResult = Result<Bytes, reqwest::Error>;

/// The result of an audio API calling.
pub type AudioApiResult<T> = Result<T, AudioApiError>;
