//! Completes chat by /chat/completions endpoint with limited tokens and stop sequences.
//!
//! ```shell
//! $ cargo run --example chat_completions_limited -- --prompt <prompt> --message <message> --max-tokens <max-tokens>
//! ```

use clap::Parser;

use openai::chat::complete;
use openai::chat::ChatModel;
use openai::chat::CompletionsRequestBody;
use openai::chat::MaxTokens;
use openai::chat::StopOption;
use openai::chat::SystemMessage;
use openai::chat::UserMessage;
use openai::ApiKey;

#[derive(Parser)]
struct Arguments {
    #[arg(short, long)]
    prompt: String,
    #[arg(long)]
    message: String,
    #[arg(long)]
    max_tokens: u32,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let arguments = Arguments::parse();
    let client = reqwest::Client::new();
    let api_key = ApiKey::from_env()?;

    let model = ChatModel::Gpt35Turbo;
    let max_tokens = MaxTokens::new(
        arguments.max_tokens, // Max tokens.
        model,
    )?;
    let stop_option = StopOption::new_up_to_4(vec!["\\n", ".", "。"])?;

    let request_body = CompletionsRequestBody {
        messages: vec![
            SystemMessage::new(arguments.prompt, None).into(),
            UserMessage::new(arguments.message.into(), None).into(),
        ],
        model,
        max_tokens: Some(max_tokens), // Specify max tokens.
        stop: Some(stop_option),      // Specify stop sequences.
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
