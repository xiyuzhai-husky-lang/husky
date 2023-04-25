use crate::*;
use husky_ethereal_signature::EtherealSignatureError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub enum FluffyTermError {
    #[error("todo")]
    Todo,
}

impl From<EtherealSignatureError> for FluffyTermError {
    fn from(value: EtherealSignatureError) -> Self {
        FluffyTermError::Todo
    }
}

pub type FluffyTermResult<T> = Result<T, FluffyTermError>;
pub type FluffyTermMaybeResult<T> = MaybeResult<T, FluffyTermError>;
