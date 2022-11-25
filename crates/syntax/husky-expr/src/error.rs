use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum ExprSyntaxError {}

pub type ExprSyntaxResult<T> = Result<T, ExprSyntaxError>;
