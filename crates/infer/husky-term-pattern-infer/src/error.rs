use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TermPatternInferError {
    #[error("`{0}`")]
    Original(#[from] OriginalTermPatternInferError),
    #[error("`{0}`")]
    Derived(#[from] DerivedTermPatternInferError),
}

pub type TermPatternInferResult<T> = Result<T, TermPatternInferError>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalTermPatternInferError {}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedTermPatternInferError {}
