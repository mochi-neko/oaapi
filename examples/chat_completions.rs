//! Completes chat by /chat/completions endpoint.
//!
//! ```shell
//! $ cargo run --example chat_completions -- --prompt <prompt> --message <message>
//! ```

use clap::Parser;

use openai::chat::complete;
use openai::chat::ChatModel;
use openai::chat::CompletionsRequestBody;
use openai::chat::SystemMessage;
use openai::chat::UserMessage;
use openai::ApiKey;

#[derive(Parser)]
struct Arguments {
    #[arg(short, long)]
    prompt: String,
    #[arg(short, long)]
    message: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let arguments = Arguments::parse();
    let client = reqwest::Client::new();
    let api_key = ApiKey::from_env()?;

    let request_body = CompletionsRequestBody {
        messages: vec![
            SystemMessage::new(arguments.prompt, None).into(),
            UserMessage::new(arguments.message.into(), None).into(),
        ],
        model: ChatModel::Gpt35Turbo,
        ..Default::default()
    };

    let response = complete(&client, &api_key, request_body).await?;

    println!(
        "Result: {:?}",
        response
            .choices
            .first()
            .unwrap()
            .message
    );

    Ok(())
}
