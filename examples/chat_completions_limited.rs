//! Completes chat by /chat/completions endpoint with limited tokens and stop sequences.
//!
//! ```shell
//! $ cargo run --example chat_completions_limited --features chat -- --prompt <prompt> --message <message> --max-tokens <max-tokens>
//! ```

use clap::Parser;

use oaapi::chat::ChatModel;
use oaapi::chat::CompletionsRequestBody;
use oaapi::chat::MaxTokens;
use oaapi::chat::StopOption;
use oaapi::chat::SystemMessage;
use oaapi::chat::UserMessage;
use oaapi::Client;

#[derive(Parser)]
struct Arguments {
    #[arg(short, long)]
    prompt: String,
    #[arg(long)]
    message: String,
    #[arg(long)]
    max_tokens: u32,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let arguments = Arguments::parse();

    // 1. Create a client with the API key from the environment variable: "OPENAI_API_KEY"
    let client = Client::from_env()?;
    // or specify the API key directly.
    // let client = Client::new(oaapi::ApiKey::new("OPENAI_API_KEY"), None, None);

    // 2. Set up options for limited tokens.
    let model = ChatModel::Gpt35Turbo;
    let max_tokens = MaxTokens::new(
        arguments.max_tokens, // Max tokens
        model.clone(),        // Model
    )?;
    let stop_option = StopOption::new_up_to_4(vec!["\\n", ".", "。"])?; // Optional

    // 3. Create a request body parameters.
    let request_body = CompletionsRequestBody {
        messages: vec![
            SystemMessage::new(arguments.prompt, None).into(),
            UserMessage::new(arguments.message.into(), None).into(),
        ],
        model,
        max_tokens: Some(max_tokens), // Specify max tokens.
        stop: Some(stop_option),      // Specify stop sequences.
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
