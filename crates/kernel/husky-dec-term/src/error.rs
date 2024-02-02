use husky_entity_path::{EntityPathError, ItemPath};

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone, Copy)]
pub enum DecTermError {
    #[error("term is not reduced")]
    DecTermIsNotReduced,
    #[error("term is not type")]
    DecTermIsNotTy,
    #[error("universe overflows")]
    UniverseOverflow,
    #[error("monad is not input")]
    MonadIsNotInput,
    #[error("no decl for item path")]
    NoDeclForEntityPath { item_path: ItemPath },
    // #[error("data store disconnected")]
    // Disconnect(#[from] io::Error),
    // #[error("the data for key `{0}` is not available")]
    // Redaction(String),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader { expected: String, found: String },
    // #[error("unknown data store error")]
    // Unknown,
}

impl From<&EntityPathError> for DecTermError {
    fn from(_value: &EntityPathError) -> Self {
        todo!()
    }
}

pub type DecTermResult<T> = Result<T, DecTermError>;
pub type DecTermResultRef<'a, T> = Result<T, &'a DecTermError>;
