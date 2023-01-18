use outcome::Outcome;

#[derive(Debug, PartialEq, Eq)]
pub enum SignatureTermError {}

#[derive(Debug, PartialEq, Eq)]
pub enum SignatureTermAbortion {}

pub type SignatureTermResult<T> = Result<T, SignatureTermError>;
pub type SignatureTermOutcome<T> = Outcome<T, SignatureTermError, SignatureTermAbortion>;
