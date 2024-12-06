#[derive(Debug, thiserror::Error)]
pub enum VdPipelineError {
    #[error("config parsing error: {0}")]
    ConfigParsing(String),
    #[error("IO error: {0}")]
    Io(String),
}

pub type VdPipelineResult<T> = Result<T, VdPipelineError>;
