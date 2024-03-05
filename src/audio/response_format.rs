use std::fmt::{Display, Formatter};
use std::str::FromStr;
use subtp::srt::SubRip;
use subtp::vtt::WebVtt;
use subtp::ParseError;

use crate::macros::impl_enum_string_serialization;

/// Format of a response text.
pub trait TextResponseFormat {
    /// Returns the format type as a request parameter.
    fn format() -> &'static str;
}

/// Formatter of a response text to a specific type: `<T>`.
pub trait TextResponseFormatter<T> {
    /// Formats the response text to a specific type: `<T>`.
    fn format(raw_text: String) -> TextFormatResult<T>;
}

/// The error of formatting a response text.
#[derive(Debug, thiserror::Error)]
pub enum TextFormatError {
    /// The error of formatting a response text into JSON.
    #[error("Failed to format into JSON: {error:?}, {text}")]
    FormatJsonFailed {
        /// The error of deserializing a response text.
        error: serde_json::Error,
        /// The raw response text.
        text: String,
    },
    /// The error of formatting a response text into SubRip Subtitle format.
    #[error("Failed to parse into SubRip Subtitle format: {error:?}, {text}")]
    ParseSrtFailed {
        /// The error of parsing a response text.
        error: ParseError,
        /// The raw response text.
        text: String,
    },
    /// The error of formatting a response text into WebVTT format.
    #[error("Failed to parse into WebVTT format: {error:?}, {text}")]
    ParseVttFailed {
        /// The error of parsing a response text.
        error: ParseError,
        /// The raw response text.
        text: String,
    },
}

/// The result of formatting a response text.
pub(crate) type TextFormatResult<T> = std::result::Result<T, TextFormatError>;

/// The JSON response.
#[derive(serde::Deserialize)]
pub struct JsonResponse {
    /// The text of the transcription or translation.
    pub text: String,
}

impl TextResponseFormat for JsonResponse {
    fn format() -> &'static str {
        "json"
    }
}

impl Display for JsonResponse {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

/// The JSON response formatter.
pub struct JsonResponseFormatter {}

impl TextResponseFormatter<JsonResponse> for JsonResponseFormatter {
    fn format(raw_text: String) -> TextFormatResult<JsonResponse> {
        serde_json::from_str(raw_text.as_str()).map_err(|error| {
            TextFormatError::FormatJsonFailed {
                error,
                text: raw_text,
            }
        })
    }
}

impl TextResponseFormat for String {
    fn format() -> &'static str {
        "text"
    }
}

/// The plain text response formatter.
pub struct PlainTextResponseFormatter;

impl TextResponseFormatter<String> for PlainTextResponseFormatter {
    fn format(raw_text: String) -> TextFormatResult<String> {
        Ok(raw_text)
    }
}

/// The verbose JSON response.
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct VerboseJsonResponse {
    pub task: String,
    pub language: String,
    pub duration: f32,
    pub text: String,
    pub segments: Vec<VerboseJsonResponseSegment>,
}

impl Display for VerboseJsonResponse {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "task: {}", self.task)?;
        write!(f, "language: {}", self.language)?;
        write!(f, "duration: {}", self.duration)?;
        write!(f, "text: {}", self.text)?;
        write!(f, "segments: [",)?;
        for segment in self.segments.iter() {
            write!(f, "{}", segment)?;
        }
        write!(f, "]")
    }
}

/// The segment of a verbose JSON response.
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct VerboseJsonResponseSegment {
    pub id: u32,
    pub seek: u32,
    pub start: f32,
    pub end: f32,
    pub text: String,
    pub tokens: Vec<u32>,
    pub temperature: f32,
    pub avg_logprob: f32,
    pub compression_ratio: f32,
    pub no_speech_prob: f32,
}

impl Display for VerboseJsonResponseSegment {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "id: {}", self.id)?;
        write!(f, "seek: {}", self.seek)?;
        write!(f, "start: {}", self.start)?;
        write!(f, "end: {}", self.end)?;
        write!(f, "text: {}", self.text)?;
        write!(f, "tokens: [",)?;
        for token in self.tokens.iter() {
            write!(f, "{}", token)?;
        }
        write!(f, "]")?;
        write!(f, "temperature: {}", self.temperature)?;
        write!(f, "avg_logprob: {}", self.avg_logprob)?;
        write!(
            f,
            "compression_ratio: {}",
            self.compression_ratio
        )?;
        write!(
            f,
            "no_speech_prob: {}",
            self.no_speech_prob
        )
    }
}

impl TextResponseFormat for VerboseJsonResponse {
    fn format() -> &'static str {
        "verbose_json"
    }
}

/// The verbose JSON response formatter.
pub struct VerboseJsonResponseFormatter;

impl TextResponseFormatter<VerboseJsonResponse>
    for VerboseJsonResponseFormatter
{
    fn format(raw_text: String) -> TextFormatResult<VerboseJsonResponse> {
        serde_json::from_str(raw_text.as_str()).map_err(|error| {
            TextFormatError::FormatJsonFailed {
                error,
                text: raw_text,
            }
        })
    }
}

impl TextResponseFormat for SubRip {
    fn format() -> &'static str {
        "srt"
    }
}

/// The SubRip Subtitle response formatter.
pub struct SrtResponseFormatter;

impl TextResponseFormatter<SubRip> for SrtResponseFormatter {
    fn format(raw_text: String) -> TextFormatResult<SubRip> {
        SubRip::parse(raw_text.as_str()).map_err(|error| {
            TextFormatError::ParseSrtFailed {
                error,
                text: raw_text,
            }
        })
    }
}

impl TextResponseFormat for WebVtt {
    fn format() -> &'static str {
        "vtt"
    }
}

/// The WebVTT response formatter.
pub struct VttResponseFormatter;

impl TextResponseFormatter<WebVtt> for VttResponseFormatter {
    fn format(raw_text: String) -> TextFormatResult<WebVtt> {
        WebVtt::parse(raw_text.as_str()).map_err(|error| {
            TextFormatError::ParseVttFailed {
                error,
                text: raw_text,
            }
        })
    }
}

/// The response format of a speech audio.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpeechResponseFormat {
    /// mp3
    Mp3,
    /// opus
    Opus,
    /// aac
    Aac,
    /// flac
    Flac,
}

impl Default for SpeechResponseFormat {
    fn default() -> Self {
        Self::Mp3
    }
}

impl Display for SpeechResponseFormat {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | Self::Mp3 => {
                write!(f, "mp3")
            },
            | Self::Opus => {
                write!(f, "opus")
            },
            | Self::Aac => {
                write!(f, "aac")
            },
            | Self::Flac => {
                write!(f, "flac")
            },
        }
    }
}

impl FromStr for SpeechResponseFormat {
    type Err = crate::ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            | "mp3" => Ok(Self::Mp3),
            | "opus" => Ok(Self::Opus),
            | "aac" => Ok(Self::Aac),
            | "flac" => Ok(Self::Flac),
            | _ => Err(crate::ValidationError {
                type_name: "SpeechResponseFormat".to_string(),
                reason: format!("Unknown speech response format: {}", s),
            }),
        }
    }
}

impl_enum_string_serialization!(
    SpeechResponseFormat,
    Mp3 => "mp3",
    Opus => "opus",
    Aac => "aac",
    Flac => "flac"
);
