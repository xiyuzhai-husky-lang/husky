use super::*;
use disk_cache::error::LlmCacheError;

#[derive(Debug, thiserror::Error)]
pub enum SpacyConstituentParserError {
    #[error("Python error: {0}")]
    PyErr(#[from] PyErr),
    #[error("Python error: {0}")]
    PyErr2(String),
    #[error("Llm cache error: {0}")]
    LlmCacheError(#[from] LlmCacheError),
}

pub type SpacyConstituentParsingResult<T> = Result<T, SpacyConstituentParserError>;
