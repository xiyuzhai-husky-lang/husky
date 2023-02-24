use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum DefnError {
    #[error("{0}")]
    Original(#[from] OriginalDefnError),
    #[error("{0}")]
    Derived(#[from] DerivedDefnError),
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum OriginalDefnError {
    #[error("expect body")]
    ExpectBody,
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum DerivedDefnError {}

pub type DefnResult<T> = Result<T, DefnError>;
