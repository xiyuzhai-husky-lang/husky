#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum StmtError {}

pub type StmtResult<T> = Result<T, StmtError>;
