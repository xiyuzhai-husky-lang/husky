use crate::request::GeminiRequest;
use crate::response::GeminiResponse;
use crate::*;
use reqwest::Client;

pub struct GeminiClient {
    api_key: String,
    client: Client,
}

impl GeminiClient {
    pub fn new() -> Self {
        let api_key = std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set");
        Self {
            api_key,
            client: Client::new(),
        }
    }

    fn parse_response(response_bytes: &[u8]) -> GeminiApiResult<GeminiResponse> {
        serde_json::from_slice(response_bytes).map_err(|error| {
            GeminiApiError::ResponseParseFailed {
                error,
                response_text: String::from_utf8_lossy(response_bytes).to_string(),
            }
        })
    }

    pub async fn generate_content(&self, prompt: impl Into<String>) -> GeminiApiResult<String> {
        let request = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part {
                    text: prompt.into(),
                }],
            }],
        };

        let response = self.client
            .post(format!(
                "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",
                self.api_key
            ))
            .json(&request)
            .send()
            .await?;

        let response_bytes = response.bytes().await?;
        let response = Self::parse_response(&response_bytes)?;

        Ok(response.candidates[0].content.parts[0].text.clone())
    }
}
