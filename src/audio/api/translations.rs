use reqwest::multipart::Form;
use reqwest::Client;

use crate::audio::response_format::{
    VerboseJsonResponse, VerboseJsonResponseFormatter,
};
use crate::audio::File;
use crate::audio::TextResponseFormat;
use crate::audio::TextResponseFormatter;
use crate::audio::{AudioModel, JsonResponse, JsonResponseFormatter};
use crate::ApiKey;
use crate::Error;
use crate::Prompt;
use crate::Result;
use crate::Temperature;

/// The response from the /audio/translations endpoint.
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

impl Default for TranslationsRequestBody {
    fn default() -> Self {
        Self {
            file: File::Binary {
                file_name: String::new(),
                data: Vec::new(),
            },
            model: AudioModel::Whisper1,
            prompt: None,
            temperature: None,
        }
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
    async fn build_form<F, T>(self) -> Result<Form>
    where
        F: TextResponseFormat,
        T: TextResponseFormatter<F>,
    {
        let file = self.file.build_part().await?;

        let mut form = Form::new()
            .part("file", file)
            .text("model", self.model.format())
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

/// Translates audio into English.
///
/// ## Arguments
/// - `client` - The HTTP client.
/// - `api_key` - Your API key of the OpenAI API.
/// - `request_body` - The request body.
///
/// ## Type Parameters
/// - `F` - The response format type.
/// - `T` - The response formatter type.
///
/// ## Returns
/// Formatted response specified by the type parameters.
///
/// ## Example
/// ```
/// use openai::audio::TranslationsRequestBody;
/// use openai::audio::File;
/// use std::path::Path;
/// use openai::audio::AudioModel;
/// use openai::audio::JsonResponse;
/// use openai::audio::JsonResponseFormatter;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let request_body = TranslationsRequestBody {
///         file: File::from_file_path(
///             Path::new("path/to/audio/file").to_path_buf(),
///          )?,
///         model: AudioModel::Whisper1,
///         prompt: None,
///         temperature: None,
///     };
///     
///     let response = openai::audio::translate::<JsonResponse, JsonResponseFormatter>(
///         &reqwest::Client::new(),
///         &openai::ApiKey::new("your-api-key"),
///         request_body,
///     ).await?;
/// }
/// ```
pub async fn translate<F, T>(
    client: &Client,
    api_key: &ApiKey,
    request_body: TranslationsRequestBody,
) -> Result<F>
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
        .header(
            "Authorization",
            api_key.authorization_header(),
        )
        .multipart(form)
        .send()
        .await
        .map_err(Error::HttpRequestError)?;

    // Check the response status code.
    let status_code = response.status();

    // Read the response text.
    let response_text = response
        .text()
        .await
        .map_err(Error::ReadResponseTextFailed)?;

    // Ok
    if status_code.is_success() {
        // Format the response text.
        T::format(response_text).map_err(Error::FormatResponseFailed)
    }
    // Error
    else {
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

/// Translates audio into English as JSON.
///
/// ## Arguments
/// - `client` - The HTTP client.
/// - `api_key` - Your API key of the OpenAI API.
/// - `request_body` - The request body.
///
/// ## Example
/// ```
/// use openai::audio::TranslationsRequestBody;
/// use openai::audio::File;
/// use std::path::Path;
/// use openai::audio::AudioModel;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let request_body = TranslationsRequestBody {
///         file: File::from_file_path(
///             Path::new("path/to/audio/file").to_path_buf(),
///          )?,
///         model: AudioModel::Whisper1,
///         prompt: None,
///         temperature: None,
///     };
///     
///     let response = openai::audio::translate_into_json(
///         &reqwest::Client::new(),
///         &openai::ApiKey::new("your-api-key"),
///         request_body,
///     ).await?;
/// }
/// ```
pub async fn translate_into_json(
    client: &Client,
    api_key: &ApiKey,
    request_body: TranslationsRequestBody,
) -> Result<JsonResponse> {
    translate::<JsonResponse, JsonResponseFormatter>(
        client,
        api_key,
        request_body,
    )
    .await
}

/// Translates audio into English as plain text.
///
/// ## Arguments
/// - `client` - The HTTP client.
/// - `api_key` - Your API key of the OpenAI API.
/// - `request_body` - The request body.
///
/// ## Example
/// ```
/// use openai::audio::TranslationsRequestBody;
/// use openai::audio::File;
/// use std::path::Path;
/// use openai::audio::AudioModel;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let request_body = TranslationsRequestBody {
///         file: File::from_file_path(
///             Path::new("path/to/audio/file").to_path_buf(),
///          )?,
///         model: AudioModel::Whisper1,
///         prompt: None,
///         temperature: None,
///     };
///     
///     let response = openai::audio::translate_into_plain_text(
///         &reqwest::Client::new(),
///         &openai::ApiKey::new("your-api-key"),
///         request_body,
///     ).await?;
/// }
/// ```
pub async fn translate_into_plain_text(
    client: &Client,
    api_key: &ApiKey,
    request_body: TranslationsRequestBody,
) -> Result<JsonResponse> {
    translate::<JsonResponse, JsonResponseFormatter>(
        client,
        api_key,
        request_body,
    )
    .await
}

/// Translates audio into English as verbose JSON.
///
/// ## Arguments
/// - `client` - The HTTP client.
/// - `api_key` - Your API key of the OpenAI API.
/// - `request_body` - The request body.
///
/// ## Example
/// ```
/// use openai::audio::TranslationsRequestBody;
/// use openai::audio::File;
/// use std::path::Path;
/// use openai::audio::AudioModel;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let request_body = TranslationsRequestBody {
///         file: File::from_file_path(
///             Path::new("path/to/audio/file").to_path_buf(),
///          )?,
///         model: AudioModel::Whisper1,
///         prompt: None,
///         temperature: None,
///     };
///     
///     let response = openai::audio::translate_into_verbose_json(
///         &reqwest::Client::new(),
///         &openai::ApiKey::new("your-api-key"),
///         request_body,
///     ).await?;
/// }
/// ```
pub async fn translate_into_verbose_json(
    client: &Client,
    api_key: &ApiKey,
    request_body: TranslationsRequestBody,
) -> Result<VerboseJsonResponse> {
    translate::<VerboseJsonResponse, VerboseJsonResponseFormatter>(
        client,
        api_key,
        request_body,
    )
    .await
}
