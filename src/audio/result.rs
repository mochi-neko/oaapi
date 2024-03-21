use bytes::Bytes;

use crate::audio::AudioApiError;
use crate::audio::TextFormatError;

/// The result of a speech stream.
pub type SpeechStreamResult = Result<Bytes, reqwest::Error>;

/// The result of an audio API calling.
pub type AudioApiResult<T> = Result<T, AudioApiError>;

/// The result of formatting a response text.
pub type TextFormatResult<T> = Result<T, TextFormatError>;
