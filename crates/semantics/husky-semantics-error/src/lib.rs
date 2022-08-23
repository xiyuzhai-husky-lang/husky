use husky_dev_utils::{dev_src, DevSource};
use husky_entity_syntax::EntitySyntaxError;
use husky_infer_error::InferError;
use husky_vm::VMCompileError;
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

#[macro_export]
macro_rules! err {
    ($msg:expr) => {{
        Err(SemanticError {
            variant: SemanticErrorVariant::Original {
                message: $msg.into(),
            },
            dev_src: husky_dev_utils::dev_src!(),
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
            dev_src: husky_dev_utils::dev_src!(),
        })?
    }};
}

#[macro_export]
macro_rules! derived_unwrap {
    ($syntax_result: expr) => {{
        $syntax_result.map_err(|e| SemanticError {
            variant: SemanticErrorVariant::Derived {
                message: format!("{:?}", e),
            },
            dev_src: husky_dev_utils::dev_src!(),
        })?
    }};
}
