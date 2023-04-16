use crate::*;
use husky_decl::DeclExprError;
use husky_decr::DecrError;
use husky_entity_path::{EntityPath, EntityPathError};
use husky_entity_tree::{EntityTreeBundleError, EntityTreeError};
use husky_raw_ty::RawTypeError;
use husky_signature::SignatureError;
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = EtherealTermDb)]
pub enum TermError {
    #[error("EtherealTerm Error: term is not reduced")]
    TermIsNotReduced,
    #[error("EtherealTerm Error: term is not type")]
    TermIsNotTy,
    #[error("EtherealTerm Error: universe overflows")]
    UniverseOverflow,
    #[error("EtherealTerm Error: monad is not input")]
    MonadIsNotInput,
    #[error("EtherealTerm Error: no decl for entity path")]
    NoDeclForEntityPath { entity_path: EntityPath },
    #[error("EntityPathError")]
    EntityPathError,
    #[error("RawTypeError")]
    RawTypeError(#[from] RawTypeError),
    #[error("ExpectationNotMatchedForCurry")]
    ExpectationNotMatchedForCurry,
    #[error("RawTermSymbolTypeErrorKind")]
    RawTermSymbolTypeErrorKind(#[from] RawTermSymbolTypeErrorKind),
    #[error("ExpectFinalDestinationEqsNonSortTypePath")]
    ExpectFinalDestinationEqsNonSortTypePath {
        path_expected: TypePath,
        path: TypePath,
    },
    #[error("TermPreludeError")]
    TermPreludeError(#[from] TermPreludeError),
    #[error("TermApplicationWrongArgumentType")]
    TermApplicationWrongArgumentType {
        parameter_ty: EtherealTerm,
        argument_ty: Either<EtherealTerm, PreludeTypePath>,
    },
    #[error("TypePathApplicationMethodDeclError")]
    TypePathApplicationMethodDeclError,
    #[error("SignatureError")]
    SignatureError(#[from] SignatureError),
    #[error("EntityTreeBundleError")]
    EntityTreeBundleError,
    #[error("ForDeriveArgument")]
    ExpectTraitForDeriveArgument,
    #[error("NoSuchMethod")]
    NoSuchMethod,
}

impl From<EntityPathError> for TermError {
    fn from(_value: EntityPathError) -> Self {
        TermError::EntityPathError
    }
}

impl From<&EntityPathError> for TermError {
    fn from(_value: &EntityPathError) -> Self {
        TermError::EntityPathError
    }
}

impl From<EntityTreeError> for TermError {
    fn from(value: EntityTreeError) -> Self {
        todo!()
    }
}

impl From<&EntityTreeBundleError> for TermError {
    fn from(value: &EntityTreeBundleError) -> Self {
        todo!()
    }
}

impl From<&DeclExprError> for TermError {
    fn from(value: &DeclExprError) -> Self {
        todo!()
    }
}

impl From<EntityTreeBundleError> for TermError {
    fn from(value: EntityTreeBundleError) -> Self {
        todo!()
    }
}

impl From<&DecrError> for TermError {
    fn from(value: &DecrError) -> Self {
        todo!()
    }
}

pub type TermResult<T> = Result<T, TermError>;
pub type TermResultArc<T> = Result<Arc<T>, TermError>;
