//! The audio API of the OpenAI API.
//!
//! ## NOTE
//! This is only available for `audio` feature flag.
//!
//! ## Supported APIs
//! - [x] [speech](https://platform.openai.com/docs/api-reference/audio/createSpeech)
//! - [x] [transcriptions](https://platform.openai.com/docs/api-reference/audio/createTranscription)
//! - [x] [translations](https://platform.openai.com/docs/api-reference/audio/createTranslation)
//!
//! ## Supported text response formats
//! - [x] Plain text
//! - [x] JSON
//! - [x] Verbose JSON
//! - [x] SubRip Subtitle
//! - [x] WebVTT

pub use api::speech::SpeechRequestBody;
pub use api::transcriptions::TranscriptionsRequestBody;
pub use api::translations::TranslationsRequestBody;
pub use error::AudioApiError;
pub use error::SpeechStreamError;
pub use file::File;
pub use language::Iso639_1;
pub use model::AudioModel;
pub use model::SpeechModel;
pub use response_format::JsonResponse;
pub use response_format::JsonResponseFormatter;
pub use response_format::PlainTextResponseFormatter;
pub use response_format::SpeechResponseFormat;
pub use response_format::SrtResponseFormatter;
pub use response_format::TextFormatError;
pub use response_format::TextResponseFormat;
pub use response_format::TextResponseFormatter;
pub use response_format::VerboseJsonResponse;
pub use response_format::VerboseJsonResponseFormatter;
pub use response_format::VttResponseFormatter;
pub use result::AudioApiResult;
pub use result::SpeechStreamResult;
pub use speech_input::SpeechInput;
pub use speed::Speed;
pub use timestamp_granularity::TimestampGranularity;
pub use voice::Voice;

pub(crate) use api::speech::speech;
pub(crate) use api::transcriptions::transcribe_into_json;
pub(crate) use api::transcriptions::transcribe_into_plain_text;
pub(crate) use api::transcriptions::transcribe_into_srt;
pub(crate) use api::transcriptions::transcribe_into_verbose_json;
pub(crate) use api::transcriptions::transcribe_into_vtt;
pub(crate) use api::translations::translate_into_json;
pub(crate) use api::translations::translate_into_plain_text;
pub(crate) use api::translations::translate_into_srt;
pub(crate) use api::translations::translate_into_verbose_json;
pub(crate) use api::translations::translate_into_vtt;

mod api;
mod error;
mod file;
mod language;
mod model;
mod response_format;
mod result;
mod speech_input;
mod speed;
mod timestamp_granularity;
mod voice;
