mod pattern;
mod term;

pub use self::pattern::*;
pub use self::term::*;

use husky_expr::ExprIdx;
use husky_ty::*;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ExprTypeError {
    #[error("original {0}")]
    Original(#[from] OriginalExprTypeError),
    #[error("original {0}")]
    Derived(#[from] DerivedExprTypeError),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalExprTypeError {
    #[error("unresolved term")]
    UnresolvedTerm,
    #[error("field type error {0}")]
    FieldTypeError(OriginalTypeError),
    #[error("type method type error {0}")]
    TypeMethodTypeError(OriginalTypeError),
    #[error("type call type error {0}")]
    TypeCallTypeError(OriginalTypeError),
    #[error("TodoScopeResolution")]
    TodoScopeResolution,
    #[error("TodoSuffix")]
    TodoSuffix,
    #[error("TodoBoxColon")]
    TodoBoxColon,
    #[error("final destination")]
    FinalDestination,
    #[error("form path type error {0}")]
    FormPathTypeError(OriginalTypeError),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedExprTypeError {
    #[error("field type error {0}")]
    FieldTypeError(DerivedTypeError),
    #[error("type method type error {0}")]
    TypeMethodTypeError(DerivedTypeError),
    #[error("type call type error {0}")]
    TypeCallTypeError(DerivedTypeError),
    #[error("type info error")]
    TypeInfoError,
    #[error("expr error")]
    ExprError,
    #[error("entity type error")]
    EntityTypeError,
    #[error("boxList application first argument error")]
    BoxListApplicationFirstArgumentError,
    #[error("application or Ritchie call function type not inferred")]
    ApplicationOrRitchieCallFunctionTypeNotInferred,
    #[error("application argument type not inferred")]
    ApplicationArgumentTypeNotInferred,
    #[error("prefix operand type not inferred")]
    PrefixOperandTypeNotInferred,
    #[error("binary opn left operand type not inferred")]
    BinaryOperationLeftOperandTypeNotInferred,
    #[error("binary operation right operand type not inferred")]
    BinaryOperationRightOperandTypeNotInferred,
    #[error("field owner type not inferred")]
    FieldOwnerTypeNotInferred,
    #[error("method owner type not inferred")]
    MethodOwnerTypeNotInferred,
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
    #[error("function type not inferred in application or function call")]
    FunctionTypeNotInferredInApplicationOrFunctionCall,
    // MOM
    #[error("AsOperationRightOperandTermNotInferred")]
    AsOperationRightOperandTermNotInferred,
    #[error("ReturnTypeNotGivenInRitchieCall")]
    ReturnTypeNotGivenInRitchieCall,
    #[error("SelfTypeNotInferredForSelfValue")]
    SelfTypeNotInferredForSelfValue,
    #[error("UnresolvedLocalTerm")]
    UnresolvedLocalTerm,
    #[error("EntityPathError")]
    EntityPathError,
    #[error("final destination")]
    FinalDestination,
    #[error("cannot disambiguate list expression")]
    AmbiguateListExpr,
    #[error("form path type error {0}")]
    FormPathTypeError(DerivedTypeError),
}

pub type ExprTypeResult<T> = Result<T, ExprTypeError>;
pub type ExprTypeResultRef<'a, T> = Result<T, &'a ExprTypeError>;
