use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticError {
    pub variant: SemanticErrorVariant,
    pub dev_src: DevSource,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticErrorVariant {
    Derived,
    Original,
}

impl SemanticError {
    pub fn from_infer_error(error: InferError, dev_src: DevSource) -> SemanticError {
        Self {
            variant: SemanticErrorVariant::Derived,
            dev_src,
        }
    }
}

impl From<InferError> for SemanticError {
    fn from(e: InferError) -> Self {
        Self {
            variant: SemanticErrorVariant::Derived,
            dev_src: e.dev_src,
        }
    }
}

pub type SemanticResult<T> = Result<T, SemanticError>;

pub type SemanticResultArc<T> = Result<Arc<T>, SemanticError>;

pub type SemanticResultOptionArc<T> = Result<Option<Arc<T>>, SemanticError>;

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
            variant: SemanticErrorVariant::Derived,
            dev_src: error.dev_src.clone(),
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
            variant: SemanticErrorVariant::Derived,
            dev_src: dev_utils::dev_src!(),
        })?
    }};
}

#[macro_export]
macro_rules! not_none {
    ($option:expr) => {{
        $option.ok_or(SemanticError {
            variant: SemanticErrorVariant::Derived,
            dev_src: dev_utils::dev_src!(),
        })?
    }};
}

#[macro_export]
macro_rules! try_infer {
    ($syntax_result: expr) => {{
        $syntax_result.map_err(|e| SemanticError::from_infer_error(e, dev_utils::dev_src!()))?
    }};
}

use dev_utils::DevSource;
use entity_route_query::ScopeError;
use infer_error::InferError;
use vm::VMError;
