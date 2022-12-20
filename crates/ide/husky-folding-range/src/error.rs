use crate::*;
use husky_vfs::VfsError;
use salsa::DebugWithDb;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum FoldingRangeError {}

impl From<&VfsError> for FoldingRangeError {
    fn from(value: &VfsError) -> Self {
        todo!()
    }
}

pub type FoldingRangeResult<T> = Result<T, FoldingRangeError>;

impl<Db: FoldingRangeDb> DebugWithDb<Db> for FoldingRangeError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        todo!()
    }
}
