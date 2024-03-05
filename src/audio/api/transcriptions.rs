use reqwest::multipart::Form;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use subtp::srt::SubRip;
use subtp::vtt::WebVtt;

use crate::audio::AudioApiError;
use crate::audio::AudioApiResult;
use crate::audio::AudioModel;
use crate::audio::File;
use crate::audio::Iso639_1;
use crate::audio::JsonResponse;
use crate::audio::JsonResponseFormatter;
use crate::audio::PlainTextResponseFormatter;
use crate::audio::SrtResponseFormatter;
use crate::audio::TextResponseFormat;
use crate::audio::TextResponseFormatter;
use crate::audio::TimestampGranularity;
use crate::audio::VerboseJsonResponse;
use crate::audio::VerboseJsonResponseFormatter;
use crate::audio::VttResponseFormatter;
use crate::ApiError;
use crate::Client;
use crate::Prompt;
use crate::Temperature;

/// The response from the /audio/transcriptions endpoint.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TranscriptionsRequestBody {
    /// The audio file object (not file name) to transcribe, in one of these formats: flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, or webm.
    pub file: File,
    /// ID of the model to use. Only whisper-1 is currently available.
    pub model: AudioModel,
    /// The language of the input audio. Supplying the input language in ISO-639-1 format will improve accuracy and latency.
    pub language: Option<Iso639_1>,
    /// An optional text to guide the model's style or continue a previous audio segment. The prompt should match the audio language.
    pub prompt: Option<Prompt>,
    // NOTE: Specify response format by generic types.
    // The format of the transcript output, in one of these options: json, text, srt, verbose_json, or vtt.
    // pub response_format: Option<ResponseFormat>,
    /// The sampling temperature, between 0 and 1. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. If set to 0, the model will use log probability to automatically increase the temperature until certain thresholds are hit.
    pub temperature: Option<Temperature>,
    /// The timestamp granularities to populate for this transcription. response_format must be set verbose_json to use timestamp granularities. Either or both of these options are supported: word, or segment. Note: There is no additional latency for segment timestamps, but generating word timestamps incurs additional latency.
    pub timestamp_granularities: Option<Vec<TimestampGranularity>>,
}

impl Display for TranscriptionsRequestBody {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "file: {}", self.file)?;
        write!(f, ", model: {}", self.model)?;

        if let Some(language) = self.language {
            write!(f, ", language: {}", language)?;
        }
        if let Some(prompt) = self.prompt.clone() {
            write!(f, ", prompt: {}", prompt)?;
        }
        if let Some(temperature) = self.temperature {
            write!(f, ", temperature: {}", temperature)?;
        }
        if let Some(timestamp_granularities) = self
            .timestamp_granularities
            .clone()
        {
            write!(
                f,
                ", timestamp_granularities: [{}]",
                timestamp_granularities
                    .iter()
                    .map(|granularity| granularity.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )?;
        }

        Ok(())
    }
}

impl TranscriptionsRequestBody {
    /// Creates a new [`TranscriptionsRequestBody`].
    pub fn new(
        file: File,
        model: AudioModel,
        language: Option<Iso639_1>,
        prompt: Option<Prompt>,
        temperature: Option<Temperature>,
        timestamp_granularities: Option<Vec<TimestampGranularity>>,
    ) -> Self {
        Self {
            file,
            model,
            language,
            prompt,
            temperature,
            timestamp_granularities,
        }
    }

    /// Builds a multipart form from the request body.
    async fn build_form<F, T>(self) -> AudioApiResult<Form>
    where
        F: TextResponseFormat,
        T: TextResponseFormatter<F>,
    {
        let file = self.file.build_part().await?;

        let mut form = Form::new()
            .part("file", file)
            .text("model", self.model.to_string())
            .text("response_format", F::format());

        if let Some(language) = self.language {
            form = form.text("language", language.to_string());
        }

        if let Some(prompt) = self.prompt {
            form = form.text("prompt", prompt.to_string());
        }

        if let Some(temperature) = self.temperature {
            form = form.text("temperature", temperature.to_string());
        }

        if let Some(timestamp_granularities) = self.timestamp_granularities {
            for granularity in timestamp_granularities {
                form = form.text(
                    "timestamp_granularities[]",
                    granularity.to_string(),
                )
            }
        }

        Ok(form)
    }
}

async fn transcribe<F, T>(
    client: &Client,
    request_body: TranscriptionsRequestBody,
) -> AudioApiResult<F>
where
    F: TextResponseFormat,
    T: TextResponseFormatter<F>,
{
    if request_body
        .timestamp_granularities
        .is_some()
        && F::format() != VerboseJsonResponse::format()
    {
        return Err(AudioApiError::TimestampOptionMismatch);
    }

    // Build the multipart form.
    let form = request_body
        .build_form::<F, T>()
        .await?;

    // Send the request.
    let response = client
        .post("https://api.openai.com/v1/audio/transcriptions")
        .multipart(form)
        .send()
        .await
        .map_err(ApiError::HttpRequestError)?;

    // Check the response status code.
    let status_code = response.status();

    // Read the response text.
    let response_text = response
        .text()
        .await
        .map_err(ApiError::ReadResponseTextFailed)?;

    // OK
    if status_code.is_success() {
        // Format the response text.
        println!("response_text: {}", response_text);
        T::format(response_text).map_err(AudioApiError::FormatResponseFailed)
    }
    // Error
    else {
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
        }
        .into())
    }
}

pub(crate) async fn transcribe_into_json(
    client: &Client,
    request_body: TranscriptionsRequestBody,
) -> AudioApiResult<JsonResponse> {
    transcribe::<JsonResponse, JsonResponseFormatter>(client, request_body)
        .await
}

pub(crate) async fn transcribe_into_plain_text(
    client: &Client,
    request_body: TranscriptionsRequestBody,
) -> AudioApiResult<String> {
    transcribe::<String, PlainTextResponseFormatter>(client, request_body).await
}

pub(crate) async fn transcribe_into_verbose_json(
    client: &Client,
    request_body: TranscriptionsRequestBody,
) -> AudioApiResult<VerboseJsonResponse> {
    transcribe::<VerboseJsonResponse, VerboseJsonResponseFormatter>(
        client,
        request_body,
    )
    .await
}

pub(crate) async fn transcribe_into_srt(
    client: &Client,
    request_body: TranscriptionsRequestBody,
) -> AudioApiResult<SubRip> {
    transcribe::<SubRip, SrtResponseFormatter>(client, request_body).await
}

pub(crate) async fn transcribe_into_vtt(
    client: &Client,
    request_body: TranscriptionsRequestBody,
) -> AudioApiResult<WebVtt> {
    transcribe::<WebVtt, VttResponseFormatter>(client, request_body).await
}
