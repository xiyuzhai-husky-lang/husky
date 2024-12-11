#[derive(Debug, thiserror::Error)]
pub enum LlmError {}

pub type LlmResult<T> = Result<T, LlmError>;
