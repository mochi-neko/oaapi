use crate::audio::{
    JsonResponse, SpeechRequestBody, SpeechStreamResult,
    TranscriptionsRequestBody, TranslationsRequestBody, VerboseJsonResponse,
};
use crate::chat::{
    ChatCompletionObject, ChatStreamResult, CompletionsRequestBody,
};
use crate::ApiKey;
use crate::OrganizationId;
use crate::Result;

use std::env::VarError;

use subtp::srt::SubRip;
use subtp::vtt::WebVtt;
use tokio::sync::mpsc::Receiver;
use tokio::task::JoinHandle;

/// The client of the OpenAI API.
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
    pub fn from_env() -> std::result::Result<Self, VarError> {
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
    ) -> Result<(
        Receiver<SpeechStreamResult>,
        JoinHandle<()>,
    )> {
        crate::audio::speech(&self, request_body, buffer_size).await
    }

    pub async fn audio_transcribe_into_json(
        &self,
        request_body: TranscriptionsRequestBody,
    ) -> Result<JsonResponse> {
        crate::audio::transcribe_into_json(&self, request_body).await
    }

    pub async fn audio_transcribe_into_plain_text(
        &self,
        request_body: TranscriptionsRequestBody,
    ) -> Result<String> {
        crate::audio::transcribe_into_plain_text(&self, request_body).await
    }

    pub async fn audio_transcribe_into_verbose_json(
        &self,
        request_body: TranscriptionsRequestBody,
    ) -> Result<VerboseJsonResponse> {
        crate::audio::transcribe_into_verbose_json(&self, request_body).await
    }

    pub async fn audio_transcribe_into_srt(
        &self,
        request_body: TranscriptionsRequestBody,
    ) -> Result<SubRip> {
        crate::audio::transcribe_into_srt(&self, request_body).await
    }

    pub async fn audio_transcribe_into_vtt(
        &self,
        request_body: TranscriptionsRequestBody,
    ) -> Result<WebVtt> {
        crate::audio::transcribe_into_vtt(&self, request_body).await
    }

    pub async fn audio_translate_into_json(
        &self,
        request_body: TranslationsRequestBody,
    ) -> Result<JsonResponse> {
        crate::audio::translate_into_json(&self, request_body).await
    }

    pub async fn audio_translate_into_plain_text(
        &self,
        request_body: TranslationsRequestBody,
    ) -> Result<String> {
        crate::audio::translate_into_plain_text(&self, request_body).await
    }

    pub async fn audio_translate_into_verbose_json(
        &self,
        request_body: TranslationsRequestBody,
    ) -> Result<VerboseJsonResponse> {
        crate::audio::translate_into_verbose_json(&self, request_body).await
    }

    pub async fn audio_translate_into_srt(
        &self,
        request_body: TranslationsRequestBody,
    ) -> Result<SubRip> {
        crate::audio::translate_into_srt(&self, request_body).await
    }

    pub async fn audio_translate_into_vtt(
        &self,
        request_body: TranslationsRequestBody,
    ) -> Result<WebVtt> {
        crate::audio::translate_into_vtt(&self, request_body).await
    }
}

// Chat APIs
impl Client {
    pub async fn chat_complete(
        &self,
        request_body: CompletionsRequestBody,
    ) -> Result<ChatCompletionObject> {
        crate::chat::complete(&self, request_body).await
    }

    pub async fn chat_complete_stream(
        &self,
        request_body: CompletionsRequestBody,
        buffer_size: Option<usize>,
    ) -> Result<(
        Receiver<ChatStreamResult>,
        JoinHandle<()>,
    )> {
        crate::chat::complete_stream(&self, request_body, buffer_size).await
    }
}
