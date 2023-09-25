use super::*;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PatternExprTypeError {
    #[error("original {0}")]
    Original(#[from] OriginalPatternExprTypeError),
    #[error("derived {0}")]
    Derived(#[from] DerivedPatternExprTypeError),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalPatternExprTypeError {}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedPatternExprTypeError {
    #[error("pattern expr type error")]
    PatternExprTypeError,
}

pub type PatternExprTypeResult<T> = Result<T, PatternExprTypeError>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PatternSymbolTypeError {
    #[error("original {0}")]
    Original(#[from] OriginalPatternSymbolTypeError),
    #[error("derived {0}")]
    Derived(#[from] DerivedPatternSymbolTypeError),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalPatternSymbolTypeError {}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedPatternSymbolTypeError {
    #[error("pattern expr type error")]
    PatternExprTypeError,
}

pub type PatternSymbolTypeResult<T> = Result<T, PatternSymbolTypeError>;
