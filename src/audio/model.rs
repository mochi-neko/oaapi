use crate::macros::impl_enum_string_serialization;
use std::fmt::{Display, Formatter};

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
        match self {
            | AudioModel::Whisper1 => {
                write!(f, "whisper-1")
            },
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
        match self {
            | SpeechModel::Tts1 => {
                write!(f, "tts-1")
            },
            | SpeechModel::Tts1Hd => {
                write!(f, "tts-1-hd")
            },
        }
    }
}

impl_enum_string_serialization!(
    SpeechModel,
    Tts1 => "tts-1",
    Tts1Hd => "tts-1-hd"
);
