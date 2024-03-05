//! Transcribes audio into a text with timestamp granularities in verbose JSON format of the input language by /audio/transcriptions endpoint.
//!
//! ```shell
//! $ cargo run --example audio_transcriptions_timestamp_granularities -- --file-path <file-path>
//! ```

use std::path::Path;

use clap::Parser;

use oaapi::audio::TranscriptionsRequestBody;
use oaapi::audio::{File, TimestampGranularity};
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
        timestamp_granularities: Some(vec![TimestampGranularity::Word]),
        ..Default::default()
    };

    // NOTE: Timestamp granularities are only available in verbose JSON format.
    let response = client
        .audio_transcribe_into_verbose_json(request_body)
        .await?;

    println!("Result:\n{}", response);

    Ok(())
}
