use crate::*;
use husky_path_utils::PathUtilsError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum ToolchainError {
    #[error("vfs")]
    Vfs(#[from] VfsError),
    #[error("path utils")]
    PathUtils(#[from] PathUtilsError),
}

pub type ToolchainResult<T> = Result<T, ToolchainError>;
