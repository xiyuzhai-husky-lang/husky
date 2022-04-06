use crate::*;

use std::sync::Arc;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FileError {
    NoSuchpack,
    FileNotFound,
    DuplicateModuleFiles,
}

impl<T> Into<Result<T, Arc<Vec<FileError>>>> for FileError {
    fn into(self) -> Result<T, Arc<Vec<FileError>>> {
        Err(Arc::new(vec![self]))
    }
}

pub type FileResultArc<T> = Result<Arc<T>, FileError>;
pub type FileResult<T> = Result<T, FileError>;
