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
pub enum OriginalSignatureRawTermError {}

#[derive(Debug, PartialEq, Eq)]
pub enum DerivedSignatureRawTermError {
    InvalidEntityPath,
    CannotInferFunctionRawTermInApplication,
    CannotInferArgumentRawTermInApplication,
    CannotInferOperandRawTermInPrefix,
    ExprError,
    RawTermAbortion,
    CannotInferArgumentRawTermInBoxList,
}

pub type SignatureRawTermResult<T> = Result<T, SignatureRawTermError>;
pub type SignatureRawTermResultBorrowed<'a, T> = Result<T, &'a SignatureRawTermError>;
