#[derive(Debug, Clone)]
pub enum VMError {
    TypeMismatch,
    CannotOwn,
    ValueUndefined,
    AssertionFailure,
}

pub type VMResult<T> = Result<T, VMError>;
