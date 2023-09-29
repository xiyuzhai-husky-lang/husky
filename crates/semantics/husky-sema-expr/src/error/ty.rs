use super::*;

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::debug_with_db(db = SemaExprDb)]
pub enum SemaExprTypeError {
    #[error("original {0}")]
    Original(#[from] OriginalSemaExprTypeError),
    #[error("derived {0}")]
    Derived(#[from] DerivedSemaExprTypeError),
}

impl From<EtherealSignatureError> for SemaExprTypeError {
    fn from(e: EtherealSignatureError) -> Self {
        SemaExprTypeError::Derived(e.into())
    }
}

impl From<EtherealTermError> for SemaExprTypeError {
    fn from(e: EtherealTermError) -> Self {
        SemaExprTypeError::Derived(e.into())
    }
}

impl From<FluffyTermError> for SemaExprTypeError {
    fn from(e: FluffyTermError) -> Self {
        SemaExprTypeError::Derived(e.into())
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::debug_with_db(db = SemaExprDb)]
pub enum OriginalSemaExprTypeError {
    #[error("unresolved term")]
    UnresolvedTerm,
    #[error("type method type error")]
    TypeMethodTypeError,
    #[error("type call type error")]
    TypeCallTypeError,
    #[error("TodoScopeResolution")]
    TodoScopeResolution,
    #[error("TodoBoxColon")]
    TodoBoxColon,
    #[error("final destination")]
    FinalDestination,
    #[error("form path type error")]
    FugitivePathTypeError,
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
    #[error("ExpectedCurryButGotRitchieInstead")]
    ExpectedCurryButGotRitchieInstead,
    #[error("cannot unveil")]
    CannotUnveil,
    #[error("cannot unwrap")]
    CannotUnwrap,
}

impl OriginalError for OriginalSemaExprTypeError {
    type Error = SemaExprTypeError;
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::debug_with_db(db = SemaExprDb)]
pub enum DerivedSemaExprTypeError {
    #[error("field type error {0}")]
    FieldTypeTermError(EtherealTermError),
    #[error("type method type error {0}")]
    TypeMethodTypeError(EtherealTermError),
    #[error("type call type error {0}")]
    TypeCallTypeError(EtherealTermError),
    #[error("type info error")]
    TypeInfoError,
    #[error("expr error")]
    SynExprError,
    #[error("item type error")]
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
    TypeError(#[from] EtherealTermError),
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
    #[error("item path error")]
    EntityPathError,
    #[error("final destination")]
    FinalDestination,
    #[error("cannot disambiguate list expression")]
    AmbiguateListExpr,
    #[error("form path type error {0}")]
    FugitivePathTypeError(EtherealTermError),
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
    #[error("BinaryShiftRightOperandTypeNotInferred")]
    BinaryShiftRightOperandTypeNotInferred,
    #[error("Fluffy term error")]
    FluffyTermError(#[from] FluffyTermError),
    #[error("SuffixOperandTypeNotInferred")]
    SuffixOperandTypeNotInferred,
    #[error("UnableToInferSuffixOperandType")]
    UnableToInferSuffixOperandType,
    #[error("UnableToInferFunctionApplicationArgumentType")]
    UnableToInferFunctionApplicationArgumentType,
    #[error("UnableToInferAssociatedItemParentTerm")]
    UnableToInferAssociatedItemParentTerm,
    #[error("UnableToInferArgumentTermForDependentType")]
    UnableToInferArgumentTermForDependentType,
    #[error("UnableToInferReturnTypeForUnveiling")]
    UnableToInferReturnTypeForUnveiling,
    #[error("EtherealSignature")]
    EtherealSignature(#[from] EtherealSignatureError),
    #[error("UnveilerError")]
    UnveilerError,
    #[error("UnableToInferUnwrapOperand")]
    UnableToInferUnwrapOperand,
    #[error("EvalExprTypeNotInferred")]
    EvalExprTypeNotInferred,
    #[error("BranchTypeMerge")]
    BranchTypeMerge,
}

pub type SemaExprTypeResult<T> = Result<T, SemaExprTypeError>;
pub type SemaExprTypeResultRef<'a, T> = Result<T, &'a SemaExprTypeError>;
