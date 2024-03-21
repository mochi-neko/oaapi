//! An unofficial Rust client for [the OpenAI API](https://platform.openai.com/docs/api-reference).
//!
//! ## Feature flags
//! - [`audio`](`crate::audio`)
//! - [`chat`](`crate::chat`)
//!
//! > [!NOTE]
//! > You need to enable feature flags to use the corresponding APIs.
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
//! 2. Create a [Client](`crate::Client`) with the API key and the other optional settings.
//! 3. Use the client to call the APIs, e.g. [`Client::chat_complete`].
//!
//! See also examples in documents of each feature module for more details.
//!
//! ## Examples
//! An example to call the chat completions API with the `chat` feature flag:
//!
//! ```toml
//! [dependencies]
//! oaapi = { version = "0.2.0", features = ["chat"] }
//! ```
//!
//! and setting the API key to the environment variable :`OPENAI_API_KEY`
//!
//! ```env
//! OPENAI_API_KEY={your-openai-api-key}
//! ```
//!
//! is as follows:
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
//!     // 1. Create a client with the API key from the environment variable: "OPENAI_API_KEY"
//!     let client = Client::from_env()?;
//!     // or specify the API key directly.
//!     // let client = Client::new(oaapi::ApiKey::new("OPENAI_API_KEY"), None, None);
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
//!     // 4. Use the response.
//!     println!("Result:\n{}", response);
//!
//!     Ok(())
//! }
//! ```
//!
//! See also examples in documents of each feature module for more details.

// Re-exports
pub use crate::api_key::ApiKey;
pub use crate::client::Client;
pub use crate::error::ApiError;
pub use crate::error::ValidationError;
pub use crate::organization_id::OrganizationId;
pub use crate::prompt::Prompt;
pub use crate::result::ApiResult;
pub use crate::result::ValidationResult;
pub use crate::temperature::Temperature;

// Third party re-exports
pub use reqwest;
pub use serde_json;
#[cfg(feature = "audio")]
pub use subtp;

// Feature modules
#[cfg(feature = "audio")]
pub mod audio;
#[cfg(feature = "chat")]
pub mod chat;

// Internal modules
pub(crate) mod macros;

// Private modules
mod api_key;
mod client;
mod error;
mod organization_id;
mod prompt;
mod result;
mod temperature;
