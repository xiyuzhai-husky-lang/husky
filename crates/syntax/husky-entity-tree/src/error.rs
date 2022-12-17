use husky_vfs::VfsError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum EntityTreeError {
    #[error("todo")]
    TODO,
    #[error("derived {0}")]
    DerivedSelf(Box<Self>),
    #[error("expect identifier after keyword")]
    ExpectIdentifierAfterKeyword,
    #[error("derived {0}")]
    DerivedVfs(#[from] VfsError),
}

impl From<&EntityTreeError> for EntityTreeError {
    fn from(value: &EntityTreeError) -> Self {
        EntityTreeError::DerivedSelf(Box::new(value.clone()))
    }
}

impl From<&VfsError> for EntityTreeError {
    fn from(e: &VfsError) -> Self {
        EntityTreeError::DerivedVfs(e.clone())
    }
}

pub type EntityTreeResult<T> = Result<T, EntityTreeError>;
