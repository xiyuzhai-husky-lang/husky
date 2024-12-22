//! Gemini API client implementation for interacting with Google's Gemini language model.

use self::common::*;

pub mod client {
    use crate::request::GeminiRequest;
    use crate::response::GeminiResponse;
    use crate::*;
    use reqwest::Client;

    pub struct GeminiClient {
        api_key: String,
        client: Client,
    }

    impl GeminiClient {
        pub fn new(api_key: impl Into<String>) -> Self {
            Self {
                api_key: api_key.into(),
                client: Client::new(),
            }
        }

        pub async fn generate_content(
            &self,
            prompt: impl Into<String>,
        ) -> Result<String, Box<dyn std::error::Error>> {
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
                .await?
                .json::<GeminiResponse>()
                .await?;

            Ok(response.candidates[0].content.parts[0].text.clone())
        }
    }
}

pub mod request {
    use crate::common::Content;
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct GeminiRequest {
        pub contents: Vec<Content>,
    }
}

pub mod response {
    use crate::common::Content;
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct GeminiResponse {
        pub candidates: Vec<Candidate>,
    }

    #[derive(Deserialize, Debug)]
    pub struct Candidate {
        pub content: Content,
    }
}

pub mod common {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Content {
        pub parts: Vec<Part>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Part {
        pub text: String,
    }
}
