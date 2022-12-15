use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum AbsolutePathError {
    #[error("fail to absolutize {path:?} due to IO `{error_message}")]
    FailToAbsolutize {
        path: PathBuf,
        error_message: String,
    },
}

pub type AbsolutePathResult<T> = Result<T, AbsolutePathError>;
