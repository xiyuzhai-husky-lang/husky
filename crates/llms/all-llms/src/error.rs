use openai_api::error::OaiError;

#[derive(Debug, thiserror::Error)]
pub enum AnyLlmError {
    #[error("OpenAI error: {0}")]
    OpenAi(#[from] OaiError),
}

pub type AllLlmsResult<T> = Result<T, AnyLlmError>;
