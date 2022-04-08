use std::sync::Arc;

use file::FilePtr;
use text::TextRange;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AstError {
    pub file: Option<FilePtr>,
    pub range: TextRange,
    pub kind: AstErrorKind,
    pub src: DevSource,
}

impl AstError {
    pub fn message(&self) -> String {
        match self.kind {
            AstErrorKind::Message(ref message) => format!("Syntax Error: {}", message),
        }
    }
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
    ($file:expr, $range:expr, $kind: expr, $src: expr) => {{
        AstError {
            file: $file,
            range: $range,
            kind: $kind.into(),
            src: $src,
        }
    }};

    ($file:expr, $range:expr, $kind: expr) => {{
        error!($file, $range, $kind, src!())
    }};
}
pub(crate) use error;

macro_rules! err {
    ($file:expr, $range:expr, $kind: expr) => {{
        Err(error!($file, $range, $kind, src!()))
    }};
}
pub(crate) use err;
