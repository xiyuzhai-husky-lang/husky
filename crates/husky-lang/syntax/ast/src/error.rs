use std::sync::Arc;

use common::*;

use atom::{AtomError, AtomErrorKind};
use text::TextRange;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AstError {
    pub range: TextRange,
    pub kind: AstErrorKind,
    pub src: DevSource,
}

impl From<AtomError> for AstError {
    fn from(error: AtomError) -> Self {
        Self {
            range: error.range,
            kind: AstErrorKind::AtomError(error.kind),
            src: error.src,
        }
    }
}

impl From<ExprError> for AstError {
    fn from(error: ExprError) -> Self {
        Self {
            range: error.range,
            kind: AstErrorKind::ExprError(error.kind),
            src: error.src,
        }
    }
}

pub type AstResultArc<T> = Result<Arc<T>, AstError>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AstErrorKind {
    Message(String),
    AtomError(AtomErrorKind),
    ExprError(ExprErrorKind),
}

impl From<String> for AstErrorKind {
    fn from(msg: String) -> Self {
        Self::Message(msg)
    }
}

impl From<&'static str> for AstErrorKind {
    fn from(msg: &'static str) -> Self {
        Self::Message(msg.into())
    }
}

pub type AstResult<T> = Result<T, AstError>;

macro_rules! ast_error {
    ($range:expr,$kind: expr, $src: expr) => {{
        AstError {
            range: $range.clone(),
            kind: $kind.into(),
            src: $src,
        }
    }};

    ($range:expr,$kind: expr) => {{
        ast_error!($range, $kind, src!())
    }};
}
pub(crate) use ast_error;

macro_rules! ast_err {
    ($range:expr,$kind: expr, $src: expr) => {{
        Err(ast_error!($range, $kind, $src))
    }};

    ($range:expr,$kind: expr) => {{
        Err(ast_error!($range, $kind, src!()))
    }};
}
pub(crate) use ast_err;
