use futures_core::Stream;
use serde::{Deserialize, Serialize};

use crate::audio::SpeechInput;
use crate::audio::SpeechModel;
use crate::audio::SpeechResponseFormat;
use crate::audio::SpeechStreamResult;
use crate::audio::Speed;
use crate::audio::Voice;
use crate::macros::impl_display_for_serialize;
use crate::ApiError;
use crate::ApiResult;
use crate::Client;

/// The request body for the `/audio/speech` endpoint.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SpeechRequestBody {
    /// One of the available TTS models: tts-1 or tts-1-hd
    pub model: SpeechModel,
    /// The text to generate audio for. The maximum length is 4096 characters.
    pub input: SpeechInput,
    /// The voice to use when generating the audio. Supported voices are alloy, echo, fable, onyx, nova, and shimmer.
    pub voice: Voice,
    /// The format to audio in. Supported formats are mp3, opus, aac, and flac.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<SpeechResponseFormat>,
    /// The speed of the generated audio. Select a value from 0.25 to 4.0. 1.0 is the default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<Speed>,
}

impl_display_for_serialize!(SpeechRequestBody);

pub(crate) async fn speech(
    client: &Client,
    request_body: SpeechRequestBody,
) -> ApiResult<impl Stream<Item = SpeechStreamResult>> {
    // Send the request.
    let response = client
        .post("https://api.openai.com/v1/audio/speech")
        .json(&request_body)
        .send()
        .await
        .map_err(ApiError::HttpRequestError)?;

    // Check the response status code.
    let status_code = response.status();

    // Ok
    if status_code.is_success() {
        Ok(response.bytes_stream())
    }
    // Error
    else {
        let response_text = response
            .text()
            .await
            .map_err(ApiError::ReadResponseTextFailed)?;

        // Deserialize the error response.
        let error_response =
            serde_json::from_str(&response_text).map_err(|error| {
                ApiError::ErrorResponseDeserializationFailed {
                    error,
                    text: response_text,
                }
            })?;

        Err(ApiError::ApiResponseError {
            status_code,
            error_response,
        })
    }
}
