use crate::{request::GeminiRawRequest, response::GeminiRawResponse, *};
use disk_cache::DiskCache;
use enum_index::full_map::EnumFullVecMap;
use eterned::db::EternerDb;
use model::GeminiModel;
use request::GeminiRequest;
use reqwest::Client;
use response::{parse_response_result, GeminiResponse};
use std::path::{Path, PathBuf};
use usage_cap::UsageCap;

const DEFAULT_RETRY_DELAY: std::time::Duration = std::time::Duration::from_secs(30);

pub struct GeminiClient<'db> {
    caches: EnumFullVecMap<GeminiModel, DiskCache<&'db EternerDb, GeminiRequest, GeminiResponse>>,
    api_key: String,
    client: Client,
    retry_delay: std::time::Duration,
}

impl<'db> GeminiClient<'db> {
    pub fn new(db: &'db EternerDb, cache_dir: &Path) -> GeminiResult<Self> {
        let api_key = std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set");
        let caches = EnumFullVecMap::try_new(|model: GeminiModel| {
            if !cache_dir.is_dir() {
                return Err(GeminiError::InvalidCacheDir(cache_dir.to_owned()));
            }
            DiskCache::new(db, cache_dir.join(model.as_str())).map_err(Into::into)
        })?;

        Ok(Self {
            caches,
            api_key,
            client: Client::new(),
            retry_delay: DEFAULT_RETRY_DELAY,
        })
    }

    pub fn generate_text(
        &self,
        model: GeminiModel,
        input: impl Into<String>,
    ) -> GeminiResult<String> {
        match self.generate(
            model,
            GeminiRequest::TextGeneration {
                input: input.into(),
            },
        )? {
            GeminiResponse::TextGeneration { text } => Ok(text),
        }
    }

    pub fn generate(
        &self,
        model: GeminiModel,
        request: GeminiRequest,
    ) -> GeminiResult<GeminiResponse> {
        let min_usage = request.min_usage();
        let response =
            self.caches[model].get_or_call(request, |request| -> GeminiResult<GeminiResponse> {
                match try_call_gemini::<GeminiResult<GeminiResponse>>(min_usage, || {
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    rt.block_on(self.generate_aux(model, request))
                })? {
                    Ok(result) => match result {
                        Ok(s) => Ok(s),
                        Err(e) => Err(e),
                    },
                    Err(e) => todo!(),
                }
            })?;

        Ok(response)
    }

    async fn generate_aux(
        &self,
        model: GeminiModel,
        request: &GeminiRequest,
    ) -> (usize, GeminiResult<GeminiResponse>) {
        loop {
            match self.generate_step(model, request).await {
                Some(result) => return result,
                None => continue,
            }
        }
    }

    async fn generate_step(
        &self,
        model: GeminiModel,
        request: &GeminiRequest,
    ) -> Option<(usize, GeminiResult<GeminiResponse>)> {
        let mut usage = 0;
        let raw_request: GeminiRawRequest = request.into();
        let response = match self
            .client
            .post(format!(
                "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
                model.url_name(),
                self.api_key
            ))
            .json(&raw_request)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => return Some((usage, Err(e.into()))),
        };

        let response_bytes = match response.bytes().await {
            Ok(bytes) => {
                usage += POST_CALL_USAGE_MULTIPLIER * bytes.len();
                bytes
            }
            Err(e) => return Some((usage, Err(e.into()))),
        };

        match parse_response_result(&response_bytes) {
            Ok(resp_result) => match resp_result {
                Ok(resp) => {
                    usage +=
                        POST_CALL_USAGE_MULTIPLIER * resp.candidates[0].content.parts[0].text.len();
                    Some((usage, Ok((resp, request).into())))
                }
                Err(e) => match e.error.status.as_str() {
                    "RESOURCE_EXHAUSTED" => {
                        tracing::info!("RESOURCE_EXHAUSTED, retrying in {:?}...", self.retry_delay);
                        tokio::time::sleep(tokio::time::Duration::from(self.retry_delay)).await;
                        None
                    }
                    _ => todo!(),
                },
            },
            Err(e) => Some((usage, Err(e))),
        }
    }
}
