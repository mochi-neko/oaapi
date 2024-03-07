//! An unofficial Rust client for the OpenAI API.
//!
//! ## Features
//! - [`audio`](`crate::audio`)
//! - [`chat`](`crate::chat`)
//!
//! ## Supported APIs
//! - [x] [audio](https://platform.openai.com/docs/api-reference/audio)
//!     - [x] [speech](https://platform.openai.com/docs/api-reference/audio/createSpeech)
//!     - [x] [transcriptions](https://platform.openai.com/docs/api-reference/audio/createTranscription)
//!     - [x] [translations](https://platform.openai.com/docs/api-reference/audio/createTranslation)
//! - [x] [chat](https://platform.openai.com/docs/api-reference/chat)
//!     - [x] [completions](https://platform.openai.com/docs/api-reference/chat/create)
//!     - [x] [completions streaming](https://platform.openai.com/docs/api-reference/chat/create)
//!
//! ## Usages
//! 1. Enable API feature flags that you want to use, e.g. `chat`.
//! 2. Create a [`crate::Client`] with the API key and the other optional settings.
//! 3. Use the client to call the APIs, e.g. [`crate::Client::chat_complete`].

pub use crate::api_key::ApiKey;
pub use crate::client::Client;
pub use crate::error::ApiError;
pub use crate::error::ValidationError;
pub use crate::organization_id::OrganizationId;
pub use crate::prompt::Prompt;
pub use crate::result::ApiResult;
pub use crate::result::ValidationResult;
pub use crate::temperature::Temperature;

pub use reqwest;

#[cfg(feature = "audio")]
pub mod audio;
#[cfg(feature = "chat")]
pub mod chat;

pub(crate) mod macros;
pub(crate) mod stream_line_reader;

mod api_key;
mod client;
mod error;
mod organization_id;
mod prompt;
mod result;
mod temperature;
