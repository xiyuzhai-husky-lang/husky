use thiserror::Error;

#[salsa::debug_with_db]
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ManifestAstError {
    #[error("{0}")]
    Original(#[from] OriginalManifestAstError),
    #[error("{0}")]
    Derived(#[from] DerivedManifestAstError),
}

#[salsa::debug_with_db]
#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalManifestAstError {
    #[error("MissingPackageSection")]
    MissingPackageSection,
    #[error("InvalidName")]
    InvalidName,
    #[error("todo")]
    Todo,
}

#[salsa::debug_with_db]
#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedManifestAstError {
    #[error("todo")]
    Todo,
}

pub type ManifestAstResult<T> = Result<T, ManifestAstError>;
pub type ManifestAstResultRef<'a, T> = Result<T, &'a ManifestAstError>;
