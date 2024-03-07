//! Completes chat by /chat/completions endpoint with vision input.
//!
//! ```shell
//! $ cargo run --example chat_completions_vision --features chat -- --prompt <prompt> --message <message> --image-file <image-file>
//! ```

use base64::Engine;
use clap::Parser;
use std::path::Path;

use oaapi::chat::ChatModel;
use oaapi::chat::CompletionsRequestBody;
use oaapi::chat::ImageContentPart;
use oaapi::chat::ImageFormat;
use oaapi::chat::ImageUrl;
use oaapi::chat::SystemMessage;
use oaapi::chat::TextContentPart;
use oaapi::chat::UserMessage;
use oaapi::Client;

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
    let client = Client::from_env()?;

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

    let response = client
        .chat_complete(request_body)
        .await?;

    println!(
        "Result:\n{}",
        response
            .choices
            .first()
            .unwrap()
            .message
    );

    Ok(())
}
