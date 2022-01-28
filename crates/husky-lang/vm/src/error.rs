#[derive(Debug, Clone)]
pub enum VMError {
    TypeMismatch,
    CannotOwn,
    ValueUndefined,
}

pub type VMResult<T> = Result<T, VMError>;
