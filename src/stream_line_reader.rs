use std::pin::Pin;
use std::task::Context;

use bytes::{Buf, BytesMut};
use futures::{task::Poll, Stream};

/// A stream that reads text lines from a stream of bytes.
pub(crate) struct StreamLineReader<S>
where
    S: Stream<Item = Result<bytes::Bytes, reqwest::Error>>,
{
    stream: S,
    buffer: BytesMut,
}

impl<S> StreamLineReader<S>
where
    S: Stream<Item = Result<bytes::Bytes, reqwest::Error>>,
{
    pub fn new(stream: S) -> Self {
        StreamLineReader {
            stream,
            buffer: BytesMut::new(),
        }
    }
}

impl<S> Stream for StreamLineReader<S>
where
    S: Stream<Item = Result<bytes::Bytes, reqwest::Error>> + Unpin,
{
    type Item = Result<String, reqwest::Error>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        loop {
            if let Some(pos) = self
                .buffer
                .iter()
                .position(|&x| x == b'\n')
            {
                let line = self.buffer.split_to(pos);
                self.buffer.advance(1); // 改行文字をスキップ
                let line =
                    String::from_utf8(line.to_vec()).expect("Invalid UTF-8");
                return Poll::Ready(Some(Ok(line)));
            }

            match Pin::new(&mut self.stream).poll_next(cx) {
                | Poll::Ready(Some(Ok(chunk))) => {
                    self.buffer
                        .extend_from_slice(&chunk);
                },
                | Poll::Ready(Some(Err(e))) => {
                    return Poll::Ready(Some(Err(e)));
                },
                | Poll::Ready(None) => {
                    return if self.buffer.is_empty() {
                        Poll::Ready(None)
                    } else {
                        let line = self.buffer.split_off(0);
                        let line = String::from_utf8(line.to_vec())
                            .expect("Invalid UTF-8");
                        Poll::Ready(Some(Ok(line)))
                    };
                },
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
