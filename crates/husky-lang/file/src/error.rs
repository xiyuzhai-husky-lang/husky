use crate::*;

use std::{
    error::Error,
    fmt::{Debug, Display},
    sync::Arc,
};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum FileError {
    NoSuchPackage,
    FileNotFound,
    DuplicateModuleFiles,
}

impl<T> Into<Result<T, Arc<Vec<FileError>>>> for FileError {
    fn into(self) -> Result<T, Arc<Vec<FileError>>> {
        Err(Arc::new(vec![self]))
    }
}
impl Display for FileError {
    fn fmt(&self, f: &mut common::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSuchPackage => write!(f, "NoSuchPackage"),
            Self::FileNotFound => write!(f, "FileNotFound"),
            Self::DuplicateModuleFiles => write!(f, "DuplicateModuleFiles"),
        }
    }
}
impl Debug for FileError {
    fn fmt(&self, f: &mut common::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSuchPackage => write!(f, "NoSuchPackage"),
            Self::FileNotFound => write!(f, "FileNotFound"),
            Self::DuplicateModuleFiles => write!(f, "DuplicateModuleFiles"),
        }
    }
}

pub type FileResultArc<T> = Result<Arc<T>, FileError>;
pub type FileResult<T> = Result<T, FileError>;
