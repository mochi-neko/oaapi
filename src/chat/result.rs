use crate::chat::ChatStreamError;
use crate::chat::{ChatCompletionChunkObject, StreamLineError};

/// The result of streaming chat completions.
pub type ChatStreamResult = Result<ChatCompletionChunkObject, ChatStreamError>;

/// The result of a chat API calling.
pub type ChatApiResult<T> = Result<T, crate::chat::error::ChatApiError>;

/// The result of a stream line.
pub(crate) type StreamLineResult = Result<String, StreamLineError>;
