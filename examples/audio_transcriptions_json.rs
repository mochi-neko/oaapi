//! Transcribes audio into a text with JSON format of the input language by /audio/transcriptions endpoint.
//!
//! ```shell
//! $ cargo run --example audio_transcriptions_json --features audio -- --file-path <file-path>
//! ```

use clap::Parser;
use oaapi::audio::File;
use oaapi::audio::TranscriptionsRequestBody;
use oaapi::Client;
use std::path::Path;

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
        .audio_transcribe_into_json(request_body)
        .await?;

    println!("Result:\n{}", response);

    Ok(())
}
