use super::*;

#[salsa::debug_with_db]
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SemaExprTermError {
    #[error("original expr term error: {0}")]
    Original(#[from] OriginalSemaExprTermError),
    #[error("derived expr term error: {0}")]
    Derived(#[from] DerivedSemaExprTermError),
}

impl From<EthTermError> for SemaExprTermError {
    fn from(value: EthTermError) -> Self {
        todo!()
    }
}

impl From<FlyTermError> for SemaExprTermError {
    fn from(value: FlyTermError) -> Self {
        todo!()
    }
}

impl From<&SemaExprDataError> for SemaExprTermError {
    fn from(value: &SemaExprDataError) -> Self {
        DerivedSemaExprTermError::ExprError.into()
    }
}

#[salsa::debug_with_db]
#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalSemaExprTermError {
    #[error("todo")]
    Todo,
}

#[salsa::debug_with_db]
#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedSemaExprTermError {
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
    TildeApplicationTerm(EthTermError),
    #[error("type path type error")]
    TypePathTypeError,
    #[error("option application term error")]
    OptionApplicationTerm(EthTermError),
    #[error("explicit application term")]
    ExplicitApplicationTerm(EthTermError),
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
    #[error("SelfTypeTermNotInferred")]
    SelfTypeTermNotInferred,
}

pub type SemaExprTermResult<T> = Result<T, SemaExprTermError>;
pub type SemaExprTermResultRef<'a, T> = Result<T, &'a SemaExprTermError>;
