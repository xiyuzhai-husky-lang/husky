use super::*;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum LocalTermResolveError {
    #[error("original {0}")]
    Original(#[from] OriginalLocalTermResolveError),
    #[error("derived {0}")]
    Derived(#[from] DerivedLocalTermResolveError),
}

pub type LocalTermResolveResult<T> = Result<T, LocalTermResolveError>;
pub type LocalTermResolveResultRef<'a, T> = Result<T, &'a LocalTermResolveError>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalLocalTermResolveError {
    #[error("unresolved term")]
    UnresolvedTerm,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedLocalTermResolveError {
    #[error("duplication")]
    Duplication,
    #[error("unresolved local term")]
    UnresolvedLocalTerm,
}
