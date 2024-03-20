//! Transcribes audio into a text with timestamp granularities in verbose JSON format of the input language by /audio/transcriptions endpoint.
//!
//! ```shell
//! $ cargo run --example audio_transcriptions_timestamp_granularities --features audio -- --file-path <file-path>
//! ```

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

    let file = tokio::fs::read(arguments.file_path.clone()).await?;
    let file = File::new(arguments.file_path, file)?;

    let request_body = TranscriptionsRequestBody {
        file,
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
