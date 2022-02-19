use std::borrow::Cow;

#[derive(Debug, Clone, Copy)]
pub enum VMError {
    TypeMismatch,
    CannotOwn,
    ValueUndefined,
    AssertionFailure,
}

pub type VMResult<T> = Result<T, VMError>;

impl Into<Cow<'static, str>> for VMError {
    fn into(self) -> Cow<'static, str> {
        match self {
            VMError::TypeMismatch => "type mismatch".into(),
            VMError::CannotOwn => "cannot own".into(),
            VMError::ValueUndefined => "value undefined".into(),
            VMError::AssertionFailure => "assertion failure".into(),
        }
    }
}
