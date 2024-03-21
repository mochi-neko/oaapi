//! Transcribes audio into a text with WebVTT (.vtt) format of the input language by /audio/transcriptions endpoint.
//!
//! ```shell
//! $ cargo run --example audio_transcriptions_vtt --features audio -- --file-path <file-path>
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

    // 1. Create a client with the API key from the environment variable: "OPENAI_API_KEY"
    let client = Client::from_env()?;
    // or specify the API key directly.
    // let client = Client::new(oaapi::ApiKey::new("OPENAI_API_KEY"), None, None);

    // 2. Load the audio file that you want to transcribe.
    let file = tokio::fs::read(&arguments.file_path).await?;
    let file = File::new(arguments.file_path, file)?;

    // 3. Create a request body parameters.
    let request_body = TranscriptionsRequestBody {
        file,
        ..Default::default()
    };

    // 4. Call the API with specifying the response format.
    let response = client
        .audio_transcribe_into_vtt(request_body)
        .await?;

    // 5. Use the response.
    println!("Result:\n{}", response);

    Ok(())
}
