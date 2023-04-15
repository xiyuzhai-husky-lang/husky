use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TypeError {
    #[error("{0}")]
    Original(#[from] OriginalTypeError),
    #[error("{0}")]
    Derived(#[from] DerivedTypeError),
}

pub type TypeResult<T> = Result<T, TypeError>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalTypeError {
    #[error("NoSuchMethod")]
    NoSuchMethod,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedTypeError {}
