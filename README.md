# oaapi
An unofficial Rust client for OpenAI API.

## Installation
Run the following Cargo command in your project directory:

```shell
cargo add oaapi
```

or add the following line to your Cargo.toml:

```toml
[dependencies]
oaapi = "0.1.0"
```

## Features
- [`audio`](/src/audio.rs)
- [`chat`](/src/chat.rs)

## Supported APIs
- [x] [Audio](https://platform.openai.com/docs/api-reference/audio)
    - [x] [speech](https://platform.openai.com/docs/api-reference/audio/createSpeech)
    - [x] [transcriptions](https://platform.openai.com/docs/api-reference/audio/createTranscription)
    - [x] [translations](https://platform.openai.com/docs/api-reference/audio/createTranslation)
- [x] [Chat](https://platform.openai.com/docs/api-reference/chat)
    - [x] [completions](https://platform.openai.com/docs/api-reference/chat/create)
    - [x] [completions streaming](https://platform.openai.com/docs/api-reference/chat/create)
- [ ] [Embeddings](https://platform.openai.com/docs/api-reference/embeddings)
- [ ] [Fine-tuning](https://platform.openai.com/docs/api-reference/fine-tuning)
- [ ] [Files](https://platform.openai.com/docs/api-reference/files)
- [ ] [Images](https://platform.openai.com/docs/api-reference/images)
- [ ] [Models](https://platform.openai.com/docs/api-reference/models)
- [ ] [Moderations](https://platform.openai.com/docs/api-reference/moderations)

Beta version APIs:
- [ ] [Assistants](https://platform.openai.com/docs/api-reference/assistants)
- [ ] [Threads](https://platform.openai.com/docs/api-reference/threads)
- [ ] [Messages](https://platform.openai.com/docs/api-reference/messages)
- [ ] [Runs](https://platform.openai.com/docs/api-reference/runs)

## Usage
1. Enable API feature flags that you want to use, e.g. `chat`.
2. Create a [`crate::Client`] with the API key and the other optional settings.
3. Use the client to call the APIs, e.g. [`crate::Client::chat_complete`].

## Example
An example to call the chat completions API with `chat` feature is as follows:

```rust
use oaapi::Client;
use oaapi::chat::CompletionsRequestBody;
use oaapi::chat::SystemMessage;
use oaapi::chat::UserMessage;
use oaapi::chat::ChatModel;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Create a client, e.g. with API key loaded from the environment variable: `OPENAI_API_KEY`.
    let client = Client::from_env()?;

    // 2. Create a request body parameters.
    let request_body = CompletionsRequestBody {
        messages: vec![
            SystemMessage::new("Prompt.", None).into(),
            UserMessage::new("Chat message from user.".into(), None).into(),
        ],
        model: ChatModel::Gpt35Turbo,
        ..Default::default()
    };

    // 3. Call the API.
    let response = client
        .chat_complete(request_body)
        .await?;

    Ok(())
}
```

## Other examples

See the [./examples](./examples) directory.

## Changelog

See [CHANGELOG](./CHANGELOG.md).

## License

Licensed under either of the [Apache License, Version 2.0](./LICENSE-APACHE) or the [MIT](./LICENSE-MIT) license at your option.
