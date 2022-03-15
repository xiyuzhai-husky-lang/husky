use std::sync::Arc;

use common::*;

use text::TextRange;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AstError {
    pub range: TextRange,
    pub kind: AstErrorKind,
    pub src: DevSource,
}

pub type AstResultArc<T> = Result<Arc<T>, AstError>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AstErrorKind {
    Message(String),
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

macro_rules! error {
    ($range:expr, $kind: expr, $src: expr) => {{
        AstError {
            range: $range,
            kind: $kind.into(),
            src: $src,
        }
    }};

    ($range:expr, $kind: expr) => {{
        error!($range, $kind, src!())
    }};
}
pub(crate) use error;

macro_rules! err {
    ($range:expr, $kind: expr) => {{
        Err(error!($range, $kind, src!()))
    }};
}
pub(crate) use err;
