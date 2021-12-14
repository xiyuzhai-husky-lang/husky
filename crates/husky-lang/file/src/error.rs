use std::{
    error::Error,
    fmt::{Debug, Display},
    sync::Arc,
};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum FileError {
    NoSuchPackage,
    FileNotFound,
}
impl Display for FileError {
    fn fmt(&self, f: &mut common::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSuchPackage => write!(f, "NoSuchPackage"),
            Self::FileNotFound => write!(f, "FileNotFound"),
        }
    }
}
impl Debug for FileError {
    fn fmt(&self, f: &mut common::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSuchPackage => write!(f, "NoSuchPackage"),
            Self::FileNotFound => write!(f, "FileNotFound"),
        }
    }
}
impl Error for FileError {}

pub type FileResultArc<T> = Result<Arc<T>, FileError>;
