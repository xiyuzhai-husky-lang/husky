use husky_ast::AstError;
use husky_text::BindTextRangeInto;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InferQueryError {
    pub kind: InferQueryErrorKind,
    pub message: String,
    pub dev_src: DevSource,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InferQueryErrorKind {
    Derived,
    Original,
}

pub type InferQueryResultArc<T> = Result<Arc<T>, InferQueryError>;
pub type InferQueryResult<T> = Result<T, InferQueryError>;

impl BindTextRangeInto<InferError> for InferQueryError {
    fn ref_bind_text_range_into(&self, range: TextRange) -> InferError {
        InferError {
            variant: match self.kind {
                InferQueryErrorKind::Derived => InferErrorVariant::Derived {
                    message: self.message.clone(),
                },
                InferQueryErrorKind::Original => InferErrorVariant::Original {
                    message: self.message.clone(),
                    range,
                },
            },
            dev_src: self.dev_src.clone(),
        }
    }

    fn bind_text_range_into(self, range: TextRange) -> InferError {
        InferError {
            variant: match self.kind {
                InferQueryErrorKind::Derived => InferErrorVariant::Derived {
                    message: self.message,
                },
                InferQueryErrorKind::Original => InferErrorVariant::Original {
                    message: self.message,
                    range,
                },
            },
            dev_src: self.dev_src,
        }
    }
}

impl From<InferError> for InferQueryError {
    fn from(e: InferError) -> Self {
        Self {
            kind: InferQueryErrorKind::Derived,
            message: match e.variant {
                InferErrorVariant::Derived { message } => message,
                InferErrorVariant::Original { message, .. } => message,
            },
            dev_src: e.dev_src,
        }
    }
}

impl From<EntitySyntaxError> for InferQueryError {
    fn from(e: EntitySyntaxError) -> Self {
        Self {
            kind: InferQueryErrorKind::Derived,
            message: e.message,
            dev_src: e.dev_src,
        }
    }
}

impl From<&AstError> for InferQueryError {
    fn from(e: &AstError) -> Self {
        Self {
            kind: InferQueryErrorKind::Derived,
            message: e.message(),
            dev_src: e.dev_src.clone(),
        }
    }
}

#[macro_export]
macro_rules! query_error {
    ($msg: expr) => {{
        InferQueryError {
            message: $msg,
            kind: InferQueryErrorKind::Original,
            dev_src: dev_utils::dev_src!(),
        }
    }};
}

#[macro_export]
macro_rules! query_derived_not_none {
    ($opt_value: expr) => {{
        $opt_value.ok_or(infer_error::InferQueryError {
            kind: infer_error::InferQueryErrorKind::Derived,
            message: "expect not none".to_string(),
            dev_src: dev_utils::dev_src!(),
        })
    }};
}

#[macro_export]
macro_rules! throw_query_derived {
    ($result: expr) => {{
        $result.map_err(|_| infer_error::InferQueryError {
            kind: infer_error::InferQueryErrorKind::Derived,
            message: "expect ok".to_string(),
            dev_src: dev_utils::dev_src!(),
        })?
    }};
}
