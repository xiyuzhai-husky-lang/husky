use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum TomlAstError {
    #[error("todo")]
    MisplacedKeyValue(usize),
    #[error("todo")]
    Expect,
}

pub type TomlAstResult<T> = Result<T, TomlAstError>;
