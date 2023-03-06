use crate::*;
use thiserror::Error;

pub type TypeResult<T> = Result<T, TypeError>;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum TypeError {
    #[error("original `{0}`")]
    Original(#[from] OriginalTypeError),
    #[error("derived `{0}`")]
    Derived(#[from] TermError),
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum OriginalTypeError {
    #[error("term error")]
    Term(#[from] TermError),
    #[error("todo")]
    Todo,
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum TermError {
    #[error("signature error")]
    SignatureError,
    #[error("declaration error")]
    DeclError,
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
