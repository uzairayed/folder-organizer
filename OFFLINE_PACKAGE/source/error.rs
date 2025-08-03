use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OrganizerError {
    #[error("Failed to read directory {path}: {source}")]
    ReadDirError {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to create directory {path}: {source}")]
    CreateDirError {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to move file from {from} to {to}: {source}")]
    MoveFileError {
        from: PathBuf,
        to: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to get file extension for {path}")]
    NoExtensionError { path: PathBuf },

    #[error("Invalid target directory: {path}")]
    InvalidTargetDir { path: PathBuf },

    #[error("Failed to serialize/deserialize log: {source}")]
    SerializationError {
        #[source]
        source: serde_json::Error,
    },

    #[error("Failed to write log file: {source}")]
    LogWriteError {
        #[source]
        source: std::io::Error,
    },
}

pub type Result<T> = std::result::Result<T, OrganizerError>; 