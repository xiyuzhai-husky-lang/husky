use husky_entity_path::EntityPathError;
use husky_vfs::{ToolchainError, VfsError};
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum TermPreludeError {
    #[error("UniverseOverflow")]
    UniverseOverflow,
    #[error("EntityPathError")]
    EntityPathError(#[from] EntityPathError),
    #[error("VfsError")]
    VfsError(#[from] VfsError),
    #[error("ToolchainError")]
    ToolchainError(#[from] ToolchainError),
}

pub type TermPreludeResult<T> = Result<T, TermPreludeError>;
