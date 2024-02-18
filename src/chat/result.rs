use crate::chat::ChatCompletionChunkObject;
use crate::chat::ChatStreamError;

/// The result of streaming chat completions.
pub type ChatStreamResult = Result<ChatCompletionChunkObject, ChatStreamError>;
