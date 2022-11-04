use thiserror::Error;

#[derive(Error, Debug)]
pub enum BorrowError {
    #[error("borrow for invalid lifetime")]
    BorrowForInvalidLifetime,
}

pub type BorrowResult<T> = Result<T, BorrowError>;
