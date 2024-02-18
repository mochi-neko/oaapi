use futures_util::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Receiver;
use tokio::task::JoinHandle;

use crate::audio::SpeechInput;
use crate::audio::SpeechModel;
use crate::audio::SpeechResponseFormat;
use crate::audio::SpeechStreamError;
use crate::audio::SpeechStreamResult;
use crate::audio::Speed;
use crate::audio::Voice;
use crate::ApiKey;
use crate::Error;

/// The request body for the /audio/speech endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpeechRequestBody {
    /// One of the available TTS models: tts-1 or tts-1-hd
    pub model: SpeechModel,
    /// The text to generate audio for. The maximum length is 4096 characters.
    pub input: SpeechInput,
    /// The voice to use when generating the audio. Supported voices are alloy, echo, fable, onyx, nova, and shimmer.
    pub voice: Voice,
    /// The format to audio in. Supported formats are mp3, opus, aac, and flac.
    pub response_format: Option<SpeechResponseFormat>,
    /// The speed of the generated audio. Select a value from 0.25 to 4.0. 1.0 is the default.
    pub speed: Option<Speed>,
}

impl Default for SpeechRequestBody {
    fn default() -> Self {
        Self {
            model: SpeechModel::Tts1,
            input: SpeechInput::new("".to_string()).unwrap(),
            voice: Voice::Alloy,
            response_format: None,
            speed: None,
        }
    }
}

/// Speeches the given text by /audio/speech endpoint.
///
/// ## NOTE
/// Please abort `JoinHandle<()>` when you want to stop receiving the stream.
///
/// ## Arguments
/// - `client` - The HTTP client.
/// - `api_key` - Your API key of the OpenAI API.
/// - `request_body` - The request body.
/// - `buffer_size` - The size of the buffer to receive the stream.
///
/// ## Returns
/// 1. A receiver to receive the stream.
/// 2. A handle to join the task.
///
/// ## Example
/// ```
/// use oaapi::audio::SpeechRequestBody;
/// use oaapi::audio::SpeechInput;
/// use oaapi::audio::Voice;
/// use oaapi::ApiKey;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let request_body = SpeechRequestBody {
///         input: SpeechInput::new("Hello, world!".to_string())?,
///         voice: Voice::Alloy,
///         ..Default::default()
///     };
///
///     let (mut receiver, handle) = oaapi::audio::speech(
///         &reqwest::Client::new(),
///         &ApiKey::new("your-api-key"),
///         request_body,
///         None, // Buffer size.
///     ).await?;
///
///     // Receive the stream chunks
///     while let Some(chunk) = receiver.recv().await {
///         match chunk {
///             | Ok(chunk) => {
///                 // Do something with the chunk.
///             }
///             | Err(error) => {
///                 // Do something with the error.
///             }
///         }
///     };
///
///     // Abort the stream.
///     handle.abort();
///
///     Ok(())
/// }
/// ```
pub async fn speech(
    client: &Client,
    api_key: &ApiKey,
    request_body: SpeechRequestBody,
    buffer_size: Option<usize>,
) -> crate::Result<(
    Receiver<SpeechStreamResult>,
    JoinHandle<()>,
)> {
    let buffer_size = buffer_size.unwrap_or(16 * 1024);

    // Send the request.
    let response = client
        .post("https://api.openai.com/v1/audio/speech")
        .header(
            "Authorization",
            api_key.authorization_header(),
        )
        .json(&request_body)
        .send()
        .await
        .map_err(Error::HttpRequestError)?;

    // Check the response status code.
    let status_code = response.status();

    // Ok
    if status_code.is_success() {
        // Read the response stream.
        let mut stream = response.bytes_stream();

        // Create a channel to receive the stream.
        let (sender, receiver) = tokio::sync::mpsc::channel(buffer_size);

        // Spawn a task to handle the stream.
        let handle = tokio::spawn(async move {
            // Read the stream.
            while let Some(chunk) = stream.next().await {
                match chunk {
                    | Ok(chunk) => {
                        // Send the chunk to the receiver.
                        _ = sender.send(Ok(chunk)).await;
                    },
                    | Err(error) => {
                        // Send the error to the receiver.
                        _ = sender
                            .send(Err(SpeechStreamError::ErrorChunk(
                                error,
                            )))
                            .await;

                        break;
                    },
                }
            }
        });

        Ok((receiver, handle))
    }
    // Error
    else {
        let response_text = response
            .text()
            .await
            .map_err(Error::ReadResponseTextFailed)?;

        // Deserialize the error response.
        let error_response =
            serde_json::from_str(&response_text).map_err(|error| {
                Error::ErrorResponseDeserializationFailed {
                    error,
                    text: response_text,
                }
            })?;

        Err(Error::ApiError {
            status_code,
            error_response,
        })
    }
}
