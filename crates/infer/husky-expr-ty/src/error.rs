mod pattern;
mod term;

pub use self::pattern::*;
pub use self::term::*;

use crate::*;
use husky_entity_path::EntityPathError;
use husky_expr::ExprIdx;
use husky_term::TermError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ExprTypeError {
    #[error("original {0}")]
    Original(#[from] OriginalExprTypeError),
    #[error("original {0}")]
    Derived(#[from] DerivedExprTypeError),
}

impl From<TermError> for ExprTypeError {
    fn from(e: TermError) -> Self {
        ExprTypeError::Derived(e.into())
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalExprTypeError {
    #[error("unresolved term")]
    UnresolvedTerm,
    #[error("type method type error")]
    TypeMethodTypeError,
    #[error("type call type error")]
    TypeCallTypeError,
    #[error("TodoScopeResolution")]
    TodoScopeResolution,
    #[error("TodoSuffix")]
    TodoSuffix,
    #[error("TodoBoxColon")]
    TodoBoxColon,
    #[error("final destination")]
    FinalDestination,
    #[error("form path type error")]
    FormPathTypeError,
    #[error("ambiguous type path")]
    AmbiguousTypePath,
    #[error("ritchie call wrong number of arguments")]
    RitchieCallWrongNumberOfArguments {
        number_of_nonself_parameters: u8,
        number_of_nonself_arguments: u8,
    },
    #[error("ambiguous list expr")]
    AmbiguousListExpr,
    #[error("AmbiguousTildeExpr")]
    AmbiguousTildeExpr,
    #[error("no such field")]
    NoSuchField,
    #[error("no such method")]
    NoSuchMethod,
    #[error("TodoIndexOrComposeWithList")]
    TodoIndexOrComposeWithList,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedExprTypeError {
    #[error("field type error {0}")]
    FieldTypeTermError(TermError),
    #[error("type method type error {0}")]
    TypeMethodTypeError(TermError),
    #[error("type call type error {0}")]
    TypeCallTypeError(TermError),
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
    TypeError(#[from] TermError),
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
    #[error("as operation right operand term not inferred")]
    AsOperationRightOperandTermNotInferred,
    #[error("return type not given in ritchie call")]
    ReturnTypeNotGivenInRitchieCall,
    #[error("self type not inferred for self value")]
    SelfTypeNotInferredForSelfValue,
    #[error("unresolved local term")]
    UnresolvedLocalTerm,
    #[error("entity path error")]
    EntityPathError,
    #[error("final destination")]
    FinalDestination,
    #[error("cannot disambiguate list expression")]
    AmbiguateListExpr,
    #[error("form path type error {0}")]
    FormPathTypeError(TermError),
    #[error("ambiguous type path")]
    AmbiguousTypePath,
    #[error("explicit application function type not inferred")]
    ExplicitApplicationFunctionTypeNotInferred,
    #[error("AmbiguousTildeExpr")]
    AmbiguousTildeExpr,
    #[error("IntegerLiteralExpectationTypePathRefine")]
    IntegerLiteralExpectationTypePathRefine(EntityPathError),
    #[error("BitNotOperandTypeNotInferred")]
    BitNotOperandTypeNotInferred,
}

pub type ExprTypeResult<T> = Result<T, ExprTypeError>;
pub type ExprTypeResultRef<'a, T> = Result<T, &'a ExprTypeError>;
