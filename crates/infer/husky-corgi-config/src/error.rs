use crate::*;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CorgiConfigError {
    #[error("{0}")]
    Original(#[from] OriginalCorgiConfigError),
    #[error("{0}")]
    Derived(#[from] DerivedCorgiConfigError),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalCorgiConfigError {}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedCorgiConfigError {}
