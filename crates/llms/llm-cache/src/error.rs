use std::{io, path::PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum LlmCacheError {
    #[error("io error: {0}")]
    Io(PathBuf, io::Error),
    #[error("cache file is locked by another process")]
    CacheFileLockedByAnotherProcess,
}

pub type LlmCacheResult<T> = Result<T, LlmCacheError>;
