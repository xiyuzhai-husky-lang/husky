use husky_entity_path::{EntityPath, EntityPathError};
use husky_raw_term::RawTermSymbolTypeErrorKind;
use husky_raw_ty::RawTypeError;
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum RawTermError {
    #[error("precise_term is not reduced")]
    RawTermIsNotReduced,
    #[error("precise_term is not type")]
    RawTermIsNotTy,
    #[error("universe overflows")]
    UniverseOverflow,
    #[error("monad is not input")]
    MonadIsNotInput,
    #[error("no decl for entity path")]
    NoDeclForEntityPath { entity_path: EntityPath },
    #[error("raw type error")]
    RawTypeError(#[from] RawTypeError),
    #[error("ExpectationNotMatchedForCurry")]
    ExpectationNotMatchedForCurry,
    #[error("EntityPathError")]
    EntityPathError(#[from] EntityPathError),
}

impl From<&EntityPathError> for RawTermError {
    fn from(_value: &EntityPathError) -> Self {
        todo!()
    }
}

impl From<RawTermSymbolTypeErrorKind> for RawTermError {
    fn from(value: RawTermSymbolTypeErrorKind) -> Self {
        todo!()
    }
}

pub type RawTermResult<T> = Result<T, RawTermError>;
pub type RawTermResultArc<T> = Result<Arc<T>, RawTermError>;
