use husky_dev_utils::DevSource;
use husky_file::FileError;
use husky_text::TextRange;
use std::fmt::Write;
use std::{fmt::Formatter, sync::Arc};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EntitySyntaxError {
    pub kind: EntitySyntaxErrorKind,
    pub dev_src: DevSource,
    pub message: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EntitySyntaxErrorKind {
    Defn { range: TextRange },
    Query,
    Derived,
}

impl std::fmt::Display for EntitySyntaxError {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        todo!()
    }
}

impl HuskyDisplay for EntitySyntaxError {
    fn write_inherent(&self, config: husky_display_utils::HuskyDisplayConfig, result: &mut String) {
        if config.colored {
            todo!()
        } else {
            match self.kind {
                EntitySyntaxErrorKind::Defn { range } => {
                    write!(result, "Defn Error: {:?}({:?})", self.message, range).unwrap()
                }
                EntitySyntaxErrorKind::Query => {
                    write!(result, "Query Error: {:?}", self.message).unwrap()
                }
                EntitySyntaxErrorKind::Derived => {
                    write!(result, "Derived Error: {:?}", self.message).unwrap()
                }
            }
        }
    }
}

pub type EntitySyntaxResult<T> = Result<T, EntitySyntaxError>;
pub type EntitySyntaxResultArc<T> = Result<Arc<T>, EntitySyntaxError>;

impl From<FileError> for EntitySyntaxError {
    fn from(error: FileError) -> Self {
        EntitySyntaxError {
            kind: EntitySyntaxErrorKind::Derived,
            message: format!("{:?}", error.kind),
            dev_src: error.dev_src,
        }
    }
}

macro_rules! defn_error {
    ($msg: expr, $range: expr) => {{
        crate::EntitySyntaxError {
            kind: crate::error::EntitySyntaxErrorKind::Defn { range: $range },
            message: $msg.into(),
            dev_src: dev_src!(),
        }
    }};
}
pub(crate) use defn_error;

macro_rules! query_error {
    ($msg: expr) => {{
        crate::EntitySyntaxError {
            kind: crate::error::EntitySyntaxErrorKind::Query,
            message: $msg.into(),
            dev_src: dev_src!(),
        }
    }};
}
pub(crate) use query_error;

macro_rules! derived_error {
    ($msg: expr) => {{
        crate::EntitySyntaxError {
            kind: crate::error::EntitySyntaxErrorKind::Derived,
            message: $msg.into(),
            dev_src: dev_src!(),
        }
    }};
}
pub(crate) use derived_error;

macro_rules! query_not_none {
    ($result: expr, $msg: expr) => {{
        match $result {
            Some(value) => Ok(value),
            None => Err(query_error!($msg)),
        }
    }};
}
use husky_display_utils::HuskyDisplay;
use husky_token::LexError;
pub(crate) use query_not_none;
