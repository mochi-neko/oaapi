use serde::{Deserialize, Serialize};

/// Log probability information for the choice.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Logprobs {
    /// A list of message content tokens with log probability information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<LogprobsContent>>,
}

/// The content of logprobs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogprobsContent {
    /// The token.
    pub token: String,
    /// The log probability of this token.
    pub logprob: f32,
    /// A list of integers representing the UTF-8 bytes representation of the token.
    /// Useful in instances where characters are represented by multiple tokens and their byte representations must be combined to generate the correct text representation.
    /// Can be null if there is no bytes representation for the token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<u8>>,
    /// List of the most likely tokens and their log probability, at this token position.
    /// In rare cases, there may be fewer than the number of requested top_logprobs returned.
    pub top_logprobs: Vec<TopLogprobsContent>,
}

/// The top logprobs content.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopLogprobsContent {
    /// The token.
    pub token: String,
    /// The log probability of this token.
    pub logprob: f32,
    /// A list of integers representing the UTF-8 bytes representation of the token.
    /// Useful in instances where characters are represented by multiple tokens and their byte representations must be combined to generate the correct text representation.
    /// Can be null if there is no bytes representation for the token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<u8>>,
}
