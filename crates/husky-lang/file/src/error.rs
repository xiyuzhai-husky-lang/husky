use std::{
    error::Error,
    fmt::{Debug, Display},
};

pub enum FileError {
    NoSuchPackage,
}
impl Display for FileError {
    fn fmt(&self, f: &mut common::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSuchPackage => write!(f, "NoSuchPackage"),
        }
    }
}
impl Debug for FileError {
    fn fmt(&self, f: &mut common::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSuchPackage => write!(f, "NoSuchPackage"),
        }
    }
}
impl Error for FileError {}
