use super::*;

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::debug_with_db(db = ExprTypeDb)]
pub enum ExprTermError {
    #[error("original expr term error: {0}")]
    Original(#[from] OriginalExprTermError),
    #[error("derived expr term error: {0}")]
    Derived(#[from] DerivedExprTermError),
}

impl From<EtherealTermError> for ExprTermError {
    fn from(value: EtherealTermError) -> Self {
        todo!()
    }
}

impl From<&ExprTypeError> for ExprTermError {
    fn from(value: &ExprTypeError) -> Self {
        todo!()
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::debug_with_db(db = ExprTypeDb)]
pub enum OriginalExprTermError {
    #[error("todo")]
    Todo,
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::debug_with_db(db = ExprTypeDb)]
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
    TildeApplicationTerm(EtherealTermError),
    #[error("TypePathTypeError")]
    TypePathTypeError,
    #[error("OptionApplicationTermError")]
    OptionApplicationTerm(EtherealTermError),
    #[error("ExplicitApplicationTerm")]
    ExplicitApplicationTerm(EtherealTermError),
    #[error("ExplicitApplicationFunctionTermNotInferred")]
    ExplicitApplicationFunctionTermNotInferred,
    #[error("ExplicitApplicationArgumentTermNotInferred")]
    ExplicitApplicationArgumentTermNotInferred,
    #[error("expr type error")]
    ExprTypeError,
}

pub type ExprTermResult<T> = Result<T, ExprTermError>;
pub type ExprTermResultRef<'a, T> = Result<T, &'a ExprTermError>;
