use crate::audio::{
    AudioApiResult, JsonResponse, SpeechRequestBody, SpeechStreamResult,
    TranscriptionsRequestBody, TranslationsRequestBody, VerboseJsonResponse,
};
use crate::chat::{
    ChatApiResult, ChatCompletionObject, ChatStreamResult,
    CompletionsRequestBody,
};
use crate::ApiKey;
use crate::ApiResult;
use crate::OrganizationId;

use std::env::VarError;

use subtp::srt::SubRip;
use subtp::vtt::WebVtt;
use tokio::sync::mpsc::Receiver;
use tokio::task::JoinHandle;

/// The client of the OpenAI API.
#[derive(Clone)]
pub struct Client {
    api_key: ApiKey,
    client: reqwest::Client,
    organization_id: Option<OrganizationId>,
}

impl Client {
    /// Creates a new client.
    ///
    /// ## Arguments
    /// - `api_key` - The API key of the OpenAI API.
    ///
    /// ## Example
    /// ```
    /// use oaapi::ApiKey;
    /// use oaapi::Client;
    ///
    /// let api_key = ApiKey::new("your-api-key");
    ///
    /// let client = Client::new(api_key, None, None);
    /// ```
    pub fn new(
        api_key: ApiKey,
        client: Option<reqwest::Client>,
        organization_id: Option<OrganizationId>,
    ) -> Self {
        Self {
            api_key,
            client: client.unwrap_or(reqwest::Client::new()),
            organization_id,
        }
    }

    /// Creates a new client with the API key loaded from the environment variable: `OPENAI_API_KEY`.
    ///
    /// ## Example
    /// ```
    /// use oaapi::Client;
    ///
    /// let client = Client::from_env().unwrap();
    /// ```
    pub fn from_env() -> Result<Self, VarError> {
        let api_key = ApiKey::from_env()?;

        Ok(Self::new(api_key, None, None))
    }

    pub(crate) fn post(
        &self,
        endpoint: &str,
    ) -> reqwest::RequestBuilder {
        let mut builder = self
            .client
            .post(endpoint)
            .header(
                "Authorization",
                self.api_key
                    .authorization_header(),
            );

        if let Some(organization_id) = self.organization_id.clone() {
            builder = builder.header(
                "OpenAI-Organization",
                organization_id.organization_header(),
            );
        }

        builder
    }
}

// Audio APIs
impl Client {
    pub async fn audio_speech(
        &self,
        request_body: SpeechRequestBody,
        buffer_size: Option<usize>,
    ) -> ApiResult<(
        Receiver<SpeechStreamResult>,
        JoinHandle<()>,
    )> {
        crate::audio::speech(&self, request_body, buffer_size).await
    }

    pub async fn audio_transcribe_into_json(
        &self,
        request_body: TranscriptionsRequestBody,
    ) -> AudioApiResult<JsonResponse> {
        crate::audio::transcribe_into_json(&self, request_body).await
    }

    pub async fn audio_transcribe_into_plain_text(
        &self,
        request_body: TranscriptionsRequestBody,
    ) -> AudioApiResult<String> {
        crate::audio::transcribe_into_plain_text(&self, request_body).await
    }

    pub async fn audio_transcribe_into_verbose_json(
        &self,
        request_body: TranscriptionsRequestBody,
    ) -> AudioApiResult<VerboseJsonResponse> {
        crate::audio::transcribe_into_verbose_json(&self, request_body).await
    }

    pub async fn audio_transcribe_into_srt(
        &self,
        request_body: TranscriptionsRequestBody,
    ) -> AudioApiResult<SubRip> {
        crate::audio::transcribe_into_srt(&self, request_body).await
    }

    pub async fn audio_transcribe_into_vtt(
        &self,
        request_body: TranscriptionsRequestBody,
    ) -> AudioApiResult<WebVtt> {
        crate::audio::transcribe_into_vtt(&self, request_body).await
    }

    pub async fn audio_translate_into_json(
        &self,
        request_body: TranslationsRequestBody,
    ) -> AudioApiResult<JsonResponse> {
        crate::audio::translate_into_json(&self, request_body).await
    }

    pub async fn audio_translate_into_plain_text(
        &self,
        request_body: TranslationsRequestBody,
    ) -> AudioApiResult<String> {
        crate::audio::translate_into_plain_text(&self, request_body).await
    }

    pub async fn audio_translate_into_verbose_json(
        &self,
        request_body: TranslationsRequestBody,
    ) -> AudioApiResult<VerboseJsonResponse> {
        crate::audio::translate_into_verbose_json(&self, request_body).await
    }

    pub async fn audio_translate_into_srt(
        &self,
        request_body: TranslationsRequestBody,
    ) -> AudioApiResult<SubRip> {
        crate::audio::translate_into_srt(&self, request_body).await
    }

    pub async fn audio_translate_into_vtt(
        &self,
        request_body: TranslationsRequestBody,
    ) -> AudioApiResult<WebVtt> {
        crate::audio::translate_into_vtt(&self, request_body).await
    }
}

// Chat APIs
impl Client {
    pub async fn chat_complete(
        &self,
        request_body: CompletionsRequestBody,
    ) -> ChatApiResult<ChatCompletionObject> {
        crate::chat::complete(&self, request_body).await
    }

    pub async fn chat_complete_stream(
        &self,
        request_body: CompletionsRequestBody,
        buffer_size: Option<usize>,
    ) -> ChatApiResult<(
        Receiver<ChatStreamResult>,
        JoinHandle<()>,
    )> {
        crate::chat::complete_stream(&self, request_body, buffer_size).await
    }
}
