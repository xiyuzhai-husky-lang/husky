use crate::{raw::GeminiRawContent, GeminiRawPart};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct GeminiRawRequest {
    pub contents: Vec<GeminiRawContent>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, Hash)]
pub enum GeminiRequest {
    TextGeneration { input: String },
}

impl Into<GeminiRawRequest> for &GeminiRequest {
    fn into(self) -> GeminiRawRequest {
        match self {
            GeminiRequest::TextGeneration { input } => GeminiRawRequest {
                contents: vec![GeminiRawContent {
                    parts: vec![GeminiRawPart {
                        text: input.clone(),
                    }],
                }],
            },
        }
    }
}

impl GeminiRequest {
    pub fn min_usage(&self) -> usize {
        match self {
            GeminiRequest::TextGeneration { input } => input.len(),
        }
    }
}
