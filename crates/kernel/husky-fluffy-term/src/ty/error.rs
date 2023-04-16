use super::*;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum FluffyTypeError {
    #[error("{0}")]
    Original(#[from] OriginalFluffyTypeError),
    #[error("{0}")]
    Derived(#[from] DerivedFluffyTypeError),
}

// impl From<TypeError> for FluffyTypeError {
//     fn from(value: TypeError) -> Self {
//         todo!()
//     }
// }

impl From<TermError> for FluffyTypeError {
    fn from(value: TermError) -> Self {
        todo!()
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalFluffyTypeError {
    #[error("NoSuchMethod")]
    NoSuchMethod,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedFluffyTypeError {}

impl From<FluffyCardError> for FluffyTypeError {
    fn from(value: FluffyCardError) -> Self {
        todo!()
    }
}

pub type FluffyTypeResult<T> = Result<T, FluffyTypeError>;
