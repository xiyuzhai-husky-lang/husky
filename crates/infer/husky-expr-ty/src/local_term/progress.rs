use super::*;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum LocalTermResolveProgress {
    Unresolved,
    PartiallyResolved(UnresolvedTermIdx),
    FullyResolved(ReducedTerm),
    Err(LocalTermResolveError),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum LocalTermResolveError {
    #[error("original {0}")]
    Original(#[from] OriginalLocalTermResolveError),
    #[error("derived {0}")]
    Derived(#[from] DerivedLocalTermResolveError),
}

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

impl LocalTermResolveProgress {
    // it will use derived type error
    pub(crate) fn duplicate(&self) -> Self {
        match self {
            LocalTermResolveProgress::Unresolved => LocalTermResolveProgress::Unresolved,
            LocalTermResolveProgress::PartiallyResolved(unresolved_term) => {
                LocalTermResolveProgress::PartiallyResolved(*unresolved_term)
            }
            LocalTermResolveProgress::FullyResolved(resolved_term) => {
                LocalTermResolveProgress::FullyResolved(*resolved_term)
            }
            LocalTermResolveProgress::Err(_) => {
                LocalTermResolveProgress::Err(DerivedLocalTermResolveError::Duplication.into())
            }
        }
    }

    pub(crate) fn reduced_term(&self) -> Option<ReducedTerm> {
        match self {
            LocalTermResolveProgress::FullyResolved(reduced_term) => Some(*reduced_term),
            LocalTermResolveProgress::Unresolved
            | LocalTermResolveProgress::PartiallyResolved(_)
            | LocalTermResolveProgress::Err(_) => None,
        }
    }

    pub(crate) fn new(substitution: LocalTerm) -> LocalTermResolveProgress {
        match substitution {
            LocalTerm::Resolved(term) => LocalTermResolveProgress::FullyResolved(term),
            LocalTerm::Unresolved(term) => LocalTermResolveProgress::PartiallyResolved(term),
        }
    }
}
