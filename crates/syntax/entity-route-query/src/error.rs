pub(crate) mod def;

use std::{fmt::Formatter, sync::Arc};

pub(crate) use def::*;
use dev_utils::{dev_src, DevSource};
use file::FileError;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EntityRouteError {
    pub variant: EntityRouteErrorVariant,
    pub dev_src: DevSource,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EntityRouteErrorVariant {
    FileError(FileError),
    DefError(def::EntityDefnError),
    Message(String),
    Derived,
}

impl std::fmt::Display for EntityRouteError {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        todo!()
    }
}

pub type EntityRouteResult<T> = Result<T, EntityRouteError>;
pub type EntityRouteResultArc<T> = Result<Arc<T>, EntityRouteError>;

impl From<FileError> for EntityRouteError {
    fn from(error: FileError) -> Self {
        EntityRouteError {
            variant: EntityRouteErrorVariant::FileError(error),
            dev_src: dev_src!(),
        }
    }
}

macro_rules! err {
    ($msg: expr) => {{
        Err(crate::EntityRouteError {
            variant: crate::error::EntityRouteErrorVariant::Message($msg),
            dev_src: dev_src!(),
        })
    }};
}
pub(crate) use err;

macro_rules! not_none {
    ($result: expr, $msg: expr) => {{
        match $result {
            Some(value) => Ok(value),
            None => err!($msg),
        }
    }};
}
pub(crate) use not_none;
use token::LexError;
