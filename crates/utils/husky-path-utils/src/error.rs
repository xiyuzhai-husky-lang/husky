use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum PathUtilsError {
    #[error("cargo manifest dir not found")]
    CargoManifestDirNotFound,
}

pub type PathUtilsResult<T> = Result<T, PathUtilsError>;
