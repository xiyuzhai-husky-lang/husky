use crate::*;
use husky_precise_ty::PreciseTypeError;
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
    fn from(value: PreciseTypeError) -> Self {
        todo!()
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
