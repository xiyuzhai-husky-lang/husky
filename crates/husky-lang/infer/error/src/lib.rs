use std::sync::Arc;

#[derive(Clone, PartialEq, Eq)]
pub struct InferError {
    pub kind: InferErrorKind,
    pub src: DevSource,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InferErrorKind {
    Derived,
    Message(String),
}

impl std::fmt::Debug for InferError {
    fn fmt(&self, f: &mut common::Formatter<'_>) -> std::fmt::Result {
        // f.debug_struct("InferError")
        //     .field("message", &self.message)
        //     .field("src", &self.src)
        //     .finish()
        f.write_fmt(format_args!(
            "InferError:\n\
    src: {:?}\n\
    kind:\n\
{:?}",
            &self.src, &self.kind
        ))
    }
}

impl InferError {
    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

pub type InferResult<T> = Result<T, InferError>;

pub type InferResultArc<T> = Result<Arc<T>, InferError>;

impl From<ScopeError> for InferError {
    fn from(error: ScopeError) -> Self {
        todo!()
        // Self {
        //     message: format!("ScopeError {:?}", error),
        //     src: error.src,
        // }
    }
}

impl From<&ast::AstError> for InferError {
    fn from(error: &ast::AstError) -> Self {
        Self {
            kind: InferErrorKind::Message(format!("AstError {:?}", error)),
            src: error.src,
        }
    }
}

impl From<VMError> for InferError {
    fn from(_: VMError) -> Self {
        todo!()
    }
}

#[macro_export]
macro_rules! err {
    ($msg:expr) => {{
        Err(InferError {
            kind: InferErrorKind::Message($msg.into()),
            src: common::src!(),
        })?
    }};
}

#[macro_export]
macro_rules! ok_or {
    ($opt_value: expr, $msg:expr) => {{
        $opt_value.ok_or(InferError {
            kind: InferErrorKind::Message($msg.into()),
            src: common::src!(),
        })
    }};
}

#[macro_export]
macro_rules! not_none_or_derived {
    ($opt_value: expr) => {{
        $opt_value.ok_or(InferError {
            kind: InferErrorKind::Derived,
            src: common::src!(),
        })?
    }};
}

use common::DevSource;
use scope_query::ScopeError;
use vm::VMError;
