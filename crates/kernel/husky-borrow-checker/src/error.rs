pub struct BorrowError;

pub type BorrowResult<T> = Result<T, BorrowError>;
