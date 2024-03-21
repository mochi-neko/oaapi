//! Completes chat by /chat/completions endpoint with JSON response.
//!
//! ```shell
//! $ cargo run --example chat_completions_json --features chat -- --message <message>
//! ```

use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use oaapi::chat::ChatModel;
use oaapi::chat::CompletionsRequestBody;
use oaapi::chat::ResponseFormatType;
use oaapi::chat::SystemMessage;
use oaapi::chat::UserMessage;
use oaapi::Client;

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

impl Display for CustomResponse {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "Expert: {}, Advice: {}",
            self.expert, self.advice
        )
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let arguments = Arguments::parse();

    // 1. Create a client with the API key from the environment variable: "OPENAI_API_KEY"
    let client = Client::from_env()?;
    // or specify the API key directly.
    // let client = Client::new(oaapi::ApiKey::new("OPENAI_API_KEY"), None, None);

    // 2. Set up a prompt that instructs the JSON response format.
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

    // 3. Create a request body parameters with the JSON response format.
    let request_body = CompletionsRequestBody {
        messages: vec![
            SystemMessage::new(prompt, None).into(),
            UserMessage::new(arguments.message.into(), None).into(),
        ],
        model: ChatModel::Gpt35Turbo1106, // Specify JSON response enabled model
        response_format: Some(ResponseFormatType::Json.into()), // Specify JSON response format
        ..Default::default()
    };

    // 3. Call the API.
    let response = client
        .chat_complete(request_body)
        .await?;

    // 4. Take message content from the response.
    let content = response
        .choices
        .first()
        .unwrap()
        .message
        .content
        .as_ref()
        .unwrap();

    // 5. Deserialize message to the custom JSON response.
    let custom_response = serde_json::from_str::<CustomResponse>(&content)?;

    // 6. Use the response.
    println!("Result:\n{}", custom_response);

    Ok(())
}
