use husky_vfs::VfsError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum EntityPathError {
    // #[error("{0}")]
    // AbsolutePath(#[from] AbsolutePathError),
    #[error("{0}")]
    Vfs(#[from] VfsError),
}

pub type EntityPathResult<T> = Result<T, EntityPathError>;

impl From<&EntityPathError> for EntityPathError {
    fn from(value: &EntityPathError) -> Self {
        todo!()
    }
}
