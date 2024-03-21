//! The chat APIs of the OpenAI API.
//!
//! ## NOTE
//! This is only available for `chat` feature flag.
//!
//! ## Supported APIs
//! - [x] [Completions](https://platform.openai.com/docs/api-reference/chat/create)
//! - [x] [Completions streaming](https://platform.openai.com/docs/api-reference/chat/create)
//!
//! ## Examples
//!
//! ### Completions
//! An example to call the chat completions API with the `chat` feature flag, `tokio` and `anyhow` crate is as follows:
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
//! ### Completions streaming
//! An example to call the chat completions streaming API with the `chat` feature flag, `tokio`, `anyhow` and `tokio_stream` crate is as follows:
//!
//! ```no_run
//! use oaapi::Client;
//! use oaapi::chat::CompletionsRequestBody;
//! use oaapi::chat::SystemMessage;
//! use oaapi::chat::UserMessage;
//! use oaapi::chat::ChatModel;
//! use oaapi::chat::StreamOption;
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
//!     // 2. Create a request body parameters with specifying the streaming option: `StreamOption::ReturnStream`.
//!     let request_body = CompletionsRequestBody {
//!         messages: vec![
//!             SystemMessage::new("Prompt.", None).into(),
//!             UserMessage::new("Chat message from user.".into(), None).into(),
//!         ],
//!         model: ChatModel::Gpt35Turbo,
//!         stream: Some(StreamOption::ReturnStream),
//!         ..Default::default()
//!     };
//!
//!     // 3. Call the API.
//!     let mut stream = client
//!         .chat_complete_stream(request_body)
//!         .await?;
//!
//!     // 4. Receive the response stream.
//!     while let Some(response) = stream.next().await {
//!         // Do something with the response.
//!         println!("Chunk:\n{}", response?);
//!     }
//!
//!     Ok(())
//! }
//! ```

pub use api::completions::CompletionsRequestBody;
pub use assistant_message::AssistantMessage;
pub use assistant_message::CalledFunction;
pub use assistant_message::ToolCall;
pub use bias::Bias;
pub use chat_completion_chunk_object::ChatCompletionChunkObject;
pub use chat_completion_object::ChatCompletionObject;
pub use error::ChatApiError;
pub use error::ChatChunkError;
pub use logprobs::Logprobs;
pub use logprobs::LogprobsContent;
pub use logprobs::TopLogprobsContent;
pub use logprobs_option::LogprobsOption;
pub use max_tokens::MaxTokens;
pub use message::Message;
pub use model::ChatModel;
pub use penalty::Penalty;
pub use response_format::ResponseFormat;
pub use response_format::ResponseFormatType;
pub use result::ChatApiResult;
pub use result::ChatChunkResult;
pub use role::Role;
pub use stop_option::StopOption;
pub use stream_option::StreamOption;
pub use system_message::SystemMessage;
pub use tool::Function;
pub use tool::Tool;
pub use tool::ToolType;
pub use tool_choice::SpecifiedFunction;
pub use tool_choice::SpecifiedTool;
pub use tool_choice::TooChoiceOption;
pub use tool_choice::ToolChoice;
pub use tool_message::ToolMessage;
pub use top_logprobs::TopLogprobs;
pub use top_p::TopP;
pub use user_message::ImageContentPart;
pub use user_message::ImageDetail;
pub use user_message::ImageFormat;
pub use user_message::ImageUrl;
pub use user_message::MessageContent;
pub use user_message::MessageContentPart;
pub use user_message::TextContentPart;
pub use user_message::UserMessage;

pub(crate) use api::completions::complete;
pub(crate) use api::completions::complete_stream;

mod api;
mod assistant_message;
mod bias;
mod chat_completion_chunk_object;
mod chat_completion_object;
mod error;
mod logprobs;
mod logprobs_option;
mod max_tokens;
mod message;
mod model;
mod penalty;
mod response_format;
mod result;
mod role;
mod stop_option;
mod stream_option;
mod system_message;
mod tool;
mod tool_choice;
mod tool_message;
mod top_logprobs;
mod top_p;
mod user_message;

mod chunk_stream;
