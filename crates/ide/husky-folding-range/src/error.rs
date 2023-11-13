use crate::*;
use husky_ast::AstError;
use husky_vfs::error::VfsError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum FoldingRangeError {
    #[error("ast error {0}")]
    Ast(String),
    #[error("vfs error {0}")]
    Vfs(#[from] VfsError),
}

impl From<&AstError> for FoldingRangeError {
    fn from(value: &AstError) -> Self {
        todo!()
    }
}

impl From<&VfsError> for FoldingRangeError {
    fn from(_value: &VfsError) -> Self {
        todo!()
    }
}

impl From<&FoldingRangeError> for FoldingRangeError {
    fn from(_value: &FoldingRangeError) -> Self {
        todo!()
    }
}

pub type FoldingRangeResult<T> = Result<T, FoldingRangeError>;

impl<Db: FoldingRangeDb> salsa::DebugWithDb<Db> for FoldingRangeError {
    fn fmt(
        &self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &Db,
        _level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        todo!()
    }
}
