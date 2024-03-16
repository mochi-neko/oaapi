use crate::chat::ChatApiError;
use crate::chat::ChatChunkError;
use crate::chat::ChatCompletionChunkObject;

/// The result of a chat API calling.
pub type ChatApiResult<T> = Result<T, ChatApiError>;

/// The result of a chunk for streaming chat completions.
pub type ChatChunkResult = Result<ChatCompletionChunkObject, ChatChunkError>;
