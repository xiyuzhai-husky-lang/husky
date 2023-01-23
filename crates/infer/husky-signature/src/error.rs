use outcome::Outcome;
use std::convert::Infallible;

#[derive(Debug, PartialEq, Eq)]
pub enum SignatureAbortion {
    ExprError,
    TermError,
}

pub type SignatureOutcome<T> = Outcome<T, Infallible, SignatureAbortion>;
pub type SignatureOutcomeBorrowed<'a, T> = Outcome<T, Infallible, &'a SignatureAbortion>;
