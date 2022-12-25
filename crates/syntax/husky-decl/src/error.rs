use husky_entity_tree::EntityTreeError;
use husky_token::{TokenError, TokenIdx};
use husky_vfs::VfsError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DeclError {
    #[error("token error")]
    Token(#[from] TokenError),
    #[error("derived {0}")]
    EntityTree(#[from] EntityTreeError),
    #[error("derived {0}")]
    Vfs(#[from] VfsError),
}

pub type DeclResult<T> = Result<T, DeclError>;

impl From<&DeclError> for DeclError {
    fn from(value: &DeclError) -> Self {
        todo!()
    }
}
