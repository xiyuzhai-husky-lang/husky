use thiserror::Error;

pub type TypeResult<T> = Result<T, TypeError>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TypeError {}
