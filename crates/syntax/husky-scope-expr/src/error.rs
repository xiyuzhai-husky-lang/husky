use husky_token::{TokenIdx, TokenStreamState};
use husky_token_data::TokenDataError;
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
    ExpectedRightParenthesis(TokenStreamState),
    #[error("ExpectCrateOrSuper")]
    ExpectedCrateOrSuper(TokenStreamState),
}

impl OriginalError for OriginalVisibilityExprError {
    type Error = VisibilityExprError;
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedVisibilityExprError {
    #[error("{0}")]
    TokenDataError(#[from] TokenDataError),
}

pub type VisibilityExprResult<T> = Result<T, VisibilityExprError>;

impl From<TokenDataError> for VisibilityExprError {
    fn from(e: TokenDataError) -> Self {
        VisibilityExprError::Derived(e.into())
    }
}
