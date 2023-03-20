use crate::*;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CorgiConfigError {
    #[error("{0}")]
    Original(#[from] OriginalCorgiConfigError),
    #[error("{0}")]
    Derived(#[from] DerivedCorgiConfigError),
}
pub type CorgiConfigResult<T> = Result<T, CorgiConfigError>;
pub type CorgiConfigResultRef<'a, T> = Result<T, &'a CorgiConfigError>;

impl From<&CorgiConfigError> for CorgiConfigError {
    fn from(value: &CorgiConfigError) -> Self {
        todo!()
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalCorgiConfigError {}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedCorgiConfigError {}

impl From<VfsError> for CorgiConfigError {
    fn from(value: VfsError) -> Self {
        todo!()
    }
}

impl From<&VfsError> for CorgiConfigError {
    fn from(value: &VfsError) -> Self {
        todo!()
    }
}

impl From<&CorgiConfigAstError> for CorgiConfigError {
    fn from(value: &CorgiConfigAstError) -> Self {
        todo!()
    }
}
