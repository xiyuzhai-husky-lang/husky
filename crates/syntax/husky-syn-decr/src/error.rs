use husky_item_tree::EntityTreeError;
use husky_syn_expr::ExprError;
use husky_token::{TokenError, TokenIdx, TokenStreamState};
use husky_vfs::VfsError;
use original_error::IntoError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DecrError {
    #[error("{0}")]
    Original(#[from] OriginalDecrError),
    #[error("{0}")]
    Derived(#[from] DerivedDecrError),
}

impl From<VfsError> for DecrError {
    fn from(value: VfsError) -> Self {
        todo!()
    }
}

impl From<EntityTreeError> for DecrError {
    fn from(value: EntityTreeError) -> Self {
        todo!()
    }
}

impl From<TokenError> for DecrError {
    fn from(value: TokenError) -> Self {
        todo!()
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalDecrError {
    #[error("ExpectLeftBracketInDerive")]
    ExpectLeftBracketInDerive(TokenStreamState),
    #[error("ExpectClosingBracket")]
    ExpectRightBracketInDerive(TokenStreamState),
    #[error("")]
    ExprError(ExprError), // ad hoc
}

impl IntoError for OriginalDecrError {
    type Error = DecrError;
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedDecrError {
    #[error("InvalidParent")]
    InvalidParent,
}

pub type DecrResult<T> = Result<T, DecrError>;
pub type DecrResultRef<'a, T> = Result<T, &'a DecrError>;
