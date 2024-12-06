use crate::*;
use llm_cache::error::LlmCacheError;
use llm_cap::error::LlmCapError;

#[derive(Debug, thiserror::Error)]
pub enum OaiError {
    #[error("LlmCache error: {0}")]
    LlmCache(#[from] LlmCacheError),
    #[error("LlmCap error: {0}")]
    Cap(#[from] LlmCapError),
}

pub type OaiResult<T> = Result<T, OaiError>;
