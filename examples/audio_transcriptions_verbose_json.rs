//! Transcribes audio into a text with verbose JSON format of the input language by /audio/transcriptions endpoint.
//!
//! ```shell
//! $ cargo run --example audio_transcriptions_verbose_json --features audio -- --file-path <file-path>
//! ```

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

    let file = tokio::fs::read(arguments.file_path.clone()).await?;
    let file = File::new(arguments.file_path, file)?;

    let request_body = TranscriptionsRequestBody {
        file,
        ..Default::default()
    };

    let response = client
        .audio_transcribe_into_verbose_json(request_body)
        .await?;

    println!("Result:\n{}", response.text);

    Ok(())
}
