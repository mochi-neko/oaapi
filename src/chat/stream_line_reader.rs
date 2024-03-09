use crate::chat::{StreamLineError, StreamLineResult};
use bytes::{Buf, BytesMut};
use futures::{Stream, StreamExt};
use std::task::Poll;

/// A stream reader that reads lines from a reqwest response stream.
pub(crate) struct StreamLineReader<S>
where
    S: Stream<Item = ReqwestStreamItem>,
{
    stream: S,
    buffer: BytesMut,
}

type ReqwestStreamItem = Result<bytes::Bytes, reqwest::Error>;

impl<S> StreamLineReader<S>
where
    S: Stream<Item = ReqwestStreamItem>,
{
    pub(crate) fn new(stream: S) -> Self {
        StreamLineReader {
            stream,
            buffer: BytesMut::new(),
        }
    }
}

impl<S> Stream for StreamLineReader<S>
where
    S: Stream<Item = ReqwestStreamItem> + Unpin,
{
    type Item = StreamLineResult;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<StreamLineResult>> {
        loop {
            if let Some(position) = self
                .buffer
                .iter()
                .position(|b| *b == b'\n')
            {
                let line = self.buffer.split_to(position);
                self.buffer.advance(1); // Skip the newline character.
                let line = String::from_utf8(line.to_vec())
                    .map_err(StreamLineError::StringDeserializationError)?;
                return Poll::Ready(Some(Ok(line)));
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
                        StreamLineError::ReqwestError(error),
                    )));
                },
                // The stream has no more data.
                | Poll::Ready(None) => {
                    return if self.buffer.is_empty() {
                        Poll::Ready(None)
                    } else {
                        let line = self.buffer.split_off(0);
                        let line = String::from_utf8(line.to_vec()).map_err(
                            StreamLineError::StringDeserializationError,
                        )?;
                        Poll::Ready(Some(Ok(line)))
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
    use futures::{stream, StreamExt};

    use super::*;

    #[test]
    fn test_stream_line_reader() {
        let input = vec![
            Ok(bytes::Bytes::from("Hello\nWorld\n")),
            Ok(bytes::Bytes::from(
                "This is a test\nof the StreamLineReader",
            )),
        ];
        let stream = stream::iter(input);
        let mut line_reader = StreamLineReader::new(stream);

        let runtime = tokio::runtime::Runtime::new().unwrap();

        runtime.block_on(async {
            assert_eq!(
                line_reader
                    .next()
                    .await
                    .unwrap()
                    .unwrap(),
                "Hello".to_string()
            );
            assert_eq!(
                line_reader
                    .next()
                    .await
                    .unwrap()
                    .unwrap(),
                "World".to_string()
            );
            assert_eq!(
                line_reader
                    .next()
                    .await
                    .unwrap()
                    .unwrap(),
                "This is a test".to_string()
            );
            assert_eq!(
                line_reader
                    .next()
                    .await
                    .unwrap()
                    .unwrap(),
                "of the StreamLineReader".to_string()
            );
            assert_eq!(
                line_reader
                    .next()
                    .await
                    .is_none(),
                true
            );
        });
    }
}
