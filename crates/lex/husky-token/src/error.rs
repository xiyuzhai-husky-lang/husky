use husky_dev_utils::DevSource;
use husky_text::TextRange;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum TokenError {
    #[error("incomplet string literal")]
    IncompleteStringLiteral,
    #[error("unexpect char after backslash")]
    UnexpectedCharAfterBackslash,
}

pub type TokenResult<T> = Result<T, TokenError>;
