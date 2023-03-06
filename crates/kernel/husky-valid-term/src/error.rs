use crate::*;
use husky_entity_path::{EntityPath, EntityPathError};
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum RawTermError {
    #[error("valid_term is not reduced")]
    RawTermIsNotReduced,
    #[error("valid_term is not type")]
    RawTermIsNotTy,
    #[error("universe overflows")]
    UniverseOverflow,
    #[error("monad is not input")]
    MonadIsNotInput,
    #[error("no decl for entity path")]
    NoDeclForEntityPath { entity_path: EntityPath },
    #[error("precise term error {0}")]
    RawTermError(#[from] RawTermError),
    #[error("precise type error {0}")]
    PreciseTypeError(#[from] PreciseTypeError),
    #[error("EntityPathError")]
    EntityPathError(#[from] EntityPathError),
}

impl From<&EntityPathError> for RawTermError {
    fn from(_value: &EntityPathError) -> Self {
        todo!()
    }
}

pub type RawTermResult<T> = Result<T, RawTermError>;
pub type RawTermResultArc<T> = Result<Arc<T>, RawTermError>;
