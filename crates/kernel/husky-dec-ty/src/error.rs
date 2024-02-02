use crate::*;
use husky_dec_term::term::DecTermSymbolTypeErrorKind;
use husky_syn_decl::DeclError;
use thiserror::Error;

pub type DeclarativeTypeResult<T> = Result<T, DeclarativeTypeError>;

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db]
pub enum DeclarativeTypeError {
    #[error("original `{0}`")]
    Original(#[from] OriginalDeclarativeTypeError),
    #[error("derived `{0}`")]
    Derived(#[from] DerivedDeclarativeTypeError),
}

impl From<DecTermSymbolTypeErrorKind> for DeclarativeTypeError {
    fn from(e: DecTermSymbolTypeErrorKind) -> Self {
        DerivedDeclarativeTypeError::SymbolType(e).into()
    }
}

impl From<&DeclError> for DeclarativeTypeError {
    fn from(_value: &DeclError) -> Self {
        todo!()
    }
}

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db]
pub enum OriginalDeclarativeTypeError {
    #[error("declarative_term error")]
    DecTerm(#[from] DecTermError),
    #[error("EnumTypeHasNoConstructor")]
    EnumTypeNoConstructor,
    #[error("InductiveTypeHasNoConstructor")]
    InductiveTypeHasNoConstructor,
    #[error("todo")]
    Todo,
    #[error("todo")]
    ExternTypeHasNoConstructor,
}

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db]
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
    #[error("SymbolType")]
    SymbolType(DecTermSymbolTypeErrorKind),
}
