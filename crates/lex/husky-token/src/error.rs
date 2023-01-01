use crate::*;
use husky_dev_utils::DevSource;
use husky_text::TextRange;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum TokenError {
    #[error("incomplet string literal")]
    IncompleteStringLiteral,
    #[error("unexpect char after backslash")]
    UnexpectedCharAfterBackslash,
    #[error("unrecognized char")]
    UnrecognizedChar(char),
    #[error("ill-formed literal")]
    IllFormedLiteral(Literal),
    #[error("number pseudoliteral")]
    NumberPseudoLiteral(NumberPseudoLiteral),
}

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
pub enum NumberPseudoLiteral {}

pub type TokenResult<T> = Result<T, TokenError>;
