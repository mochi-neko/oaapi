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

    // 1. Create a client with the API key from the environment variable: "OPENAI_API_KEY"
    let client = Client::from_env()?;
    // or specify the API key directly.
    // let client = Client::new(oaapi::ApiKey::new("OPENAI_API_KEY"), None, None);

    // 2. Create a request body parameters.
    let request_body = SpeechRequestBody {
        input: SpeechInput::new(arguments.text)?,
        voice: Voice::from_str(&arguments.voice)?,
        response_format: Some(SpeechResponseFormat::Mp3),
        ..Default::default()
    };

    // 3. Set up an output file to write speech audio.
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(arguments.output.clone())
        .await?;

    let mut writer = BufWriter::new(file);

    // 4. Call the API.
    let mut stream = client
        .audio_speech(request_body)
        .await?;

    // 4. Read the stream of the speech data.
    while let Some(chunk) = stream.next().await {
        // 5. Write the chunk data to the output file.
        let chunk = chunk?;
        writer
            .write_all(&chunk)
            .await?;
    }

    // 6. Flush the output file.
    writer.flush().await?;

    println!("Speech written to {}", arguments.output);

    Ok(())
}
