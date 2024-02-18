use crate::audio::error::SpeechStreamError;
use bytes::Bytes;

/// The result of a speech stream.
pub type SpeechStreamResult = Result<Bytes, SpeechStreamError>;
