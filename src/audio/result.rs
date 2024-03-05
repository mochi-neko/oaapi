use bytes::Bytes;

/// The result of a speech stream.
pub type SpeechStreamResult =
    Result<Bytes, crate::audio::error::SpeechStreamError>;

/// The result of an audio API calling.
pub type AudioApiResult<T> = Result<T, crate::audio::error::AudioApiError>;
