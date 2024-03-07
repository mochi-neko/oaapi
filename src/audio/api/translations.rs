use reqwest::multipart::Form;
use std::fmt::Display;
use subtp::srt::SubRip;
use subtp::vtt::WebVtt;

use crate::audio::AudioApiError;
use crate::audio::AudioApiResult;
use crate::audio::AudioModel;
use crate::audio::File;
use crate::audio::JsonResponse;
use crate::audio::JsonResponseFormatter;
use crate::audio::PlainTextResponseFormatter;
use crate::audio::SrtResponseFormatter;
use crate::audio::TextResponseFormat;
use crate::audio::TextResponseFormatter;
use crate::audio::VerboseJsonResponse;
use crate::audio::VerboseJsonResponseFormatter;
use crate::audio::VttResponseFormatter;
use crate::ApiError;
use crate::Client;
use crate::Prompt;
use crate::Temperature;

/// The request boyd for the `/audio/translations` endpoint.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TranslationsRequestBody {
    /// The audio file object (not file name) to transcribe, in one of these formats: flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, or webm.
    pub file: File,
    /// ID of the model to use. Only whisper-1 is currently available.
    pub model: AudioModel,
    /// An optional text to guide the model's style or continue a previous audio segment. The prompt should match the audio language.
    pub prompt: Option<Prompt>,
    // NOTE: Specify response format by generic types.
    // The format of the transcript output, in one of these options: json, text, srt, verbose_json, or vtt.
    // pub response_format: Option<ResponseFormat>,
    /// The sampling temperature, between 0 and 1. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. If set to 0, the model will use log probability to automatically increase the temperature until certain thresholds are hit.
    pub temperature: Option<Temperature>,
}

impl Display for TranslationsRequestBody {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "file: {}", self.file)?;
        write!(f, ", model: {}", self.model)?;

        if let Some(prompt) = self.prompt.clone() {
            write!(f, ", prompt: {}", prompt)?;
        }
        if let Some(temperature) = self.temperature {
            write!(f, ", temperature: {}", temperature)?;
        }

        Ok(())
    }
}

impl TranslationsRequestBody {
    /// Creates a new [`TranslationsRequestBody`].
    pub fn new(
        file: File,
        model: AudioModel,
        prompt: Option<Prompt>,
        temperature: Option<Temperature>,
    ) -> Self {
        Self {
            file,
            model,
            prompt,
            temperature,
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

        if let Some(prompt) = self.prompt {
            form = form.text("prompt", prompt.format());
        }

        if let Some(temperature) = self.temperature {
            form = form.text("temperature", temperature.format());
        }

        Ok(form)
    }
}

async fn translate<F, T>(
    client: &Client,
    request_body: TranslationsRequestBody,
) -> AudioApiResult<F>
where
    F: TextResponseFormat,
    T: TextResponseFormatter<F>,
{
    // Build the multipart form.
    let form = request_body
        .build_form::<F, T>()
        .await?;

    // Send the request.
    let response = client
        .post("https://api.openai.com/v1/audio/translations")
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

    // Ok
    if status_code.is_success() {
        // Format the response text.
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

pub(crate) async fn translate_into_json(
    client: &Client,
    request_body: TranslationsRequestBody,
) -> AudioApiResult<JsonResponse> {
    translate::<JsonResponse, JsonResponseFormatter>(client, request_body).await
}

pub(crate) async fn translate_into_plain_text(
    client: &Client,
    request_body: TranslationsRequestBody,
) -> AudioApiResult<String> {
    translate::<String, PlainTextResponseFormatter>(client, request_body).await
}

pub(crate) async fn translate_into_verbose_json(
    client: &Client,
    request_body: TranslationsRequestBody,
) -> AudioApiResult<VerboseJsonResponse> {
    translate::<VerboseJsonResponse, VerboseJsonResponseFormatter>(
        client,
        request_body,
    )
    .await
}

pub(crate) async fn translate_into_srt(
    client: &Client,
    request_body: TranslationsRequestBody,
) -> AudioApiResult<SubRip> {
    translate::<SubRip, SrtResponseFormatter>(client, request_body).await
}

pub(crate) async fn translate_into_vtt(
    client: &Client,
    request_body: TranslationsRequestBody,
) -> AudioApiResult<WebVtt> {
    translate::<WebVtt, VttResponseFormatter>(client, request_body).await
}
