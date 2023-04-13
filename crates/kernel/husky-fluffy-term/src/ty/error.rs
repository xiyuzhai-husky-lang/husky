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

#[derive(Debug, Error)]
pub enum OriginalFluffyTypeError {}

#[derive(Debug, Error)]
pub enum DerivedFluffyTypeError {}
