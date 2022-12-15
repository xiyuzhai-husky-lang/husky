use husky_dev_utils::DevSource;
use husky_text::TextRange;
use std::fmt::Write;
use std::{fmt::Formatter, sync::Arc};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EntityTreeError {
    pub kind: EntityTreeErrorKind,
    pub dev_src: DevSource,
    pub message: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EntityTreeErrorKind {
    Defn { range: TextRange },
    Query,
    Derived,
}

impl std::fmt::Display for EntityTreeError {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        todo!()
    }
}

impl HuskyDisplay for EntityTreeError {
    fn write_inherent(&self, config: husky_display_utils::HuskyDisplayConfig, result: &mut String) {
        if config.colored {
            todo!()
        } else {
            match self.kind {
                EntityTreeErrorKind::Defn { range } => {
                    write!(result, "Defn Error: {:?}({:?})", self.message, range).unwrap()
                }
                EntityTreeErrorKind::Query => {
                    write!(result, "Query Error: {:?}", self.message).unwrap()
                }
                EntityTreeErrorKind::Derived => {
                    write!(result, "Derived Error: {:?}", self.message).unwrap()
                }
            }
        }
    }
}

pub type EntityTreeResult<T> = Result<T, EntityTreeError>;
pub type EntityTreeResultArc<T> = Result<Arc<T>, EntityTreeError>;

macro_rules! defn_error {
    ($msg: expr, $range: expr) => {{
        crate::EntityTreeError {
            kind: crate::error::EntityTreeErrorKind::Defn { range: $range },
            message: $msg.into(),
            dev_src: dev_src!(),
        }
    }};
}
pub(crate) use defn_error;

macro_rules! query_error {
    ($msg: expr) => {{
        crate::EntityTreeError {
            kind: crate::error::EntityTreeErrorKind::Query,
            message: $msg.into(),
            dev_src: dev_src!(),
        }
    }};
}
pub(crate) use query_error;

macro_rules! derived_error {
    ($msg: expr) => {{
        crate::EntityTreeError {
            kind: crate::error::EntityTreeErrorKind::Derived,
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
pub(crate) use query_not_none;
