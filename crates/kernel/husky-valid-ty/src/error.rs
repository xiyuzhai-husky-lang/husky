use crate::*;
use thiserror::Error;

pub type ValidTypeResult<T> = Result<T, ValidTypeError>;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum ValidTypeError {
    #[error("original `{0}`")]
    Original(OriginalValidTypeError),
    #[error("derived `{0}`")]
    Derived(DerivedValidTypeError),
}

impl From<OriginalValidTypeError> for ValidTypeError {
    fn from(v: OriginalValidTypeError) -> Self {
        Self::Original(v)
    }
}

impl From<DerivedValidTypeError> for ValidTypeError {
    fn from(v: DerivedValidTypeError) -> Self {
        Self::Derived(v)
    }
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum OriginalValidTypeError {
    #[error("term error")]
    ValidTerm(#[from] ValidTermError),
    #[error("todo")]
    Todo,
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum DerivedValidTypeError {
    #[error("signature error")]
    SignatureError,
    #[error("declaration error")]
    DeclError,
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
