use crate::*;
use husky_declarative_term::term::DeclarativeTermSymbolTypeErrorKind;
use husky_entity_tree::EntityTreeError;
use husky_ethereal_signature::EtherealSignatureError;
use thiserror::Error;

#[salsa::debug_with_db]
#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
pub enum FluffyTermError {
    #[error("ethereal signature")]
    EtherealSignature(EtherealSignatureError),
    #[error("ethereal term")]
    EtherealTerm(EtherealTermError),
}

impl From<EtherealSignatureError> for FluffyTermError {
    fn from(e: EtherealSignatureError) -> Self {
        FluffyTermError::EtherealSignature(e)
    }
}

impl From<EtherealTermError> for FluffyTermError {
    fn from(e: EtherealTermError) -> Self {
        FluffyTermError::EtherealTerm(e)
    }
}

impl From<&EntityTreeError> for FluffyTermError {
    fn from(value: &EntityTreeError) -> Self {
        todo!()
    }
}

impl From<DeclarativeTermSymbolTypeErrorKind> for FluffyTermError {
    fn from(value: DeclarativeTermSymbolTypeErrorKind) -> Self {
        todo!()
    }
}

pub type FluffyTermResult<T> = Result<T, FluffyTermError>;
pub type FluffyTermMaybeResult<T> = MaybeResult<T, FluffyTermError>;
