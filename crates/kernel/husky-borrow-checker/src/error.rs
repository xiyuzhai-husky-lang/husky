use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum BorrowError {
    #[error("borrow for invalid lifetime")]
    InvalidLifetime,
}

pub type BorrowResult<T> = Result<T, BorrowError>;
