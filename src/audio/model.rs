use crate::macros::impl_enum_string_serialization;

/// Models for audio APIs.
pub enum AudioModel {
    /// "whisper-1" model.
    Whisper1,
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

impl_enum_string_serialization!(
    SpeechModel,
    Tts1 => "tts-1",
    Tts1Hd => "tts-1-hd"
);
