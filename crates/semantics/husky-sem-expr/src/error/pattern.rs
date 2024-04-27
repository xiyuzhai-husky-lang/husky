use super::*;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PatternSemExprError {
    #[error("original {0}")]
    Original(#[from] OriginalPatternSemExprError),
    #[error("derived {0}")]
    Derived(#[from] DerivedPatternSemExprError),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalPatternSemExprError {}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedPatternSemExprError {
    #[error("pattern expr type error")]
    PatternSemExprError,
}

pub type PatternSemExprResult<T> = Result<T, PatternSemExprError>;

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
    PatternSemExprError,
}

pub type PatternSymbolTypeResult<T> = Result<T, PatternSymbolTypeError>;
