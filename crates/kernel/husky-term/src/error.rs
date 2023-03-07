use crate::*;
use husky_entity_path::{EntityPath, EntityPathError};
use husky_raw_ty::RawTypeError;
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = TermDb)]
pub enum TermError {
    #[error("Term Error: term is not reduced")]
    TermIsNotReduced,
    #[error("Term Error: term is not type")]
    TermIsNotTy,
    #[error("Term Error: universe overflows")]
    UniverseOverflow,
    #[error("Term Error: monad is not input")]
    MonadIsNotInput,
    #[error("Term Error: no decl for entity path")]
    NoDeclForEntityPath { entity_path: EntityPath },
    #[error("EntityPathError")]
    EntityPathError(#[from] EntityPathError),
    #[error("RawTypeError")]
    RawTypeError(#[from] RawTypeError),
    #[error("ExpectationNotMatchedForCurry")]
    ExpectationNotMatchedForCurry,
    #[error("RawTermSymbolTypeErrorKind")]
    RawTermSymbolTypeErrorKind(#[from] RawTermSymbolTypeErrorKind),
}

impl From<&EntityPathError> for TermError {
    fn from(_value: &EntityPathError) -> Self {
        todo!()
    }
}

pub type TermResult<T> = Result<T, TermError>;
pub type TermResultRef<'a, T> = Result<T, &'a TermError>;
pub type TermResultArc<T> = Result<Arc<T>, TermError>;
