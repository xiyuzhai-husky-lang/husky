use crate::*;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::debug_with_db(db = ManifestAstDb, jar = ManifestAstJar)]
pub enum ManifestAstError {
    #[error("{0}")]
    Original(#[from] OriginalManifestAstError),
    #[error("{0}")]
    Derived(#[from] DerivedManifestAstError),
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::debug_with_db(db = ManifestAstDb, jar = ManifestAstJar)]
pub enum OriginalManifestAstError {
    #[error("MissingPackageSection")]
    MissingPackageSection,
    #[error("InvalidName")]
    InvalidName,
    #[error("todo")]
    Todo,
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::debug_with_db(db = ManifestAstDb, jar = ManifestAstJar)]
pub enum DerivedManifestAstError {
    #[error("todo")]
    Todo,
}

pub type ManifestAstResult<T> = Result<T, ManifestAstError>;
pub type ManifestAstResultRef<'a, T> = Result<T, &'a ManifestAstError>;
