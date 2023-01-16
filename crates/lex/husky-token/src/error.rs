use crate::*;

use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
pub enum TokenError {
    #[error("incomplete string literal")]
    IncompleteStringLiteral,
    #[error("unexpect char after backslash")]
    UnexpectedCharAfterBackslash,
    #[error("unrecognized char")]
    UnrecognizedChar(char),
    #[error("ill-formed literal")]
    IllFormedLiteral(Literal),
    #[error("number pseudoliteral")]
    NumberPseudoLiteral(NumberPseudoLiteral),
    #[error("parse int error")]
    ParseIntError,
    #[error("invalid integer suffix")]
    InvalidIntegerSuffix,
    #[error("invalid identifier")]
    InvalidIdentifier,
}

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
pub enum NumberPseudoLiteral {}

pub type TokenResult<T> = Result<T, TokenError>;
