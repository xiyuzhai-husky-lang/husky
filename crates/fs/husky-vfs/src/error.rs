use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum VfsError {
    #[error("file {0:?} not found")]
    FileNotFound(PathBuf),
}

pub type VfsResult<T> = Result<T, VfsError>;
