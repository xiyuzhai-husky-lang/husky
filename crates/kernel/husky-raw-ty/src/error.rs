use crate::*;
use thiserror::Error;

pub type RawTypeResult<T> = Result<T, RawTypeError>;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum RawTypeError {
    #[error("original `{0}`")]
    Original(OriginalRawTypeError),
    #[error("derived `{0}`")]
    Derived(DerivedRawTypeError),
}

impl From<OriginalRawTypeError> for RawTypeError {
    fn from(v: OriginalRawTypeError) -> Self {
        Self::Original(v)
    }
}

impl From<DerivedRawTypeError> for RawTypeError {
    fn from(v: DerivedRawTypeError) -> Self {
        Self::Derived(v)
    }
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum OriginalRawTypeError {
    #[error("raw_term error")]
    RawTerm(#[from] RawTermError),
    #[error("todo")]
    Todo,
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum DerivedRawTypeError {
    #[error("signature error")]
    SignatureError,
    #[error("declaration error")]
    DeclError,
}

impl<Db: RawTypeDb + ?Sized> salsa::DebugWithDb<Db> for RawTypeError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(&self, f)
    }
}
