use crate::*;
use husky_precise_ty::{DerivedPreciseTypeError, OriginalPreciseTypeError, PreciseTypeError};
use thiserror::Error;

pub type ValidTypeResult<T> = Result<T, ValidTypeError>;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum ValidTypeError {
    #[error("original `{0}`")]
    Original(#[from] OriginalValidTypeError),
    #[error("derived `{0}`")]
    Derived(#[from] DerivedValidTypeError),
}

impl From<PreciseTypeError> for ValidTypeError {
    fn from(e: PreciseTypeError) -> Self {
        match e {
            PreciseTypeError::Original(e) => ValidTypeError::Original(e.into()),
            PreciseTypeError::Derived(e) => ValidTypeError::Derived(e.into()),
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum OriginalValidTypeError {
    #[error("term error")]
    ValidTerm(#[from] ValidTermError),
    #[error("todo")]
    Todo,
    #[error("Original Valid Type Error {0}")]
    PreciseTypeError(#[from] OriginalPreciseTypeError),
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum DerivedValidTypeError {
    #[error("signature error")]
    SignatureError,
    #[error("declaration error")]
    DeclError,
    #[error("Derived Valid Type Error {0}")]
    PreciseTypeError(#[from] DerivedPreciseTypeError),
}

impl<Db: ValidTypeDb + ?Sized> salsa::DebugWithDb<Db> for ValidTypeError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(&self, f)
    }
}
