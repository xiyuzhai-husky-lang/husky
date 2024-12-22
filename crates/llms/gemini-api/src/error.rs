use thiserror::Error;

#[derive(Debug, Error)]
pub enum GeminiApiError {
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
}

pub type GeminiApiResult<T> = Result<T, GeminiApiError>;
