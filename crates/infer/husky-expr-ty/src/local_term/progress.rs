use super::*;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum LocalTermResolveProgress {
    Ok(LocalTerm),
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
            LocalTermResolveProgress::Ok(local_term) => LocalTermResolveProgress::Ok(*local_term),
            LocalTermResolveProgress::Err(_) => {
                LocalTermResolveProgress::Err(DerivedLocalTermResolveError::Duplication.into())
            }
        }
    }

    pub(crate) fn reduced_term(&self) -> Option<ReducedTerm> {
        match self {
            LocalTermResolveProgress::Ok(local_term) => match local_term {
                LocalTerm::Resolved(reduced_term) => Some(*reduced_term),
                LocalTerm::Unresolved(_) => todo!(),
            },
            LocalTermResolveProgress::Err(_) => None,
        }
    }
}
