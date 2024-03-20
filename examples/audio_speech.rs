//! Speech a text by /audio/speech endpoint.
//!
//! ```shell
//! $ cargo run --example audio_speech --features audio -- --text <text> --voice <voice> --output <path/to/output>
//! ```

use std::str::FromStr;

use clap::Parser;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncWriteExt, BufWriter};
use tokio_stream::StreamExt;

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

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(arguments.output.clone())
        .await?;

    let mut writer = BufWriter::new(file);

    let mut stream = client
        .audio_speech(request_body)
        .await?;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        writer
            .write_all(&chunk)
            .await?;
    }

    writer.flush().await?;

    println!("Speech written to {}", arguments.output);

    Ok(())
}
