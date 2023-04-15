use super::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FluffyCardError {
    #[error("{0}")]
    Original(OriginalFluffyCardError),
    #[error("{0}")]
    Derived(DerivedFluffyCardError),
}

pub type FluffyCardResult<T> = Result<T, FluffyCardError>;

impl From<TermError> for FluffyCardError {
    fn from(value: TermError) -> Self {
        todo!()
    }
}

#[derive(Debug, Error)]
pub enum OriginalFluffyCardError {}

#[derive(Debug, Error)]
pub enum DerivedFluffyCardError {}
