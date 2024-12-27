#[derive(Debug, thiserror::Error)]
pub enum UsageCapError {
    #[error("LLM API calling is disabled for `{entity_name}` because the environment variable `{var_name}` is not set.")]
    LlmApiCallingDisabled {
        entity_name: &'static str,
        var_name: &'static str,
    },
    #[error("This is the final count down!")]
    FinalCountDown,
}

pub type LlmCapResult<T> = Result<T, UsageCapError>;
