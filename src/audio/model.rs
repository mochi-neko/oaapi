use crate::macros::impl_enum_string_serialization;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Models for audio APIs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AudioModel {
    /// "whisper-1" model.
    Whisper1,
}

impl Default for AudioModel {
    fn default() -> Self {
        Self::Whisper1
    }
}

impl Display for AudioModel {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.format())
    }
}

impl FromStr for AudioModel {
    type Err = crate::ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            | "whisper-1" => Ok(Self::Whisper1),
            | _ => Err(crate::ValidationError {
                type_name: "AudioModel".to_string(),
                reason: format!("Unknown audio model: {}", s),
            }),
        }
    }
}

impl AudioModel {
    /// Returns the format of the model.
    pub(crate) fn format(self) -> &'static str {
        match self {
            | AudioModel::Whisper1 => "whisper-1",
        }
    }
}

/// Text-to-speech models.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpeechModel {
    /// tts-1
    Tts1,
    /// tts-1-hd
    Tts1Hd,
}

impl Default for SpeechModel {
    fn default() -> Self {
        SpeechModel::Tts1
    }
}

impl Display for SpeechModel {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.format())
    }
}

impl FromStr for SpeechModel {
    type Err = crate::ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            | "tts-1" => Ok(SpeechModel::Tts1),
            | "tts-1-hd" => Ok(SpeechModel::Tts1Hd),
            | _ => Err(crate::ValidationError {
                type_name: "SpeechModel".to_string(),
                reason: format!("Unknown speech model: {}", s),
            }),
        }
    }
}

impl_enum_string_serialization!(
    SpeechModel,
    Tts1 => "tts-1",
    Tts1Hd => "tts-1-hd"
);

impl SpeechModel {
    pub(crate) fn format(self) -> &'static str {
        match self {
            | SpeechModel::Tts1 => "tts-1",
            | SpeechModel::Tts1Hd => "tts-1-hd",
        }
    }
}
