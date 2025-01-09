use super::*;

#[derive(Debug, thiserror::Error, PartialEq, Eq, Clone)]
pub enum IntError {
    #[error("overflow")]
    Overflow,
    #[error("as_pow")]
    AsPow,
    #[error("into_usize")]
    IntoUsize,
    #[error("from_usize")]
    FromUsize,
}

pub type IntResult<T> = Result<T, IntError>;
