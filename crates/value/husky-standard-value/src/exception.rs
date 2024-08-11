use crate::Value;
use thiserror::Error;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum Exception {
    #[error("unwrap none")]
    UnwrapNone,
}

pub type ExceptedValue = Result<Value, Exception>;
