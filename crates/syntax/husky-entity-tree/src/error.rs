use husky_entity_path::EntityPathError;
use husky_vfs::VfsError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum EntityTreeError {
    // original
    #[error("todo")]
    TODO,
    #[error("expect identifier after keyword")]
    ExpectIdentifierAfterKeyword,
    // derived
    #[error("derived {0}")]
    DerivedSelf(Box<Self>),
    #[error("derived {0}")]
    Vfs(#[from] VfsError),
    #[error("derived {0}")]
    EntityPath(#[from] EntityPathError),
}

impl From<&EntityTreeError> for EntityTreeError {
    fn from(value: &EntityTreeError) -> Self {
        EntityTreeError::DerivedSelf(Box::new(value.clone()))
    }
}

impl From<&VfsError> for EntityTreeError {
    fn from(e: &VfsError) -> Self {
        EntityTreeError::Vfs(e.clone())
    }
}

pub type EntityTreeResult<T> = Result<T, EntityTreeError>;
