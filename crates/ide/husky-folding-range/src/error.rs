use husky_vfs::VfsError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum FoldingRangeError {}

impl From<&VfsError> for FoldingRangeError {
    fn from(value: &VfsError) -> Self {
        todo!()
    }
}

pub type FoldingRangeResult<T> = Result<T, FoldingRangeError>;
