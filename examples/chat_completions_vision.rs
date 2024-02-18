//! Completes chat by /chat/completions endpoint with vision input.
//!
//! ```shell
//! $ cargo run --example chat_completions_vision -- --prompt <prompt> --message <message> --image-file <image-file>
//! ```

use base64::Engine;
use clap::Parser;
use std::path::Path;

use openai::chat::complete;
use openai::chat::ChatModel;
use openai::chat::CompletionsRequestBody;
use openai::chat::ImageContentPart;
use openai::chat::ImageFormat;
use openai::chat::ImageUrl;
use openai::chat::SystemMessage;
use openai::chat::TextContentPart;
use openai::chat::UserMessage;
use openai::ApiKey;

#[derive(Parser)]
struct Arguments {
    #[arg(short, long)]
    prompt: String,
    #[arg(short, long)]
    message: String,
    #[arg(short, long)]
    image_file: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let arguments = Arguments::parse();
    let client = reqwest::Client::new();
    let api_key = ApiKey::from_env()?;

    // Read image file and encode it to base64.
    let image_file = std::fs::read(&arguments.image_file)?;
    let image_base64 = base64::prelude::BASE64_STANDARD.encode(&image_file);
    let image_format =
        ImageFormat::from_path(Path::new(&arguments.image_file).to_path_buf())?;

    let request_body = CompletionsRequestBody {
        messages: vec![
            SystemMessage::new(arguments.prompt, None).into(),
            UserMessage::new(
                vec![
                    TextContentPart::new(arguments.message).into(),
                    ImageContentPart::new(ImageUrl::upload_base64(
                        image_base64, // Base64 encoded image.
                        image_format, // Image format.
                        None,         // Image detail info.
                    ))
                    .into(),
                ]
                .into(),
                None,
            )
            .into(), // Array message content.
        ],
        model: ChatModel::Gpt4VisionPreview, // Must be a model with vision.
        ..Default::default()
    };

    let response = complete(&client, &api_key, request_body).await?;

    println!(
        "Result: {:?}",
        response
            .choices
            .first()
            .unwrap()
            .message
    );

    Ok(())
}
