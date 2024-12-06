pub mod error;
pub mod request;
pub mod response;

use self::{error::*, request::*, response::*};
use lazy_static::lazy_static;
use llm_cache::LlmCache;
use llm_cap::LlmCap;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

struct OaiClient {
    lock: LlmCap,
    cache: LlmCache<OaiRequest, OaiResponse>,
}

impl OaiClient {
    fn new(file_path: PathBuf) -> OaiResult<Self> {
        Ok(Self {
            lock: LlmCap::new("OpenAI", "ENABLE_OPENAI_API_CALLING"),
            cache: LlmCache::new(file_path)?,
        })
    }
}

lazy_static! {
    static ref CLIENT: OaiClient = OaiClient::new(PathBuf::from("openai_cache.json")).unwrap();
}
