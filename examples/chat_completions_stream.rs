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

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use clap::Parser;
use futures_util::stream::StreamExt;

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
    let client = Client::from_env()?;

    let request_body = CompletionsRequestBody {
        messages: vec![
            SystemMessage::new(arguments.prompt, None).into(),
            UserMessage::new(arguments.message.into(), None).into(),
        ],
        model: ChatModel::Gpt35Turbo,
        stream: Some(StreamOption::ReturnStream), // Enable streaming.
        ..Default::default()
    };

    let mut stream = client
        .chat_complete_stream(request_body)
        .await?;

    let mut text_buffer = String::new();

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;

    while let Some(response) = stream.next().await {
        if !running.load(Ordering::SeqCst) {
            println!(
                "Cancel streaming with buffer: {}",
                text_buffer
            );
            return Ok(());
        }

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

    println!("Result:\n{}", text_buffer);

    Ok(())
}
