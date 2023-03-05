use crate::*;
use thiserror::Error;

pub type PreciseTypeResult<T> = Result<T, PreciseTypeError>;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum PreciseTypeError {
    #[error("original `{0}`")]
    Original(OriginalPreciseTypeError),
    #[error("derived `{0}`")]
    Derived(DerivedPreciseTypeError),
}

impl From<OriginalPreciseTypeError> for PreciseTypeError {
    fn from(v: OriginalPreciseTypeError) -> Self {
        Self::Original(v)
    }
}

impl From<DerivedPreciseTypeError> for PreciseTypeError {
    fn from(v: DerivedPreciseTypeError) -> Self {
        Self::Derived(v)
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
