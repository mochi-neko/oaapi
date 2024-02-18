//! Transcribes audio into a text of the input language by /audio/transcriptions endpoint.
//!
//! ```shell
//! $ cargo run --example audio_transcriptions -- --file-path <file-path>
//! ```

use std::path::Path;

use clap::Parser;

use openai::audio::File;
use openai::audio::TranscriptionsRequestBody;
use openai::ApiKey;

#[derive(Parser)]
struct Arguments {
    #[arg(short, long)]
    file_path: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let arguments = Arguments::parse();
    let client = reqwest::Client::new();
    let api_key = ApiKey::from_env()?;

    let request_body = TranscriptionsRequestBody {
        file: File::from_file_path(
            Path::new(&arguments.file_path).to_path_buf(),
        )?,
        ..Default::default()
    };

    let response = openai::audio::transcribe_into_plain_text(
        &client,
        &api_key,
        request_body,
    )
    .await?;

    println!("Result: {}", response);

    Ok(())
}
