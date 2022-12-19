use crate::*;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PackageDependencyError {}

pub type PackageDependencyResult<T> = Result<T, PackageDependencyError>;
