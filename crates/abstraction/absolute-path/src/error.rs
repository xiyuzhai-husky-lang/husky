use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum AbsolutePathError {
    #[error("can't absolutize {path:?} due to IO `{error_message}")]
    IO {
        path: PathBuf,
        error_message: String,
    },
}

pub type AbsolutePathResult<T> = Result<T, AbsolutePathError>;
