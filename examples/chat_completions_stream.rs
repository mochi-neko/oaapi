//! Completes chat streaming by /chat/completions endpoint with stream response.
//!
//! ```shell
//! $ cargo run --example chat_completions_stream -- --prompt <prompt> --message <message>
//! ```

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use clap::Parser;

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

    let (mut receiver, stream_handle) = client
        .chat_complete_stream(request_body, None)
        .await?;

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
