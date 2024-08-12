use crate::value::Value;
use husky_value_interface::exception::IsException;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Exception {}

pub type ExceptedValue = Result<Value, Exception>;

impl IsException for Exception {}
