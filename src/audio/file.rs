use std::borrow::Cow;
use std::fmt::Display;

use reqwest::multipart::Part;

use crate::ValidationError;
use crate::ValidationResult;

/// The audio file to be used for the request.
#[derive(Debug)]
pub struct File {
    /// The name of the audio file.
    name: String,
    /// The part of the audio file.
    pub(crate) part: Part,
}

impl Default for File {
    fn default() -> Self {
        Self {
            name: String::new(),
            part: Part::bytes(Vec::new()),
        }
    }
}

impl Display for File {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "File: {}", self.name)
    }
}

/// Supported audio file formats.
const SUPPORTED_FILE_FORMATS: [&str; 9] = [
    "flac", "mp3", "mp4", "mpeg", "mpga", "m4a", "ogg", "wav", "webm",
];

fn supported_file_format_list() -> String {
    format!(
        "[{}]",
        SUPPORTED_FILE_FORMATS.join(", ")
    )
}

impl File {
    /// Creates a new [`File`] from the given binary.
    ///
    /// ## Arguments
    /// - `file_name` - The name of the audio file.
    /// - `data` - The binary data of the audio file.
    ///
    /// ## Examples
    /// ```no_run
    /// use oaapi::audio::File;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let file_name = "path/to/audio/file.mp3";
    ///     let file = tokio::fs::read(file_name).await?;
    ///
    ///     let file = File::new(file_name, file)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn new<S, T>(
        file_name: S,
        data: T,
    ) -> ValidationResult<Self, String>
    where
        S: Into<String>,
        T: Into<Cow<'static, [u8]>>,
    {
        let file_name = file_name.into();

        // Check if the file format is supported.
        if let Some(extension) = file_name.split('.').last() {
            if SUPPORTED_FILE_FORMATS.contains(&extension) {
                return Ok(Self {
                    name: file_name.clone(),
                    part: Part::bytes(data).file_name(file_name),
                });
            }
        }

        Err(ValidationError {
            type_name: "File".to_string(),
            reason: format!(
                "The file format is not found or not supported.\nSupported file formats are {}",
                supported_file_format_list()
            ),
            value: file_name,
        })
    }
}
