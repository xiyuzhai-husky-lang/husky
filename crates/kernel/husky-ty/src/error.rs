use crate::*;
use thiserror::Error;

pub type TypeResult<T> = Result<T, TypeError>;
pub type TypeResultRef<'a, T> = Result<T, &'a TypeError>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TypeError {
    #[error("original `{0}`")]
    Original(OriginalTypeError),
    #[error("derived `{0}`")]
    Derived(DerivedTypeError),
}

impl From<OriginalTypeError> for TypeError {
    fn from(v: OriginalTypeError) -> Self {
        Self::Original(v)
    }
}

impl From<DerivedTypeError> for TypeError {
    fn from(v: DerivedTypeError) -> Self {
        Self::Derived(v)
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalTypeError {}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedTypeError {
    #[error("signature error")]
    SignatureError,
    #[error("declaration error")]
    DeclError,
}

impl<Db: TypeDb + ?Sized> salsa::DebugWithDb<Db> for TypeError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(&self, f)
    }
}
