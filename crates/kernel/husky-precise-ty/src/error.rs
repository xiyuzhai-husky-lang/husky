use crate::*;
use husky_raw_term::RawTermError;
use husky_raw_ty::{DerivedRawTypeError, OriginalRawTypeError, RawTypeError};
use thiserror::Error;

pub type PreciseTypeResult<T> = Result<T, PreciseTypeError>;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum PreciseTypeError {
    #[error("original `{0}`")]
    Original(#[from] OriginalPreciseTypeError),
    #[error("derived `{0}`")]
    Derived(#[from] DerivedPreciseTypeError),
}

impl From<PreciseTermError> for PreciseTypeError {
    fn from(value: PreciseTermError) -> Self {
        todo!()
    }
}

impl From<RawTypeError> for PreciseTypeError {
    fn from(e: RawTypeError) -> Self {
        match e {
            RawTypeError::Original(e) => PreciseTypeError::Original(e.into()),
            RawTypeError::Derived(e) => PreciseTypeError::Derived(e.into()),
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum OriginalPreciseTypeError {
    #[error("term error")]
    PreciseTerm(#[from] PreciseTermError),
    #[error("Original Precise Type Error ← {0}")]
    RawTypeError(#[from] OriginalRawTypeError),
    #[error("todo")]
    Todo,
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum DerivedPreciseTypeError {
    #[error("signature error")]
    SignatureError,
    #[error("declaration error")]
    DeclError,
    #[error("Derived Precise Type Error ← {0}")]
    RawTypeError(#[from] DerivedRawTypeError),
}

impl<Db: PreciseTypeDb + ?Sized> salsa::DebugWithDb<Db> for PreciseTypeError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(&self, f)
    }
}
