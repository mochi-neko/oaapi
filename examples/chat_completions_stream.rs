//! Completes chat streaming by /chat/completions endpoint with stream response.
//!
//! ```shell
//! $ cargo run --example chat_completions_stream -- --prompt <prompt> --message <message>
//! ```

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use clap::Parser;

use openai::chat::complete_stream;
use openai::chat::ChatModel;
use openai::chat::CompletionsRequestBody;
use openai::chat::StreamOption;
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
        stream: Some(StreamOption::ReturnStream), // Enable streaming.
        ..Default::default()
    };

    let (mut receiver, stream_handle) =
        complete_stream(&client, &api_key, request_body, 100).await?;

    let mut text_buffer = String::new();

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;

    while running.load(Ordering::SeqCst) {
        while let Some(response) = receiver.recv().await {
            match response {
                | Ok(chunk) => {
                    println!("Delta: {:?}", chunk);
                    text_buffer.push_str(
                        chunk
                            .choices
                            .first()
                            .unwrap()
                            .delta
                            .content
                            .as_ref()
                            .unwrap()
                            .as_str(),
                    );
                },
                | Err(error) => {
                    eprintln!(
                        "Error: {}, buffer: {}",
                        error, text_buffer
                    );
                    stream_handle.abort();
                    return Err(error.into());
                },
            }
        }

        println!(
            "Finish streaming with buffer: {}",
            text_buffer
        );
        stream_handle.abort();
        return Ok(());
    }

    println!(
        "Cancel streaming with buffer: {}",
        text_buffer
    );
    stream_handle.abort();
    Ok(())
}
