//! Speech a text by /audio/speech endpoint.
//!
//! ```shell
//! $ cargo run --example audio_speech -- --text <text> --voice <voice> --output <path/to/output>
//! ```

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use clap::Parser;
use tokio::io::AsyncWriteExt;

use openai::audio::speech;
use openai::audio::SpeechInput;
use openai::audio::SpeechRequestBody;
use openai::audio::SpeechResponseFormat;
use openai::ApiKey;

#[derive(Parser)]
struct Arguments {
    #[arg(short, long)]
    text: String,
    #[arg(short, long)]
    voice: String,
    #[arg(short, long)]
    output: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let arguments = Arguments::parse();
    let client = reqwest::Client::new();
    let api_key = ApiKey::from_env()?;

    let request_body = SpeechRequestBody {
        input: SpeechInput::new(arguments.text)?,
        voice: arguments.voice.try_into()?,
        response_format: Some(SpeechResponseFormat::Mp3),
        ..Default::default()
    };

    let (mut receiver, handle) =
        speech(&client, &api_key, request_body, None).await?;

    let mut file = tokio::fs::File::create(arguments.output.clone()).await?;

    let handle = tokio::spawn(async move {
        while let Some(chunk) = receiver.recv().await {
            match chunk {
                | Ok(chunk) => {
                    _ = file.write(&chunk).await;
                },
                | Err(error) => {
                    eprintln!("Error to receive data: {:?}", error);
                    break;
                },
            }
        }

        println!(
            "Save the speech to {}",
            arguments.output
        );
    });

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;

    while running.load(Ordering::SeqCst) {}

    handle.abort();

    Ok(())
}
