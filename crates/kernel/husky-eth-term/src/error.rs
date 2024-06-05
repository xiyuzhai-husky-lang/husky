use crate::{term::EthTerm, *};
use husky_dec_signature::{error::DecSignatureError, region::error::DerivedSynExprDecTermError};
use husky_dec_term::term::DecSymbolicVariableTypeErrorKind;
use husky_dec_ty::DeclarativeTypeError;
use husky_entity_path::{
    error::EntityPathError,
    path::{
        major_item::ty::{PreludeTypePath, TypePath},
        ItemPath,
    },
};
use maybe_result::MaybeResult;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db]
pub enum EthTermError {
    #[error("EthTerm Error: term is not reduced")]
    TermIsNotReduced,
    #[error("EthTerm Error: term is not type")]
    TermIsNotTy,
    #[error("EthTerm Error: universe overflows")]
    UniverseOverflow,
    #[error("EthTerm Error: monad is not input")]
    MonadIsNotInput,
    #[error("EthTerm Error: no decl for item path")]
    NoDeclForEntityPath { item_path: ItemPath },
    #[error("EntityPathError")]
    EntityPathError,
    #[error("DecTermError")]
    DecTermError(#[from] DecTermError),
    #[error("DeclarativeTypeError")]
    DeclarativeTypeError(#[from] DeclarativeTypeError),
    #[error("ExpectationNotMatchedForCurry")]
    ExpectationNotMatchedForCurry,
    #[error("DecTermSymbolTypeErrorKind")]
    DecTermSymbolTypeErrorKind(#[from] DecSymbolicVariableTypeErrorKind),
    #[error("ExpectFinalDestinationEqsNonSortTypePath")]
    ExpectFinalDestinationEqsNonSortTypePath {
        path_expected: TypePath,
        path: TypePath,
    },
    #[error("TermPreludeError")]
    TermPreludeError(#[from] TermPreludeError),
    #[error("TermApplicationWrongArgumentType")]
    TermApplicationWrongArgumentType {
        parameter_ty: EthTerm,
        argument_ty: Either<EthTerm, PreludeTypePath>,
    },
    #[error("TypePathApplicationMethodFnDeclError")]
    TypePathApplicationMethodFnDeclError,
    #[error("SignatureError")]
    SignatureError(#[from] DecSignatureError),
    #[error("EntityTreeBundleError")]
    EntityTreeBundleError,
    #[error("ForDeriveArgument")]
    ExpectTraitForDeriveArgument,
    #[error("NoSuchMethod")]
    NoSuchMethod,
    #[error("ExpectedCurryForApplicationFunctionType")]
    ExpectedCurryForApplicationFunctionType,
    #[error("ExpectedType")]
    ExpectedType { expectee: DecTerm },
    #[error("TaskTypeNotInferred")]
    TaskTypeNotInferred,
}

impl From<EntityPathError> for EthTermError {
    fn from(_value: EntityPathError) -> Self {
        EthTermError::EntityPathError
    }
}

impl From<&EntityPathError> for EthTermError {
    fn from(_value: &EntityPathError) -> Self {
        EthTermError::EntityPathError
    }
}

impl From<DerivedSynExprDecTermError> for EthTermError {
    fn from(e: DerivedSynExprDecTermError) -> Self {
        todo!()
    }
}

pub type EthTermResult<T> = Result<T, EthTermError>;

pub type EthTermMaybeResult<T> = MaybeResult<T, EthTermError>;
