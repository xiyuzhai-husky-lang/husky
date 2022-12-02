use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum TomlAstError {
    #[error("todo")]
    MisplacedKeyValue(usize),
}

pub type TomlAstResult<T> = Result<T, TomlAstError>;
