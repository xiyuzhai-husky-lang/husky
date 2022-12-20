use husky_absolute_path::AbsolutePathError;
use husky_package_path::PackagePathError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum EntityPathError {
    // #[error("{0}")]
    // AbsolutePath(#[from] AbsolutePathError),
    #[error("{0}")]
    PackagePath(#[from] PackagePathError),
}

pub type EntityPathResult<T> = Result<T, EntityPathError>;
