#[derive(Debug, thiserror::Error)]
pub enum LlmLockError {
    #[error("LLM API calling is disabled")]
    LlmApiCallingDisabled,
    #[error("This is the final count down!")]
    FinalCountDown,
}
