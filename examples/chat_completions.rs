//! Completes chat by /chat/completions endpoint.
//!
//! ```shell
//! $ cargo run --example chat_completions --features chat -- --prompt <prompt> --message <message>
//! ```

use clap::Parser;

use oaapi::chat::ChatModel;
use oaapi::chat::CompletionsRequestBody;
use oaapi::chat::SystemMessage;
use oaapi::chat::UserMessage;
use oaapi::Client;

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
    let client = Client::from_env()?;

    let request_body = CompletionsRequestBody {
        messages: vec![
            SystemMessage::new(arguments.prompt, None).into(),
            UserMessage::new(arguments.message.into(), None).into(),
        ],
        model: ChatModel::Gpt35Turbo,
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
