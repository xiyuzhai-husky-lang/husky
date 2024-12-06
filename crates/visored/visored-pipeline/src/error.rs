use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum VdPipelineError {
    #[error("config parsing error: {0}")]
    ConfigParsing(String),
    #[error("IO error for file: {0}, error: {1}")]
    Io(PathBuf, std::io::Error),
}

pub type VdPipelineResult<T> = Result<T, VdPipelineError>;
