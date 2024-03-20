//! Translates audio into a text of English by /audio/translations endpoint.
//!
//! ```shell
//! $ cargo run --example audio_translations --features audio -- --file-path <file-path>
//! ```

use std::path::Path;

use clap::Parser;

use oaapi::audio::File;
use oaapi::audio::TranslationsRequestBody;
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

    let request_body = TranslationsRequestBody {
        file,
        ..Default::default()
    };

    let response = client
        .audio_translate_into_json(request_body)
        .await?;

    println!("Result:\n{}", response.text);

    Ok(())
}
