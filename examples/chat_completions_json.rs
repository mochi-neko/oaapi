//! Completes chat by /chat/completions endpoint with JSON response.
//!
//! ```shell
//! $ cargo run --example chat_completions_json -- --message <message>
//! ```

use clap::Parser;
use serde::{Deserialize, Serialize};

use openai::chat::complete;
use openai::chat::ChatModel;
use openai::chat::CompletionsRequestBody;
use openai::chat::JsonResponseFormat;
use openai::chat::SystemMessage;
use openai::chat::UserMessage;
use openai::ApiKey;

#[derive(Parser)]
struct Arguments {
    #[arg(short, long)]
    message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct CustomResponse {
    expert: String,
    advice: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let arguments = Arguments::parse();
    let client = reqwest::Client::new();
    let api_key = ApiKey::from_env()?;

    let prompt = r#"
        You are an AI assitant that helps people with their problems.
        You can give advice on any topic as its expert.
        Please reply to the following user message with your advice as an expert what you think suitable by using the following JSON format:
        
        {
            "expert": "expert name",
            "advice": "advice"
        }
        
        EXAMPLE:
        Q. "I have a headache."
        A.
        {
            "expert": "Doctor",
            "advice": "Take some medicine."
        }
        
        Q. "I want to be professional football player."
        A.
        {
            "expert": "Football coach",
            "advice": "Practice more."
        }
        END OF EXAMPLE
    "#; // Instruct to custom JSON response format with few examples.

    let request_body = CompletionsRequestBody {
        messages: vec![
            SystemMessage::new(prompt, None).into(),
            UserMessage::new(arguments.message.into(), None).into(),
        ],
        model: ChatModel::Gpt35Turbo1106, // Specify JSON response enabled model
        response_format: Some(JsonResponseFormat::new().into()), // Specify JSON response format
        ..Default::default()
    };

    let response = complete(&client, &api_key, request_body).await?;

    let content = response
        .choices
        .first()
        .unwrap()
        .message
        .content
        .as_ref()
        .unwrap();

    // Deserialize custom JSON response.
    let custom_response = serde_json::from_str::<CustomResponse>(&content)?;

    println!("Result: {:?}", custom_response);

    Ok(())
}
