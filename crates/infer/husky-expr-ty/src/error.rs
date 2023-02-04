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
    #[error("type info error")]
    TypeInfoError,
    #[error("expr error")]
    ExprError,
    #[error("entity type error")]
    EntityTypeError,
    #[error("boxList application first argument error")]
    BoxListApplicationFirstArgumentError,
    #[error("application argument type not inferred")]
    ApplicationArgumentTypeNotInferred,
    #[error("prefix operand type not inferred")]
    PrefixOperandTypeNotInferred,
    #[error("binary opn first argument type not inferred")]
    BinaryOpnFirstArgumentTypeNotInferred,
    #[error("binary opn second argument type not inferred")]
    BinaryOpnSecondArgumentTypeNotInferred,
    #[error("block type error")]
    BlockTypeError,
    #[error("term symbol type error")]
    TermSymbolTypeError,
    #[error("type error {0}")]
    TypeError(#[from] DerivedTypeError),
    #[error("bracketed item type error")]
    BracketedItemTypeError,
    #[error("current symbol type error")]
    CurrentSymbolTypeError,
    #[error("inherited symbol type error")]
    InheritedSymbolTypeError,
    #[error("callable type error")]
    CallableTypeError,
    #[error("unresolved local term")]
    UnresolvedLocalTerm,
}

pub type ExprTypeResult<T> = Result<T, ExprTypeError>;
pub type ExprTypeResultRef<'a, T> = Result<T, &'a ExprTypeError>;
