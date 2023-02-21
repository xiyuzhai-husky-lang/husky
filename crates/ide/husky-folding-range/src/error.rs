use crate::*;
use husky_ast::AstError;
use husky_vfs::VfsError;
use salsa::DebugWithDb;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum FoldingRangeError {
    #[error("ast error {0}")]
    Ast(#[from] AstError),
    #[error("vfs error {0}")]
    Vfs(#[from] VfsError),
}

impl From<&VfsError> for FoldingRangeError {
    fn from(_value: &VfsError) -> Self {
        todo!()
    }
}

impl From<&FoldingRangeError> for FoldingRangeError {
    fn from(value: &FoldingRangeError) -> Self {
        todo!()
    }
}

pub type FoldingRangeResult<T> = Result<T, FoldingRangeError>;

impl<Db: FoldingRangeDb> DebugWithDb<Db> for FoldingRangeError {
    fn fmt(
        &self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &Db,
        _level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        todo!()
    }
}
