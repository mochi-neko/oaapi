//! Completes chat by /chat/completions endpoint with tool calling.
//!
//! ```shell
//! $ cargo run --example chat_completions_tool -- --message <message>
//! ```

use clap::Parser;
use serde::{Deserialize, Serialize};

use openai::chat::complete;
use openai::chat::ChatModel;
use openai::chat::CompletionsRequestBody;
use openai::chat::Function;
use openai::chat::SystemMessage;
use openai::chat::TooChoiceOption;
use openai::chat::UserMessage;
use openai::ApiKey;

#[derive(Parser)]
struct Arguments {
    #[arg(short, long)]
    message: String,
}

/// Get the current weather in a given location.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct GetCurrentWeather {
    /// The city and state, e.g. San Francisco, CA
    location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<Unit>,
}

#[derive(Debug, Clone, PartialEq)]
enum Unit {
    /// celsius
    Celsius,
    /// fahrenheit
    Fahrenheit,
}

impl Serialize for Unit {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(match self {
            | Unit::Celsius => "celsius",
            | Unit::Fahrenheit => "fahrenheit",
        })
    }
}

impl<'de> Deserialize<'de> for Unit {
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D
    ) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;

        match s.as_str() {
            | "celsius" => Ok(Unit::Celsius),
            | "fahrenheit" => Ok(Unit::Fahrenheit),
            | _ => Err(serde::de::Error::custom(format!(
                "invalid unit: {}",
                s
            ))),
        }
    }
}

const GET_CURRENT_WEATHER_SCHEMA: &str = r#"
{
    "type": "object",
    "properties": {
        "location": {
            "type": "string",
            "description": "The city and state, e.g. San Francisco, CA"
        },
        "unit": {
            "type": "string",
            "enum": ["celsius", "fahrenheit"]
        }
    },
    "required": ["location"]
}"#;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let arguments = Arguments::parse();
    let client = reqwest::Client::new();
    let api_key = ApiKey::from_env()?;

    let prompt = r#"Respond for user message by using some tool."#;
    let function = Function {
        description: Some(
            "Get the current weather in a given location".to_string(),
        ),
        name: "get_current_weather".to_string(),
        parameters: Some(serde_json::from_str(
            GET_CURRENT_WEATHER_SCHEMA,
        )?),
    };
    let tools = Some(vec![function.into()]); // Register tools
    let tool_choice = Some(TooChoiceOption::Auto.into()); // Specify tool choice rule

    let request_body = CompletionsRequestBody {
        messages: vec![
            SystemMessage::new(prompt, None).into(),
            UserMessage::new(arguments.message.into(), None).into(),
        ],
        model: ChatModel::Gpt35Turbo1106,
        tools,
        tool_choice,
        ..Default::default()
    };

    let response = complete(&client, &api_key, request_body).await?;

    let arguments = response
        .choices
        .first()
        .unwrap()
        .message
        .tool_calls
        .as_ref()
        .unwrap()
        .first()
        .unwrap()
        .function
        .arguments
        .as_ref();

    let called_function =
        serde_json::from_str::<GetCurrentWeather>(&arguments)?;

    println!("Result: {:?}", called_function);

    Ok(())
}
