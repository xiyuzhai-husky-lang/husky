use husky_dec_term::term::DecSymbolicVariableTypeErrorKind;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum SynExprDecTermError {
    Original(OriginalSynExprDecTermError),
    Derived(DerivedSynExprDecTermError),
}

impl Into<DecSymbolicVariableTypeErrorKind> for SynExprDecTermError {
    fn into(self) -> DecSymbolicVariableTypeErrorKind {
        DecSymbolicVariableTypeErrorKind::SignatureDecTermError
    }
}

impl Into<DecSymbolicVariableTypeErrorKind> for DerivedSynExprDecTermError {
    fn into(self) -> DecSymbolicVariableTypeErrorKind {
        DecSymbolicVariableTypeErrorKind::SignatureDecTermError
    }
}

impl From<OriginalSynExprDecTermError> for SynExprDecTermError {
    fn from(v: OriginalSynExprDecTermError) -> Self {
        Self::Original(v)
    }
}

impl From<DerivedSynExprDecTermError> for SynExprDecTermError {
    fn from(v: DerivedSynExprDecTermError) -> Self {
        Self::Derived(v)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum OriginalSynExprDecTermError {
    ExpectedLiteralForArrayLength,
    InvalidSymbolForTerm,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DerivedSynExprDecTermError {
    InvalidEntityPath,
    CannotInferFunctionDecTermInApplication,
    CannotInferArgumentDecTermInApplication,
    CannotInferOperandDecTermInPrefix,
    ExprError,
    DecTermAbortion,
    CannotInferArgumentDecTermInBoxList,
    CannotInferArrayLength,
    // should have been reported as syntax error
    SelfTypeNotAllowedInThisRegion,
    // should have been reported as syntax error
    SelfValueNotAllowedInThisRegion,
    InheritedVariableIsNotValidTerm,
}

pub type SynExprDecTermResult<T> = Result<T, SynExprDecTermError>;
pub type DerivedSynExprDecTermResult<T> = Result<T, DerivedSynExprDecTermError>;
pub type SynExprDecTermResultRef<'a, T> = Result<T, &'a SynExprDecTermError>;
