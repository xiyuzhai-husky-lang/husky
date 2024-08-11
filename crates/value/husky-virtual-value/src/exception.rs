use crate::value::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Exception {}

pub type ExceptedValue = Result<Value, Exception>;
