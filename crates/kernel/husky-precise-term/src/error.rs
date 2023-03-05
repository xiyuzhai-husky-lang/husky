use husky_entity_path::{EntityPath, EntityPathError};
use husky_raw_term::RawTermSymbolTypeErrorKind;
use husky_raw_ty::RawTypeError;
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum PreciseTermError {
    #[error("precise_term is not reduced")]
    PreciseTermIsNotReduced,
    #[error("precise_term is not type")]
    PreciseTermIsNotTy,
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
}

impl From<&EntityPathError> for PreciseTermError {
    fn from(_value: &EntityPathError) -> Self {
        todo!()
    }
}

impl From<RawTermSymbolTypeErrorKind> for PreciseTermError {
    fn from(value: RawTermSymbolTypeErrorKind) -> Self {
        todo!()
    }
}

pub type PreciseTermResult<T> = Result<T, PreciseTermError>;
pub type PreciseTermResultArc<T> = Result<Arc<T>, PreciseTermError>;
