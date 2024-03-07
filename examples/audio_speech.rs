//! Speech a text by /audio/speech endpoint.
//!
//! ```shell
//! $ cargo run --example audio_speech --features audio -- --text <text> --voice <voice> --output <path/to/output>
//! ```

use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use clap::Parser;
use tokio::io::AsyncWriteExt;

use oaapi::audio::SpeechInput;
use oaapi::audio::SpeechRequestBody;
use oaapi::audio::SpeechResponseFormat;
use oaapi::audio::Voice;
use oaapi::Client;

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
    let client = Client::from_env()?;

    let request_body = SpeechRequestBody {
        input: SpeechInput::new(arguments.text)?,
        voice: Voice::from_str(&arguments.voice)?,
        response_format: Some(SpeechResponseFormat::Mp3),
        ..Default::default()
    };

    let (mut receiver, handle) = client
        .audio_speech(request_body, None)
        .await?;

    let mut file = tokio::fs::File::create(arguments.output.clone()).await?;

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;

    let r = running.clone();

    let receive_handle = tokio::spawn(async move {
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

        r.store(false, Ordering::SeqCst);
    });

    while running.load(Ordering::SeqCst) {}

    receive_handle.abort();
    handle.abort();

    Ok(())
}
