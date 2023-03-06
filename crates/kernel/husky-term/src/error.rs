use crate::*;
use husky_entity_path::{EntityPath, EntityPathError};
use husky_precise_term::PreciseTermError;
use husky_valid_ty::ValidTypeError;
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone)]
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
    #[error("Term Error ← {0}")]
    PreciseTermError(#[from] PreciseTermError),
    #[error("Term Error ← {0}")]
    ValidTermError(#[from] ValidTermError),
    #[error("Term Error ← {0}")]
    ValidTypeError(#[from] ValidTypeError),
    #[error("EntityPathError")]
    EntityPathError(#[from] EntityPathError),
}

impl From<&EntityPathError> for TermError {
    fn from(_value: &EntityPathError) -> Self {
        todo!()
    }
}

pub type TermResult<T> = Result<T, TermError>;
pub type TermResultRef<'a, T> = Result<T, &'a TermError>;
pub type TermResultArc<T> = Result<Arc<T>, TermError>;
