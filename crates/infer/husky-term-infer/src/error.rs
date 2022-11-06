use crate::*;
use husky_symbol_syntax::Symbol;
use husky_term::TermError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InferError {
    #[error("ident unrecognized")]
    IdentUnrecognized,
    #[error("term error")]
    Term(#[from] TermError),
    #[error("derived")]
    Derived(#[from] DerivedInferError),
}

#[derive(Error, Debug)]
pub enum DerivedInferError {
    #[error("todo")]
    InferTermUnrecogizedSymbol { symbol: Symbol },
}

pub type InferResult<T> = Result<T, InferError>;
