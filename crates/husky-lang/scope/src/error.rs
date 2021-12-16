pub(crate) mod def;

use std::sync::Arc;

pub(crate) use def::*;
use file::FileError;
use text::TextRange;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ScopeError {
    FileError(FileError),
    DefError(def::ScopeDefError),
    NoSuchScope,
}

pub type ScopeResult<T> = Result<T, ScopeError>;
pub type ScopeResultArc<T> = Result<Arc<T>, ScopeError>;

impl From<FileError> for ScopeError {
    fn from(error: FileError) -> Self {
        ScopeError::FileError(error)
    }
}
