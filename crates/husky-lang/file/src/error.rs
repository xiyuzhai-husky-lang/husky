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
impl Error for FileError {}

pub type FileResultArc<T> = Result<Arc<T>, FileError>;
