use crate::*;
use thiserror::Error;

pub type RawTypeResult<T> = Result<T, RawTypeError>;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum RawTypeError {
    #[error("original `{0}`")]
    Original(#[from] OriginalRawTypeError),
    #[error("derived `{0}`")]
    Derived(#[from] DerivedRawTypeError),
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
    #[error("type constructor decl error")]
    TypeConstructorDeclError,
    #[error("type ontology decl error")]
    TypeOntologyDeclError,
    #[error("trait decl error")]
    TraitDeclError,
    #[error("form decl error")]
    FormDeclError,
    #[error("type path field decl error")]
    TypePathFieldDeclError,
    #[error("type path application field decl error")]
    TypePathApplicationFieldDeclError,
    #[error("type path method decl error")]
    TypePathMethodDeclError,
    #[error("type path application method decl error")]
    TypePathApplicationMethodDeclError,
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
