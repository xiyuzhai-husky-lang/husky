#[derive(Debug, PartialEq, Eq)]
pub enum SignatureTermError {
    Original(OriginalSignatureTermError),
    Derived(DerivedSignatureTermError),
}

impl From<OriginalSignatureTermError> for SignatureTermError {
    fn from(v: OriginalSignatureTermError) -> Self {
        Self::Original(v)
    }
}

impl From<DerivedSignatureTermError> for SignatureTermError {
    fn from(v: DerivedSignatureTermError) -> Self {
        Self::Derived(v)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum OriginalSignatureTermError {}

#[derive(Debug, PartialEq, Eq)]
pub enum DerivedSignatureTermError {
    InvalidEntityPath,
    CannotInferFunctionTermInApplication,
    CannotInferArgumentTermInApplication,
    CannotInferOperandTermInPrefix,
    ExprError,
    TermAbortion,
    CannotInferArgumentTermInBoxList,
}

pub type SignatureTermResult<T> = Result<T, SignatureTermError>;
pub type SignatureTermResultBorrowed<'a, T> = Result<T, &'a SignatureTermError>;
