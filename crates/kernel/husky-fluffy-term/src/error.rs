use crate::*;
use husky_entity_syn_tree::EntityTreeError;
use husky_ethereal_signature::EtherealSignatureError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = FluffyTermDb)]
pub enum FluffyTermError {
    #[error("todo")]
    Todo,
}

impl From<EtherealSignatureError> for FluffyTermError {
    fn from(value: EtherealSignatureError) -> Self {
        FluffyTermError::Todo
    }
}

impl From<EtherealTermError> for FluffyTermError {
    fn from(e: EtherealTermError) -> Self {
        FluffyTermError::Todo
    }
}

impl From<&EntityTreeError> for FluffyTermError {
    fn from(value: &EntityTreeError) -> Self {
        todo!()
    }
}

pub type FluffyTermResult<T> = Result<T, FluffyTermError>;
pub type FluffyTermMaybeResult<T> = MaybeResult<T, FluffyTermError>;
