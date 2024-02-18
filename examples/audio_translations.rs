//! Translates audio into a text of English by /audio/translations endpoint.
//!
//! ```shell
//! $ cargo run --example audio_translations -- --file-path <file-path>
//! ```

use std::path::Path;

use clap::Parser;

use openai::audio::File;
use openai::audio::TranslationsRequestBody;
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

    let request_body = TranslationsRequestBody {
        file: File::from_file_path(
            Path::new(&arguments.file_path).to_path_buf(),
        )?,
        ..Default::default()
    };

    let response =
        openai::audio::translate_into_json(&client, &api_key, request_body)
            .await?;

    println!("Result: {}", response.text);

    Ok(())
}
