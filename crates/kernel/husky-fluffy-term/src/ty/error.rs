use super::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FluffyTypeError {
    #[error("{0}")]
    Original(OriginalFluffyTypeError),
    #[error("{0}")]
    Derived(DerivedFluffyTypeError),
}

pub type FluffyTypeResult<T> = Result<T, FluffyTypeError>;

impl From<TermError> for FluffyTypeError {
    fn from(value: TermError) -> Self {
        todo!()
    }
}

#[derive(Debug, Error)]
pub enum OriginalFluffyTypeError {}

#[derive(Debug, Error)]
pub enum DerivedFluffyTypeError {}
