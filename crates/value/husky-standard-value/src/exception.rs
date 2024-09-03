use crate::Value;
use husky_value::exception::IsException;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Exception {
    #[error("unwrap none")]
    UnwrapNone,
}

pub type Excepted<T> = Result<T, Exception>;
pub type ExceptedValue = Result<Value, Exception>;

impl IsException for Exception {}
