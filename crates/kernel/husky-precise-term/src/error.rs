use husky_entity_path::{EntityPath, EntityPathError};
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
    // #[error("data store disconnected")]
    // Disconnect(#[from] io::Error),
    // #[error("the data for key `{0}` is not available")]
    // Redaction(String),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader { expected: String, found: String },
    // #[error("unknown data store error")]
    // Unknown,
}

impl From<&EntityPathError> for PreciseTermError {
    fn from(_value: &EntityPathError) -> Self {
        todo!()
    }
}

pub type PreciseTermResult<T> = Result<T, PreciseTermError>;
pub type PreciseTermResultArc<T> = Result<Arc<T>, PreciseTermError>;
