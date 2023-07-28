use crate::*;
use husky_declarative_signature::DeclarativeSignatureError;
use husky_declarative_ty::DeclarativeTypeError;
use husky_entity_path::{EntityPathError, ItemPath};
use husky_entity_syn_tree::{EntityTreeBundleError, EntityTreeError};
use husky_syn_decl::NodeDeclError;
use husky_syn_decr::DecrError;
use maybe_result::MaybeResult;
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = EtherealTermDb)]
pub enum EtherealTermError {
    #[error("EtherealTerm Error: term is not reduced")]
    TermIsNotReduced,
    #[error("EtherealTerm Error: term is not type")]
    TermIsNotTy,
    #[error("EtherealTerm Error: universe overflows")]
    UniverseOverflow,
    #[error("EtherealTerm Error: monad is not input")]
    MonadIsNotInput,
    #[error("EtherealTerm Error: no decl for item path")]
    NoDeclForEntityPath { item_path: ItemPath },
    #[error("EntityPathError")]
    EntityPathError,
    #[error("DeclarativeTypeError")]
    DeclarativeTypeError(#[from] DeclarativeTypeError),
    #[error("ExpectationNotMatchedForCurry")]
    ExpectationNotMatchedForCurry,
    #[error("DeclarativeTermSymbolTypeErrorKind")]
    DeclarativeTermSymbolTypeErrorKind(#[from] DeclarativeTermSymbolTypeErrorKind),
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
    #[error("TypePathApplicationMethodFnDeclError")]
    TypePathApplicationMethodFnDeclError,
    #[error("SignatureError")]
    SignatureError(#[from] DeclarativeSignatureError),
    #[error("EntityTreeBundleError")]
    EntityTreeBundleError,
    #[error("ForDeriveArgument")]
    ExpectTraitForDeriveArgument,
    #[error("NoSuchMethod")]
    NoSuchMethod,
    #[error("ExpectedCurryForApplicationFunctionType")]
    ExpectedCurryForApplicationFunctionType,
}

impl From<EntityPathError> for EtherealTermError {
    fn from(_value: EntityPathError) -> Self {
        EtherealTermError::EntityPathError
    }
}

impl From<&EntityPathError> for EtherealTermError {
    fn from(_value: &EntityPathError) -> Self {
        EtherealTermError::EntityPathError
    }
}

impl From<EntityTreeError> for EtherealTermError {
    fn from(value: EntityTreeError) -> Self {
        todo!()
    }
}

impl From<&EntityTreeBundleError> for EtherealTermError {
    fn from(value: &EntityTreeBundleError) -> Self {
        todo!()
    }
}

impl From<&NodeDeclError> for EtherealTermError {
    fn from(value: &NodeDeclError) -> Self {
        todo!()
    }
}

impl From<EntityTreeBundleError> for EtherealTermError {
    fn from(value: EntityTreeBundleError) -> Self {
        todo!()
    }
}

impl From<&DecrError> for EtherealTermError {
    fn from(value: &DecrError) -> Self {
        todo!()
    }
}

impl From<DeclarativeTermError> for EtherealTermError {
    fn from(value: DeclarativeTermError) -> Self {
        todo!()
    }
}

pub type EtherealTermResult<T> = Result<T, EtherealTermError>;

pub type EtherealTermMaybeResult<T> = MaybeResult<T, EtherealTermError>;
