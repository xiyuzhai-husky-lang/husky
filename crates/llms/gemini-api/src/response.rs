use crate::{common::Content, *};
use serde::Deserialize;
use thiserror::Error;

#[derive(Deserialize, Debug)]
pub struct GeminiResponse {
    pub candidates: Vec<Candidate>,
}

#[derive(Deserialize, Debug)]
pub struct Candidate {
    pub content: Content,
}

// Add new response types
#[derive(Deserialize, Debug)]
pub struct GeminiErrorResponse {
    pub error: GeminiError,
}

#[derive(Deserialize, Debug)]
pub struct GeminiError {
    pub code: i32,
    pub message: String,
    pub status: String,
}

fn parse_response(response_bytes: &[u8]) -> GeminiApiResult<GeminiResponse> {
    // First try to parse as an error response
    if let Ok(error_response) = serde_json::from_slice::<GeminiErrorResponse>(response_bytes) {
        return Err(GeminiApiError::ApiError {
            code: error_response.error.code,
            message: error_response.error.message,
            status: error_response.error.status,
        });
    }

    // If not an error, try to parse as success response
    serde_json::from_slice(response_bytes).map_err(|error| GeminiApiError::ResponseParseFailed {
        error,
        response_text: String::from_utf8_lossy(response_bytes).to_string(),
    })
}
