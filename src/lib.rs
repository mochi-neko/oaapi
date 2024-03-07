//! An unofficial Rust client for the OpenAI API.
//!
//! ## Features
//! - [`audio`](`crate::audio`)
//! - [`chat`](`crate::chat`)
//!
//! > [!NOTE]
//! > You need to enable the feature flags to use the corresponding APIs.
//!
//! ## Supported APIs
//! - [x] [Audio](https://platform.openai.com/docs/api-reference/audio)
//!     - [x] [speech](https://platform.openai.com/docs/api-reference/audio/createSpeech)
//!     - [x] [transcriptions](https://platform.openai.com/docs/api-reference/audio/createTranscription)
//!     - [x] [translations](https://platform.openai.com/docs/api-reference/audio/createTranslation)
//! - [x] [Chat](https://platform.openai.com/docs/api-reference/chat)
//!     - [x] [completions](https://platform.openai.com/docs/api-reference/chat/create)
//!     - [x] [completions streaming](https://platform.openai.com/docs/api-reference/chat/create)
//! - [ ] [Embeddings](https://platform.openai.com/docs/api-reference/embeddings)
//! - [ ] [Fine-tuning](https://platform.openai.com/docs/api-reference/fine-tuning)
//! - [ ] [Files](https://platform.openai.com/docs/api-reference/files)
//! - [ ] [Images](https://platform.openai.com/docs/api-reference/images)
//! - [ ] [Models](https://platform.openai.com/docs/api-reference/models)
//! - [ ] [Moderations](https://platform.openai.com/docs/api-reference/moderations)
//!
//! Beta version APIs:
//! - [ ] [Assistants](https://platform.openai.com/docs/api-reference/assistants)
//! - [ ] [Threads](https://platform.openai.com/docs/api-reference/threads)
//! - [ ] [Messages](https://platform.openai.com/docs/api-reference/messages)
//! - [ ] [Runs](https://platform.openai.com/docs/api-reference/runs)
//!
//! ## Usage
//! 1. Enable API feature flags that you want to use, e.g. `chat`.
//! 2. Create a [`crate::Client`] with the API key and the other optional settings.
//! 3. Use the client to call the APIs, e.g. [`crate::Client::chat_complete`].
//!
//! ## Example
//! An example to call the chat completions API with `chat` feature is as follows:
//!
//! ```no_run
//! use oaapi::Client;
//! use oaapi::chat::CompletionsRequestBody;
//! use oaapi::chat::SystemMessage;
//! use oaapi::chat::UserMessage;
//! use oaapi::chat::ChatModel;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // 1. Create a client, e.g. with API key loaded from the environment variable: `OPENAI_API_KEY`.
//!     let client = Client::from_env()?;
//!
//!     // 2. Create a request body parameters.
//!     let request_body = CompletionsRequestBody {
//!         messages: vec![
//!             SystemMessage::new("Prompt.", None).into(),
//!             UserMessage::new("Chat message from user.".into(), None).into(),
//!         ],
//!         model: ChatModel::Gpt35Turbo,
//!         ..Default::default()
//!     };
//!
//!     // 3. Call the API.
//!     let response = client
//!         .chat_complete(request_body)
//!         .await?;
//!
//!     Ok(())
//! }
//! ```

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
