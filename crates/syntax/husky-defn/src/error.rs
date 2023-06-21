use crate::*;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = DefnDb)]
pub enum DefnError {
    #[error("{0}")]
    Original(#[from] OriginalDefnError),
    #[error("{0}")]
    Derived(#[from] DerivedDefnError),
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = DefnDb)]
pub enum OriginalDefnError {
    #[error("expect body")]
    ExpectBody,
}

impl From<DeclError> for DefnError {
    fn from(value: DeclError) -> Self {
        todo!()
    }
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
// #[salsa::derive_debug_with_db(db = DefnDb)]
pub enum DerivedDefnError {}

pub type DefnResult<T> = Result<T, DefnError>;
