use bytes::{Buf, BytesMut};
use futures_util::{Stream, StreamExt};
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::chat::{ChatChunkError, ChatChunkResult, ChatCompletionChunkObject};

/// A stream of message chunks.
pub(crate) struct ChunkStream<S>
where
    S: Stream<Item = ReqwestStreamItem> + Unpin,
{
    stream: S,
    buffer: BytesMut,
}

type ReqwestStreamItem = Result<bytes::Bytes, reqwest::Error>;

impl<S> ChunkStream<S>
where
    S: Stream<Item = ReqwestStreamItem> + Unpin,
{
    pub(crate) fn new(stream: S) -> Self {
        ChunkStream {
            stream,
            buffer: BytesMut::new(),
        }
    }
}

impl<S> Stream for ChunkStream<S>
where
    S: Stream<Item = ReqwestStreamItem> + Unpin,
{
    type Item = ChatChunkResult;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<ChatChunkResult>> {
        loop {
            if let Some(position) = self
                .buffer
                .iter()
                .position(|b| *b == b'\n')
            {
                let line = self.buffer.split_to(position);
                self.buffer.advance(1); // Skip the newline character.
                let line = String::from_utf8(line.to_vec())
                    .map_err(ChatChunkError::StringDecodingError)?;
                if line == "data: [DONE]" {
                    return Poll::Ready(None);
                }
                if line.is_empty() {
                    continue;
                }
                
                let data = line
                    .strip_prefix("data: ")
                    .ok_or_else(|| {
                        ChatChunkError::DataPrefixMissing(line.clone())
                    })?;
                let chunk =
                    serde_json::from_str::<ChatCompletionChunkObject>(&data)
                        .map_err(|error| {
                            ChatChunkError::DeserializeFailed(
                                error,
                                data.to_string(),
                            )
                        })?;
                return Poll::Ready(Some(Ok(chunk)));
            }

            match self
                .stream
                .poll_next_unpin(cx)
            {
                // The stream has more data.
                | Poll::Ready(Some(Ok(chunk))) => {
                    self.buffer.extend(&chunk);
                    // Continue to the next iteration of the loop.
                },
                // The stream has an error.
                | Poll::Ready(Some(Err(error))) => {
                    return Poll::Ready(Some(Err(
                        ChatChunkError::StreamError(error),
                    )));
                },
                // The stream has no more data.
                | Poll::Ready(None) => {
                    return if self.buffer.is_empty() {
                        Poll::Ready(None)
                    } else {
                        let line = self.buffer.split_off(0);
                        let line = String::from_utf8(line.to_vec())
                            .map_err(ChatChunkError::StringDecodingError)?;
                        if line == "data: [DONE]" {
                            return Poll::Ready(None);
                        }
                        
                        let data = line
                            .strip_prefix("data: ")
                            .ok_or_else(|| {
                                ChatChunkError::DataPrefixMissing(line.clone())
                            })?;
                        let chunk = serde_json::from_str::<
                            ChatCompletionChunkObject,
                        >(&data)
                        .map_err(|error| {
                            ChatChunkError::DeserializeFailed(
                                error,
                                data.to_string(),
                            )
                        })?;
                        Poll::Ready(Some(Ok(chunk)))
                    };
                },
                // The stream has no more data for now.
                | Poll::Pending => return Poll::Pending,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::chat::chat_completion_chunk_object::{
        ChatCompletionChunkChoice, ChatCompletionDelta,
    };
    use crate::chat::{ChatModel, Role};
    use bytes::Bytes;
    use futures_util::{stream, StreamExt};

    use super::*;

    #[tokio::test]
    async fn test_stream_line_reader() {
        let source = r#"data: {"id":"chatcmpl-123","object":"chat.completion.chunk","created":1694268190,"model":"gpt-3.5-turbo-0125", "system_fingerprint": "fp_44709d6fcb", "choices":[{"index":0,"delta":{"role":"assistant","content":""},"logprobs":null,"finish_reason":null}]}

data: {"id":"chatcmpl-123","object":"chat.completion.chunk","created":1694268190,"model":"gpt-3.5-turbo-0125", "system_fingerprint": "fp_44709d6fcb", "choices":[{"index":0,"delta":{"content":"Hello"},"logprobs":null,"finish_reason":null}]}

data: {"id":"chatcmpl-123","object":"chat.completion.chunk","created":1694268190,"model":"gpt-3.5-turbo-0125", "system_fingerprint": "fp_44709d6fcb", "choices":[{"index":0,"delta":{"content":"!"},"logprobs":null,"finish_reason":null}]}

data: {"id":"chatcmpl-123","object":"chat.completion.chunk","created":1694268190,"model":"gpt-3.5-turbo-0125", "system_fingerprint": "fp_44709d6fcb", "choices":[{"index":0,"delta":{},"logprobs":null,"finish_reason":"stop"}]}

"#;

        let input_stream = stream::iter(vec![Ok(Bytes::from(
            source,
        ))]);
        let mut stream = ChunkStream::new(input_stream);

        assert_eq!(
            stream
                .next()
                .await
                .unwrap()
                .unwrap(),
            ChatCompletionChunkObject {
                id: "chatcmpl-123".to_string(),
                object: "chat.completion.chunk".to_string(),
                created: 1694268190,
                model: ChatModel::Gpt35Turbo0125,
                system_fingerprint: Some("fp_44709d6fcb".to_string()),
                choices: vec![
                    ChatCompletionChunkChoice {
                        index: 0,
                        delta: Some(ChatCompletionDelta {
                            role: Some(Role::Assistant),
                            content: Some("".to_string()),
                            tool_calls: None,
                        }),
                        logprobs: None,
                        finish_reason: None,
                    }
                ],
            }
        );

        assert_eq!(
            stream
                .next()
                .await
                .unwrap()
                .unwrap(),
            ChatCompletionChunkObject {
                id: "chatcmpl-123".to_string(),
                object: "chat.completion.chunk".to_string(),
                created: 1694268190,
                model: ChatModel::Gpt35Turbo0125,
                system_fingerprint: Some("fp_44709d6fcb".to_string()),
                choices: vec![
                    ChatCompletionChunkChoice {
                        index: 0,
                        delta: Some(ChatCompletionDelta {
                            role: None,
                            content: Some("Hello".to_string()),
                            tool_calls: None,
                        }),
                        logprobs: None,
                        finish_reason: None,
                    }
                ],
            }
        );

        assert_eq!(
            stream
                .next()
                .await
                .unwrap()
                .unwrap(),
            ChatCompletionChunkObject {
                id: "chatcmpl-123".to_string(),
                object: "chat.completion.chunk".to_string(),
                created: 1694268190,
                model: ChatModel::Gpt35Turbo0125,
                system_fingerprint: Some("fp_44709d6fcb".to_string()),
                choices: vec![
                    ChatCompletionChunkChoice {
                        index: 0,
                        delta: Some(ChatCompletionDelta {
                            role: None,
                            content: Some("!".to_string()),
                            tool_calls: None,
                        }),
                        logprobs: None,
                        finish_reason: None,
                    }
                ],
            }
        );

        assert_eq!(
            stream
                .next()
                .await
                .unwrap()
                .unwrap(),
            ChatCompletionChunkObject {
                id: "chatcmpl-123".to_string(),
                object: "chat.completion.chunk".to_string(),
                created: 1694268190,
                model: ChatModel::Gpt35Turbo0125,
                system_fingerprint: Some("fp_44709d6fcb".to_string()),
                choices: vec![
                    ChatCompletionChunkChoice {
                        index: 0,
                        delta: Some(ChatCompletionDelta {
                            role: None,
                            content: None,
                            tool_calls: None,
                        }),
                        logprobs: None,
                        finish_reason: Some("stop".to_string()),
                    }
                ],
            }
        );

        assert_eq!(stream.next().await.is_none(), true);
    }
}
