use husky_manifest::ManifestError;
use husky_vfs::{error::VfsError, *};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[salsa::debug_with_db]
pub enum PreludeError {
    #[error("{0}")]
    Toolchain(#[from] ToolchainError),
    #[error("manifest error")]
    ManifestError,
    #[error("vfs error {0}")]
    VfsError(#[from] VfsError),
}
pub type PreludeResult<T> = Result<T, PreludeError>;

impl From<&PreludeError> for PreludeError {
    fn from(_value: &PreludeError) -> Self {
        todo!()
    }
}

impl From<&ManifestError> for PreludeError {
    fn from(_value: &ManifestError) -> Self {
        todo!()
    }
}
