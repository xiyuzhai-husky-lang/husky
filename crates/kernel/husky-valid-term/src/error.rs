use crate::*;
use husky_entity_path::{EntityPath, EntityPathError};
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum ValidTermError {
    #[error("valid_term is not reduced")]
    ValidTermIsNotReduced,
    #[error("valid_term is not type")]
    ValidTermIsNotTy,
    #[error("universe overflows")]
    UniverseOverflow,
    #[error("monad is not input")]
    MonadIsNotInput,
    #[error("no decl for entity path")]
    NoDeclForEntityPath { entity_path: EntityPath },
    #[error("precise term error {0}")]
    PreciseTermError(#[from] PreciseTermError),
    #[error("precise type error {0}")]
    PreciseTypeError(#[from] PreciseTypeError),
    #[error("EntityPathError")]
    EntityPathError(#[from] EntityPathError),
}

impl From<&EntityPathError> for ValidTermError {
    fn from(_value: &EntityPathError) -> Self {
        todo!()
    }
}

pub type ValidTermResult<T> = Result<T, ValidTermError>;
pub type ValidTermResultArc<T> = Result<Arc<T>, ValidTermError>;
