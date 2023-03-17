use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ManifestAstError {
    #[error("{0}")]
    Original(#[from] OriginalManifestAstError),
    #[error("{0}")]
    Derived(#[from] DerivedManifestAstError),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalManifestAstError {
    #[error("MissingPackageSection")]
    MissingPackageSection,
    #[error("todo")]
    Todo,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedManifestAstError {
    #[error("todo")]
    Todo,
}

pub type ManifestAstResult<T> = Result<T, ManifestAstError>;
