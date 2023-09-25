use super::*;

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::debug_with_db(db = SemaExprDb)]
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

impl From<&SemaExprError> for ExprTermError {
    fn from(value: &SemaExprError) -> Self {
        todo!()
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::debug_with_db(db = SemaExprDb)]
pub enum OriginalExprTermError {
    #[error("todo")]
    Todo,
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::debug_with_db(db = SemaExprDb)]
pub enum DerivedExprTermError {
    #[error("expr error")]
    ExprError,
    #[error("todo")]
    Todo,
    #[error("ambiguous type path")]
    AmbiguousTypePath,
    #[error("prefix opr term not inferred")]
    PrefixOprTermNotInferred,
    #[error("ambiguous tilde")]
    AmbiguousTilde,
    #[error("tilde application term error")]
    TildeApplicationTerm(EtherealTermError),
    #[error("type path type error")]
    TypePathTypeError,
    #[error("option application term error")]
    OptionApplicationTerm(EtherealTermError),
    #[error("explicit application term")]
    ExplicitApplicationTerm(EtherealTermError),
    #[error("explicit application function term not inferred")]
    ExplicitApplicationFunctionTermNotInferred,
    #[error("explicit application argument term not inferred")]
    ExplicitApplicationArgumentTermNotInferred,
    #[error("expr type error")]
    SemaExprError,
    #[error("literal type not resolved")]
    LiteralTypeNotResolved,
    #[error("TypeInfoNotInferred")]
    TypeInfoNotInferred,
}

pub type ExprTermResult<T> = Result<T, ExprTermError>;
pub type ExprTermResultRef<'a, T> = Result<T, &'a ExprTermError>;
