use super::*;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PatternSemaExprError {
    #[error("original {0}")]
    Original(#[from] OriginalPatternSemaExprError),
    #[error("derived {0}")]
    Derived(#[from] DerivedPatternSemaExprError),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalPatternSemaExprError {}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedPatternSemaExprError {
    #[error("pattern expr type error")]
    PatternSemaExprError,
}

pub type PatternSemaExprResult<T> = Result<T, PatternSemaExprError>;

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
    PatternSemaExprError,
}

pub type PatternSymbolTypeResult<T> = Result<T, PatternSymbolTypeError>;
