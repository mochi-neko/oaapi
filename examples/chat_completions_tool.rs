//! Completes chat by /chat/completions endpoint with tool calling.
//!
//! ```shell
//! $ cargo run --example chat_completions_tool --features chat -- --message <message>
//! ```

use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use oaapi::chat::ChatModel;
use oaapi::chat::CompletionsRequestBody;
use oaapi::chat::Function;
use oaapi::chat::SystemMessage;
use oaapi::chat::TooChoiceOption;
use oaapi::chat::UserMessage;
use oaapi::Client;

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

impl Display for GetCurrentWeather {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "Location: {}", self.location)?;

        if let Some(unit) = &self.unit {
            write!(f, ", Unit: {}", unit)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Unit {
    /// celsius
    Celsius,
    /// fahrenheit
    Fahrenheit,
}

impl Display for Unit {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                | Unit::Celsius => "celsius",
                | Unit::Fahrenheit => "fahrenheit",
            }
        )
    }
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

    // 1. Create a client with the API key from the environment variable: "OPENAI_API_KEY"
    let client = Client::from_env()?;
    // or specify the API key directly.
    // let client = Client::new(oaapi::ApiKey::new("OPENAI_API_KEY"), None, None);

    // 2. Set up options for tool calling.
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

    // 3. Call the API.
    let request_body = CompletionsRequestBody {
        messages: vec![
            SystemMessage::new(prompt, None).into(),
            UserMessage::new(arguments.message.into(), None).into(),
        ],
        model: ChatModel::Gpt35Turbo1106,
        tools,       // Specify tools
        tool_choice, // Specify tool choice option
        ..Default::default()
    };

    // 4. Call the API.
    let response = client
        .chat_complete(request_body)
        .await?;

    // 5. Take called function content as a string.
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

    // 6. Deserialize called function.
    let called_function =
        serde_json::from_str::<GetCurrentWeather>(&arguments)?;

    // 7. Use the response.
    println!("Result:\n{}", called_function);

    Ok(())
}
