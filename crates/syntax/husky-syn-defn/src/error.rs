use crate::*;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[salsa::debug_with_db(db = SynDefnDb)]
pub enum SynDefnError {
    #[error("{0}")]
    Original(#[from] OriginalSynDefnError),
    #[error("{0}")]
    Derived(#[from] DerivedSynDefnError),
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[salsa::debug_with_db(db = SynDefnDb)]
pub enum OriginalSynDefnError {
    #[error("expect body")]
    ExpectBody,
}

impl From<DeclError> for SynDefnError {
    fn from(value: DeclError) -> Self {
        SynDefnError::Derived(DerivedSynDefnError::Decl)
    }
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
// #[salsa::derive_debug_with_db(db = DefnDb)]
pub enum DerivedSynDefnError {
    #[error("decl")]
    Decl,
}

pub type SynDefnResult<T> = Result<T, SynDefnError>;
