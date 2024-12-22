use std::path::PathBuf;

use disk_cache::error::DiskCacheError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SglangError {
    #[error("DiskCache error: {0}")]
    DiskCache(#[from] DiskCacheError),
    #[error("Request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
    #[error("Failed to parse response: {error}. Response body: {response_text}")]
    ResponseParseFailed {
        error: serde_json::Error,
        response_text: String,
    },
    #[error("API error: {status} ({code}): {message}")]
    ApiError {
        code: i32,
        message: String,
        status: String,
    },
    #[error("Invalid cache directory: {0}")]
    InvalidCacheDir(PathBuf),
}

pub type SglangResult<T> = Result<T, SglangError>;
