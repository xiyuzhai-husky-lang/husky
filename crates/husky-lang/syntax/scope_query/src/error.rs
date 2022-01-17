pub(crate) mod def;

use std::sync::Arc;

pub(crate) use def::*;
use file::FileError;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ScopeError {
    FileError(FileError),
    DefError(def::ScopeDefError),
    Message(String),
}

pub type ScopeResult<T> = Result<T, ScopeError>;
pub type ScopeResultArc<T> = Result<Arc<T>, ScopeError>;

impl From<FileError> for ScopeError {
    fn from(error: FileError) -> Self {
        ScopeError::FileError(error)
    }
}

macro_rules! scope_error {
    ($msg: expr) => {{
        crate::ScopeError::Message($msg.into())
    }};
}
pub(crate) use scope_error;

macro_rules! scope_err {
    ($msg: expr) => {{
        Err(crate::ScopeError::Message($msg))
    }};
}
pub(crate) use scope_err;
