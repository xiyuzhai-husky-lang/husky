use crate::*;
use husky_entity_syn_tree::EntitySynTreeError;
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

impl From<&EntitySynTreeError> for FluffyTermError {
    fn from(value: &EntitySynTreeError) -> Self {
        todo!()
    }
}

pub type FluffyTermResult<T> = Result<T, FluffyTermError>;
pub type FluffyTermMaybeResult<T> = MaybeResult<T, FluffyTermError>;
