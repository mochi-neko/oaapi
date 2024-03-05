//! Transcribes audio into a text with WebVTT (.vtt) format of the input language by /audio/transcriptions endpoint.
//!
//! ```shell
//! $ cargo run --example audio_transcriptions_vtt -- --file-path <file-path>
//! ```

use std::path::Path;

use clap::Parser;

use oaapi::audio::File;
use oaapi::audio::TranscriptionsRequestBody;
use oaapi::Client;

#[derive(Parser)]
struct Arguments {
    #[arg(short, long)]
    file_path: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let arguments = Arguments::parse();
    let client = Client::from_env()?;

    let request_body = TranscriptionsRequestBody {
        file: File::from_file_path(
            Path::new(&arguments.file_path).to_path_buf(),
        )?,
        ..Default::default()
    };

    let response = client
        .audio_transcribe_into_vtt(request_body)
        .await?;

    println!("Result: {}", response);

    Ok(())
}
