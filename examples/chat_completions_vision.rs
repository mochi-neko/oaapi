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

    // 1. Create a client with the API key from the environment variable: "OPENAI_API_KEY"
    let client = Client::from_env()?;
    // or specify the API key directly.
    // let client = Client::new(oaapi::ApiKey::new("OPENAI_API_KEY"), None, None);

    // 2. Read image file and encode it to Base64.
    let image_file = tokio::fs::read(&arguments.image_file).await?;
    let image_base64 = base64::prelude::BASE64_STANDARD.encode(&image_file);
    let image_format =
        ImageFormat::from_path(Path::new(&arguments.image_file).to_path_buf())?;

    // 3. Create a request body parameters.
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
            .into(),
        ],
        model: ChatModel::Gpt4VisionPreview, // Must be a model with vision.
        ..Default::default()
    };

    // 4. Call the API.
    let response = client
        .chat_complete(request_body)
        .await?;

    // 5. Use the response.
    println!("Result:\n{}", response);

    Ok(())
}
