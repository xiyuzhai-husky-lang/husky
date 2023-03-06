use crate::*;
use husky_ty::{DerivedValidTypeError, OriginalValidTypeError, ValidTypeError};
use thiserror::Error;

pub type TypeResult<T> = Result<T, TypeError>;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum TypeError {
    #[error("original `{0}`")]
    Original(#[from] OriginalTypeError),
    #[error("derived `{0}`")]
    Derived(#[from] DerivedTypeError),
}

impl From<ValidTypeError> for TypeError {
    fn from(e: ValidTypeError) -> Self {
        match e {
            ValidTypeError::Original(e) => TypeError::Original(e.into()),
            ValidTypeError::Derived(e) => TypeError::Derived(e.into()),
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum OriginalTypeError {
    #[error("term error")]
    Term(#[from] TermError),
    #[error("todo")]
    Todo,
    #[error("{0}")]
    ValidTypeError(#[from] OriginalValidTypeError),
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum DerivedTypeError {
    #[error("signature error")]
    SignatureError,
    #[error("declaration error")]
    DeclError,
    #[error("{0}")]
    ValidTypeError(#[from] DerivedValidTypeError),
}

impl<Db: TermDb + ?Sized> salsa::DebugWithDb<Db> for TypeError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(&self, f)
    }
}
