use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum VfsError {
    #[error("file {0:?} not found")]
    FileNotFound(PathBuf),
    #[error("IO Error: {0}")]
    IO(#[from] std::io::Error),
}

pub type VfsResult<T> = Result<T, VfsError>;
