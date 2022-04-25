pub(crate) mod def;

use std::sync::Arc;

pub(crate) use def::*;
use file::FileError;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EntityRouteError {
    FileError(FileError),
    DefError(def::EntityDefnError),
    Message(String),
    Derived,
}

pub type EntityRouteResult<T> = Result<T, EntityRouteError>;
pub type ScopeResultArc<T> = Result<Arc<T>, EntityRouteError>;

impl From<FileError> for EntityRouteError {
    fn from(error: FileError) -> Self {
        EntityRouteError::FileError(error)
    }
}

// impl From<LexError> for EntityRouteError {
//     fn from(error: LexError) -> Self {
//         EntityRouteError::Derived
//     }
// }

macro_rules! scope_err {
    ($msg: expr) => {{
        Err(crate::EntityRouteError::Message($msg))
    }};
}
pub(crate) use scope_err;

macro_rules! not_none {
    ($result: expr, $msg: expr) => {{
        match $result {
            Some(value) => Ok(value),
            None => Err(crate::EntityRouteError::Message($msg)),
        }
    }};
}
pub(crate) use not_none;
use token::LexError;
