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

    // 1. Create a client with the API key from the environment variable: "OPENAI_API_KEY"
    let client = Client::from_env()?;
    // or specify the API key directly.
    // let client = Client::new(oaapi::ApiKey::new("OPENAI_API_KEY"), None, None);

    // 2. Create a request body parameters.
    let request_body = CompletionsRequestBody {
        messages: vec![
            SystemMessage::new(arguments.prompt, None).into(),
            UserMessage::new(arguments.message.into(), None).into(),
        ],
        model: ChatModel::Gpt35Turbo,
        ..Default::default()
    };

    // 3. Call the API.
    let response = client
        .chat_complete(request_body)
        .await?;

    // 4. Use the response.
    println!("Result:\n{}", response);

    Ok(())
}
