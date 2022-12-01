use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum TomlAstError {}

pub type TomlAstResult<T> = Result<T, TomlAstError>;
