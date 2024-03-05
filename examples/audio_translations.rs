//! Translates audio into a text of English by /audio/translations endpoint.
//!
//! ```shell
//! $ cargo run --example audio_translations -- --file-path <file-path>
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

    let request_body = TranslationsRequestBody {
        file: File::from_file_path(
            Path::new(&arguments.file_path).to_path_buf(),
        )?,
        ..Default::default()
    };

    let response = client
        .audio_translate_into_json(request_body)
        .await?;

    println!("Result:\n{}", response.text);

    Ok(())
}
