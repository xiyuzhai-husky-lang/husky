use outcome::Outcome;

#[derive(Debug, PartialEq, Eq)]
pub enum SignatureTermError {}

#[derive(Debug, PartialEq, Eq)]
pub enum SignatureTermAbortion {
    InvalidEntityPath,
    CannotInferFunctionTermInApplication,
    CannotInferArgumentTermInApplication,
}

pub type SignatureTermResult<T> = Result<T, SignatureTermError>;
pub type SignatureTermOutcome<T> = Outcome<T, SignatureTermError, SignatureTermAbortion>;
pub type SignatureTermOutcomeBorrowed<'a, T> =
    Outcome<T, &'a SignatureTermError, &'a SignatureTermAbortion>;
