use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticError {
    pub message: String,
    pub src: DevSource,
}

impl SemanticError {
    pub fn from_infer_error(error: InferError, src: DevSource) -> SemanticError {
        Self {
            message: error.to_string(),
            src,
        }
    }
}

pub type SemanticResult<T> = Result<T, SemanticError>;

pub type SemanticResultArc<T> = Result<Arc<T>, SemanticError>;

impl From<ScopeError> for SemanticError {
    fn from(error: ScopeError) -> Self {
        todo!()
        // Self {
        //     message: format!("ScopeError {:?}", error),
        //     src: error.src,
        // }
    }
}

// impl From<SyntaxError> for SemanticError {
//     fn from(error: SyntaxError) -> Self {
//         Self {
//             message: error.message,
//             src: error.src,
//         }
//     }
// }

impl From<&ast::AstError> for SemanticError {
    fn from(error: &ast::AstError) -> Self {
        Self {
            message: format!("AstError {:?}", error),
            src: error.src,
        }
    }
}

impl From<VMError> for SemanticError {
    fn from(_: VMError) -> Self {
        todo!()
    }
}

#[macro_export]
macro_rules! err {
    ($msg:expr) => {{
        Err(SemanticError {
            message: $msg.into(),
            src: common::src!(),
        })?
    }};
}

#[macro_export]
macro_rules! not_none {
    ($option:expr) => {{
        $option.ok_or(SemanticError {
            message: "expect not none".into(),
            src: common::src!(),
        })?
    }};
}

#[macro_export]
macro_rules! try_infer {
    ($syntax_result: expr) => {{
        $syntax_result.map_err(|e| SemanticError::from_infer_error(e, common::src!()))?
    }};
}

use common::DevSource;
use infer_error::InferError;
use scope_query::ScopeError;
use vm::VMError;
