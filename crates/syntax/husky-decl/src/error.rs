use husky_token::{TokenError, TokenIdx};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DeclError {
    #[error("token error")]
    Token(#[from] TokenError),
}

pub type DeclResult<T> = Result<T, DeclError>;
