use crate::{common::Content, *};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeminiResponse {
    pub candidates: Vec<Candidate>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Candidate {
    pub content: Content,
}

// Add new response types
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeminiErrorResponse {
    pub error: GeminiApiError,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeminiApiError {
    pub code: i32,
    pub message: String,
    pub status: String,
}

fn parse_response(response_bytes: &[u8]) -> GeminiResult<GeminiResponse> {
    // First try to parse as an error response
    if let Ok(error_response) = serde_json::from_slice::<GeminiErrorResponse>(response_bytes) {
        return Err(crate::error::GeminiError::ApiError {
            code: error_response.error.code,
            message: error_response.error.message,
            status: error_response.error.status,
        });
    }

    // If not an error, try to parse as success response
    serde_json::from_slice(response_bytes).map_err(|error| GeminiError::ResponseParseFailed {
        error,
        response_text: String::from_utf8_lossy(response_bytes).to_string(),
    })
}
