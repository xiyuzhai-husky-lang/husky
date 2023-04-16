#[derive(Debug, PartialEq, Eq)]
pub enum SignatureDeclarativeTermError {
    Original(OriginalSignatureDeclarativeTermError),
    Derived(DerivedSignatureDeclarativeTermError),
}

impl From<OriginalSignatureDeclarativeTermError> for SignatureDeclarativeTermError {
    fn from(v: OriginalSignatureDeclarativeTermError) -> Self {
        Self::Original(v)
    }
}

impl From<DerivedSignatureDeclarativeTermError> for SignatureDeclarativeTermError {
    fn from(v: DerivedSignatureDeclarativeTermError) -> Self {
        Self::Derived(v)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum OriginalSignatureDeclarativeTermError {
    ExpectedLiteralForArrayLength,
    InvalidSymbolForTerm,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DerivedSignatureDeclarativeTermError {
    InvalidEntityPath,
    CannotInferFunctionDeclarativeTermInApplication,
    CannotInferArgumentDeclarativeTermInApplication,
    CannotInferOperandDeclarativeTermInPrefix,
    ExprError,
    DeclarativeTermAbortion,
    CannotInferArgumentDeclarativeTermInBoxList,
    CannotInferArrayLength,
    // should have been reported as syntax error
    SelfTypeNotAllowedInThisRegion,
    // should have been reported as syntax error
    SelfValueNotAllowedInThisRegion,
}

pub type SignatureDeclarativeTermResult<T> = Result<T, SignatureDeclarativeTermError>;
pub type SignatureDeclarativeTermResultBorrowed<'a, T> =
    Result<T, &'a SignatureDeclarativeTermError>;
