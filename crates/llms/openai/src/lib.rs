pub mod cap;
pub mod error;
pub mod request;
pub mod response;

use self::{error::*, request::*, response::*};
use cap::try_call_llm;
use lazy_static::lazy_static;
use llm_cache::LlmCache;
use llm_cap::LlmCap;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

struct OaiClient {
    cache: LlmCache<OaiRequest, OaiResponse>,
}

impl OaiClient {
    fn new(file_path: PathBuf) -> OaiResult<Self> {
        Ok(Self {
            cache: LlmCache::new(file_path)?,
        })
    }
}

impl OaiClient {
    pub fn complete_chat(&self, prompt: String) -> OaiResult<String> {
        match try_call_llm::<String>(prompt.len(), || todo!())? {
            Ok(_) => todo!(),
            Err(_) => todo!(),
        }
    }
}

#[test]
fn openai_client_works() {
    let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    assert!(cargo_manifest_dir
        .canonicalize()
        .unwrap()
        .ends_with("crates/llms/openai"));
    let cache_path = cargo_manifest_dir.join("cache/openai_client_works.json");
    assert!(cache_path.exists());

    let client = OaiClient::new(cache_path).unwrap();
    let result = client.complete_chat("Hello, world!".to_string());
    assert!(result.is_ok(), "{}", result.unwrap_err());
}
