use husky_entity_path::{EntityPath, EntityPathError};
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone, Copy)]
pub enum DeclarativeTermError {
    #[error("term is not reduced")]
    DeclarativeTermIsNotReduced,
    #[error("term is not type")]
    DeclarativeTermIsNotTy,
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

impl From<&EntityPathError> for DeclarativeTermError {
    fn from(_value: &EntityPathError) -> Self {
        todo!()
    }
}

pub type DeclarativeTermResult<T> = Result<T, DeclarativeTermError>;
pub type DeclarativeTermResultRef<'a, T> = Result<T, &'a DeclarativeTermError>;
