use std::sync::Arc;

#[derive(Clone, PartialEq, Eq)]
pub struct InferError {
    pub kind: InferErrorKind,
    pub dev_src: DevSource,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InferErrorKind {
    Derived,
    Original { message: String, range: TextRange },
}

impl std::fmt::Debug for InferError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.debug_struct("InferError")
        //     .field("message", &self.message)
        //     .field("src", &self.src)
        //     .finish()
        f.write_fmt(format_args!(
            "InferError:\n\
    src: {:?}\n\
    kind:\n\
{:?}",
            &self.dev_src, &self.kind
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
            kind: InferErrorKind::Original {
                message: format!("AstError {:?}", error),
                range: error.range,
            },
            dev_src: error.dev_src.clone(),
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
    ($msg:expr, $range: expr) => {{
        Err(InferError {
            kind: InferErrorKind::Original {
                message: $msg.into(),
                range: $range,
            },
            dev_src: dev_utils::dev_src!(),
        })?
    }};
}

#[macro_export]
macro_rules! ok_or {
    ($opt_value: expr, $msg:expr, $range: expr) => {{
        $opt_value.ok_or(InferError {
            kind: InferErrorKind::Original {
                message: $msg.into(),
                range: $range,
            },
            dev_src: dev_utils::dev_src!(),
        })
    }};
}

#[macro_export]
macro_rules! derived_not_none {
    ($opt_value: expr) => {{
        $opt_value.ok_or(InferError {
            kind: InferErrorKind::Derived,
            dev_src: dev_utils::dev_src!(),
        })
    }};
}

#[macro_export]
macro_rules! derived {
    () => {{
        InferError {
            kind: InferErrorKind::Derived,
            dev_src: dev_utils::dev_src!(),
        }
    }};
}

#[macro_export]
macro_rules! derived_ok {
    ($opt_value: expr) => {{
        $opt_value.or(Err(InferError {
            kind: InferErrorKind::Derived,
            dev_src: dev_utils::dev_src!(),
        }))?
    }};
}

use dev_utils::*;
use entity_route_query::ScopeError;
use text::TextRange;
use vm::VMError;
