use openai_llm::error::OaiError;

#[derive(Debug, thiserror::Error)]
pub enum AnyLlmError {
    #[error("OpenAI error: {0}")]
    OpenAi(#[from] OaiError),
}

pub type AnyLlmResult<T> = Result<T, AnyLlmError>;
