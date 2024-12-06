#[derive(Debug, thiserror::Error)]
pub enum LlmCapError {
    #[error("LLM API calling is disabled for {0}")]
    LlmApiCallingDisabled(&'static str),
    #[error("This is the final count down!")]
    FinalCountDown,
}

pub type LlmCapResult<T> = Result<T, LlmCapError>;
