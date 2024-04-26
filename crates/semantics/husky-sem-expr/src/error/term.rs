use super::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SemExprTermError {
    #[error("original expr term error: {0}")]
    Original(#[from] OriginalSemExprTermError),
    #[error("derived expr term error: {0}")]
    Derived(#[from] DerivedSemExprTermError),
}

impl From<EthTermError> for SemExprTermError {
    fn from(value: EthTermError) -> Self {
        todo!()
    }
}

impl From<FlyTermError> for SemExprTermError {
    fn from(value: FlyTermError) -> Self {
        todo!()
    }
}

impl From<&SemExprDataError> for SemExprTermError {
    fn from(value: &SemExprDataError) -> Self {
        DerivedSemExprTermError::ExprError.into()
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalSemExprTermError {
    #[error("todo")]
    Todo,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedSemExprTermError {
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
    SemExprError,
    #[error("literal type not resolved")]
    LiteralTypeNotResolved,
    #[error("TypeInfoNotInferred")]
    LiteralTypeNotInferred,
    #[error("SelfTypeTermNotInferred")]
    SelfTypeTermNotInferred,
}

pub type SemExprTermResult<T> = Result<T, SemExprTermError>;
pub type SemExprTermResultRef<'a, T> = Result<T, &'a SemExprTermError>;
