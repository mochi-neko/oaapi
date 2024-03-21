use std::collections::HashMap;

use futures_core::Stream;
use serde::{Deserialize, Serialize};

use crate::chat::chunk_stream::ChunkStream;
use crate::chat::Bias;
use crate::chat::ChatApiError;
use crate::chat::ChatApiResult;
use crate::chat::ChatChunkResult;
use crate::chat::ChatCompletionObject;
use crate::chat::ChatModel;
use crate::chat::LogprobsOption;
use crate::chat::MaxTokens;
use crate::chat::Message;
use crate::chat::Penalty;
use crate::chat::ResponseFormat;
use crate::chat::StopOption;
use crate::chat::StreamOption;
use crate::chat::Tool;
use crate::chat::ToolChoice;
use crate::chat::TopLogprobs;
use crate::chat::TopP;
use crate::ApiError;
use crate::Client;
use crate::ClientError;
use crate::Temperature;

/// The request body for the `/chat/completions` endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionsRequestBody {
    /// A list of messages comprising the conversation so far.
    pub messages: Vec<Message>,

    /// ID of the model to use. See the model endpoint compatibility table for details on which models work with the Chat API.
    pub model: ChatModel,

    /// Number between -2.0 and 2.0.
    /// Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<Penalty>,

    /// Modify the likelihood of specified tokens appearing in the completion.
    ///
    /// Accepts a JSON object that maps tokens (specified by their token ID in the tokenizer) to an associated bias value from -100 to 100.
    /// Mathematically, the bias is added to the logits generated by the model prior to sampling.
    /// The exact effect will vary per model, but values between -1 and 1 should decrease or increase likelihood of selection; values like -100 or 100 should result in a ban or exclusive selection of the relevant token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, Bias>>,

    /// Whether to return log probabilities of the output tokens or not.
    /// If true, returns the log probabilities of each output token returned in the content of message.
    /// This option is currently not available on the gpt-4-vision-preview model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<LogprobsOption>,

    /// An integer between 0 and 5 specifying the number of most likely tokens to return at each token position, each with an associated log probability.
    /// logprobs must be set to true if this parameter is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<TopLogprobs>,

    /// The maximum number of tokens that can be generated in the chat completion.
    ///
    /// The total length of input tokens and generated tokens is limited by the model's context length.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<MaxTokens>,

    /// How many chat completion choices to generate for each input message.
    /// Note that you will be charged based on the number of generated tokens across all of the choices.
    /// Keep n as 1 to minimize costs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,

    /// Number between -2.0 and 2.0.
    /// Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<Penalty>,

    /// An object specifying the format that the model must output.
    /// Compatible with gpt-4-1106-preview and gpt-3.5-turbo-1106.
    ///
    /// Setting to { "type": "json_object" } enables JSON mode, which guarantees the message the model generates is valid JSON.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,

    /// This feature is in Beta.
    /// If specified, our system will make a best effort to sample deterministically, such that repeated requests with the same seed and parameters should return the same result.
    /// Determinism is not guaranteed, and you should refer to the system_fingerprint response parameter to monitor changes in the backend.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,

    /// Up to 4 sequences where the API will stop generating further tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<StopOption>,

    /// If set, partial message deltas will be sent, like in ChatGPT.
    /// Tokens will be sent as data-only server-sent events as they become available, with the stream terminated by a `data: [DONE]` message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<StreamOption>,

    /// What sampling temperature to use, between 0 and 2.
    /// Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.
    ///
    /// We generally recommend altering this or top_p but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<Temperature>,

    /// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass.
    /// So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    ///
    /// We generally recommend altering this or temperature but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<TopP>,

    /// A list of tools the model may call.
    /// Currently, only functions are supported as a tool.
    /// Use this to provide a list of functions the model may generate JSON inputs for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,

    /// Controls which (if any) function is called by the model.
    /// none means the model will not call a function and instead generates a message.
    /// auto means the model can pick between generating a message or calling a function.
    /// Specifying a particular function via {"type": "function", "function": {"name": "my_function"}} forces the model to call that function.
    ///
    /// none is the default when no functions are present. auto is the default if functions are present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,

    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl Default for CompletionsRequestBody {
    fn default() -> Self {
        Self {
            messages: Vec::new(),
            model: ChatModel::Gpt35Turbo,
            frequency_penalty: None,
            logit_bias: None,
            logprobs: None,
            top_logprobs: None,
            max_tokens: None,
            n: None,
            presence_penalty: None,
            response_format: None,
            seed: None,
            stop: None,
            stream: None,
            temperature: None,
            top_p: None,
            tools: None,
            tool_choice: None,
            user: None,
        }
    }
}

pub(crate) async fn complete(
    client: &Client,
    request_body: CompletionsRequestBody,
) -> ChatApiResult<ChatCompletionObject> {
    // Check stream option.
    if let Some(stream) = request_body.stream {
        if stream != StreamOption::ReturnOnce {
            return Err(ChatApiError::StreamOptionMismatch);
        }
    }

    // Send the request.
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .json(&request_body)
        .send()
        .await
        .map_err(ClientError::HttpRequestError)?;

    // Check the response status code.
    let status_code = response.status();

    // Read the response text.
    let response_text = response
        .text()
        .await
        .map_err(ClientError::ReadResponseTextFailed)?;

    // Ok
    if status_code.is_success() {
        // Deserialize the response.
        serde_json::from_str(&response_text).map_err(|error| {
            {
                ClientError::ResponseDeserializationFailed {
                    error,
                    text: response_text,
                }
            }
            .into()
        })
    }
    // Error
    else {
        // Deserialize the error response.
        let error_response =
            serde_json::from_str(&response_text).map_err(|error| {
                ClientError::ErrorResponseDeserializationFailed {
                    error,
                    text: response_text,
                }
            })?;

        Err(ApiError {
            status_code,
            error_response,
        }
        .into())
    }
}

pub(crate) async fn complete_stream(
    client: &Client,
    request_body: CompletionsRequestBody,
) -> ChatApiResult<impl Stream<Item = ChatChunkResult>> {
    // Check stream option.
    if request_body.stream.is_none() {
        return Err(ChatApiError::StreamOptionMismatch);
    }
    if let Some(stream) = request_body.stream {
        if stream != StreamOption::ReturnStream {
            return Err(ChatApiError::StreamOptionMismatch);
        }
    }

    // Send the request.
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .json(&request_body)
        .send()
        .await
        .map_err(ClientError::HttpRequestError)?;

    // Check the response status code.
    let status_code = response.status();

    // Ok
    if status_code.is_success() {
        Ok(ChunkStream::new(
            response.bytes_stream(),
        ))
    }
    // Error
    else {
        // Read the response text.
        let response_text = response
            .text()
            .await
            .map_err(ClientError::ReadResponseTextFailed)?;

        // Deserialize the error response.
        let error_response =
            serde_json::from_str(&response_text).map_err(|error| {
                ClientError::ErrorResponseDeserializationFailed {
                    error,
                    text: response_text,
                }
            })?;

        Err(ApiError {
            status_code,
            error_response,
        }
        .into())
    }
}
