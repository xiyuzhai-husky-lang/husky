use husky_term::TermError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InferError {
    #[error("ident unrecognized")]
    IdentUnrecognized,
    #[error("term error")]
    Term(#[from] TermError),
    #[error("derived")]
    Derived,
}

pub type InferResult<T> = Result<T, InferError>;
