use std::sync::Arc;

use husky_text::TextRange;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AstError {
    pub variant: AstErrorVariant,
    pub dev_src: DevSource,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AstErrorVariant {
    Original { message: String, range: TextRange },
    Derived,
}

impl AstError {
    pub fn message(&self) -> String {
        match self.variant {
            AstErrorVariant::Original { ref message, .. } => format!("Syntax Error: {}", message),
            AstErrorVariant::Derived => todo!(),
        }
    }
}

pub type AstResultArc<T> = Result<Arc<T>, AstError>;

impl From<AtomError> for AstError {
    fn from(error: AtomError) -> Self {
        Self {
            variant: match error.variant {
                AtomErrorVariant::Original { message, range } => {
                    AstErrorVariant::Original { message, range }
                }
                AtomErrorVariant::Derived => AstErrorVariant::Derived,
            },
            dev_src: error.dev_src,
        }
    }
}

pub type AstResult<T> = Result<T, AstError>;

macro_rules! error {
    ($message: expr, $range: expr) => {{
        AstError {
            variant: AstErrorVariant::Original {
                range: $range,
                message: $message.into(),
            },
            dev_src: dev_utils::dev_src!(),
        }
    }};
}
pub(crate) use error;

macro_rules! err {
    ($message: expr, $range:expr) => {{
        Err(error!($message, $range))
    }};
}
pub(crate) use err;

macro_rules! derived_err {
    () => {{
        Err(AstError {
            variant: AstErrorVariant::Derived,
            dev_src: dev_src!(),
        })
    }};
}
pub(crate) use derived_err;

macro_rules! derived_not_none {
    ($opt_value: expr) => {{
        $opt_value.ok_or(AstError {
            variant: AstErrorVariant::Derived,
            dev_src: dev_utils::dev_src!(),
        })
    }};
}
pub(crate) use derived_not_none;
