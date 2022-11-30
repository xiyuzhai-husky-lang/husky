use std::path::PathBuf;

use absolute_path::AbsolutePathError;
use thiserror::Error;

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum SourcePathError {
    #[error("{0}")]
    AbsolutePath(#[from] AbsolutePathError),
}

pub type SourcePathResult<T> = Result<T, SourcePathError>;
