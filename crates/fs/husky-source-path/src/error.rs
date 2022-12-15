use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum SourcePathError {
    #[error("fail to absolutize {path:?} due to IO `{error_message}")]
    FailToAbsolutize {
        path: PathBuf,
        error_message: String,
    },
}

impl From<&SourcePathError> for SourcePathError {
    fn from(value: &SourcePathError) -> Self {
        todo!()
    }
}

pub type SourcePathResult<T> = Result<T, SourcePathError>;
