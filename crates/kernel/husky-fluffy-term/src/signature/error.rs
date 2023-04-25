use super::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FluffySignatureError {
    #[error("{0}")]
    Original(OriginalFluffySignatureError),
    #[error("{0}")]
    Derived(DerivedFluffySignatureError),
}

impl From<EtherealSignatureError> for FluffySignatureError {
    fn from(value: EtherealSignatureError) -> Self {
        todo!()
    }
}

pub type FluffyCardResult<T> = Result<T, FluffySignatureError>;

impl From<EtherealTermError> for FluffySignatureError {
    fn from(value: EtherealTermError) -> Self {
        todo!()
    }
}

impl From<DeclarativeSignatureError> for FluffySignatureError {
    fn from(value: DeclarativeSignatureError) -> Self {
        todo!()
    }
}

#[derive(Debug, Error)]
pub enum OriginalFluffySignatureError {}

#[derive(Debug, Error)]
pub enum DerivedFluffySignatureError {}
