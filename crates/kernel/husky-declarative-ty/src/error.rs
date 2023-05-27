use crate::*;
use husky_decl::DeclError;
use thiserror::Error;

pub type DeclarativeTypeResult<T> = Result<T, DeclarativeTypeError>;

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = DeclarativeTypeDb)]
pub enum DeclarativeTypeError {
    #[error("original `{0}`")]
    Original(#[from] OriginalDeclarativeTypeError),
    #[error("derived `{0}`")]
    Derived(#[from] DerivedDeclarativeTypeError),
}

impl From<&DeclError> for DeclarativeTypeError {
    fn from(value: &DeclError) -> Self {
        todo!()
    }
}

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = DeclarativeTypeDb)]
pub enum OriginalDeclarativeTypeError {
    #[error("declarative_term error")]
    DeclarativeTerm(#[from] DeclarativeTermError),
    #[error("InductiveTypeHasNoConstructor")]
    InductiveTypeHasNoConstructor,
    #[error("todo")]
    Todo,
}

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = DeclarativeTypeDb)]
pub enum DerivedDeclarativeTypeError {
    #[error("signature error")]
    SignatureError,
    #[error("type constructor declaration error")]
    TypeConstructorDeclError,
    #[error("type ontology declaration error")]
    TypeOntologyDeclError { path: TypePath },
    #[error("trait declaration error")]
    TraitDeclError,
    #[error("form declaration error")]
    FugitiveDeclError,
    #[error("type path field declaration error")]
    TypePathFieldDeclError,
    #[error("type path application field declaration error")]
    TypePathApplicationFieldDeclError,
    #[error("type path method declaration error")]
    TypePathMethodFnDeclError,
    #[error("type path application method declaration error")]
    TypePathApplicationMethodFnDeclError,
    #[error("TypeItemNotFound")]
    TypeItemNotFound,
}
