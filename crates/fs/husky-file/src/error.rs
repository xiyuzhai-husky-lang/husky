use husky_dev_utils::DevSource;

use std::sync::Arc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FileError {
    pub kind: FileErrorKind,
    pub dev_src: DevSource,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FileErrorKind {
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
