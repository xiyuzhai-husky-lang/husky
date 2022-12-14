use std::sync::Arc;

use crate::*;
use husky_text::TextRange;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum AstError {
    #[error("excessive indent")]
    ExcessiveIndent,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AstErrorVariant {
    Original { message: String, range: TextRange },
    Derived,
}

// impl AstError {
//     pub fn message(&self) -> String {
//         match self.variant {
//             AstErrorVariant::Original { ref message, .. } => format!("Syntax Error: {}", message),
//             AstErrorVariant::Derived => todo!(),
//         }
//     }
// }

pub type AstResultArc<T> = Result<Arc<T>, AstError>;

pub type AstResult<T> = Result<T, AstError>;

// macro_rules! error {
//     ($message: expr, $range: expr) => {{
//         AstError {
//             variant: AstErrorVariant::Original {
//                 range: $range,
//                 message: $message.into(),
//             },
//             dev_src: husky_dev_utils::dev_src!(),
//         }
//     }};
// }
// pub(crate) use error;

// macro_rules! err {
//     ($message: expr, $range:expr) => {{
//         Err(error!($message, $range))
//     }};
// }
// pub(crate) use err;

// macro_rules! derived_err {
//     () => {{
//         Err(AstError {
//             variant: AstErrorVariant::Derived,
//             dev_src: dev_src!(),
//         })
//     }};
// }
// pub(crate) use derived_err;

// macro_rules! derived_not_none {
//     ($opt_value: expr) => {{
//         $opt_value.ok_or(AstError {
//             variant: AstErrorVariant::Derived,
//             dev_src: husky_dev_utils::dev_src!(),
//         })
//     }};
// }
// pub(crate) use derived_not_none;
