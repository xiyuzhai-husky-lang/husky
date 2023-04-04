use super::*;

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub enum ExprTermError {
    #[error("original expr term error: {0}")]
    Original(#[from] OriginalExprTermError),
    #[error("derived expr term error: {0}")]
    Derived(#[from] DerivedExprTermError),
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub enum OriginalExprTermError {
    #[error("todo")]
    Todo,
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub enum DerivedExprTermError {
    #[error("expr error")]
    ExprError,
    #[error("todo")]
    Todo,
    #[error("AmbiguousTypePath")]
    AmbiguousTypePath,
    #[error("PrefixOprTermNotInferred")]
    PrefixOprTermNotInferred,
    #[error("AmbiguousTilde")]
    AmbiguousTilde,
    #[error("TildeApplicationTermError")]
    TildeApplicationTerm(TermError),
    #[error("TypePathTypeError")]
    TypePathTypeError,
    #[error("OptionApplicationTermError")]
    OptionApplicationTerm(TermError),
    #[error("ExplicitApplicationTerm")]
    ExplicitApplicationTerm(TermError),
    #[error("ExplicitApplicationFunctionTermNotInferred")]
    ExplicitApplicationFunctionTermNotInferred,
    #[error("ExplicitApplicationArgumentTermNotInferred")]
    ExplicitApplicationArgumentTermNotInferred,
}

pub type ExprTermResult<T> = Result<T, ExprTermError>;
