use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum ToolchainInferError {}

pub type ToolchainInferResult<T> = Result<T, ToolchainInferError>;
