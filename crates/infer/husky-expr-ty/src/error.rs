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
    #[error("unresolved term")]
    UnresolvedTerm,
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
}

pub type ExprTypeResult<T> = Result<T, ExprTypeError>;
pub type ExprTypeResultRef<'a, T> = Result<T, &'a ExprTypeError>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PatternExprTypeError {
    #[error("original {0}")]
    Original(OriginalPatternExprTypeError),
    #[error("derived {0}")]
    Derived(DerivedPatternExprTypeError),
}

impl From<DerivedPatternExprTypeError> for PatternExprTypeError {
    fn from(v: DerivedPatternExprTypeError) -> Self {
        Self::Derived(v)
    }
}

impl From<OriginalPatternExprTypeError> for PatternExprTypeError {
    fn from(v: OriginalPatternExprTypeError) -> Self {
        Self::Original(v)
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalPatternExprTypeError {}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedPatternExprTypeError {
    #[error("pattern expr type error")]
    PatternExprTypeError,
}

pub type PatternExprTypeResult<T> = Result<T, PatternExprTypeError>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PatternSymbolTypeError {
    #[error("original {0}")]
    Original(OriginalPatternSymbolTypeError),
    #[error("derived {0}")]
    Derived(DerivedPatternSymbolTypeError),
}

impl From<DerivedPatternSymbolTypeError> for PatternSymbolTypeError {
    fn from(v: DerivedPatternSymbolTypeError) -> Self {
        Self::Derived(v)
    }
}

impl From<OriginalPatternSymbolTypeError> for PatternSymbolTypeError {
    fn from(v: OriginalPatternSymbolTypeError) -> Self {
        Self::Original(v)
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalPatternSymbolTypeError {}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedPatternSymbolTypeError {
    #[error("pattern expr type error")]
    PatternExprTypeError,
}

pub type PatternSymbolTypeResult<T> = Result<T, PatternSymbolTypeError>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ExprTermError {
    #[error("original expr term error: {0}")]
    Original(#[from] OriginalExprTermError),
    #[error("derived expr term error: {0}")]
    Derived(#[from] DerivedExprTermError),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalExprTermError {}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedExprTermError {
    #[error("expr error")]
    ExprError,
}

pub type ExprTermResult<T> = Result<T, ExprTermError>;
