use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum EntityTreeError {
    #[error("todo")]
    TODO,
    #[error("derived {0}")]
    Derived(Box<Self>),
    #[error("expect identifier after keyword")]
    ExpectIdentifierAfterKeyword,
}

impl From<&EntityTreeError> for EntityTreeError {
    fn from(value: &EntityTreeError) -> Self {
        EntityTreeError::Derived(Box::new(value.clone()))
    }
}

pub type EntityTreeResult<T> = Result<T, EntityTreeError>;
