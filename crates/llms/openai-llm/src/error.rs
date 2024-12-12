use crate::*;
use disk_cache::error::LlmCacheError;
use usage_cap::error::LlmCapError;

#[derive(Debug, thiserror::Error)]
pub enum OaiError {
    #[error("LlmCache error: {0}")]
    LlmCache(#[from] LlmCacheError),
    #[error("LlmCap error: {0}")]
    Cap(#[from] LlmCapError),
    #[error("Environment variable OPENAI_API_KEY not set")]
    EnvApiKeyNotSet,
    #[error("OpenAI client error: {0}")]
    Ext(#[from] Box<dyn std::error::Error>),
    #[error("No choices returned")]
    NoChoicesReturned,
    #[error("OpenAI chat completion error")]
    ExtChatCompletion,
}

pub type OaiResult<T> = Result<T, OaiError>;
