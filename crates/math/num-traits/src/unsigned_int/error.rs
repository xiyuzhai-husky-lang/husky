use super::*;

#[derive(Debug, thiserror::Error, PartialEq, Eq, Clone)]
pub enum UnsignedIntError {
    #[error("overflow")]
    Overflow,
    #[error("as_pow")]
    AsPow,
}

pub type UnsignedIntResult<T> = Result<T, UnsignedIntError>;
