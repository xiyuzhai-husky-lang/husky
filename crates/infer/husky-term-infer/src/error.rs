use crate::*;
use husky_symbol_syntax::Symbol;
use husky_term::TermError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum TermInferError {
    #[error("ident unrecognized")]
    IdentUnrecognized,
    #[error("term error")]
    Term(#[from] TermError),
    #[error("derived")]
    Derived(#[from] DerivedTermInferError),
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum DerivedTermInferError {
    #[error("todo")]
    InferTermUnrecogizedSymbol { symbol: Symbol },
}

pub type TermInferResult<T> = Result<T, TermInferError>;
