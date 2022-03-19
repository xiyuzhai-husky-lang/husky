use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxError {
    pub message: String,
    pub src: DevSource,
}

pub type SyntaxResult<T> = Result<T, SyntaxError>;

pub type SyntaxResultArc<T> = Result<Arc<T>, SyntaxError>;

impl From<ScopeError> for SyntaxError {
    fn from(error: ScopeError) -> Self {
        todo!()
        // Self {
        //     message: format!("ScopeError {:?}", error),
        //     src: error.src,
        // }
    }
}

impl From<&ast::AstError> for SyntaxError {
    fn from(error: &ast::AstError) -> Self {
        Self {
            message: format!("AstError {:?}", error),
            src: error.src,
        }
    }
}

impl From<VMError> for SyntaxError {
    fn from(_: VMError) -> Self {
        todo!()
    }
}

#[macro_export]
macro_rules! err {
    ($msg:expr) => {{
        Err(SyntaxError {
            message: $msg.into(),
            src: common::src!(),
        })?
    }};
}

#[macro_export]
macro_rules! not_none {
    ($option:expr) => {{
        $option.ok_or(SyntaxError {
            message: "expect not none".into(),
            src: common::src!(),
        })?
    }};
}

use common::DevSource;
use scope_query::ScopeError;
use vm::VMError;
