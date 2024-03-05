use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Log probability information for the choice.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Logprobs {
    /// A list of message content tokens with log probability information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<LogprobsContent>>,
}

impl Default for Logprobs {
    fn default() -> Self {
        Self {
            content: None,
        }
    }
}

impl Display for Logprobs {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        if let Some(content) = &self.content {
            for logprobs_content in content {
                write!(f, "content: {}", logprobs_content)?;
            }
        }
        Ok(())
    }
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

impl Default for LogprobsContent {
    fn default() -> Self {
        Self {
            token: "".to_string(),
            logprob: 0.0,
            bytes: None,
            top_logprobs: vec![],
        }
    }
}

impl Display for LogprobsContent {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "token: {}", self.token)?;
        write!(f, "logprob: {}", self.logprob)?;
        if let Some(bytes) = &self.bytes {
            write!(f, "bytes: {:?}", bytes)?;
        }
        for top_logprobs_content in &self.top_logprobs {
            write!(
                f,
                "top_logprobs: {}",
                top_logprobs_content
            )?;
        }
        Ok(())
    }
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

impl Default for TopLogprobsContent {
    fn default() -> Self {
        Self {
            token: "".to_string(),
            logprob: 0.0,
            bytes: None,
        }
    }
}

impl Display for TopLogprobsContent {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "token: {}", self.token)?;
        write!(f, "logprob: {}", self.logprob)?;
        if let Some(bytes) = &self.bytes {
            write!(f, "bytes: {:?}", bytes)?;
        }
        Ok(())
    }
}
