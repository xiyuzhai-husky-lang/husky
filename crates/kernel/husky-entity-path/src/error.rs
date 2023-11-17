use husky_vfs::{error::VfsError, ToolchainError};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum EntityPathError {
    // #[error("{0}")]
    // VirtualPath(#[from] AbsolutePathError),
    #[error("{0}")]
    Vfs(#[from] VfsError),
    #[error("{0}")]
    Toolchain(#[from] ToolchainError),
}

pub type EntityPathResult<T> = Result<T, EntityPathError>;

impl From<&EntityPathError> for EntityPathError {
    fn from(_value: &EntityPathError) -> Self {
        todo!()
    }
}
