use super::*;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum FluffyTermResolveError {
    #[error("original {0}")]
    Original(#[from] OriginalFluffyTermResolveError),
    #[error("derived {0}")]
    Derived(#[from] DerivedFluffyTermResolveError),
}

pub type FluffyTermResolveResult<T> = Result<T, FluffyTermResolveError>;
pub type FluffyTermResolveResultRef<'a, T> = Result<T, &'a FluffyTermResolveError>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalFluffyTermResolveError {
    #[error("unresolved term")]
    UnresolvedTerm,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedFluffyTermResolveError {
    #[error("duplication")]
    Duplication,
    #[error("unresolved Fluffy term")]
    UnresolvedFluffyTerm,
}
