use crate::*;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = TomlAstDb)]
pub enum TomlAstError {
    #[error("todo")]
    MisplacedKeyValue(usize),
    #[error("todo")]
    Expect,
}

pub type TomlAstResult<T> = Result<T, TomlAstError>;
