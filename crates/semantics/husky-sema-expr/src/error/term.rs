use super::*;

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::debug_with_db(db = SemaExprDb, jar = SemaExprJar)]
pub enum SemaExprTermError {
    #[error("original expr term error: {0}")]
    Original(#[from] OriginalExprTermError),
    #[error("derived expr term error: {0}")]
    Derived(#[from] DerivedExprTermError),
}

impl From<EtherealTermError> for SemaExprTermError {
    fn from(value: EtherealTermError) -> Self {
        todo!()
    }
}

impl From<FluffyTermError> for SemaExprTermError {
    fn from(value: FluffyTermError) -> Self {
        todo!()
    }
}

impl From<&SemaExprTypeError> for SemaExprTermError {
    fn from(value: &SemaExprTypeError) -> Self {
        todo!()
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::debug_with_db(db = SemaExprDb, jar = SemaExprJar)]
pub enum OriginalExprTermError {
    #[error("todo")]
    Todo,
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::debug_with_db(db = SemaExprDb, jar = SemaExprJar)]
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
    LiteralTypeNotInferred,
}

pub type SemaExprTermResult<T> = Result<T, SemaExprTermError>;
pub type SemaExprTermResultRef<'a, T> = Result<T, &'a SemaExprTermError>;
