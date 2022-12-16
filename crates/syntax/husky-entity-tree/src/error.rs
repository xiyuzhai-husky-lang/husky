use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum EntityTreeError {
    #[error("todo")]
    TODO,
    #[error("Derived")]
    Derived(Box<Self>),
}

impl From<&EntityTreeError> for EntityTreeError {
    fn from(value: &EntityTreeError) -> Self {
        EntityTreeError::Derived(Box::new(value.clone()))
    }
}

pub type EntityTreeResult<T> = Result<T, EntityTreeError>;
