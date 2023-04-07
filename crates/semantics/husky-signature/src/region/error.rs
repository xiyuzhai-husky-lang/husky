#[derive(Debug, PartialEq, Eq)]
pub enum SignatureRawTermError {
    Original(OriginalSignatureRawTermError),
    Derived(DerivedSignatureRawTermError),
}

impl From<OriginalSignatureRawTermError> for SignatureRawTermError {
    fn from(v: OriginalSignatureRawTermError) -> Self {
        Self::Original(v)
    }
}

impl From<DerivedSignatureRawTermError> for SignatureRawTermError {
    fn from(v: DerivedSignatureRawTermError) -> Self {
        Self::Derived(v)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum OriginalSignatureRawTermError {
    ExpectedLiteralForArrayLength,
    InvalidSymbolForTerm,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DerivedSignatureRawTermError {
    InvalidEntityPath,
    CannotInferFunctionRawTermInApplication,
    CannotInferArgumentRawTermInApplication,
    CannotInferOperandRawTermInPrefix,
    ExprError,
    RawTermAbortion,
    CannotInferArgumentRawTermInBoxList,
    CannotInferArrayLength,
    // should have been reported as syntax error
    SelfTypeNotAllowedInThisRegion,
    // should have been reported as syntax error
    SelfValueNotAllowedInThisRegion,
}

pub type SignatureRawTermResult<T> = Result<T, SignatureRawTermError>;
pub type SignatureRawTermResultBorrowed<'a, T> = Result<T, &'a SignatureRawTermError>;
