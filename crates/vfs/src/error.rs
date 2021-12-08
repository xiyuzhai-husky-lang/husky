use std::{
    error::Error,
    fmt::{Debug, Display},
};

pub enum VfsError {
    NoSuchPackage,
}
impl Display for VfsError {
    fn fmt(&self, f: &mut common::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSuchPackage => write!(f, "NoSuchPackage"),
        }
    }
}
impl Debug for VfsError {
    fn fmt(&self, f: &mut common::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSuchPackage => write!(f, "NoSuchPackage"),
        }
    }
}
impl Error for VfsError {}
