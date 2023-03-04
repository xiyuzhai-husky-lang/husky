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
    // #[error("data store disconnected")]
    // Disconnect(#[from] io::Error),
    // #[error("the data for key `{0}` is not available")]
    // Redaction(String),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader { expected: String, found: String },
    // #[error("unknown data store error")]
    // Unknown,
}

impl From<&EntityPathError> for ValidTermError {
    fn from(_value: &EntityPathError) -> Self {
        todo!()
    }
}

pub type ValidTermResult<T> = Result<T, ValidTermError>;
pub type ValidTermResultArc<T> = Result<Arc<T>, ValidTermError>;
