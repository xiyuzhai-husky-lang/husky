use husky_ty::*;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq)]
pub enum ExprTypeError {
    Original(OriginalExprTypeError),
    Derived(DerivedExprTypeError),
}

impl From<DerivedExprTypeError> for ExprTypeError {
    fn from(v: DerivedExprTypeError) -> Self {
        Self::Derived(v)
    }
}

impl From<OriginalExprTypeError> for ExprTypeError {
    fn from(v: OriginalExprTypeError) -> Self {
        Self::Original(v)
    }
}

impl From<TypeError> for ExprTypeError {
    fn from(value: TypeError) -> Self {
        match value {
            TypeError::Original(e) => ExprTypeError::Original(e.into()),
            TypeError::Derived(e) => ExprTypeError::Derived(e.into()),
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalExprTypeError {
    #[error("type error {0}")]
    TypeError(#[from] OriginalTypeError),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedExprTypeError {
    #[error("todo")]
    TypeInfoError,
    #[error("todo")]
    ExprError,
    #[error("todo")]
    EntityTypeError,
    #[error("todo")]
    BoxListApplicationFirstArgumentError,
    #[error("todo")]
    ApplicationArgumentTypeNotInferred,
    #[error("todo")]
    PrefixOperandTypeNotInferred,
    #[error("todo")]
    BinaryOpnFirstArgumentTypeNotInferred,
    #[error("todo")]
    BinaryOpnSecondArgumentTypeNotInferred,
    #[error("todo")]
    BlockTypeError,
    #[error("todo")]
    TermSymbolTypeError,
    #[error("type error {0}")]
    TypeError(#[from] DerivedTypeError),
}

pub type ExprTypeResult<T> = Result<T, ExprTypeError>;
pub type ExprTypeResultRef<'a, T> = Result<T, &'a ExprTypeError>;
