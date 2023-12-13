use crate::*;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[salsa::debug_with_db(db = TomlAstDb, jar = TomlAstJar)]
pub enum TomlAstError {
    #[error("todo")]
    MisplacedKeyValue(usize),
    #[error("todo")]
    Expect,
}

pub type TomlAstResult<T> = Result<T, TomlAstError>;
