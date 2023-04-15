use super::*;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum FluffyMethodTypeError {}

pub type FluffyMethodTypeResult<T> = Result<T, FluffyMethodTypeError>;
