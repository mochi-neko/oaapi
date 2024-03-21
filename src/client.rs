#[cfg(feature = "audio")]
use crate::audio::{
    AudioApiResult, JsonResponse, SpeechRequestBody, SpeechStreamResult,
    TranscriptionsRequestBody, TranslationsRequestBody, VerboseJsonResponse,
};
#[cfg(feature = "chat")]
use crate::chat::{
    ChatApiResult, ChatChunkResult, ChatCompletionObject,
    CompletionsRequestBody,
};
use crate::ApiKey;
use crate::ApiResult;
use crate::OrganizationId;

use futures_core::Stream;

use std::env::VarError;

#[cfg(feature = "audio")]
use subtp::srt::SubRip;
#[cfg(feature = "audio")]
use subtp::vtt::WebVtt;

/// The client of the OpenAI API.
#[derive(Clone)]
pub struct Client {
    /// The API key.
    api_key: ApiKey,
    /// The organization ID.
    organization_id: Option<OrganizationId>,
    /// The internal HTTP client.
    client: reqwest::Client,
}

impl Client {
    /// Creates a new client.
    ///
    /// ## Arguments
    /// - `api_key` - The API key of the OpenAI API.
    /// - `client` - The HTTP client of the `reqwest`.
    /// - `organization_id` - The organization ID of the OpenAI API.
    ///
    /// ## Example
    /// ```
    /// use oaapi::ApiKey;
    /// use oaapi::OrganizationId;
    /// use oaapi::Client;
    ///
    /// let api_key = ApiKey::new("your-api-key");
    /// let inner_client = oaapi::reqwest::Client::new();
    /// let organization_id = OrganizationId::new("your-organization-id");
    ///
    /// let client = Client::new(
    ///     api_key,
    ///     Some(organization_id),
    ///     Some(inner_client)
    /// );
    /// ```
    pub fn new(
        api_key: ApiKey,
        organization_id: Option<OrganizationId>,
        client: Option<reqwest::Client>,
    ) -> Self {
        Self {
            api_key,
            organization_id,
            client: client.unwrap_or(reqwest::Client::new()),
        }
    }

    /// Creates a new client with the API key loaded from the environment variable: `OPENAI_API_KEY`.
    ///
    /// ## Example
    /// ```no_run
    /// use oaapi::Client;
    ///
    /// let client = Client::from_env().unwrap();
    /// ```
    pub fn from_env() -> Result<Self, VarError> {
        let api_key = ApiKey::from_env()?;

        Ok(Self::new(api_key, None, None))
    }

    /// Creates a base POST request for the OpenAI API.
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
                organization_id.value,
            );
        }

        builder
    }
}

// Audio APIs
#[cfg(feature = "audio")]
impl Client {
    /// Speeches the given text.
    ///
    /// ## NOTE
    /// This is only available for the `audio` feature flag.
    ///
    /// ## Arguments
    /// - `request_body` - The request body of the speech.
    ///
    /// ## Example
    /// ```no_run
    /// use oaapi::Client;
    /// use oaapi::audio::SpeechRequestBody;
    /// use oaapi::audio::SpeechInput;
    /// use oaapi::audio::Voice;
    ///
    /// use tokio_stream::StreamExt;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     // 1. Create a client with the API key from the environment variable: "OPENAI_API_KEY"
    ///     let client = Client::from_env()?;
    ///     // or specify the API key directly.
    ///     // let client = Client::new(oaapi::ApiKey::new("OPENAI_API_KEY"), None, None);
    ///
    ///     // 2. Create a request body parameters.
    ///     let request_body = SpeechRequestBody {
    ///         input: SpeechInput::new("Text to speech.")?,
    ///         voice: Voice::Alloy,
    ///         ..Default::default()
    ///     };
    ///
    ///     // 3. Call the API.
    ///     let mut stream = client
    ///         .audio_speech(request_body)
    ///         .await?;
    ///
    ///     // 4. Read the stream of the speech data.
    ///     while let Some(chunk) = stream.next().await {
    ///         // Do something with the chunk.
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn audio_speech(
        &self,
        request_body: SpeechRequestBody,
    ) -> ApiResult<impl Stream<Item = SpeechStreamResult>> {
        crate::audio::speech(&self, request_body).await
    }

    /// Transcribes the given audio into the JSON.
    ///
    /// ## NOTE
    /// This is only available for the `audio` feature flag.
    ///
    /// ## Arguments
    /// - `request_body` - The request body of the transcriptions.
    ///
    /// ## Example
    /// ```no_run
    /// use oaapi::Client;
    /// use oaapi::audio::File;
    /// use oaapi::audio::TranscriptionsRequestBody;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     // 1. Create a client with the API key from the environment variable: "OPENAI_API_KEY"
    ///     let client = Client::from_env()?;
    ///     // or specify the API key directly.
    ///     // let client = Client::new(oaapi::ApiKey::new("OPENAI_API_KEY"), None, None);
    ///
    ///     // 2. Load the audio file that you want to transcribe.
    ///     let file_path = "path/to/audio/file.mp3";
    ///     let file = tokio::fs::read(file_path).await?;
    ///     let file = File::new(file_path, file)?;
    ///
    ///     // 3. Create a request body parameters.
    ///     let request_body = TranscriptionsRequestBody {
    ///         file,
    ///         ..Default::default()
    ///     };
    ///
    ///     // 4. Call the API with specifying the response format.
    ///     let response = client
    ///         .audio_transcribe_into_json(request_body)
    ///         .await?;
    ///
    ///     // 5. Use the response.
    ///     println!("Result:\n{}", response);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn audio_transcribe_into_json(
        &self,
        request_body: TranscriptionsRequestBody,
    ) -> AudioApiResult<JsonResponse> {
        crate::audio::transcribe_into_json(&self, request_body).await
    }

    /// Transcribes the given audio into plain text.
    ///
    /// ## NOTE
    /// This is only available for the `audio` feature flag.
    ///
    /// ## Arguments
    /// - `request_body` - The request body of the transcriptions.
    ///
    /// ## Example
    /// ```no_run
    /// use oaapi::Client;
    /// use oaapi::audio::File;
    /// use oaapi::audio::TranscriptionsRequestBody;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     // 1. Create a client with the API key from the environment variable: "OPENAI_API_KEY"
    ///     let client = Client::from_env()?;
    ///     // or specify the API key directly.
    ///     // let client = Client::new(oaapi::ApiKey::new("OPENAI_API_KEY"), None, None);
    ///
    ///     // 2. Load the audio file that you want to transcribe.
    ///     let file_path = "path/to/audio/file.mp3";
    ///     let file = tokio::fs::read(file_path).await?;
    ///     let file = File::new(file_path, file)?;
    ///
    ///     // 3. Create a request body parameters.
    ///     let request_body = TranscriptionsRequestBody {
    ///         file,
    ///         ..Default::default()
    ///     };
    ///
    ///     // 4. Call the API with specifying the response format.
    ///     let response = client
    ///         .audio_transcribe_into_plain_text(request_body)
    ///         .await?;
    ///
    ///     // 5. Use the response.
    ///     println!("Result:\n{}", response);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn audio_transcribe_into_plain_text(
        &self,
        request_body: TranscriptionsRequestBody,
    ) -> AudioApiResult<String> {
        crate::audio::transcribe_into_plain_text(&self, request_body).await
    }

    /// Transcribes the given audio into the verbose JSON.
    ///
    /// ## NOTE
    /// This is only available for the `audio` feature flag.
    ///
    /// ## Arguments
    /// - `request_body` - The request body of the transcriptions.
    ///
    /// ## Example
    /// ```no_run
    /// use oaapi::Client;
    /// use oaapi::audio::File;
    /// use oaapi::audio::TranscriptionsRequestBody;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     // 1. Create a client with the API key from the environment variable: "OPENAI_API_KEY"
    ///     let client = Client::from_env()?;
    ///     // or specify the API key directly.
    ///     // let client = Client::new(oaapi::ApiKey::new("OPENAI_API_KEY"), None, None);
    ///
    ///     // 2. Load the audio file that you want to transcribe.
    ///     let file_path = "path/to/audio/file.mp3";
    ///     let file = tokio::fs::read(file_path).await?;
    ///     let file = File::new(file_path, file)?;
    ///
    ///     // 3. Create a request body parameters.
    ///     let request_body = TranscriptionsRequestBody {
    ///         file,
    ///         ..Default::default()
    ///     };
    ///
    ///     // 4. Call the API with specifying the response format.
    ///     let response = client
    ///         .audio_transcribe_into_verbose_json(request_body)
    ///         .await?;
    ///
    ///     // 5. Use the response.
    ///     println!("Result:\n{}", response);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn audio_transcribe_into_verbose_json(
        &self,
        request_body: TranscriptionsRequestBody,
    ) -> AudioApiResult<VerboseJsonResponse> {
        crate::audio::transcribe_into_verbose_json(&self, request_body).await
    }

    /// Transcribes the given audio into the SubRip Subtitle.
    ///
    /// ## NOTE
    /// This is only available for the `audio` feature flag.
    ///
    /// ## Arguments
    /// - `request_body` - The request body of the transcriptions.
    ///
    /// ## Example
    /// ```no_run
    /// use oaapi::Client;
    /// use oaapi::audio::File;
    /// use oaapi::audio::TranscriptionsRequestBody;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     // 1. Create a client with the API key from the environment variable: "OPENAI_API_KEY"
    ///     let client = Client::from_env()?;
    ///     // or specify the API key directly.
    ///     // let client = Client::new(oaapi::ApiKey::new("OPENAI_API_KEY"), None, None);
    ///
    ///     // 2. Load the audio file that you want to transcribe.
    ///     let file_path = "path/to/audio/file.mp3";
    ///     let file = tokio::fs::read(file_path).await?;
    ///     let file = File::new(file_path, file)?;
    ///
    ///     // 3. Create a request body parameters.
    ///     let request_body = TranscriptionsRequestBody {
    ///         file,
    ///         ..Default::default()
    ///     };
    ///
    ///     // 4. Call the API with specifying the response format.
    ///     let response = client
    ///         .audio_transcribe_into_srt(request_body)
    ///         .await?;
    ///
    ///     // 5. Use the response.
    ///     println!("Result:\n{}", response);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn audio_transcribe_into_srt(
        &self,
        request_body: TranscriptionsRequestBody,
    ) -> AudioApiResult<SubRip> {
        crate::audio::transcribe_into_srt(&self, request_body).await
    }

    /// Transcribes the given audio into the WebVTT.
    ///
    /// ## NOTE
    /// This is only available for the `audio` feature flag.
    ///
    /// ## Arguments
    /// - `request_body` - The request body of the transcriptions.
    ///
    /// ## Example
    /// ```no_run
    /// use oaapi::Client;
    /// use oaapi::audio::File;
    /// use oaapi::audio::TranscriptionsRequestBody;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     // 1. Create a client with the API key from the environment variable: "OPENAI_API_KEY"
    ///     let client = Client::from_env()?;
    ///     // or specify the API key directly.
    ///     // let client = Client::new(oaapi::ApiKey::new("OPENAI_API_KEY"), None, None);
    ///
    ///     // 2. Load the audio file that you want to transcribe.
    ///     let file_path = "path/to/audio/file.mp3";
    ///     let file = tokio::fs::read(file_path).await?;
    ///     let file = File::new(file_path, file)?;
    ///
    ///     // 3. Create a request body parameters.
    ///     let request_body = TranscriptionsRequestBody {
    ///         file,
    ///         ..Default::default()
    ///     };
    ///
    ///     // 4. Call the API with specifying the response format.
    ///     let response = client
    ///         .audio_transcribe_into_vtt(request_body)
    ///         .await?;
    ///
    ///     // 5. Use the response.
    ///     println!("Result:\n{}", response);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn audio_transcribe_into_vtt(
        &self,
        request_body: TranscriptionsRequestBody,
    ) -> AudioApiResult<WebVtt> {
        crate::audio::transcribe_into_vtt(&self, request_body).await
    }

    /// Translates the given audio into the JSON.
    ///
    /// ## NOTE
    /// This is only available for the `audio` feature flag.
    ///
    /// ## Arguments
    /// - `request_body` - The request body of the translations.
    ///
    /// ## Example
    /// ```no_run
    /// use oaapi::Client;
    /// use oaapi::audio::File;
    /// use oaapi::audio::TranslationsRequestBody;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     // 1. Create a client with the API key from the environment variable: "OPENAI_API_KEY"
    ///     let client = Client::from_env()?;
    ///     // or specify the API key directly.
    ///     // let client = Client::new(oaapi::ApiKey::new("OPENAI_API_KEY"), None, None);
    ///
    ///     // 2. Load the audio file that you want to translate.
    ///     let file_path = "path/to/audio/file.mp3";
    ///     let file = tokio::fs::read(file_path).await?;
    ///     let file = File::new(file_path, file)?;
    ///
    ///     // 3. Create a request body parameters.
    ///     let request_body = TranslationsRequestBody {
    ///         file,
    ///         ..Default::default()
    ///     };
    ///
    ///     // 4. Call the API with specifying the response format.
    ///     let response = client
    ///         .audio_translate_into_json(request_body)
    ///         .await?;
    ///
    ///     // 5. Use the response.
    ///     println!("Result:\n{}", response);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn audio_translate_into_json(
        &self,
        request_body: TranslationsRequestBody,
    ) -> AudioApiResult<JsonResponse> {
        crate::audio::translate_into_json(&self, request_body).await
    }

    /// Translates the given audio into plain text.
    ///
    /// ## NOTE
    /// This is only available for the `audio` feature flag.
    ///
    /// ## Arguments
    /// - `request_body` - The request body of the translations.
    ///
    /// ## Example
    /// ```no_run
    /// use oaapi::Client;
    /// use oaapi::audio::File;
    /// use oaapi::audio::TranslationsRequestBody;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     // 1. Create a client with the API key from the environment variable: "OPENAI_API_KEY"
    ///     let client = Client::from_env()?;
    ///     // or specify the API key directly.
    ///     // let client = Client::new(oaapi::ApiKey::new("OPENAI_API_KEY"), None, None);
    ///
    ///     // 2. Load the audio file that you want to translate.
    ///     let file_path = "path/to/audio/file.mp3";
    ///     let file = tokio::fs::read(file_path).await?;
    ///     let file = File::new(file_path, file)?;
    ///
    ///     // 3. Create a request body parameters.
    ///     let request_body = TranslationsRequestBody {
    ///         file,
    ///         ..Default::default()
    ///     };
    ///
    ///     // 4. Call the API with specifying the response format.
    ///     let response = client
    ///         .audio_translate_into_plain_text(request_body)
    ///         .await?;
    ///
    ///     // 5. Use the response.
    ///     println!("Result:\n{}", response);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn audio_translate_into_plain_text(
        &self,
        request_body: TranslationsRequestBody,
    ) -> AudioApiResult<String> {
        crate::audio::translate_into_plain_text(&self, request_body).await
    }

    /// Translates the given audio into the verbose JSON.
    ///
    /// ## NOTE
    /// This is only available for the `audio` feature flag.
    ///
    /// ## Arguments
    /// - `request_body` - The request body of the translations.
    ///
    /// ## Example
    /// ```no_run
    /// use oaapi::Client;
    /// use oaapi::audio::File;
    /// use oaapi::audio::TranslationsRequestBody;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     // 1. Create a client with the API key from the environment variable: "OPENAI_API_KEY"
    ///     let client = Client::from_env()?;
    ///     // or specify the API key directly.
    ///     // let client = Client::new(oaapi::ApiKey::new("OPENAI_API_KEY"), None, None);
    ///
    ///     // 2. Load the audio file that you want to translate.
    ///     let file_path = "path/to/audio/file.mp3";
    ///     let file = tokio::fs::read(file_path).await?;
    ///     let file = File::new(file_path, file)?;
    ///
    ///     // 3. Create a request body parameters.
    ///     let request_body = TranslationsRequestBody {
    ///         file,
    ///         ..Default::default()
    ///     };
    ///
    ///     // 4. Call the API with specifying the response format.
    ///     let response = client
    ///         .audio_translate_into_verbose_json(request_body)
    ///         .await?;
    ///
    ///     // 5. Use the response.
    ///     println!("Result:\n{}", response);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn audio_translate_into_verbose_json(
        &self,
        request_body: TranslationsRequestBody,
    ) -> AudioApiResult<VerboseJsonResponse> {
        crate::audio::translate_into_verbose_json(&self, request_body).await
    }

    /// Translates the given audio into the SubRip Subtitle.
    ///
    /// ## NOTE
    /// This is only available for the `audio` feature flag.
    ///
    /// ## Arguments
    /// - `request_body` - The request body of the translations.
    ///
    /// ## Example
    /// ```no_run
    /// use oaapi::Client;
    /// use oaapi::audio::File;
    /// use oaapi::audio::TranslationsRequestBody;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     // 1. Create a client with the API key from the environment variable: "OPENAI_API_KEY"
    ///     let client = Client::from_env()?;
    ///     // or specify the API key directly.
    ///     // let client = Client::new(oaapi::ApiKey::new("OPENAI_API_KEY"), None, None);
    ///
    ///     // 2. Load the audio file that you want to translate.
    ///     let file_path = "path/to/audio/file.mp3";
    ///     let file = tokio::fs::read(file_path).await?;
    ///     let file = File::new(file_path, file)?;
    ///
    ///     // 3. Create a request body parameters.
    ///     let request_body = TranslationsRequestBody {
    ///         file,
    ///         ..Default::default()
    ///     };
    ///
    ///     // 4. Call the API with specifying the response format.
    ///     let response = client
    ///         .audio_translate_into_srt(request_body)
    ///         .await?;
    ///
    ///     // 5. Use the response.
    ///     println!("Result:\n{}", response);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn audio_translate_into_srt(
        &self,
        request_body: TranslationsRequestBody,
    ) -> AudioApiResult<SubRip> {
        crate::audio::translate_into_srt(&self, request_body).await
    }

    /// Translates the given audio into the WebVTT.
    ///
    /// ## NOTE
    /// This is only available for the `audio` feature flag.
    ///
    /// ## Arguments
    /// - `request_body` - The request body of the translations.
    ///
    /// ## Example
    /// ```no_run
    /// use oaapi::Client;
    /// use oaapi::audio::File;
    /// use oaapi::audio::TranslationsRequestBody;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     // 1. Create a client with the API key from the environment variable: "OPENAI_API_KEY"
    ///     let client = Client::from_env()?;
    ///     // or specify the API key directly.
    ///     // let client = Client::new(oaapi::ApiKey::new("OPENAI_API_KEY"), None, None);
    ///
    ///     // 2. Load the audio file that you want to translate.
    ///     let file_path = "path/to/audio/file.mp3";
    ///     let file = tokio::fs::read(file_path).await?;
    ///     let file = File::new(file_path, file)?;
    ///
    ///     // 3. Create a request body parameters.
    ///     let request_body = TranslationsRequestBody {
    ///         file,
    ///         ..Default::default()
    ///     };
    ///
    ///     // 4. Call the API with specifying the response format.
    ///     let response = client
    ///         .audio_translate_into_vtt(request_body)
    ///         .await?;
    ///
    ///     // 5. Use the response.
    ///     println!("Result:\n{}", response);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn audio_translate_into_vtt(
        &self,
        request_body: TranslationsRequestBody,
    ) -> AudioApiResult<WebVtt> {
        crate::audio::translate_into_vtt(&self, request_body).await
    }
}

// Chat APIs
#[cfg(feature = "chat")]
impl Client {
    /// Completes the given chat.
    ///
    /// ## NOTE
    /// - This is only available for the `chat` feature flag.
    /// - Specify `stream` option to `StreamOption::ReturnOnce` or `None` to disable streaming.
    ///
    /// ## Arguments
    /// - `request_body` - The request body of the completions.
    ///
    /// ## Example
    /// ```no_run
    /// use oaapi::Client;
    /// use oaapi::chat::CompletionsRequestBody;
    /// use oaapi::chat::SystemMessage;
    /// use oaapi::chat::UserMessage;
    /// use oaapi::chat::ChatModel;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     // 1. Create a client with the API key from the environment variable: "OPENAI_API_KEY"
    ///     let client = Client::from_env()?;
    ///     // or specify the API key directly.
    ///     // let client = Client::new(oaapi::ApiKey::new("OPENAI_API_KEY"), None, None);
    ///
    ///     // 2. Create a request body parameters.
    ///     let request_body = CompletionsRequestBody {
    ///         messages: vec![
    ///             SystemMessage::new("Prompt.", None).into(),
    ///             UserMessage::new("Chat message from user.".into(), None).into(),
    ///         ],
    ///         model: ChatModel::Gpt35Turbo,
    ///         ..Default::default()
    ///     };
    ///
    ///     // 3. Call the API.
    ///     let response = client
    ///         .chat_complete(request_body)
    ///         .await?;
    ///
    ///     // 4. Use the response.
    ///     println!("Result:\n{}", response);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn chat_complete(
        &self,
        request_body: CompletionsRequestBody,
    ) -> ChatApiResult<ChatCompletionObject> {
        crate::chat::complete(&self, request_body).await
    }

    /// Completes the given chat with the stream.
    ///
    /// ## NOTE
    /// - This is only available for the `chat` feature flag.
    /// - Specify `stream` option to `StreamOption::ReturnStream` to enable streaming.
    ///
    /// ## Arguments
    /// - `request_body` - The request body of the completions.
    ///
    /// ## Example
    /// ```no_run
    /// use oaapi::Client;
    /// use oaapi::chat::CompletionsRequestBody;
    /// use oaapi::chat::SystemMessage;
    /// use oaapi::chat::UserMessage;
    /// use oaapi::chat::ChatModel;
    /// use oaapi::chat::StreamOption;
    ///
    /// use tokio_stream::StreamExt;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     // 1. Create a client with the API key from the environment variable: "OPENAI_API_KEY"
    ///     let client = Client::from_env()?;
    ///     // or specify the API key directly.
    ///     // let client = Client::new(oaapi::ApiKey::new("OPENAI_API_KEY"), None, None);
    ///
    ///     // 2. Create a request body parameters with specifying the streaming option: `StreamOption::ReturnStream`.
    ///     let request_body = CompletionsRequestBody {
    ///         messages: vec![
    ///             SystemMessage::new("Prompt.", None).into(),
    ///             UserMessage::new("Chat message from user.".into(), None).into(),
    ///         ],
    ///         model: ChatModel::Gpt35Turbo,
    ///         stream: Some(StreamOption::ReturnStream),
    ///         ..Default::default()
    ///     };
    ///
    ///     // 3. Call the API.
    ///     let mut stream = client
    ///         .chat_complete_stream(request_body)
    ///         .await?;
    ///
    ///     // 4. Receive the response stream.
    ///     while let Some(response) = stream.next().await {
    ///         // Do something with the response.
    ///         println!("Chunk:\n{}", response);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn chat_complete_stream(
        &self,
        request_body: CompletionsRequestBody,
    ) -> ChatApiResult<impl Stream<Item = ChatChunkResult>> {
        crate::chat::complete_stream(&self, request_body).await
    }
}
