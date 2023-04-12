use super::*;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum HollowTermResolveError {
    #[error("original {0}")]
    Original(#[from] OriginalHollowTermResolveError),
    #[error("derived {0}")]
    Derived(#[from] DerivedHollowTermResolveError),
}

pub type FluffyTermResolveResult<T> = Result<T, HollowTermResolveError>;
pub type FluffyTermResolveResultRef<'a, T> = Result<T, &'a HollowTermResolveError>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalHollowTermResolveError {
    #[error("unresolved term")]
    UnresolvedTerm,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedHollowTermResolveError {
    #[error("duplication")]
    Duplication,
    #[error("unresolved Fluffy term")]
    UnresolvedFluffyTerm,
}
