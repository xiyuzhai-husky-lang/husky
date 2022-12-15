use std::path::{Path, PathBuf};

use husky_absolute_path::AbsolutePathError;
use husky_print_utils::p;
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum VfsError {
    #[error("file {0:?} not found")]
    FileNotFound(PathBuf),
    #[error("IO Error: ???")]
    IO {
        path: PathBuf,
        error_message: String,
    },
    #[error("not source file")]
    NotSourceFile(PathBuf),
    #[error("{0}")]
    AbsolutePath(#[from] AbsolutePathError),
}

impl From<&VfsError> for VfsError {
    fn from(value: &VfsError) -> Self {
        value.clone()
    }
}

pub type VfsResult<T> = Result<T, VfsError>;

// impl From<std::io::Error> for VfsError {
//     fn from(value: std::io::Error) -> Self {
//         p!(value);
//         todo!()
//     }
// }

impl VfsError {
    pub(crate) fn new_io_error(path: PathBuf, e: std::io::Error) -> VfsError {
        VfsError::IO {
            path,
            error_message: e.to_string(),
        }
    }
}
