use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VMError {
    TypeMismatch(String),
    CannotOwn,
    ValueUndefined,
    AssertionFailure,
    CannotPop,
    Message(String),
}

pub type VMResult<T> = Result<T, VMError>;

impl Into<Cow<'static, str>> for VMError {
    fn into(self) -> Cow<'static, str> {
        match self {
            VMError::TypeMismatch(msg) => format!("type mismatch: {}", msg).into(),
            VMError::CannotOwn => "cannot own".into(),
            VMError::ValueUndefined => "value undefined".into(),
            VMError::AssertionFailure => "assertion failure".into(),
            VMError::CannotPop => todo!(),
            VMError::Message(_) => todo!(),
        }
    }
}

macro_rules! error {
    ($message: expr) => {
        VMError::Message($message.into())
    };
}
pub(crate) use error;
