use crate::*;
use husky_symbol::Symbol;
use husky_term::TermError;
use husky_word::Identifier;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum TermInferError {
    #[error("ident unrecognized")]
    IdentUnrecognized,
    #[error("term error")]
    TermData(#[from] TermError),
    #[error("derived")]
    Derived(#[from] DerivedTermInferError),
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum DerivedTermInferError {
    #[error("infer term unrecognized `{ident}`")]
    InferTermUnrecogized { ident: String },
}

pub type TermInferResult<T> = Result<T, TermInferError>;
