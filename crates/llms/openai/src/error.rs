use crate::*;
use disk_cache::error::DiskCacheError;
use usage_cap::error::UsageCapError;

#[derive(Debug, thiserror::Error)]
pub enum OpenaiError {
    #[error("DiskCache error: {0}")]
    DiskCache(#[from] DiskCacheError),
    #[error("UsageCap error: {0}")]
    UsageCap(#[from] UsageCapError),
    #[error("Environment variable OPENAI_API_KEY not set")]
    EnvApiKeyNotSet,
    #[error("OpenAI client error: {0}")]
    Ext(#[from] Box<dyn std::error::Error>),
    #[error("No choices returned")]
    NoChoicesReturned,
    #[error("OpenAI chat completion error")]
    ExtChatCompletion,
}

pub type OpenaiResult<T> = Result<T, OpenaiError>;
