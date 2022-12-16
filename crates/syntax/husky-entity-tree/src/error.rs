use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum EntityTreeError {
    #[error("todo")]
    TODO,
}
