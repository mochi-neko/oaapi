//! The chat APIs of the OpenAI API.
//!
//! ## NOTE
//! This is only available for `chat` feature flag.
//!
//! ## Supported APIs
//! - [x] [completions](https://platform.openai.com/docs/api-reference/chat/create)
//! - [x] [completions streaming](https://platform.openai.com/docs/api-reference/chat/create)

pub use api::completions::CompletionsRequestBody;
pub use assistant_message::AssistantMessage;
pub use assistant_message::CalledFunction;
pub use assistant_message::ToolCall;
pub use bias::Bias;
pub use chat_completion_chunk_object::ChatCompletionChunkObject;
pub use chat_completion_object::ChatCompletionObject;
pub use error::ChatApiError;
pub use error::ChatStreamError;
pub use logprobs::Logprobs;
pub use logprobs::LogprobsContent;
pub use logprobs::TopLogprobsContent;
pub use logprobs_option::LogprobsOption;
pub use max_tokens::MaxTokens;
pub use message::Message;
pub use model::ChatModel;
pub use penalty::Penalty;
pub use response_format::JsonResponseFormat;
pub use response_format::ResponseFormat;
pub use response_format::TextResponseFormat;
pub use result::ChatApiResult;
pub use result::ChatStreamResult;
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
