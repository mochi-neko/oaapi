//! Completes chat streaming by /chat/completions endpoint with stream response.
//!
//! ```shell
//! $ cargo run --example chat_completions_stream --features chat -- --prompt <prompt> --message <message>
//! ```
//!
//! e.g.
//! ```shell
//! $ cargo run --example chat_completions_stream --features chat -- --prompt "You are a excellent AI assistant." --message "Where is the capital of Japan?"
//! ```

use clap::Parser;
use tokio_stream::StreamExt;

use oaapi::chat::ChatModel;
use oaapi::chat::CompletionsRequestBody;
use oaapi::chat::StreamOption;
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

    // 2. Create a request body parameters with specifying the streaming option: `StreamOption::ReturnStream`.
    let request_body = CompletionsRequestBody {
        messages: vec![
            SystemMessage::new(arguments.prompt, None).into(),
            UserMessage::new(arguments.message.into(), None).into(),
        ],
        model: ChatModel::Gpt35Turbo,
        stream: Some(StreamOption::ReturnStream),
        ..Default::default()
    };

    // 3. Call the API.
    let mut stream = client
        .chat_complete_stream(request_body)
        .await?;

    // 4. Set up text buffer.
    let mut text_buffer = String::new();

    // 5. Receive the response stream.
    while let Some(response) = stream.next().await {
        match response {
            | Ok(chunk) => {
                println!("Delta: {}", chunk);

                if let Some(delta) = chunk
                    .choices
                    .first()
                    .unwrap()
                    .delta
                    .as_ref()
                    .unwrap()
                    .content
                    .clone()
                {
                    // 6. Buffer delta result.
                    text_buffer.push_str(delta.clone().as_str());
                }
            },
            | Err(error) => {
                eprintln!(
                    "Error: {}, buffer: {}",
                    error, text_buffer
                );
                return Err(error.into());
            },
        }
    }

    // 7. Use the total response.
    println!("Result:\n{}", text_buffer);

    Ok(())
}
