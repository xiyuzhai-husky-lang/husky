use crate::*;
use husky_raw_term::RawTermError;
use husky_raw_ty::RawTypeError;
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
    fn from(value: RawTypeError) -> Self {
        todo!()
    }
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum OriginalPreciseTypeError {
    #[error("term error")]
    PreciseTerm(#[from] PreciseTermError),
    #[error("todo")]
    Todo,
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum DerivedPreciseTypeError {
    #[error("signature error")]
    SignatureError,
    #[error("declaration error")]
    DeclError,
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
