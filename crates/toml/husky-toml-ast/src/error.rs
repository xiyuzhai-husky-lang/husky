use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db]
pub enum TomlAstError {
    #[error("todo")]
    MisplacedKeyValue(usize),
    #[error("todo")]
    Expect,
}

pub type TomlAstResult<T> = Result<T, TomlAstError>;
