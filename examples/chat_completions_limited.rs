//! Completes chat by /chat/completions endpoint with limited tokens and stop sequences.
//!
//! ```shell
//! $ cargo run --example chat_completions_limited -- --prompt <prompt> --message <message> --max-tokens <max-tokens>
//! ```

use clap::Parser;

use oaapi::chat::ChatModel;
use oaapi::chat::CompletionsRequestBody;
use oaapi::chat::MaxTokens;
use oaapi::chat::StopOption;
use oaapi::chat::SystemMessage;
use oaapi::chat::UserMessage;
use oaapi::Client;

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
    let client = Client::from_env()?;

    let model = ChatModel::Gpt35Turbo;
    let max_tokens = MaxTokens::new(
        arguments.max_tokens, // Max tokens.
        model.clone(),
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

    let response = client
        .chat_complete(request_body)
        .await?;

    println!(
        "Result:\n{}",
        response
            .choices
            .first()
            .unwrap()
            .message
    );

    Ok(())
}
