use std::fmt::Display;
use std::path::PathBuf;

use reqwest::multipart::Part;

use crate::Error;
use crate::Result;
use crate::ValidationError;
use crate::ValidationResult;

/// The audio file to be used for the request.
#[derive(Debug, Clone, PartialEq)]
pub enum File {
    /// The file path of the audio file.
    FilePath {
        name: String,
        path: PathBuf,
    },
    /// The binary of the audio file.
    Binary {
        name: String,
        data: Vec<u8>,
    },
}

impl Default for File {
    fn default() -> Self {
        Self::Binary {
            name: String::new(),
            data: Vec::new(),
        }
    }
}

impl Display for File {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | File::FilePath {
                name,
                path,
            } => write!(f, "{} ({})", name, path.display()),
            | File::Binary {
                name: file_name,
                data,
            } => write!(
                f,
                "{} ({} bytes)",
                file_name,
                data.len()
            ),
        }
    }
}

/// Supported audio file formats.
const SUPPORTED_FILE_FORMATS: [&str; 9] = [
    "flac", "mp3", "mp4", "mpeg", "mpga", "m4a", "ogg", "wav", "webm",
];

impl File {
    /// Creates a new [`File`] from the given file path.
    pub fn from_file_path(file_path: PathBuf) -> ValidationResult<Self> {
        // Check file existence.
        if !file_path.exists() {
            return Err(ValidationError {
                type_name: "Resource".to_string(),
                reason: format!(
                    "The file does not exist: {}",
                    file_path.display()
                ),
            });
        }

        // Check if the file format is supported.
        if let Some(file_name) = file_path.file_name() {
            if let Some(file_name) = file_name.to_str() {
                if let Some(extension) = file_name.split('.').last() {
                    if SUPPORTED_FILE_FORMATS.contains(&extension) {
                        return Ok(Self::FilePath {
                            name: file_name.to_string(),
                            path: file_path,
                        });
                    }
                }
            }
        }

        Err(ValidationError {
            type_name: "Resource".to_string(),
            reason: format!(
                "The file format is not supported: {}.\nSupported file formats: {:?}",
                file_path.display(),
                SUPPORTED_FILE_FORMATS
            ),
        })
    }

    /// Creates a new [`File`] from the given binary.
    pub fn from_binary(
        file_name: String,
        data: Vec<u8>,
    ) -> ValidationResult<Self> {
        // Check if the file format is supported.
        if let Some(extension) = file_name.split('.').last() {
            if SUPPORTED_FILE_FORMATS.contains(&extension) {
                return Ok(Self::Binary {
                    name: file_name,
                    data,
                });
            }
        }

        Err(ValidationError {
            type_name: "Resource".to_string(),
            reason: format!(
                "The file format is not supported: {}.\nSupported file formats: {:?}",
                file_name,
                SUPPORTED_FILE_FORMATS
            ),
        })
    }

    /// Builds a multipart form from the file.
    pub(crate) async fn build_part(self) -> Result<Part> {
        match self {
            | File::FilePath {
                name,
                path,
            } => {
                let file = tokio::fs::read(path)
                    .await
                    .map_err(Error::IOError)?;

                Ok(Part::bytes(file).file_name(name))
            },
            | File::Binary {
                name: file_name,
                data,
            } => Ok(Part::bytes(data).file_name(file_name)),
        }
    }
}
