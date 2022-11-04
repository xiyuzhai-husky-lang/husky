use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum BorrowError {
    #[error("borrow for invalid lifetime")]
    InvalidLifetime,
    #[error("borrow outdated variable")]
    BorrowOutdatedVariable,
    #[error("borrow moved variable")]
    BorrowMovedVariable,
}

pub type BorrowResult<T> = Result<T, BorrowError>;
