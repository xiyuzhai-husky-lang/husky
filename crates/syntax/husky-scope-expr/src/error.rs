use husky_token::{TokenError, TokenIdx};
use original_error::OriginalError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum VisibilityExprError {
    #[error("{0}")]
    Original(#[from] OriginalVisibilityExprError),
    #[error("{0}")]
    Derived(#[from] DerivedVisibilityExprError),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalVisibilityExprError {
    #[error("NoSuperForRoot")]
    NoSuperForRoot(TokenIdx),
    #[error("ExpectRightParenthesis")]
    ExpectRightParenthesis(TokenIdx),
    #[error("ExpectCrateOrSuper")]
    ExpectCrateOrSuper(TokenIdx),
}

impl OriginalError for OriginalVisibilityExprError {
    type Error = VisibilityExprError;
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedVisibilityExprError {
    #[error("{0}")]
    TokenError(#[from] TokenError),
}

pub type VisibilityExprResult<T> = Result<T, VisibilityExprError>;

impl From<TokenError> for VisibilityExprError {
    fn from(e: TokenError) -> Self {
        VisibilityExprError::Derived(e.into())
    }
}
