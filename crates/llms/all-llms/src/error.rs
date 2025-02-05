use gemini::error::GeminiError;
use openai::error::OpenaiError;

#[derive(Debug, thiserror::Error)]
pub enum AnyLlmError {
    #[error("OpenAI error: {0}")]
    OpenAi(#[from] OpenaiError),
    #[error("Gemini error: {0}")]
    Gemini(#[from] GeminiError),
    #[error("Invalid cache dir: {0}")]
    InvalidCacheDir(#[from] std::io::Error),
}

pub type AllLlmsResult<T> = Result<T, AnyLlmError>;
