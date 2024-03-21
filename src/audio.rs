//! The audio API of the OpenAI API.
//!
//! ## NOTE
//! This is only available for the `audio` feature flag.
//!
//! ## Supported APIs
//! - [x] [Speech](https://platform.openai.com/docs/api-reference/audio/createSpeech)
//! - [x] [Transcriptions](https://platform.openai.com/docs/api-reference/audio/createTranscription)
//! - [x] [Translations](https://platform.openai.com/docs/api-reference/audio/createTranslation)
//!
//! ## Supported text response formats
//! - [x] Plain text
//! - [x] JSON
//! - [x] Verbose JSON
//! - [x] SubRip Subtitle
//! - [x] WebVTT
//!
//! ## Examples
//!
//! ### Speech
//! An example to call the speech API with the `audio` feature flag, `tokio`, `anyhow` and `tokio_stream` crate is as follows:
//!
//! ```no_run
//! use oaapi::Client;
//! use oaapi::audio::SpeechRequestBody;
//! use oaapi::audio::SpeechInput;
//! use oaapi::audio::Voice;
//!
//! use tokio_stream::StreamExt;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // 1. Create a client with the API key from the environment variable: "OPENAI_API_KEY"
//!     let client = Client::from_env()?;
//!     // or specify the API key directly.
//!     // let client = Client::new(oaapi::ApiKey::new("OPENAI_API_KEY"), None, None);
//!
//!     // 2. Create a request body parameters.
//!     let request_body = SpeechRequestBody {
//!         input: SpeechInput::new("Text to speech.")?,
//!         voice: Voice::Alloy,
//!         ..Default::default()
//!     };
//!
//!     // 3. Call the API.
//!     let mut stream = client
//!         .audio_speech(request_body)
//!         .await?;
//!
//!     // 4. Read the stream of the speech data.
//!     while let Some(chunk) = stream.next().await {
//!         // Do something with the chunk.
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Transcriptions
//! An example to call the transcriptions API with the `audio` feature flag, `tokio` and `anyhow` crate is as follows:
//!
//! ```no_run
//! use oaapi::Client;
//! use oaapi::audio::File;
//! use oaapi::audio::TranscriptionsRequestBody;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // 1. Create a client with the API key from the environment variable: "OPENAI_API_KEY"
//!     let client = Client::from_env()?;
//!     // or specify the API key directly.
//!     // let client = Client::new(oaapi::ApiKey::new("OPENAI_API_KEY"), None, None);
//!
//!     // 2. Load the audio file that you want to transcribe.
//!     let file_path = "path/to/audio/file.mp3";
//!     let file = tokio::fs::read(file_path).await?;
//!     let file = File::new(file_path, file)?;
//!
//!     // 3. Create a request body parameters.
//!     let request_body = TranscriptionsRequestBody {
//!         file,
//!         ..Default::default()
//!     };
//!
//!     // 4. Call the API with specifying the response format.
//!     let response = client
//!         .audio_transcribe_into_json(request_body)
//!         .await?;
//!
//!     // 5. Use the response.
//!     println!("Result:\n{}", response);
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Translations
//! An example to call the translations API with the `audio` feature flag, `tokio` and `anyhow` crate is as follows:
//!
//! ```no_run
//! use oaapi::Client;
//! use oaapi::audio::File;
//! use oaapi::audio::TranslationsRequestBody;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // 1. Create a client with the API key from the environment variable: "OPENAI_API_KEY"
//!     let client = Client::from_env()?;
//!     // or specify the API key directly.
//!     // let client = Client::new(oaapi::ApiKey::new("OPENAI_API_KEY"), None, None);
//!
//!     // 2. Load the audio file that you want to translate.
//!     let file_path = "path/to/audio/file.mp3";
//!     let file = tokio::fs::read(file_path).await?;
//!     let file = File::new(file_path, file)?;
//!
//!     // 3. Create a request body parameters.
//!     let request_body = TranslationsRequestBody {
//!         file,
//!         ..Default::default()
//!     };
//!
//!     // 4. Call the API with specifying the response format.
//!     let response = client
//!         .audio_translate_into_verbose_json(request_body)
//!         .await?;
//!
//!     // 5. Use the response.
//!     println!("Result:\n{}", response);
//!
//!     Ok(())
//! }
//! ```

pub use api::speech::SpeechRequestBody;
pub use api::transcriptions::TranscriptionsRequestBody;
pub use api::translations::TranslationsRequestBody;
pub use error::AudioApiError;
pub use error::TextFormatError;
pub use file::File;
pub use language::Iso639_1;
pub use model::AudioModel;
pub use model::SpeechModel;
pub use response_format::JsonResponse;
pub use response_format::JsonResponseFormatter;
pub use response_format::PlainTextResponseFormatter;
pub use response_format::SpeechResponseFormat;
pub use response_format::SrtResponseFormatter;
pub use response_format::TextResponseFormat;
pub use response_format::TextResponseFormatter;
pub use response_format::VerboseJsonResponse;
pub use response_format::VerboseJsonResponseFormatter;
pub use response_format::VttResponseFormatter;
pub use result::AudioApiResult;
pub use result::SpeechStreamResult;
pub use result::TextFormatResult;
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
