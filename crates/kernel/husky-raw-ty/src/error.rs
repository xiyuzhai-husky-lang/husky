use crate::*;
use husky_decl::DeclError;
use thiserror::Error;

pub type RawTypeResult<T> = Result<T, RawTypeError>;

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = RawTypeDb)]
pub enum RawTypeError {
    #[error("original `{0}`")]
    Original(#[from] OriginalRawTypeError),
    #[error("derived `{0}`")]
    Derived(#[from] DerivedRawTypeError),
}

impl From<&DeclError> for RawTypeError {
    fn from(value: &DeclError) -> Self {
        todo!()
    }
}

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = RawTypeDb)]
pub enum OriginalRawTypeError {
    #[error("raw_term error")]
    RawTerm(#[from] RawTermError),
    #[error("InductiveTypeHasNoConstructor")]
    InductiveTypeHasNoConstructor,
    #[error("todo")]
    Todo,
}

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = RawTypeDb)]
pub enum DerivedRawTypeError {
    #[error("signature error")]
    SignatureError,
    #[error("type constructor declaration error")]
    TypeConstructorDeclError,
    #[error("type ontology declaration error")]
    TypeOntologyDeclError { path: TypePath },
    #[error("trait declaration error")]
    TraitDeclError,
    #[error("form declaration error")]
    FormDeclError,
    #[error("type path field declaration error")]
    TypePathFieldDeclError,
    #[error("type path application field declaration error")]
    TypePathApplicationFieldDeclError,
    #[error("type path method declaration error")]
    TypePathMethodDeclError,
    #[error("type path application method declaration error")]
    TypePathApplicationMethodDeclError,
    #[error("TypeItemNotFound")]
    TypeItemNotFound,
}
