use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticError {
    pub variant: SemanticErrorVariant,
    pub dev_src: DevSource,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticErrorVariant {
    Derived { message: String },
    Original { message: String },
}

impl SemanticError {
    pub fn from_infer_error(error: InferError, dev_src: DevSource) -> SemanticError {
        Self {
            variant: SemanticErrorVariant::Derived {
                message: format!("{:?}", error),
            },
            dev_src,
        }
    }
}

impl From<InferError> for SemanticError {
    fn from(e: InferError) -> Self {
        Self {
            variant: SemanticErrorVariant::Derived {
                message: format!("{:?}", e),
            },
            dev_src: e.dev_src,
        }
    }
}

pub type SemanticResult<T> = Result<T, SemanticError>;

pub type SemanticResultArc<T> = Result<Arc<T>, SemanticError>;

pub type SemanticResultOptionArc<T> = Result<Option<Arc<T>>, SemanticError>;

impl From<EntitySyntaxError> for SemanticError {
    fn from(error: EntitySyntaxError) -> Self {
        Self {
            variant: SemanticErrorVariant::Derived {
                message: format!("Scope error: {:?}", &error),
            },
            dev_src: dev_src!(),
        }
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
            variant: SemanticErrorVariant::Derived {
                message: format!("{:?}", error),
            },
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
            variant: SemanticErrorVariant::Original {
                message: $msg.into(),
            },
            dev_src: dev_utils::dev_src!(),
        })?
    }};
}

#[macro_export]
macro_rules! not_none {
    ($option:expr) => {{
        $option.ok_or(SemanticError {
            variant: SemanticErrorVariant::Derived {
                message: "not none".into(),
            },
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

use dev_utils::{dev_src, DevSource};
use entity_route_query::EntitySyntaxError;
use infer_error::InferError;
use vm::VMError;
