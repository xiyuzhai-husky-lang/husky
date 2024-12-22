use crate::{request::GeminiRequest, response::GeminiResponse, *};
use disk_cache::DiskCache;
use eterned::db::EternerDb;
use reqwest::Client;
use std::path::PathBuf;
use usage_cap::UsageCap;

pub struct GeminiClient<'db> {
    cache: DiskCache<&'db EternerDb, GeminiRequest, GeminiResponse>,
    api_key: String,
    client: Client,
}

impl<'db> GeminiClient<'db> {
    pub fn new(db: &'db EternerDb, file_path: PathBuf) -> GeminiResult<Self> {
        let api_key = std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set");
        Ok(Self {
            cache: DiskCache::new(db, file_path)?,
            api_key,
            client: Client::new(),
        })
    }

    fn parse_response(response_bytes: &[u8]) -> GeminiResult<GeminiResponse> {
        serde_json::from_slice(response_bytes).map_err(|error| GeminiError::ResponseParseFailed {
            error,
            response_text: String::from_utf8_lossy(response_bytes).to_string(),
        })
    }

    pub fn generate_content(&self, prompt: impl Into<String>) -> GeminiResult<String> {
        let prompt = prompt.into();
        let min_usage = prompt.len();
        let request = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part { text: prompt }],
            }],
        };
        let response =
            self.cache
                .get_or_call(request, |request| -> GeminiResult<GeminiResponse> {
                    match try_call_gemini::<GeminiResult<GeminiResponse>>(min_usage, || {
                        let rt = tokio::runtime::Runtime::new().unwrap();
                        rt.block_on(self.generate_content_aux(request))
                    })? {
                        Ok(result) => match result {
                            Ok(s) => Ok(s),
                            Err(e) => Err(todo!()),
                        },
                        Err(e) => todo!(),
                    }
                })?;

        Ok(response.candidates[0].content.parts[0].text.clone())
    }

    async fn generate_content_aux(
        &self,
        request: &GeminiRequest,
    ) -> (usize, GeminiResult<GeminiResponse>) {
        let mut usage = request.contents[0].parts[0].text.len();
        let response = match self.client
            .post(format!(
                "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",
                self.api_key
            ))
            .json(&request)
            .send()
            .await {
                Ok(resp) => resp,
                Err(e) => return (usage, Err(e.into())),
            };

        let response_bytes = match response.bytes().await {
            Ok(bytes) => {
                usage += POST_CALL_USAGE_MULTIPLIER * bytes.len();
                bytes
            }
            Err(e) => return (usage, Err(e.into())),
        };

        match Self::parse_response(&response_bytes) {
            Ok(resp) => {
                usage +=
                    POST_CALL_USAGE_MULTIPLIER * resp.candidates[0].content.parts[0].text.len();
                (usage, Ok(resp))
            }
            Err(e) => (usage, Err(e)),
        }
    }
}
