pub mod cap;
pub mod error;
mod ext;
pub mod request;
pub mod response;

use self::{error::*, request::*, response::*};
use cap::try_call_llm;
use lazy_static::lazy_static;
use llm_cache::LlmCache;
use llm_cap::LlmCap;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub struct OaiClient {
    cache: LlmCache<OaiRequest, OaiResponse>,
    /// None if the environment variable `OPENAI_API_KEY` is not set.
    client_ext: Option<ext::OpenAIClient>,
}

impl OaiClient {
    pub fn new(file_path: PathBuf) -> OaiResult<Self> {
        let api_key = std::env::var("OPENAI_API_KEY").ok();
        Ok(Self {
            cache: LlmCache::new(file_path)?,
            client_ext: match api_key {
                Some(api_key) => Some(ext::OpenAIClient::builder().with_api_key(api_key).build()?),
                None => None,
            },
        })
    }
}

impl OaiClient {
    pub fn complete_chat(&self, prompt: String) -> OaiResult<String> {
        let min_usage = prompt.len();
        let request = OaiRequest::ChatCompletion(prompt);
        let OaiResponse::ChatCompletion(response) =
            self.cache
                .get_or_call(request, |request| -> OaiResult<OaiResponse> {
                    match try_call_llm::<OaiResult<String>>(min_usage, || {
                        self.complete_chat_aux(request)
                    })? {
                        Ok(result) => match result {
                            Ok(s) => Ok(OaiResponse::ChatCompletion(s)),
                            Err(e) => Err(todo!()),
                        },
                        Err(e) => todo!(),
                    }
                })?;
        Ok(response)
    }

    fn complete_chat_aux(&self, request: &OaiRequest) -> (usize, OaiResult<String>) {
        let OaiRequestExt::ChatCompletion(request_ext) = request.ext() else {
            unreachable!()
        };
        match self.complete_chat_ext(request_ext) {
            Ok(content) => (content.len(), Ok(content)),
            Err(e) => (0, Err(e)), // ad hoc
        }
    }
}

#[test]
fn openai_client_works() {
    let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    assert!(cargo_manifest_dir
        .canonicalize()
        .unwrap()
        .ends_with("crates/llms/openai-llm"));
    let cache_path = cargo_manifest_dir.join("cache/openai_client_works.json");
    assert!(cache_path.exists());

    let client = OaiClient::new(cache_path).unwrap();
    let result = client.complete_chat("Hello, world!".to_string());
    assert!(result.is_ok(), "{}", result.unwrap_err());
}
