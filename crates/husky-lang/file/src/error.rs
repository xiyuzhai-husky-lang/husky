use std::{
    error::Error,
    fmt::{Debug, Display},
};

pub enum sourceError {
    NoSuchPackage,
}
impl Display for sourceError {
    fn fmt(&self, f: &mut common::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSuchPackage => write!(f, "NoSuchPackage"),
        }
    }
}
impl Debug for sourceError {
    fn fmt(&self, f: &mut common::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSuchPackage => write!(f, "NoSuchPackage"),
        }
    }
}
impl Error for sourceError {}
