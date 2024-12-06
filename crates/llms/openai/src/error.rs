use crate::*;
use llm_cache::error::LlmCacheError;

#[derive(Debug, thiserror::Error)]
pub enum OaiError {
    #[error("LlmCache error: {0}")]
    LlmCache(#[from] LlmCacheError),
}

pub type OaiResult<T> = Result<T, OaiError>;
